extern crate env_logger;
extern crate quinn;
extern crate rustls;
extern crate tokio;

use rustls::internal::pemfile;
use std::{fs::File, io::BufReader};

fn main() {
    let certs = {
        let f = File::open("certs/server.chain").expect("cannot open 'certs/server.chain'");
        let mut reader = BufReader::new(f);
        pemfile::certs(&mut reader).expect("cannot read certificates")
    };

    let keys = {
        let f = File::open("certs/server.rsa").expect("cannot open 'certs/server.rsa'");
        let mut reader = BufReader::new(f);
        pemfile::rsa_private_keys(&mut reader).expect("cannot read private keys")
    };

    let tls_config = quinn::tls::build_server_config(certs, keys[0].clone()).unwrap();
    env_logger::init();
    tokio::run(quinn::Server::new("localhost", 4433, tls_config).unwrap());
}
