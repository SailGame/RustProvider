pub mod game_manager;

pub mod network_interface;

pub mod state_machine;

pub mod util;

pub mod msg_builder;

pub mod core_pb {
    // tonic::include_proto!("core");
    include!("../../pb/core.rs");
}
