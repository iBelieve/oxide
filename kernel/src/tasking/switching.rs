use core::sync::atomic::Ordering;
use core::ops::DerefMut;
use super::{tasks, Task, CURRENT_TASK_ID};

pub fn switch() {
    let from_ptr;
    let to_ptr;

    {
        let tasks = tasks();

        let current_lock = tasks.current()
            .expect("Attempting to switch tasks without a task running!");
        let mut current = current_lock.write();

        from_ptr = current.deref_mut() as *mut Task;

        if let Some(next_lock) = tasks.next() {
            let mut next = next_lock.write();

            to_ptr = next.deref_mut() as *mut Task;
        } else {
            return;
        }
    }

    unsafe {
        CURRENT_TASK_ID.store((&mut *to_ptr).id, Ordering::SeqCst);
        (&mut *from_ptr).context.switch_to(&mut (&mut *to_ptr).context);
    }
}
