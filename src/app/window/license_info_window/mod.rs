use crate::app::resources::app_icon_handle;
use crate::app::window::{Window, WindowMessage};
use crate::widget::styles::container::{focusable, not_focused_rect_box};
use crate::widget::{copy_button, hideable};
use iced::alignment::Horizontal;
use iced::widget::pane_grid::Axis;
use iced::widget::text::Wrapping;
use iced::widget::{button, pane_grid, scrollable, span, text_input, Container, MouseArea};
use iced::window::Id;
use iced::{padding, widget, Alignment, Background, Color, Element, Length, Task};
use license::license_bundle;
use opener::open_browser;
use prehnite_core::font::material_symbol::ARROW_UPWARD;
use prehnite_core::font::widget::material_symbol;
use prehnite_core::i18n::{i18n, i18n_w};
use prehnite_core::license_bundle::Package;
use prehnite_core::util::alert::alert_i18n;
use prehnite_core::widget::font::{ftext, get_font};
use prehnite_core::widget::text::TextBuilder;
use prehnite_core::MessageLevel;
use std::collections::{BTreeMap, BTreeSet};
use tracing::error;
use tracing::log::warn;

pub mod license;

const ROOT_PACKAGE_NAME: &str = "prehnite";

#[derive(Clone, Debug)]
pub enum LicenseInfoWindowMessage {
    LoadLicenseBundle,
    UpdateLicenseBundle(BTreeMap<String, Package>),
    PkgBack,
    PkgHome,
    SearchTextOnChanged(String),
    ChangeTarget(String),
    ChangeSelectedTarget(String),
    PageChanged,
    OpenWelcomeMessage,
    SetClipboard(String),
    LinkOnClick(String),
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
    packages: Option<BTreeMap<String, Package>>,
    window_id: Option<Id>,
    dep_package_list: BTreeSet<String>,
    selected_package: String,
    search_text_history: Vec<String>,
    target_package_history: Vec<String>,
    pane_state: pane_grid::State<WindowPane>,
}

impl LicenseInfoWindow {
    fn software_list_row(&self, pkg_name: String) -> Element<'_, LicenseInfoWindowMessage> {
        let pkg_name2 = pkg_name.clone();
        MouseArea::new(
            Container::new(widget::row![
                ftext(pkg_name.clone()),
                widget::space().width(Length::Fill),
                iced_aw::badge(ftext(format!(
                    "{}",
                    match &self.packages {
                        None => {
                            0
                        }
                        Some(v) => {
                            v.get(&pkg_name)
                                .map(|v| v.dependencies.len())
                                .unwrap_or_default()
                        }
                    }
                )))
                .style(|t, _| {
                    iced_aw::badge::Style {
                        background: Background::Color(t.palette().primary),
                        border_radius: None,
                        border_width: 0.0,
                        border_color: None,
                        text_color: t.palette().text,
                    }
                })
                .width(32)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
            ])
            .padding(5)
            .width(Length::Fill)
            .style(focusable(pkg_name.clone() == self.selected_package)),
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
                button(material_symbol(ARROW_UPWARD))
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
            scrollable(widget::column(
                self.dep_package_list
                    .iter()
                    .filter(|v| self
                        .search_text_history
                        .last()
                        .map(|x| v.contains(x))
                        .unwrap_or_default())
                    .map(|v| self.software_list_row(v.clone()))
            ))
            .spacing(1)
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
        let txt_header = TextBuilder::with_font().wrapping(Wrapping::None);
        let txt_value = TextBuilder::with_font().wrapping(Wrapping::None);
        widget::column![
            scrollable(
                widget::row![
                    widget::column![
                        txt_header.text(i18n("package-name")),
                        txt_header.text(i18n("package-authors")),
                        txt_header.text(i18n("package-homepage")),
                        txt_header.text(i18n("package-repository")),
                        txt_header.text(i18n("package-license")),
                    ]
                    .align_x(Horizontal::Center)
                    .width(Length::Shrink),
                    widget::space().width(20),
                    widget::column![
                        widget::row![
                            txt_value.text(package.name.clone()),
                            hideable(
                                copy_button().on_press(LicenseInfoWindowMessage::SetClipboard(
                                    package.name.clone(),
                                )),
                                !package.name.is_empty()
                            )
                        ],
                        widget::row![
                            txt_value.text({
                                let v = package.authors.join(", ");
                                if v.is_empty() { "-".to_string() } else { v }
                            }),
                            hideable(
                                copy_button().on_press(LicenseInfoWindowMessage::SetClipboard(
                                    package.authors.join(", "),
                                )),
                                !package.authors.is_empty()
                            )
                        ],
                        txt_value
                            .rich([package
                                .homepage
                                .clone()
                                .map(|v| span(v).color(Color::from_rgb8(0, 0, 0xEE)).link(0))
                                .unwrap_or(span::<i32, _>("-"))])
                            .on_link_click(move |_| LicenseInfoWindowMessage::LinkOnClick(
                                package.homepage.clone().unwrap_or_default()
                            )),
                        txt_value
                            .rich([package
                                .repository
                                .clone()
                                .map(|v| span(v).color(Color::from_rgb8(0, 0, 0xEE)).link(0))
                                .unwrap_or(span::<i32, _>("-"))])
                            .on_link_click(move |_| LicenseInfoWindowMessage::LinkOnClick(
                                package.repository.clone().unwrap_or_default()
                            )),
                        widget::row![
                            txt_value.text(package.license_info.clone()),
                            hideable(
                                copy_button().on_press(LicenseInfoWindowMessage::SetClipboard(
                                    package.license_info.clone(),
                                )),
                                !package.license_info.is_empty()
                            )
                        ]
                    ],
                ]
                .padding(padding::bottom(10))
            )
            .horizontal(),
            widget::rule::horizontal(1),
            widget::column(package.licenses.into_iter().map(|v| {
                widget::column![widget::text(v.full_text), widget::rule::horizontal(1),].into()
            }),),
        ]
        .into()
    }

    #[tracing::instrument]
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
                match &self.packages {
                    None => {}
                    Some(packages) => {
                        if packages
                            .get(&v)
                            .map(|pkg| pkg.dependencies.is_empty())
                            .unwrap_or_default()
                        {
                            return Task::none();
                        }
                    }
                };
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
            LicenseInfoWindowMessage::OpenWelcomeMessage => {
                return alert_i18n(
                    self.window_id,
                    ("info", "license-info_message"),
                    MessageLevel::Info,
                );
            }
            LicenseInfoWindowMessage::SetClipboard(v) => {
                return iced::clipboard::write(v);
            }
            LicenseInfoWindowMessage::LinkOnClick(url) => {
                open_browser(url)
                    .inspect_err(|e| warn!("Failed to open browser. E: {e:?}"))
                    .ok();
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
        Task::done(LicenseInfoWindowMessage::LoadLicenseBundle.into()).chain(Task::done(
            LicenseInfoWindowMessage::OpenWelcomeMessage.into(),
        ))
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
            pane_grid::Content::new(match state {
                WindowPane::List => Element::from(
                    Container::new(self.software_list_pane().map(|v| v.into()))
                        .style(not_focused_rect_box),
                ),
                WindowPane::Detail => scrollable(
                    Container::new(self.software_details_pane().map(|v| v.into()))
                        .style(not_focused_rect_box)
                        .padding(5),
                )
                .spacing(1)
                .into(),
            })
        })
        .into()
    }

    fn title(&'_ self) -> String {
        i18n("license-info")
    }

    fn set_window_id(&mut self, window_id: Id) {
        self.window_id = Some(window_id);
    }
}
