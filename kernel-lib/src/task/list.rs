use alloc::vec::Vec;

use crate::error::{KernelError, KernelResult};
use crate::kernel_error;
use crate::task::status::Status;
use crate::task::switch::SwitchCommand;
use crate::task::Task;

#[derive(Default, Debug)]
pub struct TaskList {
    tasks: Vec<Task>,
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


    #[inline]
    pub fn push(&mut self, task: Task) {
        self.tasks.push(task);
    }


    pub fn wakeup_at(&mut self, task_id: u64) -> KernelResult {
        let task = self.find_where_sleeps(task_id)?;
        task.status = Status::Pending;

        Ok(())
    }


    pub fn next_switch_command(&mut self) -> KernelResult<SwitchCommand> {
        Ok(SwitchCommand::new(self.running_task()?, self.next_run_task()?))
    }


    pub fn sleep_at(&mut self, task_id: u64) -> KernelResult {
        if let Some(mut command) = self.sleep_and_create_switch_command_if_running(task_id)? {
            command.switch_and_sleep();
        }

        Ok(())
    }


    fn sleep_and_create_switch_command_if_running(&mut self, task_id: u64) -> KernelResult<Option<SwitchCommand>> {
        let (task, is_running) = self.sleep_and_check_running(task_id)?;

        if is_running {
            let next = self.next_run_task()?;

            Ok(Some(SwitchCommand::new(task, next)))
        } else {
            Ok(None)
        }
    }


    fn sleep_and_check_running(&mut self, task_id: u64) -> KernelResult<(&Task, bool)> {
        let task = self.find_mut(task_id)?;

        if task.status.is_running() {
            task.status = Status::Sleep;

            Ok((task, true))
        } else {
            task.status = Status::Sleep;

            Ok((task, false))
        }
    }


    pub(crate) fn find_mut(&mut self, task_id: u64) -> KernelResult<&mut Task> {
        self.tasks
            .iter_mut()
            .find(|task| task.id == task_id)
            .ok_or(error_not_found_task(task_id))
    }


    fn running_task(&mut self) -> KernelResult<&mut Task> {
        self.tasks
            .iter_mut()
            .find(|task| task.status.is_running())
            .ok_or(kernel_error!("No Task Running"))
    }


    fn find_where_sleeps(&mut self, task_id: u64) -> KernelResult<&mut Task> {
        self.tasks
            .iter_mut()
            .filter(|task| task.status.is_sleep())
            .find(|task| task.id == task_id)
            .ok_or(kernel_error!("Not found specified sleep the Task! id =  {task_id}"))
    }


    fn next_run_task(&mut self) -> KernelResult<&Task> {
        self
            .tasks
            .sort_by(|t1, t2| t2.priority_level.cmp(&t1.priority_level));

        self
            .tasks
            .iter()
            .find(|task| task.status.is_pending())
            .ok_or(kernel_error!("Couldn't find a task to run next"))
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
    use crate::task::status::Status::{Running, Sleep};
    use crate::task::Task;

    #[test]
    fn it_next_run_task() {
        let mut q = TaskList::new();
        q.push(Task::new(0, PriorityLevel::new(3)));
        q.push(Task::new(1, PriorityLevel::new(1)));

        let priority_level = q
            .next_run_task()
            .map(|task| task.priority_level)
            .unwrap();

        assert_eq!(priority_level, PriorityLevel::new(3));
    }


    #[test]
    fn it_next_switch_command() {
        let mut q = TaskList::new();
        q.push(Task::new(0, PriorityLevel::new(3)));
        q.push(Task::new(1, PriorityLevel::new(1)));
        q.push(Task::new(2, PriorityLevel::new(2)));
        q.tasks[1].status = Running;

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

        q.tasks[0].status = Sleep;
        q.tasks[2].status = Sleep;

        q.wakeup_at(0).unwrap();
        q.wakeup_at(3).unwrap();

        assert_eq!(q.tasks[0].status, Status::Pending);
        assert_eq!(q.tasks[2].status, Status::Pending);
    }


    #[test]
    fn it_sleep() {
        let mut q = TaskList::new();
        q.push(Task::new(0, PriorityLevel::new(1)));
        q.push(Task::new(1, PriorityLevel::new(0)));
        q.push(Task::new(3, PriorityLevel::new(2)));
        q.push(Task::new(2, PriorityLevel::new(3)));

        q.tasks[0].status = Running;
        q.tasks[2].status = Status::Pending;

        {
            let sleep_id0 = q.sleep_and_create_switch_command_if_running(0).unwrap();
            let sleep_id0 = sleep_id0.unwrap();

            assert_eq!(sleep_id0.running_id(), 0);
            assert_eq!(sleep_id0.next_id(), 2);
        }

        {
            let sleep_id3 = q.sleep_and_create_switch_command_if_running(3).unwrap();
            assert!(sleep_id3.is_none());
        }

        assert_eq!(q.find_mut(0).unwrap().status, Running);
        assert_eq!(q.find_mut(3).unwrap().status, Sleep);
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
        assert_eq!(task.id, 1);
        assert_eq!(task.priority_level, PriorityLevel::new(3));

        macro_rules! assert_task {
            ($index: literal, $id: literal, $level: literal) => {
                assert_eq!(q.tasks[$index].id, $id, "id");
                assert_eq!(q.tasks[$index].priority_level, PriorityLevel::new($level), "level");
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