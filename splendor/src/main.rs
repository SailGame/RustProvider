use rust_provider_lib::game_manager::GameManager;
use rust_provider_lib::network_interface::GrpcNetworkInterface;
use rust_provider_lib::core_pb::{RegisterArgs, GameSetting};
use splendor::state_machine::SplendorStateMachine;

#[tokio::main]
async fn main() {
    println!("Hello, here is Splendor!");
    let mut game_manager = GameManager::new(
        Box::new(GrpcNetworkInterface::new()),
        Box::new(SplendorStateMachine::new())
    );

    game_manager.start_with_register_args(RegisterArgs {
        id: String::from("id-splendor"),
        game_name: String::from("Splendor"),
        game_setting: Some(GameSetting {
            max_users: 4,
            min_users: 2
        })
    });
}