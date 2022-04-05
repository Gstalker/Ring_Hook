use std::path::Path;
use std::sync::Mutex;
use super::utils::{ENABLE_DALVIK_HOOK_CONFIG_FILE,ENABLE_NATIVE_HOOK_CONFIG_FILE};


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

    pub fn env(&self) -> Option<*mut ()>{
        if self.activate {
            return Some(self.env);
        }
        else{
            error!(target:"RING","InitHelper DEACTIVATED!");
            None
        }
    }

    //// FIXME: SELinux Problem
    //// 由于SELinux限制，在prexxxx阶段无法读取/data/data目录下的文件。该问题需要修复
    pub fn read_config(&self,application_name: String) -> Option<(bool,bool,String)> {
        if !self.activate {
            error!(target:"RING","InitHelper DEACTIVATED!");
            return None;
        }
        let root_path = String::from("/data/data/") + application_name.as_str();
        let native_config_path = root_path.clone() + &*ENABLE_NATIVE_HOOK_CONFIG_FILE.lock().unwrap().clone().as_str();
        let dalvik_config_path = root_path.clone() + &*ENABLE_DALVIK_HOOK_CONFIG_FILE.lock().unwrap().clone().as_str();
        info!("native_config_path: {}", native_config_path.clone());
        info!("dalvik_config_path: {}", dalvik_config_path.clone());
        let native_config = match nix::unistd::access(native_config_path.as_str(),nix::unistd::AccessFlags::F_OK) {
            Ok(()) => true,
            Err(e) => {
                error!("Exception while accessing target file: {}",native_config_path.clone());
                error!("Cause: {}",e);
                let result = Path::new(native_config_path.clone().as_str()).exists();
                error!("rust Path::exist result: {}",result);
                result
            }
        };
        let dalvik_config = match nix::unistd::access(dalvik_config_path.as_str(),nix::unistd::AccessFlags::F_OK) {
            Ok(()) => true,
            Err(_) => false
        };
        return Some((native_config,dalvik_config,root_path));
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