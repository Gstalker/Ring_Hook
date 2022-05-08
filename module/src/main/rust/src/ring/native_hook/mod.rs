mod manager;
mod symbol_info;
mod native_hook_config;

pub use symbol_info::SymbolInfo;
pub use manager::{
    register_native_hooker,
    get_backup_trampoline_by_hook_function_addr,
    process_hookers,
    resolve_symbol,
};
pub use native_hook_config::{
    NativeHookConfig,NativeHookType,InlineHookConfig,SymTableHijackConfig,
    PltTableHijackConfig,
};


