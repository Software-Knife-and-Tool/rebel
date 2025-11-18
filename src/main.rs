//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! main
mod config;
mod image;
mod rebel;
mod status_line;

#[rustfmt::skip]
use {
    iced::{
        Font,
        Settings,
    },
    rebel::Rebel,
};

fn main() -> iced::Result {
    iced::application(Rebel::title, Rebel::update, Rebel::view)
        .settings(Settings {
            default_font: Font::MONOSPACE,
            fonts: vec![
                include_bytes!("../assets/fonts/dripicons-v2.ttf")
                    .as_slice()
                    .into(),
            ],
            ..Settings::default()
        })
        .run_with(Rebel::new)
}
