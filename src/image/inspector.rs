//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use {
    crate::image::{config::Config, env_::Env_, repl},
    mu::{Mu, Tag},
};

pub struct Inspector {
    pub env: Env_,
}

impl Inspector {
    pub fn new() -> Self {
        let env = Env_::new(Config::new());

        Self { env }
    }

    pub fn version(&self) -> String {
        Mu::version().into()
    }

    pub fn load(&self, path: &str) -> bool {
        let env = self.env.env;

        Mu::load(env, path).unwrap()
    }

    pub fn read(&self, form: String) -> Tag {
        let env = self.env.env;

        Mu::read_str(env, &form).unwrap()
    }

    pub fn compile(&self, form: Tag) -> Tag {
        let env = self.env.env;

        Mu::compile(env, form).unwrap()
    }

    pub fn eval(&self, form: Tag) -> Tag {
        let env = self.env.env;

        Mu::eval(env, form).unwrap()
    }

    pub fn write(&self, form: Tag, escapep: bool) -> String {
        let env = self.env.env;

        Mu::write_to_string(env, form, escapep)
    }

    pub fn inspect(&self, _form: Tag) -> String {
        String::new()
    }

    pub fn listener(&self) {
        repl::listener(&self.env).expect("listener: listener error");
    }
}
