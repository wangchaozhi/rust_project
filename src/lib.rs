// 户籍管理系统库模块

pub mod app;
pub mod config;
pub mod data;
pub mod ui;
pub mod utils;

// 重新导出主要类型
pub use app::HouseholdApp;
pub use data::models::*;
