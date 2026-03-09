use std::net::UdpSocket;

use serde::de::value::Error;

use crate::codec::{compose_display, decompose_msg};
use crate::message::Message;

use crate::net::send_message;

pub fn handle_message(
    message: String,
    socket: &UdpSocket,
    response_address_port: String,
) -> Result<(), String> {

    let msg_data: Message = decompose_msg(&message)?;

    println!("MOST EZT NEZD: {:?}", msg_data);

    if msg_data.command == "NEXT" {
        println!("NEXT MESSAGE");
        println!("ANSWER: DISP MESSAGE");

        let response: Message = Message {
            left_id: "1".to_string(),
            left_name: "SZIA".to_string(),
            right_id: "2".to_string(),
            right_name: "SZERELEM".to_string(),
            ..Message::display(msg_data.piste, msg_data.competition)
        };

        let buffer = compose_display(response);
        println!("{:?}", buffer);

        let _ = send_message(socket, response_address_port, buffer);

    }
    if msg_data.command == "PREV" {
        println!("PREV MESSAGE");
    }

    Ok(())
}
