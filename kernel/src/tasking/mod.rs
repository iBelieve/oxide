use core::sync::atomic::Ordering;
use self::list::TaskList;
use spin::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub use self::task::{Task, TaskId, TaskMain};
pub use self::switching::switch;

mod list;
mod switching;
mod task;

pub const MAX_TASKS: usize = usize::max_value() - 1;

static TASKS: Once<RwLock<TaskList>> = Once::new();

static CURRENT_TASK_ID: task::AtomicTaskId = task::AtomicTaskId::default();

pub fn init() {
    let mut tasks = tasks_mut();
    let task_lock = tasks.new_task(::kernel_main)
        .expect("Unable to initialize the primary kernel task!");
    let task = task_lock.write();

    CURRENT_TASK_ID.store(task.id, Ordering::SeqCst);

    ok!("Tasking initialized.");
}

fn init_tasks() -> RwLock<TaskList> {
    RwLock::new(TaskList::new())
}

pub fn tasks() -> RwLockReadGuard<'static, TaskList> {
    TASKS.call_once(init_tasks).read()
}

pub fn tasks_mut() -> RwLockWriteGuard<'static, TaskList> {
    TASKS.call_once(init_tasks).write()
}

pub fn current_task_id() -> TaskId {
    CURRENT_TASK_ID.load(Ordering::SeqCst)
}

pub fn spawn(main: TaskMain) {
    let mut tasks = tasks_mut();

    tasks.spawn(main).expect("Unable to spawn new kernel task!!");
}

pub fn exit() -> ! {
    {
        let tasks = tasks();
        let current_lock = tasks.current()
                .expect("Attempting to switch tasks without a task running!");
        let mut current = current_lock.write();
        current.finished = true;
    }
    switch();
    panic!("Returned to a dead task!");
}
