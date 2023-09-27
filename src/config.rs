use anyhow::Context;
use anyhow::Ok;
use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use toml::from_str;
use toml::Table;
use toml::Value;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub blog_author: String,
    pub blog_copyright_year: i64,
    pub blog_email: String,
    pub features: Vec<String>,
}

pub fn parse_config(config_path: &PathBuf) -> Result<Config> {
    let cfg_str = fs::read_to_string(config_path).context("Failed to open mublog.conf")?;
    let table: Table = from_str(&cfg_str).context("Failed to parse mublog.conf file")?;

    // Parse General Section
    let table = table.get("general").context("Table 'general' not found")?;

    let blog_author = get_config_string(&table, "blog_author", "general")?;
    let blog_copyright_year = get_config_integer(&table, "blog_copyright_year", "general")?;
    let blog_email = get_config_string(&table, "blog_email", "general")?;
    let features = get_config_string_array(&table, "features", "general")?;

    let cfg = Config {
        blog_author,
        blog_copyright_year,
        blog_email,
        features,
    };

    Ok(cfg)
}

fn get_config_string(table: &Value, fieldname: &str, tablename: &str) -> Result<String> {
    let res = table
        .get(format!("{fieldname}"))
        .context(format!(
            "Field '{fieldname}' not found in table '{tablename}'"
        ))?
        .as_str()
        .context(format!("Field '{fieldname}' is not of type string"))?
        .to_owned();
    Ok(res)
}

fn get_config_integer(table: &Value, fieldname: &str, tablename: &str) -> Result<i64> {
    let res = table
        .get(format!("{fieldname}"))
        .context(format!(
            "Field '{fieldname}' not found in table '{tablename}'"
        ))?
        .as_integer()
        .context(format!("Field '{fieldname}' is not of type integer"))?
        .to_owned();
    Ok(res)
}

fn get_config_string_array(table: &Value, fieldname: &str, tablename: &str) -> Result<Vec<String>> {
    table
        .get(format!("{fieldname}"))
        .context(format!(
            "Field '{fieldname}' not found in table '{tablename}'"
        ))?
        .as_array()
        .context(format!("Field '{fieldname}' is not of type array"))?
        .iter()
        .map(|s| {
            Ok(s.as_str()
                .context(format!(
                    "Array elements of field '{fieldname}' are not of type string"
                ))?
                .to_owned())
        })
        .collect::<anyhow::Result<Vec<String>>>()
}
