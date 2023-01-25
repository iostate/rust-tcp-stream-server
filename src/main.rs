use std::net::TcpListener;

const SIROCCO_SERVER: &str = "127.0.0.1:8000";
fn main() {
    // starting
    println!("sirocco starting {}", SIROCCO_SERVER);

    // listen - create a connection then listen for incoming connections
    // bind
    let listener = TcpListener::bind(SIROCCO_SERVER).unwrap();

    // start
    println!("sirocco listening {}", SIROCCO_SERVER);

    // as each conection comes in, we want to open up that connection and have a little stream for that
    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("connection established!")
    }
}
