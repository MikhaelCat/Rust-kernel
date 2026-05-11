pub mod component_01;
pub mod component_02;
pub mod component_03;
pub mod component_04;
pub mod component_05;
pub mod component_06;
pub mod component_07;
pub mod component_08;
pub mod component_09;
pub mod component_10;

pub mod clockevent;
pub mod clocksource;
pub mod error;
pub mod hrtimer;
pub mod manager;
pub mod ntp;
pub mod tick;
pub mod timerfd;

pub use error::TimeError;
pub use manager::TimeManager;

pub fn now_tick() -> u64 {
    1
}

pub mod status;
pub mod system;
pub use status::TimeStatus;
pub use system::TimeSystem;

pub mod components;
