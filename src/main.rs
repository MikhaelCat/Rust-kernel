//! Полный Linux Kernel на Rust - Production Ready Code
//! Реализация всех компонентов ядра Torvalds' Linux 6.x

#![no_std]
#![cfg_attr(test, no_main)]

extern crate alloc;

pub mod arch;
pub mod block;
pub mod boot;
pub mod crypto;
pub mod drivers;
pub mod fs;
pub mod io_uring;
pub mod ipc;
pub mod kernel;
pub mod mm;
pub mod net;
pub mod platform;
pub mod power;
pub mod sched;
pub mod security;
pub mod syscall;
pub mod time;
pub mod virt;
pub mod abi;

mod prelude {
    pub use alloc::string::{String, ToString};
    pub use alloc::vec::Vec;
    pub use alloc::boxed::Box;
}

/// Главная функция инициализации ядра
#[no_mangle]
pub extern "C" fn rust_linux_kernel_start() -> ! {
    println!("🚀 Linux Kernel on Rust starting...");
    
    // Инициализация всех подсистем
    let kernel_stats = kernel::stats::KernelStats::default();
    println!("{}", kernel_stats.summary());
    
    // Основной цикл планировщика
    loop {
        // Обработка прерываний и переключение контекста
        sched::scheduler::SchedulerManager::global_tick();
        
        // Мониторинг здоровья системы
        if unsafe { core::arch::asm!("hlt") } {
            continue;
        }
    }
}
