use crate::println;
use crate::scheduler::process::Process;
use crate::scheduler::{CpuScheduler, PROCESS_MANAGER};
use crate::syncronisation::Mutex;
use elf::ElfBytes;
use elf::endian::AnyEndian;
use limine::modules::InternalModule;
use limine::request::ModuleRequest;

#[used]
#[unsafe(link_section = ".requests")]
static INIT: ModuleRequest =
    ModuleRequest::new().with_internal_modules(&[&InternalModule::new().with_path(c"/init.elf")]);

pub fn launch_init() {
    let res = INIT.get_response();
    let init_file = res
        .unwrap()
        .modules()
        .iter()
        .find(|file| file.path() == c"/init.elf")
        .expect("couldnt find init file,\n add one to the boot partition at /init.elf");
    let init_bytes;
    unsafe {
        // SAFETY: we blindly trust the bootloader like sheep in a (gnu)hurd
        init_bytes = core::slice::from_raw_parts(init_file.addr(), init_file.size() as usize);
    };
    let init_elf = ElfBytes::<AnyEndian>::minimal_parse(init_bytes).expect("INVALID INIT FILE");
    let init_process = Process::from_elf(init_elf).expect("failed to map init process");
    let init_pid = PROCESS_MANAGER
        .lock(|manager| manager.launch_process(init_process))
        .expect("failed to launch init");
    println!("launched pid {}", init_pid);
}
