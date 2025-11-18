//  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]
use {
    json::{self, JsonValue},
    std::fs,
    std::path::PathBuf,
};

#[derive(Debug, Clone)]
pub enum Config {
    Json(JsonValue),
    None,
}

/*
pub enum JsonValue {
    Null,
    Short(Short),
    String(String),
    Number(Number),
    Boolean(bool),
    Object(Object),
    Array(Vec<JsonValue>),
}
*/

#[derive(Debug, Clone)]
pub enum ConfigOpt {
    Array(Vec<ConfigOpt>),
    Boolean(bool),
    Null,
    Number(i32),
    Object(String),
    String(String),
}

impl Config {
    // if we have a .rebel in the current directory, use it.
    // otherwise, see if there's one in the home directory
    pub fn new() -> Self {
        let mut home_path = PathBuf::new();

        home_path.push(std::env::home_dir().unwrap().as_path());
        home_path.push(".rebel");

        let mut cwd_path = PathBuf::new();

        cwd_path.push(std::env::current_dir().unwrap().as_path());
        cwd_path.push(".rebel");

        match fs::read_to_string(cwd_path) {
            Ok(json) => match json::parse(&json) {
                Ok(opts) => Config::Json(opts),
                Err(_) => Config::None,
            },
            Err(_) => match fs::read_to_string(home_path) {
                Ok(json) => match json::parse(&json) {
                    Ok(opts) => Config::Json(opts),
                    Err(_) => {
                        eprintln!("rebel: failed to parse config JSON, using null config");
                        Config::None
                    }
                },
                Err(_) => Config::None,
            },
        }
    }

    fn json_opt(&self, value: &JsonValue) -> ConfigOpt {
        match value {
            JsonValue::Short(str) => ConfigOpt::String(str.as_str().to_string()),
            JsonValue::String(str) => ConfigOpt::String(str.clone()),
            JsonValue::Object(obj) => ConfigOpt::Object(obj.dump()),
            JsonValue::Boolean(bool) => ConfigOpt::Boolean(*bool),
            JsonValue::Number(_number) => panic!(),
            JsonValue::Array(arr) => ConfigOpt::Array(
                arr.iter()
                    .map(|json_value| self.json_opt(json_value))
                    .collect::<Vec<ConfigOpt>>(),
            ),
            JsonValue::Null => ConfigOpt::Null,
        }
    }

    pub fn map(&self, key: &str) -> Option<String> {
        match self {
            Config::Json(opts) => match &opts[key] {
                JsonValue::Short(str) => Some(str.as_str().to_string()),
                JsonValue::String(str) => Some(str.as_str().to_string()),
                JsonValue::Object(obj) => Some(obj.dump()),
                JsonValue::Array(arr) => Some(
                    arr.iter()
                        .map(|module| match module {
                            JsonValue::Short(str) => str.as_str().to_string(),
                            JsonValue::String(str) => str.as_str().to_string(),
                            _ => panic!(),
                        })
                        .collect::<String>(),
                ),
                _ => None,
            },
            Config::None => None,
        }
    }

    pub fn map_opt(&self, key: &str) -> Option<ConfigOpt> {
        match self {
            Config::Json(opts) => Some(self.json_opt(&opts[key])),
            Config::None => None,
        }
    }
}
