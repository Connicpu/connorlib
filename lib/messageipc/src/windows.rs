use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError, sync_channel, SyncSender};
use std::io::{Read, Write};
use std::{io, thread};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use named_pipe::{PipeOptions, OpenMode, PipeClient};
use libc;

fn run<F: FnOnce(&SyncSender<()>) -> io::Result<()> + Send + 'static>(_: &'static str, f: F) {
    let (sync_tx, sync_rx) = sync_channel(0);
    thread::spawn(move || {
        match f(&sync_tx) {
            Ok(_) => {},
            Err(_) => {
                sync_tx.try_send(()).ok();
                //println!("MIPC {} thread failed with {:?}", name, e);
            }
        };
    });
    sync_rx.recv().unwrap();
}

pub struct IpcClient {
    send: Sender<Vec<u8>>,
    recv: Receiver<Vec<u8>>,
}

struct S<T>(T);
unsafe impl<T> Send for S<T> {}

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
        let path = format!("\\\\.\\pipe\\messageipc_{}_{}", name, pid);

        let mut servers = try!(PipeOptions::new(path).open_mode(OpenMode::Duplex).multiple(2));
        let read_server = S(servers.pop().unwrap());
        let write_server = S(servers.pop().unwrap());

        // Read thread
        run("server-read", move |sync| {
            sync.send(()).unwrap();

            let mut read_server = try!(read_server.0.wait());
            
            // Write thread
            run("server-write", move |sync| {
                let mut write_server = try!(write_server.0.wait());
                sync.send(()).unwrap();
                
                while let Ok(buffer) = send_rx.recv() {
                    let size = buffer.len() as u32;
                    try!(write_server.write_u32::<LittleEndian>(size));
                    try!(write_server.write_all(&buffer[..]));
                }

                Ok(())
            });

            loop {
                let bytes = try!(read_server.read_u32::<LittleEndian>());
                let mut buffer = vec![0; bytes as usize];
                try!(read_server.read_exact(&mut buffer[..]));

                if let Err(_) = recv_tx.send(buffer) {
                    return Ok(());
                }
            }
        });

        Ok(IpcClient {
            send: send_tx,
            recv: recv_rx,
        })
    }
    
    pub fn open_client(name: &str, pid: u32) -> io::Result<IpcClient> {
        let (send_tx, send_rx) = channel::<Vec<u8>>();
        let (recv_tx, recv_rx) = channel::<Vec<u8>>();

        let read_path = format!("\\\\.\\pipe\\messageipc_{}_{}", name, pid);
        let write_path = format!("\\\\.\\pipe\\messageipc_{}_{}", name, pid);

        // Read thread
        run("client-read", move |sync| {
            let mut read_client = try!(PipeClient::connect(read_path));

            // Write thread
            run("client-write", move |sync| {
                let mut write_client = try!(PipeClient::connect(write_path));
                sync.send(()).unwrap();
                
                while let Ok(buffer) = send_rx.recv() {
                    let size = buffer.len() as u32;
                    try!(write_client.write_u32::<LittleEndian>(size));
                    try!(write_client.write_all(&buffer[..]));
                }

                Ok(())
            });

            sync.send(()).unwrap();

            loop {
                let bytes = try!(read_client.read_u32::<LittleEndian>());
                let mut buffer = vec![0; bytes as usize];
                try!(read_client.read_exact(&mut buffer[..]));

                if let Err(_) = recv_tx.send(buffer) {
                    return Ok(());
                }
            }
        });

        Ok(IpcClient {
            send: send_tx,
            recv: recv_rx,
        })
    }
}
