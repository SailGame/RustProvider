pub trait StateMachine {
    fn transition(&mut self, msg: String) -> String;
}