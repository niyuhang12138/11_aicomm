mod commands;
mod config;
mod utils;

use std::sync::Arc;

use arc_swap::ArcSwap;
use commands::get_app_dir;
use commands::get_config;
use commands::greet;
use config::AppConfig;
use tauri::menu::CheckMenuItem;
use tauri::menu::Menu;
use tauri::menu::MenuItem;
use tauri::menu::SubmenuBuilder;
use tauri::tray::TrayIcon;
use tauri::tray::TrayIconBuilder;
use tauri::AppHandle;
use tauri::EventLoopMessage;
use tauri::Manager;
use tauri::Runtime;
use tauri::{
    webview::PageLoadPayload, App, Builder, Webview, WebviewUrl, WebviewWindowBuilder, Window,
    WindowEvent, Wry,
};
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tauri_plugin_log::{Target, TargetKind};
use tracing::debug;
use tracing::info;
use utils::log_dir;

const APP_NAME: &str = "chatapp";

#[allow(unused)]
pub struct AppState {
    config: Arc<ArcSwap<AppConfig>>,
}

pub fn app() -> anyhow::Result<Builder<Wry>> {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(logger().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        // .plugin(tauri_plugin_deep_link::init())
        // .plugin(tauri_plugin_window_state::Builder::new().build())
        // .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_app_dir, get_config,])
        .setup(setup)
        .on_page_load(page_load_handler)
        .on_window_event(window_event_handler);

    Ok(builder)
}

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up the app");
    let handle = app.handle();

    let state = AppState {
        config: Arc::new(ArcSwap::from_pointee(AppConfig::try_new()?)),
    };

    app.manage(state);

    #[cfg(desktop)]
    {
        handle.plugin(tauri_plugin_window_state::Builder::new().build())?;
    }

    set_up_menu(handle)?;

    let mut builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default());

    #[allow(unused_assignments)]
    #[cfg(desktop)]
    {
        builder = builder
            .user_agent(&format!("{} - {}", APP_NAME, std::env::consts::OS))
            .title("Chat App")
            .inner_size(1200_f64, 800_f64)
            .min_inner_size(800_f64, 600_f64)
            .resizable(true)
            .content_protected(true);
    }

    #[allow(unused_variables)]
    let web_view = builder.build()?;

    // #[cfg(debug_assertions)]
    // web_view.open_devtools();

    // initialize updater...

    Ok(())
}

// on_page_load is called when the webview is create
// signature should be: Fn(&Webview<R>, &PageLoadPayload<'_>) + Send + Sync + 'static
fn page_load_handler(webview: &Webview, _payload: &PageLoadPayload<'_>) {
    info!("Page load loaded on {:?}", webview.label())
}

// on_window_event is called when the window event is triggered
// signature should be: Fn(&Window<R>, &WindowEvent) + Send + Sync + 'static
fn window_event_handler(window: &Window, event: &WindowEvent) {
    debug!("Window event {:?} on {:?}", event, window.label());

    if let WindowEvent::CloseRequested { api, .. } = event {
        // info!("Window Close event: {:?}", window.label());
        if window.label() == "min" {
            api.prevent_close();
            window.hide().unwrap();
        }
    }
}

fn logger() -> tauri_plugin_log::Builder {
    tauri_plugin_log::Builder::default()
        .targets([
            Target::new(TargetKind::Webview),
            Target::new(TargetKind::Folder {
                path: log_dir(),
                file_name: Some("app".to_string()),
            }),
            Target::new(TargetKind::Stdout),
        ])
        .with_colors(ColoredLevelConfig::default())
        .level(tracing::log::LevelFilter::Info)
}

fn set_up_menu<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()>
where
    AppHandle<R>: Manager<R>,
    AppHandle<R>: Manager<tauri_runtime_wry::Wry<EventLoopMessage>>,
{
    let icon = app.default_window_icon().unwrap().clone();
    // create submenu
    let file_menu = SubmenuBuilder::with_id(app, "file", "File")
        .item(&MenuItem::with_id(
            app,
            "open",
            "Open",
            true,
            Some("CmdOrCtrl+O"),
        )?)
        .item(&MenuItem::with_id(
            app,
            "save",
            "Save",
            true,
            Some("CmdOrCtrl+S"),
        )?)
        .item(&MenuItem::with_id(
            app,
            "save as",
            "Save As",
            true,
            Some("CmdOrCtrl+Shift+S"),
        )?)
        .separator()
        .quit()
        .build()?;
    let edit_menu = SubmenuBuilder::with_id(app, "edit", "Edit")
        .item(&MenuItem::with_id(
            app,
            "process",
            "Process",
            true,
            Some("CmdOrCtrl+P"),
        )?)
        .separator()
        .undo()
        .redo()
        .cut()
        .copy()
        .paste()
        .select_all()
        .item(&CheckMenuItem::with_id(
            app,
            "check me",
            "Check Me",
            true,
            true,
            None::<&str>,
        )?)
        .build()?;
    // let tray_menu: tauri::menu::Submenu<R> = SubmenuBuilder::with_id(app, "tray", "Tray")
    //     .item(&MenuItem::with_id(app, "open", "Open", true, None::<&str>)?)
    //     .item(&MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?)
    //     .separator()
    //     .quit()
    //     .build()?;

    let open: MenuItem<R> = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
    let hide: MenuItem<R> = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
    let menu1: Menu<R> = Menu::with_items(app, &[&open, &hide])?;
    // create menu

    // add menu to tray
    let _ = TrayIconBuilder::with_id(format!("{APP_NAME}-tray"))
        .tooltip("Hacker News")
        .icon(icon)
        .menu(&menu1)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
            // info!("on_menu_event ---> Tray menu event: {:?}", event);
            match event.id.as_ref() {
                "open" => open_main(app).unwrap(),
                "hide" => {}
                _ => {}
            }
        })
        .on_tray_icon_event(|_tray: &TrayIcon<R>, _event| {
            // info!("on_tray_icon_event ---> Tray event: {:?}", event);
            // if let TrayIconEvent::Click {
            //     button: MouseButton::Right,
            //     ..
            // } = event
            // {
            //     open_main(tray.app_handle()).unwrap();
            // }
        })
        .build(app)?;
    // ad menu to window

    /* ------------------------------------------- */

    let menu = Menu::with_items(app, &[&file_menu, &edit_menu])?;

    app.set_menu(menu)?;
    app.on_menu_event(|app, event| {
        info!("Menu event: {:?}", event);
        match event.id.as_ref() {
            "open" => open_main(app).unwrap(),
            "save" => {}
            "save as" => {}
            "process" => {}
            "check me" => {}
            _ => {}
        }
    });

    Ok(())
}

fn open_main<R: Runtime>(handle: &AppHandle<R>) -> Result<(), tauri::Error> {
    handle
        .get_webview_window("main")
        .ok_or_else(|| tauri::Error::WindowNotFound)?
        .show()?;
    Ok(())
}
