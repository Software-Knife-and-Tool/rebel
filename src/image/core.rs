//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]

use {
    crate::image::{mu::Mu, repl::Repl},
    mu::{Mu as Mu_, Tag},
};

pub trait Core {
    fn version(&self) -> String;
    fn load(&self, _: &str) -> std::result::Result<bool, String>;
    fn eval_string(&self, _: String) -> std::result::Result<Tag, String>;
    fn read(&self, _: String) -> std::result::Result<Tag, String>;
    fn compile(&self, _: Tag) -> std::result::Result<Tag, String>;
    fn eval(&self, _: Tag) -> std::result::Result<Tag, String>;
    fn write(&self, _: Tag, _: bool) -> String;
    fn listener(&self);
}

impl Core for Mu {
    fn version(&self) -> String {
        Mu_::version().into()
    }

    fn load(&self, path: &str) -> std::result::Result<bool, String> {
        match Mu_::load(self.env.env, path) {
            Ok(_) => Ok(true),
            Err(ex) => Err(Mu_::exception_string(self.env.env, ex)),
        }
    }

    fn eval_string(&self, form: String) -> std::result::Result<Tag, String> {
        self.eval(self.compile(self.read(form)?)?)
    }

    fn read(&self, form: String) -> std::result::Result<Tag, String> {
        match Mu_::read_str(self.env.env, &form) {
            Ok(tag) => Ok(tag),
            Err(ex) => Err(Mu_::exception_string(self.env.env, ex)),
        }
    }

    fn compile(&self, form: Tag) -> std::result::Result<Tag, String> {
        match Mu_::compile(self.env.env, form) {
            Ok(tag) => Ok(tag),
            Err(ex) => Err(Mu_::exception_string(self.env.env, ex)),
        }
    }

    fn eval(&self, form: Tag) -> std::result::Result<Tag, String> {
        match Mu_::eval(self.env.env, form) {
            Ok(tag) => Ok(tag),
            Err(ex) => Err(Mu_::exception_string(self.env.env, ex)),
        }
    }

    fn write(&self, form: Tag, escapep: bool) -> String {
        Mu_::write_to_string(self.env.env, form, escapep)
    }

    fn listener(&self) {
        Repl::listener(self).expect("listener: listener error");
    }
}
