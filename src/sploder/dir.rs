use std::path;

use anyhow::{Error, Result};
use walkdir::WalkDir;

use crate::cli::opts::Opts;

use super::file;

pub fn process(in_path: String, out_path: String, cli: Opts) -> Result<(), Error> {
    for entry in WalkDir::new(in_path.clone())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let in_segs = entry
            .path()
            .iter()
            .map(|x| x.to_str().unwrap())
            .collect::<Vec<_>>();
        let tail: Vec<&str> = in_segs[1..].to_vec();
        let head: Vec<&str> = vec![out_path.as_str()];
        let out_segs = [head, tail].concat();
        let out_file = out_segs.join(path::MAIN_SEPARATOR_STR).to_string();
        file::process(in_path.to_string(), out_file, cli.clone())?;
    }
    Ok(())
}
