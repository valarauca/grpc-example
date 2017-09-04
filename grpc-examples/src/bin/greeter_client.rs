extern crate grpc_examples;
extern crate grpc;
extern crate futures;

use futures::{Async,Stream};

use grpc_examples::helloworld_grpc::*;
use grpc_examples::helloworld::*;

use std::env;
use std::time::Duration;
use std::thread;

struct Name {
    name: String
}
impl From<String> for Name {
    fn from(name: String) -> Name {
        Name { name }
    }
}
impl Stream for Name {
    type Item = HelloRequest;
    type Error = grpc::Error;
    fn poll(&mut self) -> Result<Async<Option<Self::Item>>,Self::Error> {
        thread::sleep(Duration::from_millis(500));
        let mut req = HelloRequest::new();
        req.set_name(self.name.clone());
        println!("Sending msg {}", &self.name);
        Ok(Async::Ready(Some(req)))
    }
}
unsafe impl Send for Name { }

fn main() {
    let name = "Cody".to_string();

    let client = GreeterClient::new_plain("localhost", 50051, Default::default()).unwrap();
    let resp = client.say_hello(grpc::RequestOptions::new(), grpc::StreamingRequest::new(Name::from(name)));
    let mut resp = resp.drop_metadata();
    loop {
        match resp.poll() {
            Ok(Async::Ready(Option::Some(msg))) => {
                println!("Server says {:?}", msg);
            }
            Ok(Async::Ready(Option::None)) => {
                panic!("Server closed connection");
            }
            Ok(Async::NotReady) => {
                println!("Server said nothing");
                ::std::thread::sleep(::std::time::Duration::from_millis(1000));
                continue;
            }
            Err(e) => {
                panic!("Server sent error {:?}", e);
            }
        }
    }
}
