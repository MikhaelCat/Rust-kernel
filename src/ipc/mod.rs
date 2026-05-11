pub mod channel;

pub use channel::{IpcError, MessageQueue};
pub mod component_01;
pub mod component_02;
pub mod component_03;
pub mod component_04;
pub mod epoll;
pub mod eventfd;
pub mod futex;
pub mod kqueue;
pub mod mailbox;
pub mod msg;
pub mod pipe;
pub mod ringbuf;
pub mod semaphore;
pub mod shm;
pub mod signal;
pub mod signalfd;
pub mod spinlock;

pub mod error;
pub mod manager;
pub use manager::IpcManager;

pub mod system;
pub use system::IpcSystem;

pub mod channel2;

pub mod components;
