use anyhow::bail;
use anyhow::Context;
use anyhow::Ok;
use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use toml_edit::Document;

#[derive(Debug, Deserialize)]
pub enum Feature {
    Navbar,
    Tags,
    Postlisting,
}

#[derive(Debug, Default, Deserialize)]
pub struct BlogConfig {
    pub blog_author: String,
    pub blog_copyright_year: i64,
    pub blog_email: String,
    pub features: Vec<Feature>,
}

pub fn parse_config(config_path: &PathBuf) -> Result<BlogConfig> {
    let contents = fs::read_to_string(config_path).context("Failed to open config file")?;
    let document = Document::from_str(&contents).context("Failed to parse toml file")?;

    let cfg = BlogConfig {
        blog_author: conf_get_string(&document, "general", "blog_author")?,
        blog_copyright_year: conf_get_integer(&document, "general", "blog_copyright_year")?,
        blog_email: conf_get_string(&document, "general", "blog_email")?,
        features: conf_get_features(&document)?,
    };

    Ok(cfg)
}

pub fn conf_get_features(doc: &Document) -> anyhow::Result<Vec<Feature>> {
    Ok(conf_get_string_array(doc, "general", "features")?
        .iter()
        .map(|s| {
            Ok(match s.as_str() {
                "navbar" => Feature::Navbar,
                "tags" => Feature::Tags,
                "postlisting" => Feature::Postlisting,
                _ => bail!("Invalid feature '{s}'"),
            })
        })
        .collect::<Result<Vec<Feature>>>()?)
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

pub fn conf_get_string_array(
    doc: &Document,
    table: &str,
    key: &str,
) -> anyhow::Result<Vec<String>> {
    match doc.get(table) {
        Some(t) => match t.as_table() {
            Some(t) => match t.get(key) {
                Some(v) => match v.as_array() {
                    Some(array) => {
                        let mut strings = Vec::new();
                        for element in array {
                            if !element.is_str() {
                                bail!("Element in array '{key}' in table '{table}' is not of type string");
                            } else {
                                strings.push(element.as_str().unwrap().to_owned());
                            }
                        }
                        Ok(strings)
                    }
                    None => bail!("Key '{key}' in table '{table}' is not of type array"),
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
    use crate::config::conf_get_string_array;

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

    #[test]
    fn conf_get_string_array_valid() {
        let document = Document::from_str("[general]\ntest = [\"A\",\"B\",\"C\"]").unwrap();
        let value = conf_get_string_array(&document, "general", "test");
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), ["A", "B", "C"]);
    }

    #[test]
    fn conf_get_string_array_table_does_not_exist() {
        let document = Document::from_str("[general]\ntest = 123").unwrap();
        let value = conf_get_string_array(&document, "table1", "test");
        assert!(value.is_err());
        assert_eq!(
            value.unwrap_err().to_string(),
            "Config does not contain table 'table1'"
        );
    }

    #[test]
    fn conf_get_string_array_key_does_not_exist() {
        let document = Document::from_str("[general]\ntest = 123").unwrap();
        let value = conf_get_string_array(&document, "general", "key1");
        assert!(value.is_err());
        assert_eq!(
            value.unwrap_err().to_string(),
            "Config does not contain key 'key1' in table 'general'"
        );
    }

    #[test]
    fn conf_get_string_array_key_is_wrong_type() {
        let document = Document::from_str("[general]\ntest = 123").unwrap();
        let value = conf_get_string_array(&document, "general", "test");
        assert!(value.is_err());
        assert_eq!(
            value.unwrap_err().to_string(),
            "Key 'test' in table 'general' is not of type array"
        );
    }

    #[test]
    fn conf_get_string_array_array_elem_is_wrong_type() {
        let document = Document::from_str("[general]\ntest = [\"A\", \"B\", 12]").unwrap();
        let value = conf_get_string_array(&document, "general", "test");
        assert!(value.is_err());
        assert_eq!(
            value.unwrap_err().to_string(),
            "Element in array 'test' in table 'general' is not of type string"
        );
    }
}
