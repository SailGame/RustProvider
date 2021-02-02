use rust_provider_lib::game_manager::GameManager;
use rust_provider_lib::network_interface::GrpcNetworkInterface;
use rust_provider_lib::core_pb::{RegisterArgs, GameSetting};
use uno::state_machine::UnoStateMachine;

#[tokio::main]
async fn main() {
    println!("Hello, here is UNO!");
    let mut game_manager = GameManager::new(
        Box::new(GrpcNetworkInterface::new()),
        Box::new(UnoStateMachine::new())
    );

    game_manager.start_with_register_args(RegisterArgs {
        id: String::from("id-uno"),
        game_name: String::from("UNO"),
        game_setting: Some(GameSetting {
            max_users: 4,
            min_users: 2
        })
    });
}
