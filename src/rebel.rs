//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! rebel
#[allow(unused_imports)]
#[rustfmt::skip]
use {
    crate::{
        image::{
            inspector::Inspector,
            mu::Mu
        },
    },
    iced::{
        widget::{
            button, column, container,
            horizontal_space, row, text,
            text_editor, tooltip,
        },
        Center,
        Element,
        Font,
        Length,
        Task,
        Theme,
        keyboard
    },
    std::{
        io,
        path::{Path, PathBuf},
        sync::Arc,
    },
};

pub struct Rebel {
    mu: Mu,
    view: text_editor::Content,
    error: Option<Error>,
    is_dirty: bool,
    path: Option<PathBuf>,
    source: text_editor::Content,
    status: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    FileSaved(Result<PathBuf, Error>),
    Browse,
    Clear,
    Eval,
    Load,
    Save,
}

impl Rebel {
    fn pad_lines(text: String, n: u32) -> String {
        let nlines: u32 = text
            .chars()
            .fold(0, |acc, ch| if ch == '\n' { acc + 1 } else { acc });

        if nlines > n {
            text
        } else {
            let padding = (0..n - nlines).map(|_| '\n').collect::<String>();

            text + &padding
        }
    }

    fn intro(image: &Mu) -> String {
        let version: String = format!(";;; mu {}\n", image.version());
        let core: String = ";;; /opt/mu/lib/core.sys loaded\n".into();

        let intro = version + &core;

        Self::pad_lines(intro, 30)
    }

    pub fn new() -> (Self, Task<Message>) {
        let mu = Mu::new();
        let intro = Self::intro(&mu);

        (
            Self {
                mu,
                path: None,
                source: text_editor::Content::with_text(&Self::pad_lines(String::new(), 30)),
                view: text_editor::Content::with_text(&intro),
                error: None,
                is_dirty: true,
                status: "initializing...".to_string(),
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        format!("rebel: {}", env!("CARGO_PKG_VERSION"))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Edit(action) => {
                self.is_dirty = self.is_dirty || action.is_edit();
                self.source.perform(action);
                self.error = None;

                Task::none()
            }
            Message::Browse => {
                self.path = None;
                self.source = text_editor::Content::new();
                self.is_dirty = true;

                Task::none()
            }
            Message::Clear => {
                self.path = None;
                self.source = text_editor::Content::with_text(&Self::pad_lines(String::new(), 30));
                self.view = text_editor::Content::with_text(&Self::pad_lines(String::new(), 30));
                self.is_dirty = true;

                Task::none()
            }
            Message::Eval => {
                let text = self.source.text();

                self.view = text_editor::Content::with_text(&Self::pad_lines(
                    match self.mu.eval_string(text) {
                        Ok(tag) => self.mu.write(tag, false),
                        Err(err) => err,
                    },
                    30,
                ));

                Task::none()
            }
            Message::Load => Task::perform(Self::pick_file(), Message::FileOpened),
            Message::FileOpened(Ok((path, content))) => {
                self.path = Some(path);
                self.source = text_editor::Content::with_text(&content);
                self.is_dirty = false;

                Task::none()
            }
            Message::FileOpened(Err(error)) => {
                self.error = Some(error);

                Task::none()
            }
            Message::Save => {
                let text = self.source.text();

                Task::perform(Self::save_buffer(None, text), Message::FileSaved)
            }
            Message::FileSaved(Ok(path)) => {
                self.path = Some(path);
                self.is_dirty = false;

                Task::none()
            }
            Message::FileSaved(Err(error)) => {
                self.error = Some(error);

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let controls = row![
            Self::action(Self::icon('\u{E01E}'), "clear buffer", Some(Message::Clear)),
            Self::action(Self::icon('\u{003A}'), "load buffer", Some(Message::Load)),
            Self::action(
                Self::icon('\u{e055}'),
                "save buffer",
                self.is_dirty.then_some(Message::Save)
            ),
            Self::action(Self::icon('\u{0034}'), "eval buffer", Some(Message::Eval)),
            Self::action(
                Self::icon('\u{E010}'),
                "browse selection",
                Some(Message::Browse)
            ),
        ]
        .spacing(10)
        .align_y(Center);

        let input = text_editor(&self.source)
            .placeholder("")
            .on_action(Message::Edit)
            .key_binding(|key_press| match key_press.key.as_ref() {
                keyboard::Key::Character("s") if key_press.modifiers.command() => {
                    Some(text_editor::Binding::Custom(Message::Save))
                }
                _ => text_editor::Binding::from_key_press(key_press),
            });

        let inspector = text_editor(&self.view)
            .placeholder("")
            .on_action(Message::Edit)
            .key_binding(|key_press| match key_press.key.as_ref() {
                keyboard::Key::Character("s") if key_press.modifiers.command() => {
                    Some(text_editor::Binding::Custom(Message::Save))
                }
                _ => text_editor::Binding::from_key_press(key_press),
            });

        self::column![
            controls,
            self::row![input, inspector].spacing(10),
            text(&self.status)
        ]
        .spacing(10)
        .padding(10)
        .into()
    }

    fn action<'a>(
        content: Element<'a, Message>,
        label: &'a str,
        on_press: Option<Message>,
    ) -> Element<'a, Message> {
        let is_disabled = on_press.is_none();

        tooltip(
            button(container(content).width(30).center_x(Length::Fixed(30.0)))
                .padding([5, 10])
                .style(move |theme: &Theme, _status| {
                    if is_disabled {
                        button::secondary(theme, button::Status::Disabled)
                    } else {
                        button::primary(theme, button::Status::Active)
                    }
                })
                .on_press_maybe(on_press),
            label,
            tooltip::Position::FollowCursor,
        )
        .style(container::rounded_box)
        .into()
    }

    fn icon<'a>(codepoint: char) -> Element<'a, Message> {
        const ICON_FONT: Font = Font::with_name("dripicons-v2");

        text(codepoint).font(ICON_FONT).into()
    }

    #[allow(dead_code)]
    fn default_file() -> PathBuf {
        PathBuf::from(format!("{}/default.l", env!("CARGO_MANIFEST_DIR")))
    }

    async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
        let handle = rfd::AsyncFileDialog::new()
            .set_title("Choose a text file")
            .pick_file()
            .await
            .ok_or(Error::DialogClosed)?;

        Self::load_buffer(handle.path().to_owned()).await
    }

    async fn load_buffer(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
        let contents = tokio::fs::read_to_string(&path)
            .await
            .map(Arc::new)
            .map_err(|error| error.kind())
            .map_err(Error::IOFailed)?;

        Ok((path, contents))
    }

    async fn save_buffer(path: Option<PathBuf>, text: String) -> Result<PathBuf, Error> {
        let path = if let Some(path) = path {
            path
        } else {
            rfd::AsyncFileDialog::new()
                .set_title("Choose a file name...")
                .save_file()
                .await
                .ok_or(Error::DialogClosed)
                .map(|handle| handle.path().to_owned())?
        };

        tokio::fs::write(&path, text)
            .await
            .map_err(|error| Error::IOFailed(error.kind()))?;

        Ok(path)
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Error {
    DialogClosed,
    IOFailed(io::ErrorKind),
}
