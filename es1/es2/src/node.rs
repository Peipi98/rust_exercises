use std::string::String;
use clap::builder::Str;

#[derive(Debug)]
pub struct Node {
    name: String,
    size: u32,
    count: u32,
}
impl Node {
    pub fn new(name: String) -> Node {
        Node {name, size: 0, count: 0 }
    }

    pub fn size(self, size: u32) -> Node {
        Node {name: self.name, size, count: self.count}
    }

    pub fn count(self, count: u32) -> Node {
        Node {name: self.name, size: self.size, count}
    }

    pub fn to_string(&self) -> String {
        let mut str = String::new();
        str.push_str("name:");
        str.push_str(self.name.as_str());
        str.push_str(" size:");
        str.push_str(self.size.to_string().as_str());
        str.push_str(" count:");
        str.push_str(self.count.to_string().as_str());
        str
    }

    pub fn grow(&mut self) {
        self.size += 1;
    }

    pub fn inc(&mut self) {
        self.count += 1;
    }
}
