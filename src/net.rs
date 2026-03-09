use std::thread::JoinHandle;
use std::thread::spawn;
use std::{io, net::UdpSocket};

pub fn send_message(
    socket: &UdpSocket,
    ip_address: String,
    port: String,
    buffer: String,
) -> io::Result<()> {
    {
        let recv_addr: String = format!("{}:{}", ip_address, port);

        let buf = buffer.as_bytes();

        socket
            .send_to(buf, &recv_addr)
            .map_err(|e| io::Error::new(e.kind(), format!("send_to({recv_addr}) failed: {e}")))?;
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
                }
                Err(e) => {
                    eprintln!("recv_from error: {}", e);
                    break;
                }
            }
        }
    })
}