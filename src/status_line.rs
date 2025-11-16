//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! status line
use crate::image::mu::Mu;

pub struct StatusLine {
    mu_version: String,
    core_version: String,
}

impl StatusLine {
    pub fn content(&self) -> String {
        format!("mu {}, {}", self.mu_version, self.core_version)
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
            None => "".into(),
        };

        Self {
            mu_version,
            core_version,
        }
    }
}
