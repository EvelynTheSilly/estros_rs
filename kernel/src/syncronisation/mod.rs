//! provides a synchronisation interface,
//!
//! for now it does nothing however it will be useful to have everything ready for when i switch to a proper mutex implementation
//!
//! credits to [rust-raspberrypi-OS-tutorials](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/blob/master/04_safe_globals/src/synchronization.rs)

mod null_lock;
mod spin_lock;

/// Any object implementing this trait guarantees exclusive access to the data wrapped within
/// the Mutex for the duration of the provided closure.
pub trait Mutex {
    /// The type of the data that is wrapped by this mutex.
    type Data;

    /// Locks the mutex and grants the closure temporary mutable access to the wrapped data.
    fn lock<'a, R>(&'a self, f: impl FnOnce(&'a mut Self::Data) -> R) -> R;
}

/// type alias for the global lock used to static global objects
pub type GlobalSharedLock<T> = spin_lock::SpinLock<T>;
