//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]
#![allow(unused)]
use {
    crate::image::{config::Config, env_::Env_, repl::Repl},
    mu::{Mu, Tag},
};

pub trait Inspector {
    fn inspect(&self, _: Tag) -> String;
}

impl Inspector for Mu {
    fn inspect(&self, _form: Tag) -> String {
        String::new()
    }
}
