use crate::config::{APP_ID, PROFILE};
use crate::conversion_worker::{ConversionWorker, ConversionWorkerInputMsg, ConversionWorkerMsg};
use crate::modals::about::AboutDialog;
use crate::select_folder::{InOut, SelectFolder, SelectFolderOut};
use gettextrs::gettext;
use gtk::prelude::*;
use gtk::{gio, glib};
use relm4::{
    actions::{RelmAction, RelmActionGroup},
    adw, gtk, main_application, Component, ComponentController, ComponentParts, ComponentSender,
    Controller, SimpleComponent, WorkerController,
};
use std::path::PathBuf;

enum Mode {
    InputSelection,
    OutputSelection,
    ConversionSelection,
    Progressing,
    Finished,
    Failed,
}

pub(super) struct App {
    about_dialog: Controller<AboutDialog>,
    input_folder_selector: Controller<SelectFolder>,
    output_folder_selector: Controller<SelectFolder>,
    input_folder: Option<PathBuf>,
    output_folder: Option<PathBuf>,
    conversion_worker: WorkerController<ConversionWorker>,
    mode: Mode,
    progress: f64,
    failure: Option<String>,
    file_count: usize,
}

#[derive(Debug)]
pub(super) enum AppMsg {
    InputFolderSelected(PathBuf),
    OutputFolderSelected(PathBuf),
    DeselectInputFolder,
    DeselectOutputFolder,
    Convert,
    ConversionStarted(usize),
    ProgressUpdate(f64),
    ConversionComplete,
    ConversionFailed(String),
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
                        set_title: &gettext("Convert Heic to JPG"),
                    },
                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&primary_menu),
                    }
                },

                #[transition = "SlideRight"]
                match model.mode {
                    Mode::Progressing => {
                        adw::StatusPage {
                            set_hexpand: true,
                            set_vexpand: true,
                            set_title: &gettext("Converting"),
                            set_description: Some(&gettext("Please wait while the conversion is in progress")),
                            gtk::Box {
                                set_halign: gtk::Align::Center,
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 24,
                                gtk::ProgressBar {
                                    #[watch]
                                    set_fraction: model.progress,
                                },

                                gtk::Text {
                                    set_halign: gtk::Align::Center,
                                    #[watch]
                                    set_visible: model.file_count > 0,
                                    #[watch]
                                    set_text: &format!(
                                        "{} / {}",
                                        model.file_count as u32 * model.progress as u32,
                                        model.file_count
                                    ),
                                }
                            }
                        }
                    }
                    Mode::Finished => {
                        adw::StatusPage {
                            set_hexpand: true,
                            set_vexpand: true,
                            set_title: &gettext("Conversion Complete"),
                            set_description: Some(&gettext("The conversion was successful")),
                            gtk::Box {
                                set_halign: gtk::Align::Center,
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 24,

                                gtk::Button {
                                    set_label: "Close",
                                    add_css_class: "suggested-action",
                                    add_css_class: "pill",
                                    connect_clicked[sender] => move |_| {
                                        sender.input(AppMsg::Quit);
                                    }
                                }
                            }
                        }
                    }
                    Mode::Failed => {
                        adw::StatusPage {
                            set_hexpand: true,
                            set_vexpand: true,
                            set_title: &gettext("Conversion Failed"),
                            #[watch]
                            set_description: model.failure.as_deref(),

                            gtk::Box {
                                set_halign: gtk::Align::Center,
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 24,
                                gtk::Button {
                                    set_label: &gettext("Close"),
                                    connect_clicked[sender] => move |_| {
                                        sender.input(AppMsg::Quit);
                                    }
                                }
                            }
                        }
                    }
                    Mode::InputSelection => {
                        gtk::Box {
                            set_vexpand: true,
                            set_hexpand: true,
                            append = model.input_folder_selector.widget(),
                        }
                    }
                    Mode::OutputSelection => {
                        gtk::Box {
                            set_vexpand: true,
                            set_hexpand: true,
                            append = model.output_folder_selector.widget(),
                        }
                    }
                    Mode::ConversionSelection => {
                        adw::StatusPage {
                            set_hexpand: true,
                            set_vexpand: true,
                            set_title: &gettext("Start Conversion"),
                            set_description: Some(&gettext("Click the button below to start the conversion")),

                            gtk::Box {
                                set_halign: gtk::Align::Center,
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 24,
                                gtk::Button {
                                    set_label: &gettext("Convert"),
                                    add_css_class: "suggested-action",
                                    add_css_class: "pill",
                                    connect_clicked[sender] => move |_| {
                                        sender.input(AppMsg::Convert);
                                    }
                                }
                            }
                        }
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
        let conversion_worker =
            ConversionWorker::builder()
                .detach_worker(())
                .forward(sender.input_sender(), |msg| match msg {
                    ConversionWorkerMsg::ConversionStarted(number) => {
                        AppMsg::ConversionStarted(number)
                    }
                    ConversionWorkerMsg::ProgressUpdate(number) => AppMsg::ProgressUpdate(number),
                    ConversionWorkerMsg::ConversionComplete => AppMsg::ConversionComplete,
                    ConversionWorkerMsg::ConversionFailed(e) => AppMsg::ConversionFailed(e),
                });

        let model = Self {
            about_dialog,
            input_folder_selector,
            output_folder_selector,
            conversion_worker,
            input_folder: None,
            output_folder: None,
            mode: Mode::InputSelection,
            progress: 0.0,
            failure: None,
            file_count: 0,
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
            AppMsg::ConversionStarted(number) => {
                self.file_count = number;
                self.mode = Mode::Progressing;
            }
            AppMsg::InputFolderSelected(path) => {
                self.input_folder = Some(path);
                self.mode = Mode::OutputSelection
            }
            AppMsg::DeselectInputFolder => {
                self.input_folder = None;
                self.mode = Mode::InputSelection
            }
            AppMsg::OutputFolderSelected(path) => {
                self.output_folder = Some(path);
                self.mode = Mode::ConversionSelection
            }
            AppMsg::DeselectOutputFolder => {
                self.output_folder = None;
                self.mode = Mode::OutputSelection
            }
            AppMsg::Quit => main_application().quit(),
            AppMsg::Convert => {
                if let (Some(input_folder), Some(output_folder)) =
                    (&self.input_folder, &self.output_folder)
                {
                    let _ = self.conversion_worker.sender().send(
                        ConversionWorkerInputMsg::ConvertFolder(
                            input_folder.clone(),
                            output_folder.clone(),
                        ),
                    );
                    self.mode = Mode::Progressing;
                } else {
                    self.mode = Mode::Failed;
                    self.failure =
                        Some(gettext("Please select both input and output folders").to_string());
                }
            }
            AppMsg::ProgressUpdate(progress) => {
                if let Mode::Progressing = self.mode {
                    self.progress = progress;
                }
            }
            AppMsg::ConversionComplete => {
                self.mode = Mode::Finished;
            }
            AppMsg::ConversionFailed(e) => {
                self.mode = Mode::Failed;
                self.failure = Some(e);
            }
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
