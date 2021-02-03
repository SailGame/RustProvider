use crate::core_pb::{
    ProviderMsg, RegisterRet, StartGameArgs, QueryStateArgs, UserOperationArgs
};
use crate::core_pb::provider_msg::Msg;

pub trait StateMachine {
    fn transition(&mut self, msg: ProviderMsg) -> Vec<ProviderMsg> {
        let msg = msg.msg.unwrap();
        match msg {
            Msg::RegisterRet(msg) => self.transition_register_ret(msg),
            Msg::StartGameArgs(msg) => self.transition_start_game_args(msg),
            Msg::QueryStateArgs(msg) => self.transition_query_state_args(msg),
            Msg::UserOperationArgs(msg) => self.transition_user_operation_args(msg),
            _ => vec![]
        }
    }

    fn transition_register_ret(&mut self, msg: RegisterRet) -> Vec<ProviderMsg>;

    fn transition_start_game_args(&mut self, msg: StartGameArgs) -> Vec<ProviderMsg>;

    fn transition_query_state_args(&mut self, msg: QueryStateArgs) -> Vec<ProviderMsg>;

    fn transition_user_operation_args(&mut self, msg: UserOperationArgs) -> Vec<ProviderMsg>;
}