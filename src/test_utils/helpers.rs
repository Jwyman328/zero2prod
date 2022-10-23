use std::net::TcpListener;

use crate::startup::run;
use tokio;

pub fn spawn_app() -> std::net::SocketAddr {
    // pass in random port
    let test_server_address = "127.0.0.1:0";
    let tcp_listener =
        TcpListener::bind(test_server_address).expect("tcp listener spawn app failed");
    let tcp_port = tcp_listener.local_addr().expect("unwrap local addr");

    let server = run(tcp_listener).expect("error starting server");
    let _ = tokio::spawn(server);
    tcp_port
}
