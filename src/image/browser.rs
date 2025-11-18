//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]
#![allow(unused)]
use {
    crate::{
        config::Config,
        image::{env_::Env_, repl::Repl},
    },
    mu::{Mu, Tag},
};

pub trait Browser {
    fn browse(&self, _: Tag) -> String;
}

impl Browser for Mu {
    fn browse(&self, _form: Tag) -> String {
        String::new()
    }
}
