use anyhow::{anyhow, Error, Result};

use crate::cli::opts::Opts;

pub fn setup(opts: Opts) -> Result<(), Error> {
    let cfg = twyg::Opts {
        coloured: !opts.no_colour,
        file: Some("stderr".to_string()),
        level: Some(opts.log_level),
        report_caller: true,

        ..Default::default()
    };
    owo_colors::set_override(cfg.coloured);

    let result = match twyg::setup(cfg.clone()) {
        Err(e) => Err(anyhow!("{:}", e)),
        Ok(_) => Ok(()),
    };
    log::debug!("Logging configured with {:?}", cfg);
    result
}
