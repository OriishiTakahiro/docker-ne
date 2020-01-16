extern crate serde_yaml;
extern crate clap;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::collections::HashMap;
use itertools::join;
use clap::{ App, Arg, SubCommand };

// use original modules
use network_runner::consts::{ NETWORK_STATE_FILE };
use network_runner::node::Nodes;
use network_runner::activate::activate_network;
use network_runner::deactivate::deactivate_network;
use network_runner::switch;

fn main() {

    let matches = App::new("network-runner")
                .version("0.1.0")
                .author("Oriishi Takahiro <takahiro0914@live.jp>")
                .about("Manage IPv6 network with docker and open v switch.")
                .subcommand(
                    SubCommand::with_name("run")
                    .about("Activate network with specified yaml.")
                    .arg(
                        Arg::with_name("file")
                        .help("Yaml file witch is specified network structure.")
                        .required(true)
                    )
                    .arg(
                        Arg::with_name("fhost")
                        .long("fhost")
                        .short("h")
                        .help("Hostname of fluentd.")
                        .takes_value(true)
                        .required(false)
                    )
                    .arg(
                        Arg::with_name("fport")
                        .long("fport")
                        .short("p")
                        .help("Port number of fluentd.")
                        .takes_value(true)
                        .required(false)
                    )
                 )
                .subcommand(
                    SubCommand::with_name("stop")
                    .about("Deactivate the network.")
                 )
                .subcommand(
                    SubCommand::with_name("nodes")
                    .about("Subcommands for manipulate containers.")
                    .subcommand(
                        SubCommand::with_name("ls")
                        .about("Show all nodes.")
                     )
                 )
                .subcommand(
                    SubCommand::with_name("switches")
                    .about("Subcommands for manipulate switches.")
                    .subcommand(
                        SubCommand::with_name("ls")
                        .about("Show all switches.")
                     )
                    .subcommand(
                        SubCommand::with_name("add")
                        .about("Add new switch.")
                        .arg(
                            Arg::with_name("name")
                            .help("Name of new switch.")
                            .required(true)
                        )
                     )
                    .subcommand(
                        SubCommand::with_name("link")
                        .about("Link a switch and a router.")
                        .arg(
                            Arg::with_name("switch")
                            .help("Name of the target switch.")
                            .required(true)
                        )
                        .arg(
                            Arg::with_name("node")
                            .long("node")
                            .short("n")
                            .help("Name of the target node.")
                            .takes_value(true)
                            .required(true)
                        )
                        .arg(
                            Arg::with_name("interface")
                            .long("interface")
                            .short("i")
                            .help("Name of the node's interface.")
                            .takes_value(true)
                            .required(true)
                        )
                        .arg(
                            Arg::with_name("ip6addr")
                            .long("ip6addr")
                            .short("6")
                            .help("IPv6 address for the interface.")
                            .takes_value(true)
                            .required(true)
                        )
                     )
                    .subcommand(
                        SubCommand::with_name("unlink")
                        .about("Remove link between a switch and a router.")
                        .arg(
                            Arg::with_name("switch")
                            .help("Name of the target switch.")
                            .required(true)
                        )
                        .arg(
                            Arg::with_name("node")
                            .short("n")
                            .help("Name of the target router.")
                            .takes_value(true)
                            .required(true)
                        )
                        .arg(
                            Arg::with_name("interface")
                            .long("interface")
                            .short("i")
                            .help("Name of the node's interface.")
                            .takes_value(true)
                            .required(true)
                        )
                     )
                 ).get_matches();

    // run subcommand
    if let Some(ref run_matches) = matches.subcommand_matches("run") {
        if let Some(file) = run_matches.value_of("file") {
            let nodes = match read_network_yaml(file.to_string()) {
               Ok(result) => result,
               Err(msg) => panic!(msg),
            };
            let nodes = match activate_network(nodes, run_matches) {
                Ok(nodes) => (nodes),
                Err(msg) => panic!(msg),
            };
            match write_network_yaml(NETWORK_STATE_FILE.to_string(), nodes, false) {
                Ok(_) => (),
                Err(msg) => panic!(msg),
            };
        }
        std::process::exit(0);
    }

    // stop subcommand
    if let Some(_) = matches.subcommand_matches("stop") {
        if Path::new(&NETWORK_STATE_FILE.to_string()).exists() {
            let nodes = match read_network_yaml(NETWORK_STATE_FILE.to_string()) {
               Ok(result) => result,
               Err(msg) => panic!(msg),
            };
            deactivate_network(nodes);
        } else {
            eprintln!("No network are running now.")
        }
        std::process::exit(0);
    }

    // nodes subcommand
    if let Some(ref nodes_matches) = matches.subcommand_matches("nodes") {
        // `nodes ls` command
        if let Some(_) = nodes_matches.subcommand_matches("ls") {
           let nodes = match read_network_yaml(NETWORK_STATE_FILE.to_string()) {
               Ok(result) => result,
               Err(msg) => panic!(msg),
           };
           for n in nodes {
               let mut ifaces_str: String = String::from("");
               for i in n.interfaces() {
                   ifaces_str.push_str( &format!("- {}({}) -> {}\n", i.name(), i.prefix(), i.switch()) );
               }
           println!( "{} ({}) \n{}", n.name(), n.image(), ifaces_str );
           }
            std::process::exit(0);
        }
        eprintln!("Please specify some sub commands for nodes. (ref. network-runner nodes help)");
    }

    // switches subcommand
    if let Some(ref switches_matches) = matches.subcommand_matches("switches") {
        // `switches ls` command
        if let Some(_) = switches_matches.subcommand_matches("ls") {
           let nodes = match read_network_yaml(NETWORK_STATE_FILE.to_string()) {
               Ok(result) => result,
               Err(msg) => panic!(msg),
           };
           let mut switches :HashMap<String, Vec<String>> = HashMap::new();
           for n in nodes {
               for i in n.interfaces() {
                   let ref sname = i.switch().to_string();
                   let mut vec = match switches.get(sname) {
                       Some(e) => e.clone(),
                       None => Vec::new(),
                   };
                   vec.push(n.name().to_string());
                   switches.insert(sname.to_string(), vec);
               }
           }
           for (key, value) in switches.iter() {
               println!("{}: [{}]", key, join(value, ","))
           }
            std::process::exit(0);
        }
        // `switches add` command
        if let Some(switches_add_matches) = switches_matches.subcommand_matches("add") {
            if let Some(name) = switches_add_matches.value_of("name") {
                match switch::add(name.to_string()) {
                    Ok(_) => std::process::exit(1),
                    Err(msg) => panic!(msg),
                };
            } else {
                eprintln!("This command take an argument `switch name`.");
                std::process::exit(1);
            }
        }
        // `switches link` command
        if let Some(switches_link_matches) = switches_matches.subcommand_matches("link") {

            let nodes = match read_network_yaml(NETWORK_STATE_FILE.to_string()) {
               Ok(result) => result,
               Err(msg) => panic!(msg),
            };

            let result = match switch::link(switches_link_matches, nodes) {
               Ok(result) => result,
               Err(msg) => panic!(msg),
            };

            match write_network_yaml(NETWORK_STATE_FILE.to_string(), result, true) {
                Ok(_) => (),
                Err(msg) => panic!(msg),
            }
            std::process::exit(0);
        }

        // `switches unlink` command
        if let Some(switches_unlink_matches) = switches_matches.subcommand_matches("unlink") {

            let nodes = match read_network_yaml(NETWORK_STATE_FILE.to_string()) {
               Ok(result) => result,
               Err(msg) => panic!(msg),
            };

            let result = match switch::unlink(switches_unlink_matches, nodes) {
               Ok(result) => result,
               Err(msg) => panic!(msg),
            };

            match write_network_yaml(NETWORK_STATE_FILE.to_string(), result, true) {
                Ok(_) => (),
                Err(msg) => panic!(msg),
            }
            std::process::exit(0);
        }
    }

}

fn read_network_yaml(filename: String) -> Result<Nodes, String> {

    if !Path::new(&filename).exists() {
        return Err( format!("A network file {} is not existed.", filename) )
    }

    let yaml_str = match fs::read_to_string(filename) {
        Ok(result) => result,
        Err(msg) => return Err(msg.to_string()),
    };

    let nodes = match serde_yaml::from_str::<Nodes>(&yaml_str) {
        Ok(result) => result,
        Err(msg) => return Err(msg.to_string()),
    };

    Ok(nodes)
}

fn write_network_yaml(filename: String, nodes: Nodes, overwrite: bool) -> Result<(), String> {

    if !overwrite && Path::new(&filename).exists() {
        return Err( format!("A network file {} is already existed.", filename) )
    }

    let mut file = match File::create(filename) {
        Ok(file) => file,
        Err(msg) => return Err(msg.to_string()),
    };

    let yaml_str = match serde_yaml::to_string(&nodes) {
        Ok(result) => result,
        Err(msg) => return Err(msg.to_string()),
    };

    println!("Network state is updated.\n{}", yaml_str);

    match file.write(yaml_str.as_bytes()) {
        Ok(_) => () ,
        Err(msg) => return Err(msg.to_string()),
    };

    Ok(())
}
