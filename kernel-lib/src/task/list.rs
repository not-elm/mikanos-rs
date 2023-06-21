use alloc::sync::Arc;
use alloc::vec::Vec;

use spin::RwLock;

use crate::error::{KernelError, KernelResult};
use crate::kernel_error;
use crate::task::status::Status;
use crate::task::switch::SwitchCommand;
use crate::task::Task;

#[derive(Default, Debug)]
pub struct TaskList {
    tasks: Vec<Arc<RwLock<Task>>>,
}


impl TaskList {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new()
        }
    }


    #[inline]
    pub fn len(&self) -> usize {
        self.tasks.len()
    }


    pub fn push(&mut self, task: Task) {
        self.push_boxed(Arc::new(RwLock::new(task)));
    }


    pub fn push_boxed(&mut self, task: Arc<RwLock<Task>>) {
        self.tasks.push(task);
    }


    pub fn wakeup_at(&mut self, task_id: u64) -> KernelResult {
        let task = self.find_where_sleeps(task_id)?;
        task.write().status = Status::Pending;

        Ok(())
    }


    pub fn next_switch_command(&mut self) -> KernelResult<SwitchCommand> {
        Ok(SwitchCommand::new(self.running_task()?, self.next_run_task()?))
    }


    pub fn sleep_at(&mut self, task_id: u64) -> KernelResult {
        if let Some(command) = self._sleep_at(task_id)? {
            command.switch_and_sleep();
        }

        Ok(())
    }


    fn _sleep_at(&mut self, task_id: u64) -> KernelResult<Option<SwitchCommand>> {
        let task = self.find(task_id)?;
        let mut t = task.write();
        if t.status.is_running() {
            drop(t);
            let next = self.next_run_task()?;
            Ok(Some(SwitchCommand::new(task, next)))
        } else {
            t.status = Status::Sleep;

            Ok(None)
        }
    }


    pub(crate) fn find(&mut self, task_id: u64) -> KernelResult<Arc<RwLock<Task>>> {
        let task = self.tasks
            .iter()
            .find(|task| task.read().id == task_id)
            .ok_or(error_not_found_task(task_id))?;

        Ok(Arc::clone(task))
    }


    fn running_task(&mut self) -> KernelResult<Arc<RwLock<Task>>> {
        let task = self.tasks
            .iter()
            .find(|task| task.read().status.is_running())
            .ok_or(kernel_error!("No Task Running"))?;

        Ok(Arc::clone(task))
    }


    fn find_where_sleeps(&mut self, task_id: u64) -> KernelResult<Arc<RwLock<Task>>> {
        let task = self.tasks
            .iter()
            .filter(|task| task.read().status.is_sleep())
            .find(|task| task.read().id == task_id)
            .ok_or(kernel_error!("Not found specified sleep the Task! id =  {task_id}"))?;

        Ok(Arc::clone(task))
    }


    fn next_run_task(&mut self) -> KernelResult<Arc<RwLock<Task>>> {
        self
            .tasks
            .sort_by(|t1, t2| t2.read().priority_level.cmp(&t1.read().priority_level));

        let task = self
            .tasks
            .iter()
            .find(|task| task.read().status.is_pending())
            .ok_or(kernel_error!("Couldn't find a task to run next"))?;

        Ok(Arc::clone(task))
    }
}


#[inline(always)]
fn error_not_found_task(task_id: u64) -> KernelError {
    kernel_error!("Not found Task: specified id = {task_id}")
}


#[cfg(test)]
mod tests {
    use crate::task::list::TaskList;
    use crate::task::priority_level::PriorityLevel;
    use crate::task::status::Status;
    use crate::task::status::Status::Running;
    use crate::task::Task;

    #[test]
    fn it_next_run_task() {
        let mut q = TaskList::new();
        q.push(Task::new(0, PriorityLevel::new(3)));
        q.push(Task::new(1, PriorityLevel::new(1)));

        let priority_level = q
            .next_run_task()
            .map(|task| task.read().priority_level)
            .unwrap();

        assert_eq!(priority_level, PriorityLevel::new(3));
    }


    #[test]
    fn it_next_switch_command() {
        let mut q = TaskList::new();
        q.push(Task::new(0, PriorityLevel::new(3)));
        q.push(Task::new(1, PriorityLevel::new(1)));
        q.push(Task::new(2, PriorityLevel::new(2)));
        q.tasks[1].write().status = Running;

        let command = q.next_switch_command().unwrap();
        assert_eq!(command.running_id(), 1);
        assert_eq!(command.next_id(), 0);
    }


    #[test]
    fn it_wakeup() {
        let mut q = TaskList::new();
        q.push(Task::new(0, PriorityLevel::new(3)));
        q.push(Task::new(1, PriorityLevel::new(0)));
        q.push(Task::new(3, PriorityLevel::new(3)));

        q.tasks[0].write().status = Status::Sleep;
        q.tasks[2].write().status = Status::Sleep;

        q.wakeup_at(0).unwrap();
        q.wakeup_at(3).unwrap();

        assert_eq!(q.tasks[0].read().status, Status::Pending);
        assert_eq!(q.tasks[2].read().status, Status::Pending);
    }


    #[test]
    fn it_sleep() {
        let mut q = TaskList::new();
        q.push(Task::new(0, PriorityLevel::new(1)));
        q.push(Task::new(1, PriorityLevel::new(0)));
        q.push(Task::new(3, PriorityLevel::new(2)));
        q.push(Task::new(2, PriorityLevel::new(3)));

        q.tasks[0].write().status = Status::Running;
        q.tasks[2].write().status = Status::Pending;

        let sleep_id0 = q._sleep_at(0).unwrap();
        let sleep_id3 = q._sleep_at(3).unwrap();

        assert_eq!(q.find(0).unwrap().read().status, Status::Running);
        assert_eq!(q.find(3).unwrap().read().status, Status::Sleep);

        let sleep_id0 = sleep_id0.unwrap();
        assert_eq!(sleep_id0.running_id(), 0);
        assert_eq!(sleep_id0.next_id(), 2);

        assert!(sleep_id3.is_none());
    }


    #[test]
    fn it_should_be_stable_sort() {
        let mut q = TaskList::new();
        q.push(Task::new(0, PriorityLevel::new(0)));
        q.push(Task::new(1, PriorityLevel::new(3)));
        q.push(Task::new(2, PriorityLevel::new(3)));
        q.push(Task::new(3, PriorityLevel::new(1)));
        q.push(Task::new(4, PriorityLevel::new(3)));
        q.push(Task::new(5, PriorityLevel::new(2)));

        let task = q.next_run_task().unwrap();
        assert_eq!(task.read().id, 1);
        assert_eq!(task.read().priority_level, PriorityLevel::new(3));

        macro_rules! assert_task {
            ($index: literal, $id: literal, $level: literal) => {
                assert_eq!(q.tasks[$index].read().id, $id, "id");
                assert_eq!(q.tasks[$index].read().priority_level, PriorityLevel::new($level), "level");
            };
        }
        assert_task!(0, 1, 3);
        assert_task!(1, 2, 3);
        assert_task!(2, 4, 3);
        assert_task!(3, 5, 2);
        assert_task!(4, 3, 1);
        assert_task!(5, 0, 0);
    }
}