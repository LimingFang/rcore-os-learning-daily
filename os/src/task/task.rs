use super::TaskContext;

pub struct TaskControlBlk {
    pub ctx: TaskContext,
    pub st: TaskStatus,
}

impl TaskControlBlk {
    pub fn Init() -> Self {
        TaskControlBlk {
            ctx: TaskContext::Init(),
            st: TaskStatus::UnInit,
        }
    }
}

#[derive(PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exit,
}
