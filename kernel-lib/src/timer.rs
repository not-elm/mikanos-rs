use core::sync::atomic::Ordering;

use crate::timer::handler::manager::TimeHandleManager;

pub mod apic;
pub mod handler;

pub static TIME_HANDLE_MANAGER: TimeHandleManager = TimeHandleManager::new();





