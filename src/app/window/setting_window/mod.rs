use crate::app::window::{Window, WindowMessage};
use crate::widget::styles::container::{focusable, not_focused_rect_box};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::pane_grid::Axis;
use iced::widget::{
    button, pane_grid, pick_list, scrollable, space, text_input, Container, MouseArea,
};
use iced::window::{Id, Settings};
use iced::{widget, Element, Length, Task};
use prehnite_core::db::Database;
use prehnite_core::i18n::{i18n, i18n_w};
use prehnite_core::settings::registry::SettingRegistry;
use prehnite_core::settings::value::SettingValueType;
use prehnite_core::settings::{GlobalSettingKey, SettingCategory, SettingEntry, SettingKey};
use prehnite_core::widget::font::{ftext, get_font};
use prehnite_font_manager::get_global_font_list;
use std::collections::{HashMap, HashSet};
use tracing::error;

impl From<SettingWindowMessage> for WindowMessage {
    fn from(value: SettingWindowMessage) -> Self {
        WindowMessage::SettingWindowMessage(value)
    }
}

#[derive(Clone, Debug)]
pub enum SettingWindowMessage {
    CategorySelected(usize),
    ValueChanged(SettingKey, SettingValueType),
    Apply,
    EntrySearchTextChanged(String),
    None,
}

#[derive(Debug)]
enum SettingWindowPane {
    Editor,
    List,
}

#[derive(Debug)]
pub struct SettingWindow {
    window_id: Option<Id>,
    current_category_idx: usize,
    pane_state: pane_grid::State<SettingWindowPane>,
    values: HashMap<SettingKey, SettingValueType>,
    changed: HashSet<SettingKey>,
    entry_search_text: String,
}

impl SettingWindow {
    fn setting_list_pane(&self) -> Element<'_, WindowMessage> {
        iced::widget::column(
            SettingRegistry::get_categories()
                .iter()
                .enumerate()
                .filter(|(_, c)| c.category_name().contains(self.entry_search_text.as_str()))
                .map(|(i, v)| self.category_row(i, v)),
        )
        .height(Length::Fill)
        .into()
    }

    fn setting_edit_pane(&self) -> Element<'_, WindowMessage> {
        iced::widget::column(
            match SettingRegistry::get_categories().get(self.current_category_idx) {
                None => return i18n_w("unknown").into(),
                Some(v) => v
                    .entries()
                    .iter()
                    .filter(|v| v.get_is_visible())
                    .map(|v| self.setting_edit_row(v)),
            },
        )
        .into()
    }

    fn is_entry_enabled(key: SettingKey) -> bool {
        match key {
            SettingKey::Global(_) => true,
            SettingKey::Book(_) => Database::is_book_opened(),
        }
    }

    fn category_row(&self, id: usize, cate: &SettingCategory) -> Element<'_, WindowMessage> {
        MouseArea::new(
            Container::new(ftext(cate.category_name()))
                .padding(5)
                .width(Length::Fill)
                .style(focusable(id == self.current_category_idx)),
        )
        .on_press(SettingWindowMessage::CategorySelected(id).into())
        .into()
    }

    fn setting_edit_row(&self, entry: &SettingEntry) -> Element<'_, WindowMessage> {
        let key = entry.get_setting_key();
        let input: Element<'_, WindowMessage> = match entry.default_value() {
            SettingValueType::Bool(_) => widget::toggler(
                self.values
                    .get(&key)
                    .cloned()
                    .unwrap_or(entry.default_value().clone())
                    .get()
                    .unwrap_or_default(),
            )
            .on_toggle_maybe(
                Some(move |v: bool| SettingWindowMessage::ValueChanged(key, v.into()).into())
                    .filter(|_| Self::is_entry_enabled(key)),
            )
            .into(),
            SettingValueType::Int(_) => iced_aw::widget::number_input(
                &self
                    .values
                    .get(&key)
                    .cloned()
                    .unwrap_or(entry.default_value().clone())
                    .get()
                    .unwrap_or_default(),
                i64::MIN..i64::MAX,
                |_| SettingWindowMessage::None.into(),
            )
            .on_input_maybe(
                Some(move |v: i64| SettingWindowMessage::ValueChanged(key, v.into()).into())
                    .filter(|_| Self::is_entry_enabled(key)),
            )
            .font(get_font())
            .into(),
            SettingValueType::Float(_) => iced_aw::widget::number_input(
                &self
                    .values
                    .get(&key)
                    .cloned()
                    .unwrap_or(entry.default_value().clone())
                    .get()
                    .unwrap_or_default(),
                f64::MIN..f64::MAX,
                |_| SettingWindowMessage::None.into(),
            )
            .on_input_maybe(
                Some(move |v: f64| SettingWindowMessage::ValueChanged(key, v.into()).into())
                    .filter(|_| Self::is_entry_enabled(key)),
            )
            .font(get_font())
            .into(),
            SettingValueType::String(_) => match entry.get_selectable_values() {
                None => text_input(
                    "value",
                    &self
                        .values
                        .get(&key)
                        .cloned()
                        .unwrap_or(entry.default_value().clone())
                        .get::<String>()
                        .unwrap_or_default(),
                )
                .on_input_maybe(
                    Some(move |v: String| SettingWindowMessage::ValueChanged(key, v.into()).into())
                        .filter(|_| Self::is_entry_enabled(key)),
                )
                .font(get_font())
                .into(),
                Some(v) => {
                    if Self::is_entry_enabled(key) {
                        if key == GlobalSettingKey::Font.into() {
                            pick_list(
                                get_global_font_list().as_slice(),
                                self.values
                                    .get(&key)
                                    .cloned()
                                    .unwrap_or(entry.default_value().clone())
                                    .get::<String>(),
                                move |v| SettingWindowMessage::ValueChanged(key, v.into()).into(),
                            )
                            .font(get_font())
                            .into()
                        } else {
                            pick_list(
                                v,
                                self.values
                                    .get(&key)
                                    .cloned()
                                    .unwrap_or(entry.default_value().clone())
                                    .get::<String>(),
                                move |v| SettingWindowMessage::ValueChanged(key, v.into()).into(),
                            )
                            .font(get_font())
                            .into()
                        }
                    } else {
                        ftext("-").into()
                    }
                }
            },
        };
        Container::new(widget::row![
            ftext(entry.to_string())
                .height(Length::Fill)
                .align_y(Vertical::Center),
            space().width(Length::Fill),
            input
        ])
        .width(Length::Fill)
        .height(40)
        .padding(5)
        .into()
    }
}

