use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct SymbolTable<'a, T> {
    symbols: HashMap<String, T>,
    parent: Option<&'a SymbolTable<'a, T>>,
}

impl<'a, T> SymbolTable<'a, T> {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            parent: None,
        }
    }

    pub fn fork(&'a self) -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            parent: Some(self),
        }
    }

    pub fn set(&mut self, key: String, value: T) {
        self.symbols.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        if let Some(value) = self.symbols.get(key) {
            Some(value)
        } else if let Some(parent) = self.parent {
            parent.get(key)
        } else {
            None
        }
    }
}
