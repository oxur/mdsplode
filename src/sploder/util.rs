use anyhow::{anyhow, Error, Result};
use serde_json::Value;

pub fn pretty_print(json: String) -> Result<String, Error> {
    match serde_json::from_str::<Value>(json.as_str()) {
        Ok(obj) => match serde_json::to_string_pretty(&obj) {
            Ok(result) => Ok(result),
            Err(e) => Err(anyhow!(
                "Could not convert json fragment to pretty-printed string: {}",
                e
            )),
        },
        Err(e) => Err(anyhow!(
            "Could not convert json string back to object for pretty-printing: {}",
            e
        )),
    }
}

pub fn run_query(json: String, query: String) -> Result<String, jq_rs::Error> {
    jq_rs::run(query.as_str(), &json)
}
