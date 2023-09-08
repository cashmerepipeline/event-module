#[derive(Debug, Clone)]
pub struct EventServiceConfigs{
  // 最多同时并发队列
  pub max_concurrent_queue: u32,
  
  // 最多允许持续监听的事件类型数量,也是事件转发队列的最大限制
  pub max_event_type_count: u32,
  
  // 最多允许持续监听的事件类型实例数量
  pub max_listener_instance_count: u32,
}

static mut EVENT_SERVICE_CONFIGS: Option<EventServiceConfigs> = None;

pub fn get_event_service_configs() -> &'static EventServiceConfigs {
  unsafe {
    if EVENT_SERVICE_CONFIGS.is_none() {
      EVENT_SERVICE_CONFIGS = Some(EventServiceConfigs{
        max_concurrent_queue: 100,
        max_event_type_count: 100,
        max_listener_instance_count: 100,
      });
    } 

    EVENT_SERVICE_CONFIGS.as_ref().unwrap()
  }
}

pub fn init_event_service_configs(configs: EventServiceConfigs) {
  unsafe {
    EVENT_SERVICE_CONFIGS = Some(configs);
  }
}