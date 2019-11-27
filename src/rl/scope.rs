use crate::value;
use crate::tokenizer;
use std::collections::HashMap;

type Val = Box<dyn value::Value>;

pub struct Scope {
    variables: HashMap<String, Val>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            variables: HashMap::new(),
        }
    }

    pub fn get(&self, key: String) -> std::option::Option<&Val> {
        self.variables.get(&key)
    }

    pub fn put(&mut self, key: String, val: Val) {
        self.variables.insert(key, val);
    }

    pub fn do_file(&mut self, filename: String) {
        tokenizer::Tokenizer::from(filename).tokenize()
    }

    pub fn print(&self) {
        println!("Hello from the Engine!")
    }
}
