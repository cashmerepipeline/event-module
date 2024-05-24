/*
Author: 闫刚 (yes7rose@sina.com)
events_manager.rs (c) 2020
Desc: 事件管理器
Created:  2020-12-03T03:33:44.641Z
Modified: !date!
*/

use std::sync::{Arc, OnceLock};

use dependencies_sync::bson::Document;
use dependencies_sync::log::{error, info, warn};
use dependencies_sync::once_cell::sync::Lazy;
use dependencies_sync::parking_lot::RwLock;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;

use managers::entity_interface::EntityInterface;
use managers::hard_coded_cache_interface::HardCodedInterface;
use managers::{declare_common_manager_interface, AllManagerInterface, Manager, ManagerInterface};

use cash_core::{manage_from_document, Manage};
use cash_result::*;

use managers::declare_get_manager;

use manage_define::manage_ids::MANAGES_MANAGE_ID;
use crate::ids_codes::manage_ids::EVENT_TYPES_MANAGE_ID;

#[derive(Default)]
pub struct EventTypesManager;

/// 缓存
static EVENT_TYPES_MANAGE: OnceLock<Arc<RwLock<Manage>>> = OnceLock::new();
static EVENT_TYPES_MANAGE_DOCUMENT: OnceLock<Arc<RwLock<Document>>> = OnceLock::new();

/// 管理器
static INNER: Lazy<Arc<Box<dyn AllManagerInterface>>> =
    Lazy::new(|| Arc::new(Box::new(EventTypesManager {})));
static EVENT_TYPES_MANAGER: OnceLock<Manager> = OnceLock::new();

// 声明管理器取得函数
declare_get_manager!(EventTypesManager, EVENT_TYPES_MANAGER, INNER.clone());

declare_common_manager_interface!(
    EventTypesManager,
    EVENT_TYPES_MANAGE,
    EVENT_TYPES_MANAGE_DOCUMENT,
    EVENT_TYPES_MANAGE_ID
);

#[async_trait]
impl AllManagerInterface for EventTypesManager {}
#[async_trait]
impl HardCodedInterface for EventTypesManager {}
#[async_trait]
impl EntityInterface for EventTypesManager {}
