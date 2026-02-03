pub mod cpu_state;

core::arch::global_asm!(include_str!("vectors.S"));

macro_rules! panic_function {
    ($func_name:ident) => {
        #[unsafe(no_mangle)]
        extern "C" fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string.
            panic!("{} triggered", stringify!($func_name));
        }
    };
}

/// adds a `panic_function!` for its arg_sync, arg_irq, etc...
macro_rules! panicing_vector_handler_block {
    ($func_name:ident) => {
        panic_function!(${concat($func_name, _sync_handler)});
        panic_function!(${concat($func_name, _irq_handler)});
        panic_function!(${concat($func_name, _fiq_handler)});
        panic_function!(${concat($func_name, _serror_handler)});
    };
}

panicing_vector_handler_block!(el1_sp0);
panicing_vector_handler_block!(el1_spx);
panicing_vector_handler_block!(el0_aarch64);
panicing_vector_handler_block!(el0_aarch32);
