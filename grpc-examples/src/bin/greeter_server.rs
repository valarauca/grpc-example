extern crate futures;

use futures::{Async,Stream};

extern crate grpc_examples;
extern crate grpc;

use std::thread;

use grpc_examples::helloworld_grpc::*;
use grpc_examples::helloworld::*;

struct GreeterImpl;

impl Greeter for GreeterImpl {
    fn say_hello(&self, _: grpc::RequestOptions, req: grpc::StreamingRequest<HelloRequest>) -> grpc::StreamingResponse<HelloReply> {
        grpc::StreamingResponse::no_metadata(Special::from(req))
    }
}

struct Special {
    arg: Box<Stream<Item=HelloRequest,Error=grpc::Error>+Send+'static>,
    state: usize
}
impl From<grpc::StreamingRequest<HelloRequest>> for Special {
    fn from(x: grpc::StreamingRequest<HelloRequest>) -> Special {
        Special {
            arg: x.0,
            state: 0,
        }
    }
}
impl Stream for Special {
    type Item = HelloReply;
    type Error = grpc::Error;
    fn poll(&mut self) -> Result<Async<Option<Self::Item>>,Self::Error> {
        match self.arg.poll() {
            Ok(Async::Ready(Some(hellorequest))) => {
                self.state += 1;
                if self.state.clone() == 3 {
                    self.state = 0;
                    let mut r = HelloReply::new();
                    let name = "Cody".to_string();
                    println!("greeting request from {}", name);
                    Ok(Async::Ready(Some(r)))
                } else {
                    println!("Msg from client, waiting to respond {:?}", hellorequest);
                    Ok(Async::NotReady)
                }
            }
            Ok(Async::Ready(None)) => {
                println!("Client closed stream");
                Ok(Async::Ready(None))
            }
            Ok(Async::NotReady) => {
                println!("Client stream not ready");
                Ok(Async::NotReady)
            }
            Err(e) => {
                println!("Error {:?}",&e);
                Err(e)
            }
        }
    }
}

fn main() {
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(50051);
    server.add_service(GreeterServer::new_service_def(GreeterImpl));
    server.http.set_cpu_pool_threads(4);
    let _server = server.build().expect("server");

    loop {
        thread::park();
    }
}
