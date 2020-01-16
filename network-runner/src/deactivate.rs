use std::fs;
use std::process::Command;

// use original modules
use crate::node::{ Nodes, switch_set };
use crate::consts::TMP_DIR;

pub fn deactivate_network(nodes: Nodes) {

    // remove all conf file
    match fs::remove_dir_all( TMP_DIR ) {
        Ok(_) => (),
        Err(_) => (),
    };

    // remove all switches
    for s in switch_set(&nodes) {
        let ports = Command::new("ovs-vsctl").args(&[ "list-ports", &s]).output().unwrap().stdout;
        for p in String::from_utf8(ports).unwrap().split('\n') {
            Command::new("ovs-vsctl").args(&[ "del-port", &p]).output().unwrap();
            println!("port {} is removed", &p);
        }
        Command::new("ovs-vsctl").args(&[ "del-br", &s]).output().unwrap();
        println!("bridge {} is removed", &s);
    }

    // remove all containers
    for n in nodes {
        Command::new("docker").args(&[ "stop", &n.name() ]).output().unwrap();
        Command::new("docker").args(&[ "rm", &n.name() ]).output().unwrap();
        println!("container {} is removed", &n.name());
    }
}
