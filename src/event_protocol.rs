#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventType {
    /// 所有发送事件类型的编号都是奇数，所有反馈事件类型的编号都是偶数
    /// 反馈事件编号 = 发送事件编号 + 1
    #[prost(string, tag="1")]
    pub type_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(bool, tag="3")]
    pub has_echo: bool,
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(string, tag="1")]
    pub type_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub emitter_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub serial_number: u64,
    #[prost(uint64, tag="4")]
    pub timestamp: u64,
    /// bson, 大小应当限制在64kb以内
    #[prost(bytes="vec", tag="5")]
    pub context: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventEmitter {
    #[prost(string, tag="1")]
    pub emitter_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub type_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventListener {
    #[prost(string, tag="1")]
    pub listener_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub type_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub is_online: bool,
}
/// 发送事件, 单次调用，不是持续发送
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmitEventRequest {
    #[prost(message, optional, tag="1")]
    pub event: ::core::option::Option<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmitEventResponse {
    /// 成功返回 event id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 发送事件，并监听反馈
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmitEventAndListenEchoRequest {
    #[prost(message, optional, tag="1")]
    pub event: ::core::option::Option<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmitEventAndListenEchoResponse {
    #[prost(message, optional, tag="1")]
    pub event: ::core::option::Option<Event>,
}
/// 监听事件类型, 持续监听
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenEventTypeRequest {
    #[prost(string, tag="2")]
    pub listener_id: ::prost::alloc::string::String,
    #[prost(string, tag="1")]
    pub type_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenEventTypeResponse {
    #[prost(message, optional, tag="1")]
    pub event: ::core::option::Option<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSystemConfigs {
    #[prost(uint64, tag="1")]
    pub max_event_queue_length: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventSystemConfigsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventSystemConfigsResponse {
    #[prost(message, optional, tag="1")]
    pub configs: ::core::option::Option<EventSystemConfigs>,
}
/// 注册事件类型
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterEventTypeRequest {
    #[prost(message, optional, tag="1")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(bool, tag="2")]
    pub has_echo: bool,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterEventTypeResponse {
    /// 成功返回 type id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取消注册事件类型
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeregisterEventTypeRequest {
    #[prost(string, tag="1")]
    pub type_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeregisterEventTypeResponse {
    /// 成功返回 type id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 注册发送者， 发送者需要先注册，然后才能上线
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterEmitterRequest {
    /// 不能重名
    #[prost(string, tag="1")]
    pub envent_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterEmitterResponse {
    /// 成功返回 emitter id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 注销发送者
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeregisterEmitterRequest {
    #[prost(string, tag="1")]
    pub emitter_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeregisterEmitterResponse {
    /// 成功返回 listener id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 注册发送者， 发送者需要先注册，然后才能上线
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterListenerRequest {
    /// 不能重名
    #[prost(string, tag="1")]
    pub envent_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterListenerResponse {
    /// 成功返回 listener id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 注销监听者
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeregisterListenerRequest {
    #[prost(string, tag="1")]
    pub listener_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeregisterListenerResponse {
    /// 成功返回 listener id
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 上线发送者, 服务器->发送者单向流
/// 1. 用于监听反馈事件
/// 2. 用于反馈发送者状态
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnlineEmitterRequest {
    #[prost(string, tag="1")]
    pub emitter_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnlineEmitterResponse {
    /// 成功返回 emitter id
    #[prost(string, tag="1")]
    pub emitter_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub echo: ::core::option::Option<Event>,
}
/// 列出所有事件类型
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventTypeRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventTypeResponse {
    #[prost(message, repeated, tag="1")]
    pub event_types: ::prost::alloc::vec::Vec<EventType>,
}
/// 列出事件类型的发送者
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEmitterRequest {
    #[prost(string, tag="1")]
    pub type_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEmitterResponse {
    #[prost(message, repeated, tag="1")]
    pub emitters: ::prost::alloc::vec::Vec<EventEmitter>,
}
/// 列出事件类型的监听者
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListListenerRequest {
    #[prost(string, tag="1")]
    pub type_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListListenerResponse {
    #[prost(message, repeated, tag="1")]
    pub listeners: ::prost::alloc::vec::Vec<EventListener>,
}
/// 取得发送者
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventEmitterRequest {
    #[prost(string, tag="1")]
    pub emitter_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventEmitterResponse {
    #[prost(message, optional, tag="1")]
    pub emitter: ::core::option::Option<EventEmitter>,
}
/// 取得监听者
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventListenerRequest {
    #[prost(string, tag="1")]
    pub listener_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventListenerResponse {
    #[prost(message, optional, tag="1")]
    pub listener: ::core::option::Option<EventListener>,
}
