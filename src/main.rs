use std::net::TcpListener;
use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tcp_listener =
        TcpListener::bind("127.0.0.1:8000").expect("error binding with tcp_listener");
    run(tcp_listener)?.await
}
