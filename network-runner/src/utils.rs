
use std::io::Write;
use std::fs::File;

use crate::consts::{ TMP_DIR, FileType };

pub fn write_file(filename: String, contents: String) {
    let mut f = match File::create(filename) {
        Ok(file) => file,
        Err(msg) => panic!(msg),
    };
    match f.write_all(contents.as_bytes()) {
        Ok(_) => "",
        Err(msg) => panic!(msg),
    };
}

pub fn get_filepath(nodename: &String, filetype: FileType, filename: &str) -> String {
    match filetype {
        FileType::Conf => format!("{}/{}/conf/{}", TMP_DIR, nodename, filename),
        FileType::Cert => format!("{}/{}/cert/{}", TMP_DIR, nodename, filename),
        FileType::SQL => format!("{}/{}/sql/{}", TMP_DIR, nodename, filename),
        FileType::ExecBin => format!("{}/{}/bin/{}", TMP_DIR, nodename, filename),
    }
}
