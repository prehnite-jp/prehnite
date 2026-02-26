use crate::app::resources::app_icon_handle;
use crate::app::window::{Window, WindowMessage};
use crate::license::license_bundle;
use iced::border::Radius;
use iced::widget::pane_grid::{Axis, ResizeEvent};
use iced::widget::{button, container, pane_grid, scrollable, text_input, Container, MouseArea};
use iced::window::Id;
use iced::{widget, Element, Length, Task};
use prehnite_core::i18n::{i18n, i18n_w};
use prehnite_core::license_bundle::Package;
use prehnite_core::util::alert::alert_i18n;
use prehnite_core::widget::font::{ftext, get_font};
use prehnite_core::MessageLevel;
use prehnite_font_manager::widget::material_symbol;
use std::collections::{BTreeSet, HashMap};
use tracing::error;

const ROOT_PACKAGE_NAME: &str = "prehnite";

#[derive(Clone, Debug)]
pub enum LicenseInfoWindowMessage {
    LoadLicenseBundle,
    UpdateLicenseBundle(HashMap<String, Package>),
    PkgBack,
    PkgHome,
    SearchTextOnChanged(String),
    ChangeTarget(String),
    ChangeSelectedTarget(String),
    PageChanged,
    PaneResized(ResizeEvent),
    OpenWelcomeMessage,
}

impl From<LicenseInfoWindowMessage> for WindowMessage {
    fn from(value: LicenseInfoWindowMessage) -> Self {
        WindowMessage::LicenseInfoWindowMessage(value)
    }
}

#[derive(Debug)]
enum WindowPane {
    List,
    Detail,
}

#[derive(Debug)]
pub struct LicenseInfoWindow {
    packages: Option<HashMap<String, Package>>,
    window_id: Option<Id>,
    dep_package_list: BTreeSet<String>,
    selected_package: String,
    search_text_history: Vec<String>,
    target_package_history: Vec<String>,
    pane_state: pane_grid::State<WindowPane>,
}

impl LicenseInfoWindow {
    fn row_style(theme: &iced::Theme, is_focused: bool) -> container::Style {
        let mut style = container::bordered_box(theme);
        style.border.radius = Radius::new(0);
        if !is_focused {
            style = style.background(theme.palette().background);
        }
        style
    }

