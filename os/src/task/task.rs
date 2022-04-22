use super::TaskContext;

#[derive(Clone, Copy, Debug)]
pub struct TaskControlBlk {
    pub ctx: TaskContext,
    pub st: TaskStatus,
}

impl TaskControlBlk {
    pub fn init() -> Self {
        TaskControlBlk {
            ctx: TaskContext::init(),
            st: TaskStatus::UnInit,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exit,
}
