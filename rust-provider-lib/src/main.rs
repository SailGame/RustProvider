use rust_provider_lib::game_manager::*;
use rust_provider_lib::state_machine::*;
use rust_provider_lib::network_interface::*;
use rust_provider_lib::core_pb::ProviderMsg;

struct MyStateMachine;

impl MyStateMachine {
    fn new() -> MyStateMachine {
        MyStateMachine{}
    }
}

impl StateMachine for MyStateMachine {
    fn transition(&mut self, msg: ProviderMsg) -> Vec<ProviderMsg> {
        vec![]
    }
}

#[tokio::main]
async fn main() {
    let mut game_manager = GameManager::new(
        Box::new(GrpcNetworkInterface::new()),
        Box::new(MyStateMachine::new())
    );

    game_manager.start();
}
