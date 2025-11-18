//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! status line
use crate::{
    config::ConfigOpt,
    image::mu::Mu,
};

pub struct StatusLine {
    mu_version: String,
    core_version: String,
    module_versions: String,
}

impl StatusLine {
    pub fn content(&self) -> String {
        format!("mu {}, {}{}", self.mu_version, self.core_version, self.module_versions)
    }

    pub fn new(mu: &Mu) -> Self {
        let mu_version: String = mu.version();

        let core_version: String = match mu.map_config("namespace") {
            Some(ns) => {                
                let version = mu.write(
                    mu.eval_string("(mu:cdr (core:assq 'version core:%sys-def))".into())
                        .unwrap(),
                    false,
                );
                format!("{ns} {version}")
            }
            None => "core.sys not loaded, browsing is disabled".into(),
        };

        let module_versions = match mu.map_config_opt("modules") {
            Some(opt) => match opt {
                ConfigOpt::Array(modules) => modules
                    .iter()
                    .map(|module| match module {
                        ConfigOpt::String(str) => {
                            let mut ns = String::from(str);
                            ns.truncate(str.len() - 4);
                            let version = mu.write(
                                mu.eval_string(format!("(mu:cdr (core:assq 'version {ns}:%sys-def))").into())
                                    .unwrap(),
                                false,
                            );
                            format!(", {ns} {version}")
                        },
                        _ => panic!(),
                    })
                    .collect::<String>(),
                _ => panic!(),
            }
            None => String::new(),
        };
                     
        Self {
            mu_version,
            core_version,
            module_versions,
        }
    }
}
