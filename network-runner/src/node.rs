extern crate serde_yaml;
extern crate askama;

use std::collections::HashSet;
use std::hash::{ Hash, Hasher };

use serde::{ Serialize, Deserialize };
use askama::Template;

// Interface struct
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Interface {
    name: String,
    prefix: String,
    area: String,
    switch: String,
    cost: String,
    priority: String,
}
// implement methods for Interface
impl Interface {
    pub fn new() -> Interface {
        Interface {
            name: String::from(""),
            prefix: String::from(""),
            area: String::from(""),
            switch: String::from(""),
            cost: String::from(""),
            priority: String::from(""),
        }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn prefix(&self) -> &String {
        &self.prefix
    }
    pub fn area(&self) -> &String {
        &self.area
    }
    pub fn switch(&self) -> &String {
        &self.switch
    }
    pub fn cost(&self) -> &String {
        &self.cost
    }
    pub fn priority(&self) -> &String {
        &self.priority
    }
    pub fn mut_name(&mut self) -> &mut String {
        &mut self.name
    }
    pub fn mut_prefix(&mut self) -> &mut String {
        &mut self.prefix
    }
    pub fn mut_switch(&mut self) -> &mut String {
        &mut self.switch
    }
}

// Node struct
// after implement Hash, PartialEq and Eq
#[derive(Serialize, Deserialize, Template, Clone, Debug)]
#[template(path="conf", print="none")]
pub struct Node {
    name: String,
    image: String,
    router_id: String,
    instance_id: String,
    is_ca: bool,
    is_registered: bool,
    set_dummycert: bool,
    enabled_prefixes: Vec<String>,
    interfaces: Vec<Interface>
} 
// implement traits for Node
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }

}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Node {}
// implement methods for Node
impl Node {
    pub fn new() -> Node {
        Node{
            name: String::from(""),
            image: String::from(""),
            router_id: String::from(""),
            instance_id: String::from(""),
            is_ca: true,
            is_registered: true,
            set_dummycert: true,
            enabled_prefixes: Vec::new(),
            interfaces: Vec::new(),
        }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn image(&self) -> &String {
        &self.image
    }
    pub fn router_id(&self) -> &String {
        &self.router_id
    }
    pub fn instance_id(&self) -> &String {
        &self.instance_id
    }
    pub fn is_ca(&self) -> bool {
        self.is_ca
    }
    pub fn is_registered(&self) -> bool {
        self.is_registered
    }
    pub fn set_dummycert(&self) -> bool {
        self.set_dummycert
    }
    pub fn enabled_prefixes(&self) -> &Vec<String> {
        &self.enabled_prefixes
    }
    pub fn interfaces(&self) -> &Vec<Interface> {
        &self.interfaces
    }
    pub fn mut_interfaces(&mut self) -> &mut Vec<Interface> {
        &mut self.interfaces
    }
}

pub type Nodes = Vec<Node>;

pub fn switch_set(nodes: &Nodes) -> HashSet<String> {
    let mut switches = HashSet::new();
    for n in nodes {
        for iface in n.interfaces() {
            switches.insert(iface.switch().clone());
        }
    };
    switches
}
