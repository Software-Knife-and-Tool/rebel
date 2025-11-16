//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use {
    crate::image::{config::Config, env_::Env_, repl::Repl},
    mu::Tag,
};

pub struct Mu {
    pub env: Env_,
    pub config: Config,
}

impl Mu {
    pub fn new() -> Self {
        let env = Env_::new(Config::new());
        let config = env.config.clone();

        Self { env, config }
    }

    pub fn version(&self) -> String {
        mu::Mu::version().into()
    }

    pub fn map_config(&self, name: &str) -> Option<String> {
        self.config.map(name)
    }

    pub fn load(&self, path: &str) -> std::result::Result<bool, String> {
        match mu::Mu::load(self.env.env, path) {
            Ok(_) => Ok(true),
            Err(ex) => Err(mu::Mu::exception_string(self.env.env, ex)),
        }
    }

    pub fn eval_string(&self, form: String) -> std::result::Result<Tag, String> {
        self.eval(self.compile(self.read(form)?)?)
    }

    pub fn read(&self, form: String) -> std::result::Result<Tag, String> {
        match mu::Mu::read_str(self.env.env, &form) {
            Ok(tag) => Ok(tag),
            Err(ex) => Err(mu::Mu::exception_string(self.env.env, ex)),
        }
    }

    pub fn compile(&self, form: Tag) -> std::result::Result<Tag, String> {
        match mu::Mu::compile(self.env.env, form) {
            Ok(tag) => Ok(tag),
            Err(ex) => Err(mu::Mu::exception_string(self.env.env, ex)),
        }
    }

    pub fn eval(&self, form: Tag) -> std::result::Result<Tag, String> {
        match mu::Mu::eval(self.env.env, form) {
            Ok(tag) => Ok(tag),
            Err(ex) => Err(mu::Mu::exception_string(self.env.env, ex)),
        }
    }

    pub fn write(&self, form: Tag, escapep: bool) -> String {
        mu::Mu::write_to_string(self.env.env, form, escapep)
    }

    pub fn listener(&self) {
        Repl::listener(self).expect("listener: listener error");
    }
}
