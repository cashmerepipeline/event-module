/// 事件是否需要反馈
#[derive(Debug, Clone, PartialEq)]
pub enum EventEchoType{
    // 无需反馈
    NoEcho,
    // 需要反馈
    NeedEcho,
}
