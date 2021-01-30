use crate::network_interface::NetworkInterface;
use crate::state_machine::StateMachine;

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

    pub fn start(&mut self) {
        self.network_interface.connect();
        self.network_interface.send(String::from("hello"));
        loop {
            let msg_recv = self.network_interface.recv();
            self.handle_message(msg_recv);
        }
    }

    fn start_with_register_args() {
    }

    fn handle_message(&mut self, msg: String) {
        let msg_to_send = self.state_machine.transition(msg);
        self.network_interface.send(msg_to_send);
    }

    fn stop() {}
}
