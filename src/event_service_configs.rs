pub struct EventServiceConfigs{
  // 最多允许持续监听的事件类型数量
  pub max_event_type_queue_size: u32,
  
  // 最多允许持续监听的事件类型实例数量
  pub max_listener_instance_size: u32,
}