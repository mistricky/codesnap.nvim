use std::net::TcpListener;

pub fn get_available_port() -> Option<u16> {
    if cfg!(debug_assertions) {
        Some(8080)
    } else {
        find_available_port()
    }
}

fn find_available_port() -> Option<u16> {
    (8000..10000).find(|port| port_is_available(*port))
}

fn port_is_available(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}
