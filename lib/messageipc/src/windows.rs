use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use std::io::{Read, Write};
use std::{io, thread};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use named_pipe::{PipeOptions, OpenMode, PipeClient};
use libc;

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
        let read_path = format!("\\\\.\\pipe\\messageipc_{}_{}_toserver", name, pid);
        let write_path = format!("\\\\.\\pipe\\messageipc_{}_{}_toclient", name, pid);

        // Read thread
        run(move || {
            let read_server = try!(PipeOptions::new(read_path).open_mode(OpenMode::Read).single());
            let mut read_server = try!(read_server.wait());

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
            let write_server = try!(PipeOptions::new(write_path).open_mode(OpenMode::Write).single());
            let mut write_server = try!(write_server.wait());
            
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

        let read_path = format!("\\\\.\\pipe\\messageipc_{}_{}_toclient", name, pid);
        let write_path = format!("\\\\.\\pipe\\messageipc_{}_{}_toserver", name, pid);

        // Read thread
        run(move || {
            let mut read_client = try!(PipeClient::connect(read_path));

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
            let mut write_client = try!(PipeClient::connect(write_path));
            
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
