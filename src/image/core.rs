//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]

use {
    crate::image::{config::Config, env_::Env_, repl},
    mu::{Mu, Tag},
};

pub struct Core {
    pub env: Env_,
}

impl Core {
    pub fn new() -> Self {
        let env = Env_::new(Config::new());

        Self { env }
    }

    pub fn version(&self) -> String {
        Mu::version().into()
    }

    pub fn load(&self, path: &str) -> std::result::Result<bool, String> {
        match Mu::load(self.env.env, path) {
            Ok(_) => Ok(true),
            Err(ex) => Err(Mu::exception_string(self.env.env, ex)),
        }
    }

    pub fn eval_string(&self, form: String) -> std::result::Result<Tag, String> {
        self.eval(self.compile(self.read(form)?)?)
    }

    pub fn read(&self, form: String) -> std::result::Result<Tag, String> {
        match Mu::read_str(self.env.env, &form) {
            Ok(tag) => Ok(tag),
            Err(ex) => Err(Mu::exception_string(self.env.env, ex)),
        }
    }

    pub fn compile(&self, form: Tag) -> std::result::Result<Tag, String> {
        match Mu::compile(self.env.env, form) {
            Ok(tag) => Ok(tag),
            Err(ex) => Err(Mu::exception_string(self.env.env, ex)),
        }
    }

    pub fn eval(&self, form: Tag) -> std::result::Result<Tag, String> {
        match Mu::eval(self.env.env, form) {
            Ok(tag) => Ok(tag),
            Err(ex) => Err(Mu::exception_string(self.env.env, ex)),
        }
    }

    pub fn write(&self, form: Tag, escapep: bool) -> String {
        Mu::write_to_string(self.env.env, form, escapep)
    }

    pub fn inspect(&self, _form: Tag) -> String {
        String::new()
    }

    pub fn listener(&self) {
        repl::listener(&self.env).expect("listener: listener error");
    }
}
