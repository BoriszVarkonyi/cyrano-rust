use std::thread::JoinHandle;
use std::thread::spawn;
use std::{io, net::UdpSocket};

//own imports
use crate::com::handle_message;

pub fn send_message(
    socket: &UdpSocket,
    ip_address_port: String,
    buffer: String,
) -> io::Result<()> {
    {

        let buf = buffer.as_bytes();

        socket
            .send_to(buf, &ip_address_port)
            .map_err(|e| io::Error::new(e.kind(), format!("send_to({ip_address_port}) failed: {e}")))?;
    }
    Ok(())
}

pub fn start_listener(socket: UdpSocket) -> JoinHandle<()> {
    spawn(move || {
        let mut buf = [0u8; 2048];

        loop {
            match socket.recv_from(&mut buf) {
                Ok((size, src)) => {
                    let msg = String::from_utf8_lossy(&buf[..size]);
                    println!("[RECV from {}] {}", src, msg);
                    let _ = handle_message(msg.to_string(), &socket, src.to_string());
                }
                Err(e) => {
                    eprintln!("recv_from error: {}", e);
                    break;
                }
            }
        }
    })
}
