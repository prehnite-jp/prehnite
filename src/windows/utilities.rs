use crate::app::settings::get_settings;
use crate::windows::main_window::get_main_window_context;
use dioxus::core::{Element, VirtualDom};
use dioxus::desktop::{window, WindowBuilder};
use dioxus::prelude::*;
use dioxus_desktop::{Config, DesktopContext, WindowEvent};
use dioxus_i18n::prelude::I18n;

pub fn owned_window(builder: WindowBuilder) -> WindowBuilder {
    #[cfg(target_os = "windows")]
    {
        use dioxus::desktop::tao::platform::windows::{WindowBuilderExtWindows, WindowExtWindows};
        window().window.set_enable(false);
        builder.with_owner_window(window().hwnd())
    }
    #[cfg(target_os = "macos")]
    {
        use dioxus::desktop::tao::platform::macos::WindowExtMacOS;
        builder.with_parent_window(window().ns_window())
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        builder
    }
}

pub async fn show_window(
    app: fn() -> Element,
    config: Config,
    builder: WindowBuilder,
) -> DesktopContext {
    let window = window()
        .new_window(
            VirtualDom::new(app).with_root_context(consume_context::<I18n>()),
            config.with_window(
                builder
                    .with_theme(Some(get_settings().read().get_theme().into()))
                    .with_visible(false)
                    .with_background_color(get_settings().read().get_theme().bg_color()),
            ),
        )
        .await;
    let window2 = window.clone();
    window2
        .clone()
        .create_wry_event_handler(move |x, _| match x {
            dioxus_desktop::tao::event::Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                get_main_window_context().map(|x| x.set_focus());
                window2.clone().set_visible(false);
            }
            _ => {}
        });
    window
}

pub async fn show_modal(
    app: fn() -> Element,
    config: Config,
    builder: WindowBuilder,
) -> DesktopContext {
    let parent = window();
    let window = show_window(app, config, owned_window(builder)).await;
    #[cfg(target_os = "windows")]
    window.create_wry_event_handler(move |x, _| {
        use dioxus::desktop::tao::platform::windows::WindowExtWindows;
        match x {
            dioxus_desktop::tao::event::Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => parent.set_enable(true),
            _ => {}
        }
    });
    window
}
