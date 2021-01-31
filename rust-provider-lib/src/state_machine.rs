use crate::core_pb::ProviderMsg;

pub trait StateMachine {
    fn transition(&mut self, msg: ProviderMsg) -> Vec<ProviderMsg>;
}