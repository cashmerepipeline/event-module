

use std::sync::{Arc, OnceLock};

use dependencies_sync::once_cell::sync::Lazy;
use dependencies_sync::log::{error, info, warn};
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;

use dependencies_sync::parking_lot::RwLock;
use managers::entity_interface::EntityInterface;
use managers::hard_coded_cache_interface::HardCodedInterface;

use crate::ids_codes::manage_ids::EVENT_LISTENERS_MANAGE_ID;

use managers::{declare_common_manager_interface, AllManagerInterface, Manager, ManagerInterface };

use cash_core::{manage_from_document, Manage};
use cash_result::*;

use dependencies_sync::bson::Document;
use manage_define::manage_ids::MANAGES_MANAGE_ID;
use managers::declare_get_manager;

#[derive(Default)]
pub struct EventListenersManager;

/// 缓存
static EVENT_LISTENERS_MANAGE: OnceLock<Arc<RwLock<Manage>>> = OnceLock::new();
static EVENT_LISTENERS_MANAGE_DOCUMENT: OnceLock<Arc<RwLock<Document>>> = OnceLock::new();

/// 管理器
static INNER: Lazy<Arc<Box<dyn AllManagerInterface>>> = Lazy::new(||
    Arc::new(Box::new(EventListenersManager {}))
);
static EVENT_LISTENERS_MANAGER: OnceLock<Manager> = OnceLock::new();

// 声明管理器取得函数
declare_get_manager!(EventListenersManager, EVENT_LISTENERS_MANAGER, INNER.clone());

declare_common_manager_interface!(EventListenersManager, EVENT_LISTENERS_MANAGE, EVENT_LISTENERS_MANAGE_DOCUMENT, EVENT_LISTENERS_MANAGE_ID);

#[async_trait]
impl AllManagerInterface for EventListenersManager {}
#[async_trait]
impl HardCodedInterface for EventListenersManager {}
#[async_trait]
impl EntityInterface for EventListenersManager {}
