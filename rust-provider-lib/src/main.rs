use rust_provider_lib::game_manager::*;
use rust_provider_lib::state_machine::*;
use rust_provider_lib::network_interface::*;

struct MyStateMachine;

impl MyStateMachine {
    fn new() -> MyStateMachine {
        MyStateMachine{}
    }
}

impl StateMachine for MyStateMachine {
    fn transition(&mut self, msg: String) -> String {
        String::new()
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
