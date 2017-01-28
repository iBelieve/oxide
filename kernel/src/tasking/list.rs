use alloc::arc::Arc;
use collections::BTreeMap;
use core::mem;
use spin::RwLock;
use super::{Task, TaskId, TaskMain, current_task_id, MAX_TASKS};
use super::task::execute_task;

pub struct TaskList {
    tasks: BTreeMap<TaskId, Arc<RwLock<Task>>>,
    next_id: usize
}

impl TaskList {
    pub fn new() -> Self {
       TaskList {
           tasks: BTreeMap::new(),
           next_id: 0
       }
    }

    pub fn iter(&self) -> ::collections::btree_map::Iter<TaskId, Arc<RwLock<Task>>> {
        self.tasks.iter()
    }

    pub fn get(&self, id: TaskId) -> Option<&Arc<RwLock<Task>>> {
        return self.tasks.get(&id)
    }

    pub fn current(&self) -> Option<&Arc<RwLock<Task>>> {
        return self.get(current_task_id())
    }

    pub fn next(&self) -> Option<&Arc<RwLock<Task>>> {
        let current_id = current_task_id();

        let can_run = |task: &mut Task| -> bool {
            !task.finished
        };

        for (id, task_lock) in self.iter() {
            if *id > current_id {
                let mut task = task_lock.write();

                if can_run(&mut task) {
                    return Some(task_lock);
                }
            }
        }

        for (id, task_lock) in self.iter() {
            if *id < current_id {
                let mut task = task_lock.write();

                if can_run(&mut task) {
                    return Some(task_lock);
                }
            }
        }

        None
    }

    pub fn new_task(&mut self, main: TaskMain) -> Result<&Arc<RwLock<Task>>, &str> {
        if self.next_id > MAX_TASKS {
            self.next_id = 0;
        }

        while self.tasks.contains_key(&TaskId::from(self.next_id)) {
            self.next_id += 1;
        }

        if self.next_id > MAX_TASKS {
            return Err("Maximum number of tasks created!");
        }

        let id = TaskId::from(self.next_id);
        self.next_id += 1;

        assert!(self.tasks.insert(id, Arc::new(RwLock::new(Task::new(id, main)))).is_none());

        Ok(self.tasks.get(&id).expect("Unable to create new task. ID was invalid."))
    }

    pub fn spawn(&mut self, main: TaskMain) -> Result<&Arc<RwLock<Task>>, &str> {
        use x86::shared::control_regs;

        let task_lock = self.new_task(main)?;
        let mut task = task_lock.write();

        let mut stack = vec![0; 65536].into_boxed_slice();
        let offset = stack.len() - mem::size_of::<usize>();
        unsafe {
            let func_ptr = stack.as_mut_ptr().offset(offset as isize);
            *(func_ptr as *mut usize) = execute_task as usize;
        }
        task.context.set_page_table(unsafe { control_regs::cr3() });
        task.context.set_stack(stack.as_ptr() as usize + offset);

        unsafe {
            let rflags: usize;
            asm!("pushfq; mov $0, [rsp]; popfq" : "=r"(rflags) : : "memory" : "intel", "volatile");
            task.context.set_rflags(rflags);
        }

        task.kernel_stack = Some(stack);

        Ok(task_lock)
    }
}
