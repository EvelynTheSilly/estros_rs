use crate::vectors::cpu_state;

#[derive(Clone)]
pub struct SchedulerThread {
    pub state: cpu_state::State,
}
