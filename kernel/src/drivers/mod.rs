#![allow(dead_code)]
use crate::{
    rng::{RNG, Rng},
    syncronisation::{GlobalSharedLock, Mutex, NullLock},
};
use alloc::boxed::Box;

type MapType<K, V> = alloc::collections::BTreeMap<K, V>;

pub struct DriverManager {
    user_drivers: GlobalSharedLock<MapType<u64, Box<dyn UserSpaceDriver>>>,
}

impl DriverManager {
    #[allow(clippy::redundant_closure_for_method_calls)]
    pub fn register_driver(
        user_drivers: &mut MapType<u64, Box<dyn UserSpaceDriver>>,
        driver: Box<dyn UserSpaceDriver>,
    ) {
        let mut device_id = RNG.lock(|rng| rng.rand_u64());
        while !user_drivers.contains_key(&device_id) {
            device_id = RNG.lock(|rng| rng.rand_u64());
        }
        user_drivers.insert(device_id, driver);
    }
    pub const fn new() -> Self {
        Self {
            user_drivers: NullLock::new(MapType::<u64, Box<dyn UserSpaceDriver>>::new()),
        }
    }
}

pub static DRIVERMANAGER: DriverManager = DriverManager::new();

pub trait UserSpaceDriver: Send + Sync {}
