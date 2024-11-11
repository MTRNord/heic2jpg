#[rustfmt::skip]
mod config;
mod app;
mod conversion_worker;
mod modals;
mod select_folder;

use config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR, RESOURCES_FILE};
use gettextrs::LocaleCategory;
use gtk::prelude::ApplicationExt;
use gtk::{gio, glib};
use magick_rust::magick_wand_genesis;
use relm4::{
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    gtk, main_application, RelmApp,
};
use std::sync::Once;

use app::App;
use relm4::set_global_css;

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

static START_IMAGICK: Once = Once::new();

fn main() {
    START_IMAGICK.call_once(|| {
        magick_wand_genesis();
    });
    gtk::init().unwrap();

    // Enable logging
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_max_level(tracing::Level::INFO)
        .init();

    // setup gettext
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    glib::set_application_name("Heic2JPG");

    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    gtk::Window::set_default_icon_name(APP_ID);

    let app = main_application();
    app.set_application_id(Some(APP_ID));
    app.set_resource_base_path(Some("/dev/nordgedanken/heic2jpg/"));

    let mut actions = RelmActionGroup::<AppActionGroup>::new();

    let quit_action = {
        let app = app.clone();
        RelmAction::<QuitAction>::new_stateless(move |_| {
            app.quit();
        })
    };
    actions.add_action(quit_action);
    actions.register_for_main_application();

    app.set_accelerators_for_action::<QuitAction>(&["<Control>q"]);

    let app = RelmApp::from_app(app);

    let data = res
        .lookup_data(
            "/dev/nordgedanken/heic2jpg/style.css",
            gio::ResourceLookupFlags::NONE,
        )
        .unwrap();
    set_global_css(&glib::GString::from_utf8_checked(data.to_vec()).unwrap());
    app.visible_on_activate(false).run::<App>(());
}
