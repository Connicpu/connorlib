use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use std::io::{Read, Write};
use std::{io, thread};
use std::ffi::CString;
use std::fs::{self, File};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use libc;

fn make_server(read_path: &str, write_path: &str) -> io::Result<(File, File)> {
    fs::remove_file(read_path).ok();
    fs::remove_file(write_path).ok();

    let c_read_path = CString::new(read_path).unwrap();
    let c_write_path = CString::new(write_path).unwrap();
    unsafe {
        use libc::*;
        if mkfifo(c_read_path.as_ptr(), S_IWUSR|S_IWGRP|S_IRUSR|S_IRGRP) == -1 {
            return Err(io::Error::last_os_error());
        }
        if mkfifo(c_write_path.as_ptr(), S_IWUSR|S_IWGRP|S_IRUSR|S_IRGRP) == -1 {
            return Err(io::Error::last_os_error());
        }
    }

    make_client(read_path, write_path)
}

fn make_client(read_path: &str, write_path: &str) -> io::Result<(File, File)> {
    Ok((try!(File::open(read_path)), try!(fs::OpenOptions::new().write(true).read(false).open(write_path))))
}

fn run<F: FnOnce() -> io::Result<()> + Send + 'static>(f: F) {
    thread::spawn(f);
}

pub struct IpcClient {
    send: Sender<Vec<u8>>,
    recv: Receiver<Vec<u8>>,
}

impl IpcClient {
    pub fn send(&self, message: Vec<u8>) -> bool {
        self.send.send(message).is_ok()
    }

    pub fn recv(&self) -> Option<Vec<u8>> {
        self.recv.recv().ok()
    }

    pub fn try_recv(&self) -> Option<Option<Vec<u8>>> {
        match self.recv.try_recv() {
            Ok(buf) => Some(Some(buf)),
            Err(TryRecvError::Empty) => Some(None),
            Err(TryRecvError::Disconnected) => None
        }
    }

    pub fn open_server(name: &str) -> io::Result<IpcClient> {
        let (send_tx, send_rx) = channel::<Vec<u8>>();
        let (recv_tx, recv_rx) = channel::<Vec<u8>>();

        let pid = unsafe { libc::getpid() as u32 };
        let read_path = format!("/tmp/messageipc_{}_{}_toserver", name, pid);
        let write_path = format!("/tmp/messageipc_{}_{}_toclient", name, pid);

        let (mut read_server, mut write_server) = try!(make_server(&read_path, &write_path));

        // Read thread
        run(move || {
            loop {
                let bytes = try!(read_server.read_u32::<LittleEndian>());
                let mut buffer = vec![0; bytes as usize];
                try!(read_server.read_exact(&mut buffer[..]));

                if let Err(_) = recv_tx.send(buffer) {
                    return Ok(());
                }
            }
        });

        // Write thread
        run(move || {
            while let Ok(buffer) = send_rx.recv() {
                let size = buffer.len() as u32;
                try!(write_server.write_u32::<LittleEndian>(size));
                try!(write_server.write_all(&buffer[..]));
            }

            Ok(())
        });

        Ok(IpcClient {
            send: send_tx,
            recv: recv_rx,
        })
    }
    
    pub fn open_client(name: &str, pid: u32) -> io::Result<IpcClient> {
        let (send_tx, send_rx) = channel::<Vec<u8>>();
        let (recv_tx, recv_rx) = channel::<Vec<u8>>();

        let read_path = format!("/tmp/messageipc_{}_{}_toclient", name, pid);
        let write_path = format!("/tmp/messageipc_{}_{}_toserver", name, pid);

        let (mut read_client, mut write_client) = try!(make_client(&read_path, &write_path));

        // Read thread
        run(move || {
            loop {
                let bytes = try!(read_client.read_u32::<LittleEndian>());
                let mut buffer = vec![0; bytes as usize];
                try!(read_client.read_exact(&mut buffer[..]));

                if let Err(_) = recv_tx.send(buffer) {
                    return Ok(());
                }
            }
        });

        // Write thread
        run(move || {
            while let Ok(buffer) = send_rx.recv() {
                let size = buffer.len() as u32;
                try!(write_client.write_u32::<LittleEndian>(size));
                try!(write_client.write_all(&buffer[..]));
            }

            Ok(())
        });

        Ok(IpcClient {
            send: send_tx,
            recv: recv_rx,
        })
    }
}
