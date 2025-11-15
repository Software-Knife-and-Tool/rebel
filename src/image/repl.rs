//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::image::mu::Mu,
    mu::{Condition, Mu as Mu_, Result},
};

pub trait Repl {
    fn listener(&self) -> Result;
}

impl Repl for Mu {
    fn listener(&self) -> Result {
        let env = self.env.env;
        let ns = self.env.ns.clone();

        let eof_value = Mu_::eval_str(env, "'%eof%")?;
        let flush_form = Mu_::compile(env, Mu_::read_str(env, "(mu:flush mu:*standard-output*)")?)?;

        let read_form = match ns.as_str() {
            "mu" => Mu_::read_str(env, "(mu:compile (mu:read mu:*standard-input* () '%eof%))")?,
            _ => Mu_::read_str(
                env,
                "(core:compile (core:read mu:*standard-input* () '%eof%))",
            )?,
        };

        let prompt = format!("{ns}> ");

        loop {
            Mu_::write_str(env, prompt.as_str(), Mu_::std_out())?;
            Mu_::eval(env, flush_form)?;

            match Mu_::eval(env, read_form) {
                Ok(expr) => {
                    if Mu_::eq(expr, eof_value) {
                        break Ok(eof_value);
                    }

                    #[allow(clippy::single_match)]
                    match Mu_::eval(env, expr) {
                        Ok(form) => {
                            Mu_::write(env, form, true, Mu_::std_out())?;
                            println!()
                        }
                        Err(e) => {
                            eprint!(
                                "exception raised by {}, {:?} condition on ",
                                Mu_::write_to_string(env, e.source, true),
                                e.condition,
                            );
                            Mu_::write(env, e.object, true, Mu_::err_out())?;
                            eprintln!()
                        }
                    }
                }
                Err(e) => {
                    if let Condition::Eof = e.condition {
                        std::process::exit(0);
                    } else {
                        eprint!(
                            "reader exception raised by {}, {:?} condition on ",
                            Mu_::write_to_string(env, e.source, true),
                            e.condition
                        );
                        Mu_::write(env, e.object, true, Mu_::err_out())?;
                        eprintln!()
                    }
                }
            }
        }
    }
}
