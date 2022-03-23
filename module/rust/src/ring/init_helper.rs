use jni::JNIEnv;
use std::sync::Mutex;

pub struct InitHelper{
    env: *mut (),
    activate: bool,
}

impl InitHelper{
    fn from(env: *mut ()) -> Self{
        InitHelper{
            env,
            activate: false,
        }
    }

    pub fn update_env(&mut self,env: *mut ()) {
        self.env = env;
        self.activate = true;
    }

    pub fn env(&self) -> Option<JNIEnv>{
        if self.activate {
            return Some(unsafe{JNIEnv::from_raw(self.env.cast()).unwrap()});
        }
        else{
            error!(target:"RING","InitHelper DEACTIVATED!");
            None
        }
    }

    pub fn deactivate(&mut self){
        self.activate = false;
    }
}

unsafe impl Send for InitHelper{}

lazy_static!{
    pub static ref INIT_HELPER: Mutex<InitHelper> = Mutex::new(InitHelper::from(
        1 as * mut ()),
    );
}