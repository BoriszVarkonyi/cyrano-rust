//Modules import
mod message;
mod codec;
mod net;

//Sys imports
use std::{io, net::UdpSocket};

//Own fn uses
use message::Message;
use codec::{compose_display, compose_hello};
use net::{send_message, start_listener};


fn main() -> io::Result<()> {
    {
        //UDP socket creation
        let socket = UdpSocket::bind("0.0.0.0:50200")?;

        let listener_socket = socket.try_clone()?;
        let _listener = start_listener(listener_socket);

        //Example message initiation
        let msg = Message {
            left_id: "2".to_string(),
            left_name: "Valaki".to_string(),
            right_id: "1".to_string(),
            right_name: "Masik".to_string(),
            ..Message::new("DISP", "1", "fjm-eq")
        };

        let to_send = compose_display(msg);

        let example_hello = compose_hello("1".to_string(), "fjm-eq".to_string());

        match send_message(
            &socket,
            "192.168.1.103".to_string(),
            "50100".to_string(),
            example_hello,
        ) {
            Ok(v) => {
                println!("{:?}", v)
            }
            Err(e) => eprintln!("failed: {e}"),
        }
        //println!("{}", to_send);
        //let _ = send_message("192.168.1.103".to_string(), "50100".to_string(), to_send);
    }

    //Keep main alive so the listener thread is alive.
    loop {
        std::thread::park();
    }
    Ok(())
}
