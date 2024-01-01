use std::{fs, path};

use anyhow::{anyhow, Error, Result};
use walkdir::WalkDir;

use crate::cli::opts::Opts;

use super::file;

pub fn process(in_path: String, out_path: String, cli: Opts) -> Result<(), Error> {
    for entry in WalkDir::new(in_path.clone())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();
        log::debug!("Entry path: {}", entry_path.display());
        if entry_path.is_dir() {
            log::debug!("Path is dir, skipping ...");
            continue;
        }
        let mut entry_file = "";
        match entry_path.to_str() {
            Some(filename) => {
                entry_file = filename;
                Ok(())
            }
            _ => Err(anyhow!("Couldn't get file name.")),
        }?;
        let mut entry_out_path = entry_path.to_path_buf();
        entry_out_path.set_extension("json");

        let in_segs = entry_out_path
            .iter()
            .map(|x| x.to_str().unwrap())
            .collect::<Vec<_>>();
        let segs_len = in_segs.len();
        let head: Vec<&str> = vec![out_path.as_str()];
        let tail: Vec<&str> = in_segs[1..].to_vec();
        let parent_tail: Vec<&str> = in_segs[1..segs_len - 1].to_vec();
        let parent_segs = [head.clone(), parent_tail].concat();
        let parent = parent_segs.join(path::MAIN_SEPARATOR_STR).to_string();
        log::debug!("Creating path: {} ...", parent);
        fs::create_dir_all(path::Path::new(parent.as_str()))?;
        let out_segs = [head, tail].concat();
        let out_file = out_segs.join(path::MAIN_SEPARATOR_STR).to_string();
        log::debug!(
            "Preparing to process file: {} -> {} ...",
            entry_file,
            out_file
        );
        file::process(entry_file.to_string(), out_file.clone(), cli.clone())?;
        log::debug!("Saved as: {}", out_file)
    }
    Ok(())
}
