// SPDX-License-Identifier: MIT

use cosmic::iced::window::Id;
use cosmic::iced_winit::commands::popup::{destroy_popup, get_popup};
use cosmic::prelude::*;
use cosmic::widget;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Default)]
pub struct AppModel {
    core: cosmic::Core,
    popup: Option<Id>,
    hosts: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    ConnectTo(String),
    Refresh,
    EditConfig,
}

fn ssh_config_path() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".ssh/config")
}

fn parse_ssh_config() -> Vec<String> {
    let content = match fs::read_to_string(ssh_config_path()) {
        Ok(c) => c,
        Err(_) => return vec![],
    };
    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.to_lowercase().starts_with("host ") {
                let host = trimmed[5..].trim().to_string();
                if host.contains('*') || host.contains('?') {
                    None
                } else {
                    Some(host)
                }
            } else {
                None
            }
        })
        .collect()
}

impl cosmic::Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "io.github.cosmic-ssh-launcher";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        let app = AppModel {
            core,
            popup: None,
            hosts: parse_ssh_config(),
        };
        (app, Task::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn view(&self) -> Element<'_, Self::Message> {
        self.core
            .applet
            .icon_button("utilities-terminal-symbolic")
            .on_press_down(Message::TogglePopup)
            .into()
    }

    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        // Action buttons on top
        let refresh_btn = cosmic::applet::menu_button(
            widget::row::with_children(vec![
                widget::icon::from_name("view-refresh-symbolic").size(16).icon().into(),
                widget::text("Neu laden").size(14).into(),
            ])
            .spacing(8),
        )
        .on_press(Message::Refresh);

        let edit_btn = cosmic::applet::menu_button(
            widget::row::with_children(vec![
                widget::icon::from_name("document-edit-symbolic").size(16).icon().into(),
                widget::text("Config bearbeiten").size(14).into(),
            ])
            .spacing(8),
        )
        .on_press(Message::EditConfig);

        // Host list
        let host_items: Vec<Element<'_, Self::Message>> = if self.hosts.is_empty() {
            vec![widget::text("Keine Hosts in ~/.ssh/config").size(14).into()]
        } else {
            self.hosts
                .iter()
                .map(|host| {
                    cosmic::applet::menu_button(widget::text(host.as_str()).size(14))
                        .on_press(Message::ConnectTo(host.clone()))
                        .into()
                })
                .collect()
        };

        let content = widget::column::with_children(vec![
            refresh_btn.into(),
            edit_btn.into(),
            widget::divider::horizontal::default().into(),
            widget::scrollable(widget::column::with_children(host_items)).into(),
        ]);

        let cosmic = self.core.system_theme().cosmic();
        let pad = cosmic::iced::Padding::from([cosmic.space_xxs() as u16, 0]);

        self.core
            .applet
            .popup_container(widget::container(content).padding(pad))
            .into()
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::Refresh => {
                self.hosts = parse_ssh_config();
            }
            Message::EditConfig => {
                let path = ssh_config_path();
                let _ = Command::new("cosmic-edit").arg(path).spawn();
                if let Some(p) = self.popup.take() {
                    return destroy_popup(p);
                }
            }
            Message::ConnectTo(host) => {
                let _ = Command::new("cosmic-term")
                    .args(["--", "bash", "-c", &format!("ssh {host}; exec bash")])
                    .spawn();
                if let Some(p) = self.popup.take() {
                    return destroy_popup(p);
                }
            }
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );
                    get_popup(popup_settings)
                };
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
        }
        Task::none()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}
