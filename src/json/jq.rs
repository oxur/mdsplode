use anyhow::{anyhow, Error, Result};

pub fn query(json: String, query: String) -> Result<String, Error> {
    match jq_rs::run(query.as_str(), &json) {
        Err(e) => Err(anyhow!(e.to_string())),
        Ok(r) => Ok(r),
    }
}
