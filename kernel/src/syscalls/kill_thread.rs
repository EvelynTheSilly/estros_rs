/// interface:
/// x0: tid
/// x1: arg (passed to x0 on new thread)
///
/// errors:
/// 1: invalid tid
/// 2: proccess doesnt exist, shouldnt be possible
///
/// returns: 0
use crate::{
    scheduler::{CpuScheduler, PROCESS_MANAGER},
    syncronisation::Mutex,
    syscalls::{SyscallResult, syscall_err},
    vectors::cpu_state::State,
};

pub fn kill_thread(state: &mut State, pid: u64, _tid: u64) -> SyscallResult {
    PROCESS_MANAGER.lock(|process_manager| {
        let tid = state.x[0];
        let Ok(proc) = process_manager.get_process_mut(pid) else {
            return syscall_err(2);
        };
        if proc.threads.remove(tid).is_err() {
            return syscall_err(1);
        };
        Some(Ok(0))
    })
}
