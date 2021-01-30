use tokio::sync::mpsc;
use tonic::Request;
use futures::executor::block_on;

use echo_pb::echo_client::EchoClient;
use echo_pb::EchoRequest;

pub mod echo_pb {
    tonic::include_proto!("echo");
}

pub trait NetworkInterface {
    fn connect(&mut self);

    fn is_connected(&self) -> bool;

    fn recv(&mut self) -> String;

    fn send(&mut self, msg: String);

    fn stop(&mut self);
}


pub struct GrpcNetworkInterface {
    inbound: Option<tonic::Streaming<echo_pb::EchoResponse>>,
    sender: Option<mpsc::Sender<String>>
}

impl GrpcNetworkInterface {
    pub fn new() -> GrpcNetworkInterface {
        GrpcNetworkInterface{
            inbound: None,
            sender: None
        }
    }
}

impl NetworkInterface for GrpcNetworkInterface {
    fn connect(&mut self) {
        let future = async {
            let mut client = EchoClient::connect("http://localhost:5678").await.unwrap();
            let (tx, mut rx) = mpsc::channel(4);

            let outbound = async_stream::stream! {
                while let Some(message) = rx.recv().await {
                    let req = EchoRequest {
                        message
                    };
                    yield req;
                }
            };
        
            let response = client.bidirectional_streaming_echo(Request::new(outbound)).await.unwrap();
            self.inbound = Some(response.into_inner());
            self.sender = Some(tx);
        };
        block_on(future);
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn recv(&mut self) -> String { 
        let future = self.inbound.as_mut().unwrap().message();
        let resp = block_on(future);
        if let Some(resp) = resp.unwrap() {
            println!("msg received: {:?}", resp);
            return resp.message;
        }
        String::new() 
    }

    fn send(&mut self, msg: String) {
        let future = self.sender.as_ref().unwrap().send(msg);
        let result = block_on(future);
        result.unwrap();
    }

    fn stop(&mut self) {}
}
