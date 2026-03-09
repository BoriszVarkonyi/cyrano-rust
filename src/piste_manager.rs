use std::{
    io,
    net::UdpSocket,
    sync::{Arc, atomic::{AtomicBool, Ordering}},
    thread::{self, JoinHandle},
    time::Duration
};

pub struct PisteManager {
    running: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>
}

impl PisteManager {
    pub fn activate(socket: UdpSocket, competition_id: String) -> Self{
        let running = Arc::new(AtomicBool::new(true));
        let running_thread = Arc::clone(&running);

        let handle = thread::spawn(move ||{
            while running_thread.load(Ordering::Relaxed) {
                if let Ok(pistes) = crate::storage::read_pistes() {
                    for p in pistes {
                        let addr = format!("{}:{}", p.ip_address, p.port);
                        let hello = crate::codec::compose_hello(p.name.clone(), competition_id.clone());
                        let _ = crate::net::send_message(&socket, addr, hello);
                    }
                }
                thread::sleep(Duration::from_secs(14));
            }
        });
        Self { running, handle: Some(handle) }
    }
    pub fn deactivate(&mut self){
        self.running.store(false, Ordering::Relaxed);
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}
