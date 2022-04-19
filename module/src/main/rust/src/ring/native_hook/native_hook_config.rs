use dobby_rs::hook;
use super::symbol_info::SymbolInfo;

pub struct NativeHookConfig{
    hook_type: NativeHookType,
    backup_trampoline: usize, // FIXME: Should be Option<usize>
    activate_status: bool,    // for hooker manager
    activate_switch: bool,    // for user
}

impl NativeHookConfig {
    pub fn inline_config(
        hook_function_addr: usize,
        target_function_addr: Option<usize>,
        symbol_info: Option<SymbolInfo>,
        activate_switch: bool
    ) -> Self {
        Self{
            hook_type: NativeHookType::Inline(InlineHookConfig{
                hook: hook_function_addr,
                target: target_function_addr,
                symbol_info
            }),
            activate_status: false,
            backup_trampoline: 0,
            activate_switch
        }
    }

    #[allow(unused_variables)]
    pub fn symtable_hijack_config(config: SymTableHijackConfig) {
        // FIXME: To be Implement
    }

    #[allow(unused_variables)]
    pub fn plt_table_hijack_config(config: PltTableHijackConfig) {
        // FIXME: To be Implement
    }
    pub fn get_hook_config_mut(&mut self) -> &mut NativeHookType{
        &mut self.hook_type
    }
    pub fn get_hook_config(&self) -> &NativeHookType{
        &self.hook_type
    }

    pub fn set_backup_trampoline(&mut self, backup: usize) {
        self.backup_trampoline = backup;
    }
    pub fn get_backup_trampoline(&self) -> usize { self.backup_trampoline }
    pub fn get_activate_status(&self) -> bool {
        self.activate_status
    }
    pub fn set_activate_status(&mut self, new_status: bool ) {
        if self.activate_status == new_status {
            warn!("Conflict status setting! Prev status: {}", self.activate_status);
        }
        self.activate_status = new_status;
    }
    pub fn set_activate_switch(&mut self, new_switch: bool) {
        self.activate_switch = new_switch
    }

    pub fn get_activate_switch(&self) -> bool {
        self.activate_switch
    }
}

pub enum NativeHookType{
    Inline(InlineHookConfig),
    SymbolTableHijack(SymTableHijackConfig),
    PltTableHijack(PltTableHijackConfig),
}

pub struct InlineHookConfig {
    pub hook: usize,
    pub target: Option<usize>,
    pub symbol_info: Option<SymbolInfo>,
}


//// 使用symbol table hijacking需要提供下面两个信息之一
pub struct SymTableHijackConfig {
    pub hook: usize,
    //// 通过目标函数地址，在目标lib的符号表中找到匹配地址，两者信息都提供时优先前者
    pub target: Option<usize>,
    //// 通过符号信息，在目标lib的符号表中找到匹配符号名
    pub symbol_info: Option<SymbolInfo>,
}

pub struct PltTableHijackConfig {
    pub hook: usize,
}