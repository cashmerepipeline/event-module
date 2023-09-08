#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventType {
    /// 所有发送事件类型的编号都是奇数，所有反馈事件类型的编号都是偶数
    /// 反馈事件编号 = 发送事件编号 + 1
    #[prost(string, tag = "1")]
    pub type_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(string, tag = "1")]
    pub type_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub emitter_id: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub emitter_instance_name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub serial_number: u64,
    #[prost(uint64, tag = "4")]
    pub timestamp: u64,
    /// bson, 大小应当限制在64kb以内
    #[prost(bytes = "vec", tag = "5")]
    pub context: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "7")]
    pub need_echo: bool,
}
/// 发送者，可以有多个实例，也就是一个事件类型可以有多个来源
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventEmitter {
    #[prost(string, tag = "1")]
    pub emitter_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub type_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
}
/// 监听者，一般只有一个实例
/// 一个事件类型可以有多个监听者，用于对同一事件类型的不同处理
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventListener {
    #[prost(string, tag = "1")]
    pub listener_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub type_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub is_online: bool,
}
/// 发送事件, 单次调用，不是持续发送
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmitEventRequest {
    #[prost(message, optional, tag = "1")]
    pub event: ::core::option::Option<Event>,
}
/// 返回反馈事件流，只是事件发送成功与否的反馈，不是事件处理的反馈
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmitEventResponse {
    #[prost(message, optional, tag = "1")]
    pub event: ::core::option::Option<Event>,
}
/// 发送到指定的监听者实例，指定的监听者实例必须在线，
/// 同一事件类型的特定事件处理
/// 返回不是流
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmitEventToInstanceRequest {
    #[prost(message, optional, tag = "1")]
    pub event: ::core::option::Option<Event>,
    #[prost(string, tag = "2")]
    pub listener_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub instance_index: u32,
}
/// 监听事件类型, 持续监听
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenEventTypeRequest {
    #[prost(string, tag = "2")]
    pub listener_id: ::prost::alloc::string::String,
    #[prost(string, tag = "1")]
    pub type_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub instance_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenEventTypeResponse {
    #[prost(message, optional, tag = "1")]
    pub event: ::core::option::Option<Event>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSystemConfigs {
    #[prost(uint64, tag = "1")]
    pub max_event_queue_length: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventSystemConfigsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventSystemConfigsResponse {
    #[prost(message, optional, tag = "1")]
    pub configs: ::core::option::Option<EventSystemConfigs>,
}
/// 注册事件类型
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterEventTypeRequest {
    #[prost(message, optional, tag = "1")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(bool, tag = "2")]
    pub has_echo: bool,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterEventTypeResponse {
    /// 成功返回 type id
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 取消注册事件类型
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeregisterEventTypeRequest {
    #[prost(string, tag = "1")]
    pub type_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeregisterEventTypeResponse {
    /// 成功返回 type id
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 注册发送者， 发送者需要先注册，然后才能上线
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterEventEmitterRequest {
    /// 不能重名
    #[prost(string, tag = "1")]
    pub event_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterEventEmitterResponse {
    /// 成功返回 emitter id
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 注销发送者
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeregisterEventEmitterRequest {
    #[prost(string, tag = "1")]
    pub emitter_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeregisterEventEmitterResponse {
    /// 成功返回 listener id
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 注册监听者
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterEventListenerRequest {
    /// 不能重名
    #[prost(string, tag = "1")]
    pub event_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub name: ::core::option::Option<::manage_define::cashmere::Name>,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterEventListenerResponse {
    /// 成功返回 listener id
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 注销监听者
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeregisterEventListenerRequest {
    #[prost(string, tag = "1")]
    pub listener_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeregisterEventListenerResponse {
    /// 成功返回 listener id
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 列出所有事件类型
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventTypesRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventTypesResponse {
    #[prost(message, repeated, tag = "1")]
    pub event_types: ::prost::alloc::vec::Vec<EventType>,
}
/// 列出事件类型的发送者
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEmitterRequest {
    #[prost(string, tag = "1")]
    pub type_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEmitterResponse {
    #[prost(message, repeated, tag = "1")]
    pub emitters: ::prost::alloc::vec::Vec<EventEmitter>,
}
/// 列出事件类型的监听者
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListListenerRequest {
    #[prost(string, tag = "1")]
    pub type_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListListenerResponse {
    #[prost(message, repeated, tag = "1")]
    pub listeners: ::prost::alloc::vec::Vec<EventListener>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenerInstance {
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub index: u32,
}
/// 列出监听者的实例
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListListenerInstancesRequest {
    #[prost(string, tag = "1")]
    pub listener_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListListenerInstancesResponse {
    #[prost(message, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<ListenerInstance>,
}
