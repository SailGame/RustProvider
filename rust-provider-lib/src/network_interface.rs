use tokio::sync::mpsc;
use tonic::Request;
use futures::executor::block_on;

use crate::core_pb::game_core_client::GameCoreClient;
use crate::core_pb::ProviderMsg;

pub trait NetworkInterface {
    fn connect(&mut self);

    fn is_connected(&self) -> bool;

    fn recv(&mut self) -> ProviderMsg;

    fn send(&mut self, msg: ProviderMsg);

    fn stop(&mut self);
}


pub struct GrpcNetworkInterface {
    inbound: Option<tonic::Streaming<ProviderMsg>>,
    sender: Option<mpsc::Sender<ProviderMsg>>
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
            let mut client = GameCoreClient::connect("http://localhost:50051").await.unwrap();
            let (tx, mut rx) = mpsc::channel(4);

            let outbound = async_stream::stream! {
                while let Some(message) = rx.recv().await {
                    yield message;
                }
            };
        
            let response = client.provider(Request::new(outbound)).await.unwrap();
            self.inbound = Some(response.into_inner());
            self.sender = Some(tx);
        };
        block_on(future);
    }

    fn is_connected(&self) -> bool {
        assert_eq!(self.inbound.is_some(), self.sender.is_some());
        self.inbound.is_some() && self.sender.is_some()
    }

    fn recv(&mut self) -> ProviderMsg { 
        let future = self.inbound.as_mut().unwrap().message();
        let resp = block_on(future);
        let msg = resp.unwrap().unwrap();
        println!("msg received: {:?}", msg);
        msg
    }

    fn send(&mut self, msg: ProviderMsg) {
        let future = self.sender.as_ref().unwrap().send(msg);
        let result = block_on(future);
        result.unwrap();
    }

    fn stop(&mut self) {}
}
