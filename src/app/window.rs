use crate::app::settings::get_global_settings;
use crate::window::main_window::get_main_window_context;
use dioxus::core::{consume_context, Element, VirtualDom};
use dioxus_desktop::{window, Config, DesktopContext, WindowBuilder, WindowEvent};
use dioxus_i18n::prelude::I18n;

// 終了時のちらつきを抑止します。
fn suppress_flickering_on_close(window: DesktopContext) {
    window
        .clone()
        .create_wry_event_handler(move |x, _| match x {
            dioxus_desktop::tao::event::Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                get_main_window_context().map(|x| x.set_focus());
                window.clone().set_visible(false);
            }
            _ => {}
        });
}

// ウインドウの親子関係を設定したWindowBuilderを取得
pub fn owned_window(builder: WindowBuilder) -> WindowBuilder {
    #[cfg(target_os = "windows")]
    {
        use dioxus::desktop::tao::platform::windows::{WindowBuilderExtWindows, WindowExtWindows};
        use dioxus_desktop::window;
        builder.with_owner_window(window().hwnd())
    }
    #[cfg(target_os = "macos")]
    {
        use dioxus::desktop::tao::platform::macos::{WindowBuilderExtMacOS, WindowExtMacOS};
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
    let theme = get_global_settings().registry().get_theme();
    let window = window()
        .new_window(
            VirtualDom::new(app).with_root_context(consume_context::<I18n>()),
            config.with_window(
                builder
                    .with_theme(Some(theme.clone().into()))
                    .with_visible(false)
                    .with_background_color(theme.bg_color()),
            ),
        )
        .await;
    suppress_flickering_on_close(window.clone());
    window
}

pub async fn show_owned(
    app: fn() -> Element,
    config: Config,
    builder: WindowBuilder,
) -> DesktopContext {
    show_window(app, config, owned_window(builder)).await
}

/// Windowsでは、小窓が存在する期間中、親窓を無効化します。
/// それ以外のプラットフォームでは、[`show_owned()`]と等価です。
pub async fn show_modal(
    app: fn() -> Element,
    config: Config,
    builder: WindowBuilder,
) -> DesktopContext {
    let parent = window();
    // 親窓を無効化する。
    #[cfg(target_os = "windows")]
    {
        use dioxus_desktop::tao::platform::windows::WindowExtWindows;
        parent.window.set_enable(false);
    }
    let window = show_owned(app, config, builder).await;
    // ウィンドウを閉じるときに親ウインドウを有効化する。
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
