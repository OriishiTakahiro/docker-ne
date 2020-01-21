use std::io::Error;
use std::io::Write;
use std::fs;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;

use crate::consts::{ TMP_DIR, FileType };

pub fn write_file(filename: String, contents: String, permissions: u32) -> Result<(), Error> {
    // Create and write file
    let mut f = File::create(&filename)?;
    f.write_all(contents.as_bytes())?;
    // Set permissions
    let mut perms = f.metadata()?.permissions();
    perms.set_mode(permissions);
    fs::set_permissions(&filename, perms)?;

    Ok(())
}

pub fn get_filepath(nodename: &String, filetype: FileType, filename: &str) -> String {
    match filetype {
        FileType::Conf => format!("{}/{}/conf/{}", TMP_DIR, nodename, filename),
        FileType::Cert => format!("{}/{}/cert/{}", TMP_DIR, nodename, filename),
        FileType::SQL => format!("{}/{}/sql/{}", TMP_DIR, nodename, filename),
        FileType::ExecBin => format!("{}/{}/bin/{}", TMP_DIR, nodename, filename),
    }
}
