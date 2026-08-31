use crate::{
    scheduler::{CpuScheduler, PROCESS_MANAGER},
    syncronisation::Mutex,
    syscalls::SyscallResult,
    vectors::cpu_state::State,
};

pub fn exit(_state: &mut State, pid: u64, _tid: u64) -> SyscallResult {
    PROCESS_MANAGER.lock(|scheduler| {
        scheduler
            .kill_process(pid)
            .expect("we trust the pid from the syscall handler");
    });
    Some(Ok(1)) // doesnt really matter what is returned as the process is gone
}
