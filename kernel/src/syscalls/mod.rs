use crate::{
    syscalls::{exit::exit, read_message::read_message, write_to_uart::write_to_uart},
    vectors::cpu_state::State,
};
use thiserror::Error;
pub mod exit;
pub mod read_message;
pub mod write_to_uart;

const SYSCALLS: &[fn(&mut State, u64) -> SyscallResult] =
    &[|_, _| None, write_to_uart, exit, read_message];

pub fn handle_syscall(state: &mut State, iss: u64, pid: u64) {
    let Some(syscall_fn) = SYSCALLS.get(iss as usize) else {
        return;
    };
    syscall_call(state, pid, *syscall_fn);
}

fn syscall_call(state: &mut State, pid: u64, syscall_fn: fn(&mut State, u64) -> SyscallResult) {
    let ret = syscall_fn(state, pid);
    if let Some(ret) = ret {
        match ret {
            Err(err) => {
                let code = err.code;
                state.x[0] = code;
            }
            Ok(ret) => {
                state.x[0] = ret;
            }
        }
    }
}

type SyscallResult = Option<Result<u64, SyscallError>>;

#[derive(Error, Debug)]
#[error("syscall error")]
pub struct SyscallError {
    code: u64,
}
