//Modules import
mod message;
mod codec;
mod net;
mod domain;
mod storage;
mod com;
mod piste_manager;

//Sys imports
use std::{io, net::UdpSocket};

//Own fn uses
use codec::{compose_display, compose_hello};
use net::{send_message, start_listener};
use storage::read_pistes;

use crate::{domain::Piste, piste_manager::PisteManager};

fn main() -> io::Result<()> {
    {
        let p = read_pistes();

        println!("{:?}", p);

        //UDP socket creation
        let socket = UdpSocket::bind("0.0.0.0:50200")?;

        let listener_socket = socket.try_clone()?;
        let _listener = start_listener(listener_socket);

        PisteManager::activate(socket.try_clone()?, "fjm-eq".to_string());
    }

    //Keep main alive so the listener thread is alive.
    loop {
        std::thread::park();
    }
    Ok(())
}
