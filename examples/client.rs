extern crate futures;
extern crate quinn;
extern crate webpki;
extern crate rustls;

use futures::Future;

extern crate untrusted;
extern crate bytes;
use self::untrusted::Input;
use std::fs::File;
use std::io::{Read};
use bytes::BufMut;
use std::str;


pub fn client_config() -> rustls::ClientConfig {
    let mut f = File::open("certs/ca.der").expect("cannot open 'certs/ca.der'");
    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes).expect("error while reading");

    let anchor =
        webpki::trust_anchor_util::cert_der_as_trust_anchor(Input::from(&bytes)).unwrap();
    let anchor_vec = vec![anchor];
    quinn::tls::build_client_config(Some(&webpki::TLSServerTrustAnchors(&anchor_vec)))
}

fn main() {
    let server_name = "localhost".to_owned();
    let mut buf = vec![];
    buf.put("hello world");
    let connect = quinn::Client::connect_with_tls_config(&server_name, 4433, client_config()).unwrap();
//    let connect = quinn::Client::connect(&server_name, 4433).unwrap();
    let mut client = quinn::Client::connect(&server_name, 4433).unwrap().client.unwrap();
    let mut streams = quinn::streams::Streams::new(quinn::types::Side::Client);
    connect.and_then(|(a, mut _streams),| {
        println!("init done");
        client = a;
        streams = _streams;
        futures::future::ok(())
    }).wait();
    println!("{:?}", str::from_utf8(&buf).unwrap());
//    stream.poll_send(&mut buf);
//    client.conn_state.handle(&mut buf);
    client.conn_state.queue.push_back(buf);
    client.wait();
//    let mut stream = streams.get_stream(0).unwrap();
//    let mut buf = vec![];
//    buf.put("stream hello world");
//    streams.poll_send(&mut buf);
//    client.wait();
//    stream.send(&buf);
//    client.wait();
//    let mut connector = quinn::client::ConnectFuture::new(client).unwrap();
//    connector.and_then(|(_a, _stream),| {
//        println!("send aaaaaaaaaaaaa");
//        futures::future::ok(())
//    }).wait();

//    connect.and_then(|_| {
//        println!("send message");
//        futures::future::ok(())
//    });
//    println!(
//        "RESULT: {:?}",
//
//        connect.and_then(|(mut _client, _streams), | {
//            let mut buf = vec![];
//                println!("client is connected {:?}",_client.conn_state.queue);
//            _client.poll_send();
//            let mut buf = vec![];
//
//            buf.put("hello world");
//            println!("stream send, {:?}", buf);
//                &streams.poll_send(&mut buf);
//            println!("stream send");
//                println!("{}", );
//                client =
//            let con = connect.client.unwrap().conn_state;
//            con.streams.poll_send(b"literal byte slice");
//                futures::future::ok(())
//            }).wait(),
//    );
}
