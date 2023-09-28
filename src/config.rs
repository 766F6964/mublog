use anyhow::bail;
use anyhow::Context;
use anyhow::Ok;
use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use toml_edit::Document;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub blog_author: String,
    pub blog_copyright_year: i64,
    pub blog_email: String,
    // general: General,
}

pub fn parse_config(config_path: &PathBuf) -> Result<Config> {
    let contents = fs::read_to_string(config_path).context("Failed to open config file")?;
    let document = Document::from_str(&contents).context("Failed to parse toml file")?;

    let cfg = Config {
        blog_author: conf_get_string(&document, "general", "blog_author")?,
        blog_copyright_year: conf_get_integer(&document, "general", "blog_copyright_year")?,
        blog_email: conf_get_string(&document, "general", "blog_email")?,
        // blog_author: conf_get_string(&document, "general", "blog_author")?,
    };

    Ok(cfg)
}

pub fn conf_get_string(doc: &Document, table: &str, key: &str) -> anyhow::Result<String> {
    match doc.get(table) {
        Some(t) => match t.as_table() {
            Some(t) => match t.get(key) {
                Some(v) => match v.as_str() {
                    Some(v) => Ok(v.to_owned()),
                    None => bail!("Key '{key}' in table '{table}' is not of type string"),
                },
                None => bail!("Config does not contain key '{key}' in table '{table}'"),
            },
            None => bail!("Config does not contain table '{table}'"),
        },
        None => bail!("Config does not contain table '{table}'"),
    }
}

pub fn conf_get_integer(doc: &Document, table: &str, key: &str) -> anyhow::Result<i64> {
    match doc.get(table) {
        Some(t) => match t.as_table() {
            Some(t) => match t.get(key) {
                Some(v) => match v.as_integer() {
                    Some(v) => Ok(v),
                    None => bail!("Key '{key}' in table '{table}' is not of type integer"),
                },
                None => bail!("Config does not contain key '{key}' in table '{table}'"),
            },
            None => bail!("Config does not contain table '{table}'"),
        },
        None => bail!("Config does not contain table '{table}'"),
    }
}

pub fn conf_get_bool(doc: &Document, table: &str, key: &str) -> anyhow::Result<bool> {
    match doc.get(table) {
        Some(t) => match t.as_table() {
            Some(t) => match t.get(key) {
                Some(v) => match v.as_bool() {
                    Some(v) => Ok(v),
                    None => bail!("Key '{key}' in table '{table}' is not of type bool"),
                },
                None => bail!("Config does not contain key '{key}' in table '{table}'"),
            },
            None => bail!("Config does not contain table '{table}'"),
        },
        None => bail!("Config does not contain table '{table}'"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn conf_get_string_valid() {
        let document = Document::from_str("[general]\ntest = \"hello\"").unwrap();
        let value = conf_get_string(&document, "general", "test");
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), "hello");
    }

    #[test]
    fn conf_get_string_table_does_not_exist() {
        let document = Document::from_str("[general]\ntest = \"hello\"").unwrap();
        let value = conf_get_string(&document, "table1", "test");
        assert!(value.is_err());
        assert_eq!(
            value.unwrap_err().to_string(),
            "Config does not contain table 'table1'"
        );
    }

    #[test]
    fn conf_get_string_key_does_not_exist() {
        let document = Document::from_str("[general]\ntest = \"hello\"").unwrap();
        let value = conf_get_string(&document, "general", "key1");
        assert!(value.is_err());
        assert_eq!(
            value.unwrap_err().to_string(),
            "Config does not contain key 'key1' in table 'general'"
        );
    }

    #[test]
    fn conf_get_string_key_is_wrong_type() {
        let document = Document::from_str("[general]\ntest = 123").unwrap();
        let value = conf_get_string(&document, "general", "test");
        assert!(value.is_err());
        assert_eq!(
            value.unwrap_err().to_string(),
            "Key 'test' in table 'general' is not of type string"
        );
    }
    #[test]
    fn conf_get_integer_valid() {
        let document = Document::from_str("[general]\ntest = 123").unwrap();
        let value = conf_get_integer(&document, "general", "test");
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), 123);
    }

    #[test]
    fn conf_get_integer_table_does_not_exist() {
        let document = Document::from_str("[general]\ntest = 123").unwrap();
        let value = conf_get_integer(&document, "table1", "test");
        assert!(value.is_err());
        assert_eq!(
            value.unwrap_err().to_string(),
            "Config does not contain table 'table1'"
        );
    }

    #[test]
    fn conf_get_integer_key_does_not_exist() {
        let document = Document::from_str("[general]\ntest = 123").unwrap();
        let value = conf_get_integer(&document, "general", "key1");
        assert!(value.is_err());
        assert_eq!(
            value.unwrap_err().to_string(),
            "Config does not contain key 'key1' in table 'general'"
        );
    }

    #[test]
    fn conf_get_integer_key_is_wrong_type() {
        let document = Document::from_str("[general]\ntest = \"123\"").unwrap();
        let value = conf_get_integer(&document, "general", "test");
        assert!(value.is_err());
        assert_eq!(
            value.unwrap_err().to_string(),
            "Key 'test' in table 'general' is not of type integer"
        );
    }

    #[test]
    fn conf_get_bool_valid() {
        let document = Document::from_str("[general]\ntest = true").unwrap();
        let value = conf_get_bool(&document, "general", "test");
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), true);
    }

    #[test]
    fn conf_get_bool_table_does_not_exist() {
        let document = Document::from_str("[general]\ntest = true").unwrap();
        let value = conf_get_bool(&document, "table1", "test");
        assert!(value.is_err());
        assert_eq!(
            value.unwrap_err().to_string(),
            "Config does not contain table 'table1'"
        );
    }

    #[test]
    fn conf_get_bool_key_does_not_exist() {
        let document = Document::from_str("[general]\ntest = true").unwrap();
        let value = conf_get_bool(&document, "general", "key1");
        assert!(value.is_err());
        assert_eq!(
            value.unwrap_err().to_string(),
            "Config does not contain key 'key1' in table 'general'"
        );
    }

    #[test]
    fn conf_get_bool_key_is_wrong_type() {
        let document = Document::from_str("[general]\ntest = \"123\"").unwrap();
        let value = conf_get_bool(&document, "general", "test");
        assert!(value.is_err());
        assert_eq!(
            value.unwrap_err().to_string(),
            "Key 'test' in table 'general' is not of type bool"
        );
    }
}
