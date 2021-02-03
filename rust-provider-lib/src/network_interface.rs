use tokio::sync::mpsc;
use tonic::Request;
use futures::executor::block_on;

use crate::core_pb::game_core_client::GameCoreClient;
use crate::core_pb::{ProviderMsg, RegisterArgs, provider_msg::Msg};

pub trait NetworkInterface {
    fn connect_with_register_args(&mut self, msg: RegisterArgs);

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
    fn connect_with_register_args(&mut self, msg: RegisterArgs) {
        let (tx, mut rx) = mpsc::channel(4);
        self.sender = Some(tx);
        
        let outbound = async_stream::stream! {
            yield ProviderMsg {
                sequence_id: 0,
                msg: Some(Msg::RegisterArgs(msg))
            };
            while let Some(message) = rx.recv().await {
                println!("msg will be sent: {:?}", message);
                yield message;
            }
        };

        let future = async {
            let mut client = GameCoreClient::connect("http://localhost:50051").await.unwrap();
        
            // will block until yield in stream! (i.e. the first msg sent out)
            let response = client.provider(Request::new(outbound)).await.unwrap();
            self.inbound = Some(response.into_inner());
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
        // Any { type_url: "type.googleapis.com/Uno.StartGameSettings", value: [8, 1, 16, 1, 40, 15] }
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
