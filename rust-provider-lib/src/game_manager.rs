use crate::network_interface::NetworkInterface;
use crate::state_machine::StateMachine;
use crate::core_pb::{ProviderMsg, RegisterArgs};

pub struct GameManager {
    network_interface: Box<dyn NetworkInterface>,
    state_machine: Box<dyn StateMachine>
}

impl GameManager {
    pub fn new(
        network_interface: Box<dyn NetworkInterface>,
        state_machine: Box<dyn StateMachine>
    ) -> GameManager {
        GameManager{ network_interface, state_machine }
    }

    pub fn start_with_register_args(&mut self, msg: RegisterArgs) {
        self.network_interface.connect_with_register_args(msg);
        assert!(self.network_interface.is_connected());
        println!("Connected successfully.");
        loop {
            let msg_recv = self.network_interface.recv();
            self.handle_message(msg_recv);
        }
    }

    fn handle_message(&mut self, msg: ProviderMsg) {
        let msgs_to_send = self.state_machine.transition(msg);
        for msg in msgs_to_send {
            self.network_interface.send(msg);
        }
    }

    fn _stop() {}
}