    fn software_list_row(&self, pkg_name: String) -> Element<'_, LicenseInfoWindowMessage> {
        let pkg_name2 = pkg_name.clone();
        MouseArea::new(
            Container::new(ftext(pkg_name.clone()))
                .padding(5)
                .width(Length::Fill)
                .style(move |t| Self::row_style(t, pkg_name.clone() == self.selected_package)),
        )
        .on_press(LicenseInfoWindowMessage::ChangeSelectedTarget(
            pkg_name2.clone(),
        ))
        .on_double_click(LicenseInfoWindowMessage::ChangeTarget(pkg_name2))
        .into()
    }

    fn software_list_pane(&self) -> Element<'_, LicenseInfoWindowMessage> {
        widget::column![
            widget::row![
                button(material_symbol("\u{E5D8}"))
                    .style(button::text)
                    .on_press_maybe(
                        Some(LicenseInfoWindowMessage::PkgBack)
                            .filter(|_| self.search_text_history.len() > 1)
                    ),
                text_input(
                    i18n("search").as_str(),
                    self.search_text_history
                        .last()
                        .cloned()
                        .unwrap_or_default()
                        .as_str()
                )
                .font(get_font())
                .on_input(LicenseInfoWindowMessage::SearchTextOnChanged),
                button(widget::image(app_icon_handle()).width(20).height(20))
                    .style(button::text)
                    .on_press(LicenseInfoWindowMessage::PkgHome)
            ],
            widget::column(
                self.dep_package_list
                    .iter()
                    .filter(|v| self
                        .search_text_history
                        .last()
                        .map(|x| v.contains(x))
                        .unwrap_or_default())
                    .map(|v| self.software_list_row(v.clone()))
            )
        ]
        .into()
    }

    fn software_details_pane(&self) -> Element<'_, LicenseInfoWindowMessage> {
        let package = match &self.packages {
            None => return i18n_w("unknown").into(),
            Some(v) => match v.get(&self.selected_package) {
                None => return i18n_w("unknown").into(),
                Some(v) => v.clone(),
            },
        };
        widget::column![
            widget::text(package.name),
            widget::text(package.authors.join(", ")),
            widget::text(package.homepage.unwrap_or("-".to_string())),
            widget::text(package.repository.unwrap_or("-".to_string())),
            widget::text(package.license_info),
            widget::rule::horizontal(1),
            widget::column(package.licenses.into_iter().map(|v| {
                widget::column![widget::text(v.full_text), widget::rule::horizontal(1),].into()
            }),),
        ]
        .into()
    }

    fn update_impl(&mut self, msg: LicenseInfoWindowMessage) -> Task<LicenseInfoWindowMessage> {
        match msg {
            LicenseInfoWindowMessage::LoadLicenseBundle => {
                return Task::future(async {
                    LicenseInfoWindowMessage::UpdateLicenseBundle(
                        license_bundle()
                            .into_iter()
                            .map(|v| (v.name.clone(), v))
                            .collect(),
                    )
                });
            }
            LicenseInfoWindowMessage::UpdateLicenseBundle(v) => {
                self.packages = Some(v);
                return Task::done(LicenseInfoWindowMessage::PageChanged);
            }
            LicenseInfoWindowMessage::PkgBack => {
                if self.search_text_history.len() > 1 {
                    self.search_text_history.pop();
                }
                if self.target_package_history.len() > 1 {
                    self.target_package_history.pop();
                }
                return Task::done(LicenseInfoWindowMessage::PageChanged);
            }
            LicenseInfoWindowMessage::SearchTextOnChanged(v) => {
                match self.search_text_history.last_mut() {
                    None => {}
                    Some(x) => *x = v,
                };
            }
            LicenseInfoWindowMessage::ChangeTarget(v) => {
                self.target_package_history.push(v);
                self.search_text_history.push("".to_string());
                return Task::done(LicenseInfoWindowMessage::PageChanged);
            }
            LicenseInfoWindowMessage::PageChanged => {
                self.dep_package_list = match &self.packages {
                    None => None,
                    Some(v) => v
                        .get(
                            &self
                                .target_package_history
                                .last()
                                .cloned()
                                .unwrap_or_default(),
                        )
                        .cloned(),
                }
                .map(|v| v.dependencies.clone())
                .unwrap_or_default();
            }
            LicenseInfoWindowMessage::PkgHome => {
                self.target_package_history.clear();
                self.search_text_history.clear();
                self.target_package_history
                    .push(ROOT_PACKAGE_NAME.to_string());
                self.search_text_history.push("".to_string());
                self.selected_package = ROOT_PACKAGE_NAME.to_string();
                return Task::done(LicenseInfoWindowMessage::PageChanged);
            }
            LicenseInfoWindowMessage::ChangeSelectedTarget(v) => {
                self.selected_package = v;
            }
            LicenseInfoWindowMessage::PaneResized(ResizeEvent { split, ratio }) => {
                if ratio > 0.33 && ratio < 0.66 {
                    self.pane_state.resize(split, ratio);
                }
            }
            LicenseInfoWindowMessage::OpenWelcomeMessage => {
                return alert_i18n(self.window_id, ("info", "license-info_message"), MessageLevel::Info);
            }
        }
        Task::none()
    }
}

impl Window for LicenseInfoWindow {
    fn new() -> Self
    where
        Self: Sized,
    {
        let (mut pane_state, pane) = pane_grid::State::new(WindowPane::List);
        match pane_state.split(Axis::Vertical, pane, WindowPane::Detail) {
            None => {}
            Some((_, split)) => pane_state.resize(split, 0.33),
        }
        Self {
            packages: None,
            window_id: None,
            dep_package_list: BTreeSet::new(),
            selected_package: ROOT_PACKAGE_NAME.to_string(),
            search_text_history: vec!["".to_string()],
            target_package_history: vec![ROOT_PACKAGE_NAME.to_string()],
            pane_state,
        }
    }

    fn init_task() -> Task<WindowMessage>
    where
        Self: Sized,
    {
        Task::done(LicenseInfoWindowMessage::LoadLicenseBundle.into())
            .chain(Task::done(LicenseInfoWindowMessage::OpenWelcomeMessage.into()))
    }

    fn update(&mut self, message: WindowMessage) -> Task<WindowMessage> {
        if let WindowMessage::LicenseInfoWindowMessage(message) = message {
            self.update_impl(message).map(|v| v.into())
        } else {
            error!("Invalid message received.");
            Task::none()
        }
    }

    fn view(&'_ self) -> Element<'_, WindowMessage> {
        widget::pane_grid(&self.pane_state, |_, state, _| {
            pane_grid::Content::new(
                scrollable(match state {
                    WindowPane::List => Container::new(self.software_list_pane().map(|v| v.into()))
                        .style(|t| Self::row_style(t, false)),
                    WindowPane::Detail => {
                        Container::new(self.software_details_pane().map(|v| v.into()))
                            .style(|t| Self::row_style(t, false))
                            .padding(5)
                    }
                })
                .spacing(1),
            )
        })
        .on_resize(10, |v| LicenseInfoWindowMessage::PaneResized(v).into())
        .into()
    }

    fn title(&'_ self) -> String {
        i18n("license-info")
    }

    fn set_window_id(&mut self, window_id: Id) {
        self.window_id = Some(window_id);
    }
}
