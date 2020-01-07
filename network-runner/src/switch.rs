use std::process::Command;
use clap::ArgMatches;
use crate::node::{ Nodes, Interface };

pub fn add(name: String) -> Result<(), String> {

    let output = Command::new("ovs-vsctl").args(&[ "add-br", &name ]).output();

    match output {
        Ok(ovs_out) => if ovs_out.status.success() {
                Ok(())
            } else {
                Err(String::from_utf8(ovs_out.stderr).unwrap())
            },
        Err(msg) => Err(format!("{}", msg)),
    }
}

pub fn link(matches: &ArgMatches, nodes :Nodes) -> Result<Nodes, String> {

    let target_node = match matches.value_of("node"){
        Some(v) => v,
        None => return Err(String::from("No matches for node value")),
    };
    let switch = match matches.value_of("switch"){
        Some(v) => v,
        None => return Err(String::from("No matches for switch value")),
    };
    let interface = match matches.value_of("interface"){
        Some(v) => v,
        None => return Err(String::from("No matches for interface value")),
    };
    let ip6addr = match matches.value_of("ip6addr"){
        Some(v) => v,
        None => return Err(String::from("No matches for ip6addr value")),
    };

    let output = Command::new("ovs-docker")
        .args(&[
              "add-port",
              switch,
              interface,
              target_node,
              &format!("--ipaddress6={}", ip6addr),
        ]).output();

    match output {
        Ok(ovs_out) => if ovs_out.status.success() {
            let mut result = nodes.clone();
            for n in &mut result {
                if n.name() == target_node {
                    // add a new interface to target node
                    let mut new_if = Interface::new();
                    *new_if.mut_name() = interface.to_string();
                    *new_if.mut_switch() = switch.to_string();
                    *new_if.mut_prefix() = ip6addr.to_string();
                    n.mut_interfaces().push(new_if);
                }
            }
            Ok(result)
        } else {
            Err(String::from_utf8(ovs_out.stderr).unwrap())
        },
        Err(msg) => Err(format!("{}", msg)),
    }

}

pub fn unlink(matches: &ArgMatches, nodes :Nodes) -> Result<Nodes, String>{

    let target_node = match matches.value_of("node"){
        Some(v) => v,
        None => return Err(String::from("No matches for node value")),
    };
    let switch = match matches.value_of("switch"){
        Some(v) => v,
        None => return Err(String::from("No matches for switch value")),
    };
    let interface = match matches.value_of("interface"){
        Some(v) => v,
        None => return Err(String::from("No matches for interface value")),
    };


    let output = Command::new("ovs-docker")
        .args(&[
              "del-port",
              switch,
              interface,
              target_node,
        ]).output();

    match output {
        Ok(ovs_out) => if ovs_out.status.success() {
            let mut result = nodes.clone();
            'n_loop: for n in &mut result {
                let ifaces = n.mut_interfaces();
                for (i, v) in ifaces.iter().enumerate() {
                    if v.name() == interface {
                        ifaces.remove(i);
                        break 'n_loop;
                    }
                }
            }
            Ok(result)
        } else {
            Err(String::from_utf8(ovs_out.stderr).unwrap())
        },
        Err(msg) => Err(format!("{}", msg)),
    }
}
