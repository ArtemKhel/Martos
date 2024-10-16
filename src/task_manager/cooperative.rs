extern crate alloc;

use alloc::vec::Vec;
use core::{future::Future, pin::Pin, task::Context};

use crate::task_manager::{
    task,
    task::{FutureTask, TaskNumberType},
    TaskManagerTrait, TASK_MANAGER,
};

#[repr(C)]
/// Task manager representation. Based on round-robin scheduling without priorities.
pub struct CooperativeTaskManager {
    /// Vector of tasks to execute.
    pub(crate) tasks: Vec<FutureTask>,
    /// Index of task, that should be executed.
    pub(crate) task_to_execute_index: TaskNumberType,
}

impl TaskManagerTrait for CooperativeTaskManager {
    fn start_task_manager() -> ! {
        loop {
            Self::task_manager_step();
        }
    }
}

impl CooperativeTaskManager {
    /// Creates new task manager.
    pub(crate) const fn new() -> CooperativeTaskManager {
        CooperativeTaskManager {
            tasks: Vec::new(),
            task_to_execute_index: 0,
        }
    }

    /// One step of task manager's work.
    // TODO: Support priorities.
    // TODO: Delete tasks from task vector if they are pending?
    fn task_manager_step() {
        if unsafe { !TASK_MANAGER.tasks.is_empty() } {
            let waker = task::task_waker();

            let task = unsafe { &mut TASK_MANAGER.tasks[TASK_MANAGER.task_to_execute_index] };
            let mut task_future_pin = Pin::new(task);
            let _ = task_future_pin
                .as_mut()
                .poll(&mut Context::from_waker(&waker));

            unsafe {
                if TASK_MANAGER.task_to_execute_index + 1 < TASK_MANAGER.tasks.len() {
                    TASK_MANAGER.task_to_execute_index += 1;
                } else {
                    TASK_MANAGER.task_to_execute_index = 0;
                }
            }
        }
    }

    /// Starts task manager work. Returns after 1000 steps only for testing task_manager_step.
    #[cfg(test)]
    fn test_start_task_manager() {
        for _n in 1..=1000 {
            Self::task_manager_step();
        }
    }
}
