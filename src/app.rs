use crate::config::{APP_ID, PROFILE};
use crate::modals::about::AboutDialog;
use crate::select_folder::{InOut, SelectFolder, SelectFolderOut};
use gtk::prelude::*;
use gtk::{gio, glib};
use relm4::{
    actions::{RelmAction, RelmActionGroup},
    adw, gtk, main_application, Component, ComponentController, ComponentParts, ComponentSender,
    Controller, SimpleComponent,
};
use std::path::PathBuf;

pub(super) struct App {
    about_dialog: Controller<AboutDialog>,
    input_folder_selector: Controller<SelectFolder>,
    output_folder_selector: Controller<SelectFolder>,
    input_folder: Option<PathBuf>,
    output_folder: Option<PathBuf>,
}

#[derive(Debug)]
pub(super) enum AppMsg {
    InputFolderSelected(PathBuf),
    OutputFolderSelected(PathBuf),
    DeselectInputFolder,
    DeselectOutputFolder,
    Quit,
    Noop,
}

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();
    type Widgets = AppWidgets;

    menu! {
        primary_menu: {
            section! {
                "_Preferences" => PreferencesAction,
                "_Keyboard" => ShortcutsAction,
                "_About Heic2JPG" => AboutAction,
            }
        }
    }

    view! {
        main_window = adw::ApplicationWindow::new(&main_application()) {
            set_visible: true,

            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::Quit);
                glib::Propagation::Stop
            },

            #[wrap(Some)]
            set_help_overlay: shortcuts = &gtk::Builder::from_resource(
                    "/dev/nordgedanken/heic2jpg/gtk/help-overlay.ui"
                )
                .object::<gtk::ShortcutsWindow>("help_overlay")
                .unwrap() -> gtk::ShortcutsWindow {
                    set_transient_for: Some(&main_window),
                    set_application: Some(&main_application()),
            },

            add_css_class?: if PROFILE == "Devel" {
                    Some("devel")
                } else {
                    None
                },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_vexpand: true,

                adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: "Convert Heic to JPG",
                    },
                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&primary_menu),
                    }
                },

                // If no folder is selected, show the folder selector
                if model.input_folder.is_some() && model.output_folder.is_some() {
                    adw::StatusPage {
                        set_hexpand: true,
                        set_vexpand: true,
                        set_title: "Start Conversion",
                        set_description: Some("Press the button to start the conversion"),

                        gtk::Box {
                            set_halign: gtk::Align::Center,
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 24,
                            gtk::Button {
                                set_label: "Convert",
                                connect_clicked[sender] => move |_| {
                                    // TODO: Implement conversion using worker and imagick
                                    sender.input(AppMsg::Quit);
                                }
                            }
                        }
                    }
                } else if model.input_folder.is_none() {
                    gtk::Box {
                        set_vexpand: true,
                        set_hexpand: true,
                        append = model.input_folder_selector.widget(),
                    }
                } else {
                    gtk::Box {
                        set_vexpand: true,
                        set_hexpand: true,
                        append = model.output_folder_selector.widget(),
                    }
                }

            }

        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let about_dialog = AboutDialog::builder()
            .transient_for(&root)
            .launch(())
            .detach();
        let input_folder_selector =
            SelectFolder::builder()
                .launch(InOut::Input)
                .forward(sender.input_sender(), |msg| match msg {
                    SelectFolderOut::FolderSelected(path) => AppMsg::InputFolderSelected(path),
                    SelectFolderOut::AbortLast => AppMsg::Noop,
                });
        let output_folder_selector =
            SelectFolder::builder()
                .launch(InOut::Output)
                .forward(sender.input_sender(), |msg| match msg {
                    SelectFolderOut::FolderSelected(path) => AppMsg::OutputFolderSelected(path),
                    SelectFolderOut::AbortLast => AppMsg::DeselectInputFolder,
                });

        let model = Self {
            about_dialog,
            input_folder_selector,
            output_folder_selector,
            input_folder: None,
            output_folder: None,
        };

        let widgets = view_output!();

        let mut actions = RelmActionGroup::<WindowActionGroup>::new();

        let shortcuts_action = {
            let shortcuts = widgets.shortcuts.clone();
            RelmAction::<ShortcutsAction>::new_stateless(move |_| {
                shortcuts.present();
            })
        };

        let about_action = {
            let sender = model.about_dialog.sender().clone();
            RelmAction::<AboutAction>::new_stateless(move |_| {
                sender.send(()).unwrap();
            })
        };

        actions.add_action(shortcuts_action);
        actions.add_action(about_action);
        actions.register_for_widget(&widgets.main_window);

        widgets.load_window_size();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::InputFolderSelected(path) => self.input_folder = Some(path),
            AppMsg::DeselectInputFolder => self.input_folder = None,
            AppMsg::OutputFolderSelected(path) => self.output_folder = Some(path),
            AppMsg::DeselectOutputFolder => self.output_folder = None,
            AppMsg::Quit => main_application().quit(),
            AppMsg::Noop => {}
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        widgets.save_window_size().unwrap();
    }
}

impl AppWidgets {
    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let settings = gio::Settings::new(APP_ID);
        let (width, height) = self.main_window.default_size();

        settings.set_int("window-width", width)?;
        settings.set_int("window-height", height)?;

        settings.set_boolean("is-maximized", self.main_window.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let settings = gio::Settings::new(APP_ID);

        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        self.main_window.set_default_size(width, height);

        if is_maximized {
            self.main_window.maximize();
        }
    }
}
