
use super::{
    config::Config,
    native_hook_instance,
    native_hook,
};
use std::sync::Mutex;
use crate::zygisk::macros::JNIEnv;


pub struct RingManager {
    config: Config,
    // native_hookers: Vec<>
}

impl RingManager{
    pub fn new() -> Option<Self> {
        let mut init_status = RING_MANAGER_INITIALIZED.lock().unwrap();
        if *init_status {
            return None
        }
        else{
            *init_status = true;
        }
        Some(Self{
            config: Config::default("None".to_string()),
        })
    }

    pub fn from_instance() -> &'static Mutex<Self> {
        &*RING_MANAGER
    }

    pub fn setup(&mut self,config: Config) {
        self.config = config;
        trace!("__________________________________");
        trace!("|    ConfigResult:");
        trace!("|    native_status: {}",self.config.ring);
        trace!("|    dalvik_status: {}",self.config.lunar);
        trace!("|    app_root_path: {}",self.config.app_data_path);
        trace!("__________________________________");
    }

    pub fn process(&mut self, env: &mut JNIEnv) -> bool{
        info!("Process Ring Hook!");
        if self.config.ring{
            native_hook_instance::register(env,self.config.app_data_path.clone());
            native_hook::Manager::from_instance().lock().unwrap().process_hookers();
        }
        self.config.ring && self.config.lunar
    }
}

unsafe impl Sync for RingManager {}
unsafe impl Send for RingManager {}

lazy_static!{
    static ref RING_MANAGER_INITIALIZED: Mutex<bool> = Mutex::new(false);
    pub static ref RING_MANAGER: Mutex<RingManager> = Mutex::new(RingManager::new().unwrap());
}