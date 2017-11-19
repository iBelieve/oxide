use alloc::boxed::Box;
use arch::tasking::Context;
use core::mem;
use core::ops::Deref;
use core::sync::atomic::AtomicUsize;
use super::{tasks, exit, switch};

int_like!(TaskId, AtomicTaskId, usize, AtomicUsize);

pub type TaskMain = fn();

pub struct Task {
    pub id: TaskId,
    pub main: TaskMain,
    pub context: Context,
    pub finished: bool,
    pub kernel_stack: Option<Box<[u8]>>
}

impl Task {
    pub fn new(id: TaskId, main: TaskMain) -> Task {
        Task { id: id, main: main, context: Context::new(), finished: false,
               kernel_stack: None }
    }

    pub fn wait_for(&self) {
        while !self.finished {
            switch();
        }
    }
}

pub fn execute_task() {
    let main = {
        let tasks = tasks();
        let current_lock = tasks.current()
            .expect("Attempted to execute main function outside of current task!");
        let current = current_lock.read();

        current.deref().main
    };
    main();
    exit();
}
