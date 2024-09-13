#![no_std]
#![no_main]

#[macro_use]
extern crate cortex_m_rt;
use core::sync::atomic::{AtomicU32, Ordering};
use martos::init_system;
use martos::task_manager::TaskManager;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

static COUNTER: AtomicU32 = AtomicU32::new(1);

/// Setup function for task to execute.
fn setup_fn() {}

/// Loop function for task to execute.
fn loop_fn() {
    COUNTER.fetch_add(1, Ordering::Relaxed);
}

/// Stop condition function for task to execute.
fn stop_condition_fn() -> bool {
    let value = unsafe { COUNTER.as_ptr().read() };
    if value % 50 == 0 {
        return true;
    }
    return false;
}

#[entry]
fn main() -> ! {
    // Initialize Martos.
    init_system();
    // Add task to execute.
    TaskManager::add_task(setup_fn, loop_fn, stop_condition_fn);
    // Start task manager.
    TaskManager::start_task_manager();
}