impl Window for SettingWindow {
    fn new() -> SettingWindow
    where
        Self: Sized,
    {
        let (mut pane_state, pane) = pane_grid::State::new(SettingWindowPane::List);
        match pane_state.split(Axis::Vertical, pane, SettingWindowPane::Editor) {
            None => {}
            Some((_, split)) => pane_state.resize(split, 0.33),
        }
        Self {
            window_id: None,
            current_category_idx: 0,
            pane_state,
            values: SettingRegistry::get_values(),
            changed: Default::default(),
            entry_search_text: "".to_string(),
        }
    }

    fn update(&mut self, message: WindowMessage) -> Task<WindowMessage> {
        if let WindowMessage::SettingWindowMessage(message) = message {
            match message {
                SettingWindowMessage::CategorySelected(id) => {
                    self.current_category_idx = id;
                }
                SettingWindowMessage::ValueChanged(key, v) => {
                    self.changed.insert(key);
                    self.values.insert(key, v);
                }
                SettingWindowMessage::Apply => {
                    let values: Vec<(SettingKey, SettingValueType)> = self
                        .values
                        .clone()
                        .into_iter()
                        .filter(|(k, _)| self.changed.contains(k))
                        .collect();
                    return Task::future(async move {
                        for (k, v) in values {
                            SettingRegistry::immediate_apply(k, v).await;
                        }
                    })
                    .discard()
                    .chain(if self.changed.contains(&GlobalSettingKey::Font.into()) {
                        Task::done(WindowMessage::ReloadFont)
                    } else {
                        Task::none()
                    })
                    .chain(
                        if self.changed.contains(&GlobalSettingKey::Locale.into()) {
                            Task::done(WindowMessage::ReloadLanguage)
                        } else {
                            Task::none()
                        },
                    );
                }
                SettingWindowMessage::EntrySearchTextChanged(v) => {
                    self.entry_search_text = v;
                }
                SettingWindowMessage::None => {}
            }
        } else {
            error!("Invalid message received.");
        }
        Task::none()
    }

    fn view(&'_ self) -> Element<'_, WindowMessage> {
        widget::column![
            widget::pane_grid(&self.pane_state, |_, state, _| {
                pane_grid::Content::new(
                    scrollable(match state {
                        SettingWindowPane::List => Container::new(widget::column![
                            Container::new(
                                text_input(
                                    i18n("search").as_str(),
                                    self.entry_search_text.as_str()
                                )
                                .font(get_font())
                                .on_input(|v| {
                                    SettingWindowMessage::EntrySearchTextChanged(v).into()
                                })
                            )
                            .width(Length::Fill)
                            .padding(5),
                            self.setting_list_pane()
                        ])
                        .style(not_focused_rect_box),
                        SettingWindowPane::Editor => Container::new(widget::column![
                            Container::new(ftext(
                                SettingRegistry::get_categories()
                                    .get(self.current_category_idx)
                                    .map(|v| v.category_name())
                                    .unwrap_or(" ".to_string())
                            ))
                            .width(Length::Fill)
                            .padding(5),
                            self.setting_edit_pane()
                        ])
                        .style(not_focused_rect_box),
                    })
                    .spacing(1),
                )
            }),
            Container::new(widget::row![
                button(i18n_w("apply"))
                    .style(button::text)
                    .on_press(SettingWindowMessage::Apply.into())
            ])
            .align_x(Horizontal::Right)
            .width(Length::Fill)
            .padding(5)
        ]
        .into()
    }

    fn title(&'_ self) -> String {
        i18n("settings")
    }

    fn set_window_id(&mut self, window_id: Id) {
        self.window_id = Some(window_id)
    }

    fn window_settings() -> Settings
    where
        Self: Sized,
    {
        Settings {
            size: (720f32, 560f32).into(),
            minimizable: false,
            resizable: false,
            ..Settings::default()
        }
    }
}
