
pub struct Config{
    pub ring: bool,
    pub lunar: bool,
    pub app_data_path: String,
    pub dex_files_data: Vec<Vec<u8>>,
}

#[allow(dead_code)]
impl Config {
    pub fn default(app_data_path: String) -> Self{
        Config{
            ring: false,
            lunar: false,
            app_data_path,
            dex_files_data: Vec::new(),
        }
    }
    pub fn enable_native_hook(mut self, option: bool) -> Self {
        self.ring = option;
        self
    }

    pub fn enable_dalvik_hook(mut self, option: bool) -> Self {
        self.lunar = option;
        self
    }

    pub fn with_dex_files_data(mut self, dex_files_data: Vec<Vec<u8>>) -> Self {
        self.dex_files_data = dex_files_data;
        self
    }

    pub fn from(enable_native_hook: bool, enable_dalvik_hook: bool,app_data_path: String, dex_files_data: Vec<Vec<u8>>) -> Self {
        Self {
            ring: enable_native_hook,
            lunar: enable_dalvik_hook,
            app_data_path,
            dex_files_data,
        }
    }
}