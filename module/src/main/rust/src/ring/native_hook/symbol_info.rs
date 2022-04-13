pub struct SymbolInfo {
    symbol_name: String,
    image_name: Option<String>,
}

impl SymbolInfo {
    pub fn from_symbol_name(symbol_name: String) -> Self{
        Self{
            symbol_name,
            image_name: None
        }
    }

    pub fn from_symbol_name_with_image_name(symbol_name: String, image_name: String) -> Self {
        Self{
            symbol_name,
            image_name: Some(image_name),
        }
    }

    pub fn get_symbol_name(&self) -> &String {
        &self.symbol_name
    }

    pub fn get_image_name(&self) -> &Option<String> {
        &self.image_name
    }

    pub fn set_image_name(&mut self, image_name: String) {
        self.image_name = Some(image_name);
    }
}