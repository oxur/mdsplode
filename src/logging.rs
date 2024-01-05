use anyhow::{anyhow, Error, Result};
use twyg::{self, out};

use crate::cli::opts::Opts;

pub fn setup(opts: Opts) -> Result<(), Error> {
    let cfg = twyg::Opts {
        coloured: !opts.no_colour,
        file: out::stdout(),
        level: Some(opts.log_level),
        report_caller: true,

        ..Default::default()
    };

    let result = match twyg::setup(cfg.clone()) {
        Err(e) => Err(anyhow!("{:}", e)),
        Ok(_) => Ok(()),
    };
    log::debug!("Logging configured with {:?}", cfg);
    result
}
