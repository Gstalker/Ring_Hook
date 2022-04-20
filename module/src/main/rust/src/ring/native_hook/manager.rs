use std::sync::Mutex;
use std::mem::transmute;
use dobby_rs::hook;

use super::{
    NativeHookConfig,
    InlineHookConfig,
    NativeHookType
};

pub struct Manager {
    // 通过Config 来找到 Hooker
    native_hookers: Vec<NativeHookConfig>,
}

pub fn register_native_hooker(config: NativeHookConfig) {
    NATIVE_HOOKERS_MANAGER.lock().unwrap().register_native_hooker(config)
}

pub fn get_backup_trampoline_by_hook_function_addr(hooker_addr: usize) -> Option<usize> {
    NATIVE_HOOKERS_MANAGER.lock().unwrap().get_backup_trampoline_by_hook_function_addr(hooker_addr)
}

pub fn process_hookers() -> usize {
    NATIVE_HOOKERS_MANAGER.lock().unwrap().process_hookers()
}

impl Manager {
    pub fn new() -> Option<Self>{
        // 单例模式，全局只允许出现一个Manager
        let mut initialized = NATIVE_HOOKERS_MANAGER_INITIALIZED.lock().unwrap();
        if *initialized == false {
            *initialized = true;
            return Some(Self{
                native_hookers: Vec::new()
            });
        }
        None
    }

    pub fn register_native_hooker(&mut self, config: NativeHookConfig) {
        self.native_hookers.push(config);
    }

    pub fn get_backup_trampoline_by_hook_function_addr(&self, hooker_addr: usize) -> Option<usize> {
        let mut result = None;
        for i in &self.native_hookers {
            let hooker_addr_iter = match i.get_hook_config() {
                NativeHookType::Inline(inner) => {
                    inner.hook
                }
                NativeHookType::SymbolTableHijack(inner) => {
                    inner.hook
                }
                NativeHookType::PltTableHijack(inner) => {
                    inner.hook
                }
            };
            if hooker_addr == hooker_addr_iter {
                result = Some(i.get_backup_trampoline());
                break;
            }
        }
        return result;
    }

    //// Return: 返回成功注册的hooker数量
    pub fn process_hookers(&mut self) -> usize {
        let mut hook_count: usize = 0;
        for config in &mut self.native_hookers {
            // 已经hook过的不再进行hook
            // 状态设置为关闭的不再进行hook
            if config.get_activate_status() || !config.get_activate_switch() {
                continue;
            }
            match config.get_hook_config_mut() {
                NativeHookType::Inline(inner) => {
                    if let Some(backup) = Manager::do_inline_hook(inner) {
                        config.set_activate_status(true);
                        config.set_backup_trampoline(backup);
                        hook_count += 1;
                    }
                    else {
                        error!("Hook error!");
                    }
                }
                NativeHookType::SymbolTableHijack(_) => {
                    error!("SymTableHijack hook hasn't been implemented");
                }
                NativeHookType::PltTableHijack(_) => {
                    error!("PltTableHijack hook hook hasn't been implemented");
                }
            }
        }
        return hook_count;
    }

    unsafe fn do_inline_hook_inner(hook: usize, target: usize) -> Option<usize> {
        trace!("do inline_hook: 0x{:x}, 0x{:x}",hook, target);
        return match { dobby_rs::hook(transmute(target), transmute(hook)) } {
            Err(err) => {
                error!("inline hook error! {:?}",err);
                None
            }
            Ok(backup) => {
                Some(backup as usize)
            }
        }
    }

    fn do_inline_hook(config: &mut InlineHookConfig) -> Option<usize> {
        let hook = config.hook;
        if let Some(target) = config.target {
            return unsafe { Manager::do_inline_hook_inner(hook,target) };
        }
        else if let Some(symbol_info) = &mut config.symbol_info {
            let symbol_name = symbol_info.get_symbol_name();
            if let Some(image_name) = symbol_info.get_image_name() {
                trace!("reslove symbol {}::{}", image_name,symbol_name);
                let target = match dobby_rs::resolve_symbol(image_name.as_str(), symbol_name.as_str()) {
                    None => {
                        error!("Cannot resolve symbol: {}::{}",image_name,symbol_name);
                        return None;
                    }
                    Some(address) => {
                        trace!("symbol found! address: {:x}",address as usize);
                        address as usize
                    }
                };
                config.target = Some(target);
                return unsafe { Manager::do_inline_hook_inner(hook, target) };
            }
        }
        return None;
    }
}

unsafe impl Sync for Manager{}
unsafe impl Send for Manager{}

lazy_static!{

    static ref NATIVE_HOOKERS_MANAGER_INITIALIZED : Mutex<bool> = Mutex::new(false);
    pub static ref NATIVE_HOOKERS_MANAGER : Mutex<Manager> = Mutex::new(
        Manager::new().unwrap()
    );
}