use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;
use std::collections::hash_map::ValuesMut;
use std::fs;
use std::path::PathBuf;
use toml::from_str;
use toml::map::Map;
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
    let cfg_general = table.get("general").context("Table 'general' not found")?;

    let blog_author = cfg_general
        .get("blog_author")
        .context("Field 'blog_author' not found in section 'general'")?
        .as_str()
        .context("Field 'blog_author' is not of type string")?
        .to_owned();

    let blog_copyright_year = cfg_general
        .get("blog_copyright_year")
        .context("Field 'blog_copyright_year' not found in section 'general'")?
        .as_integer()
        .context("Field 'blog_copyright_year' is not of type integer")?
        .to_owned();

    let blog_email = cfg_general
        .get("blog_email")
        .context("Field 'blog_email' not found in section 'general'")?
        .as_str()
        .context("Field 'blog_email' is not of type string")?
        .to_owned();

    let features = get_features(&cfg_general)?;
    let cfg = Config {
        blog_author,
        blog_copyright_year,
        blog_email,
        features,
    };

    Ok(cfg)
}

fn get_features(cfg_general: &Value) -> Result<Vec<String>> {
    cfg_general
        .get("features")
        .context("No field 'features' in [general]")?
        .as_array()
        .context("Field 'features' is not of type array")?
        .iter()
        .map(|s| {
            Ok(s.as_str()
                .context("Failed to convert array value to string")?
                .to_owned())
        })
        .collect::<anyhow::Result<Vec<String>>>()
}

// TODO: Add a read_config_string method
// TODO: Add a read_config_integer method

fn get_blog_author(cfg_general: &toml::Table) -> Result<String> {
    Ok(cfg_general
        .get("blog_author")
        .context("No field 'blog_author' in [general]")?
        .to_string())
}

fn get_blog_copyright_year(cfg_general: &toml::Table) -> Result<u32> {
    Ok(cfg_general
        .get("blog_copyright_year")
        .context("No field 'blog_copyright_year' in [general]")?
        .as_integer()
        .unwrap() as u32)
}

fn get_blog_email(cfg_general: &toml::Table) -> Result<String> {
    Ok(cfg_general
        .get("blog_email")
        .context("No field 'blog_email' in [general]")?
        .as_str()
        .unwrap()
        .to_string())
}

fn get_blog_features(cfg_general: &toml::Table) -> Result<Vec<String>> {
    Ok(cfg_general
        .get("features")
        .context("No field 'features' in [general]")?
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap().to_string())
        .collect())
}
