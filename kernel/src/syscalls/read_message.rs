/// interface:
/// x0: mid
/// x1: pointer
/// x2: len
/// contract:
/// it will read LEN bytes from MID to POINTER
use crate::{
    scheduler::{CpuScheduler, PROCESS_MANAGER},
    syncronisation::Mutex,
    syscalls::{SyscallError, SyscallResult},
    vectors::cpu_state::State,
};

pub fn read_message(state: &mut State, pid: u64) -> SyscallResult {
    let mid = state.x[0];
    let process_pointer = state.x[1];
    let len = state.x[2];
    PROCESS_MANAGER.lock(|manager| {
        let Ok(process) = manager.get_process_mut(pid) else {
            return None;
        };
        let Ok(buff) = process.message_store.read_message(mid, len as usize) else {
            return Some(Err(SyscallError { code: 1 }));
        };
        let read = buff.len() as u64;
        if process.mem_write(process_pointer as usize, buff).is_err() {
            Some(Err(SyscallError { code: 2 }))
        } else {
            Some(Ok(read))
        }
    })
}
