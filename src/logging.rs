use anyhow::{anyhow, Error, Result};

pub fn setup(log_level: String) -> Result<(), Error> {
    let cfg = twyg::LoggerOpts {
        coloured: true,
        file: None,
        level: log_level,
        report_caller: true,
    };
    match twyg::setup_logger(&cfg) {
        Err(e) => Err(anyhow!("{:}", e)),
        Ok(_) => Ok(()),
    }
}
