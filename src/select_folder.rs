use gtk::prelude::*;
use relm4::{
    adw, gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};
use relm4_components::{open_dialog::*, save_dialog::*};
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub(super) enum InOut {
    Input,
    Output,
}

#[derive(Debug)]
pub(super) struct SelectFolder {
    open_dialog: Controller<OpenDialog>,
    folder: Option<PathBuf>,
    description: String,
    button_label: String,
    direction: InOut,
}

#[derive(Debug)]
pub(super) enum SelectFolderMsg {
    OpenRequest,
    OpenResponse(PathBuf),
    Ignore,
}

#[derive(Debug)]
pub(super) enum SelectFolderOut {
    FolderSelected(PathBuf),
    AbortLast,
}

#[relm4::component(pub)]
impl SimpleComponent for SelectFolder {
    type Init = InOut;
    type Input = SelectFolderMsg;
    type Output = SelectFolderOut;

    view! {
        adw::StatusPage {
            set_hexpand: true,
            set_vexpand: true,
            set_title: "Select Folder",
            set_description: Some(&model.description),

            gtk::Box {
                set_halign: gtk::Align::Center,
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 24,

                gtk::Button {
                    set_halign: gtk::Align::Center,
                    set_label: &model.button_label,
                    connect_clicked[sender] => move |_| {
                        sender.input(SelectFolderMsg::OpenRequest);
                    }
                },

                // If this is the output folder selector, add an Abort button
                gtk::Button {
                    set_visible: model.direction == InOut::Output,
                    set_halign: gtk::Align::Center,
                    set_label: "Abort",
                    connect_clicked[sender] => move |_| {
                        let _ = sender.output(SelectFolderOut::AbortLast);
                    }
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (description, button_label) = match init {
            InOut::Input => (
                "Select the folder where the Heic files can be found",
                "Select input directory",
            ),
            InOut::Output => (
                "Select the folder where the JPG files are meant to be saved",
                "Select output directory",
            ),
        };

        let dialog_settings = OpenDialogSettings {
            folder_mode: true,
            ..Default::default()
        };

        let open_dialog = OpenDialog::builder()
            .transient_for_native(&root)
            .launch(dialog_settings)
            .forward(sender.input_sender(), |response| match response {
                OpenDialogResponse::Accept(path) => SelectFolderMsg::OpenResponse(path),
                OpenDialogResponse::Cancel => SelectFolderMsg::Ignore,
            });

        let model = Self {
            open_dialog,
            folder: None,
            description: description.to_string(),
            button_label: button_label.to_string(),
            direction: init,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            SelectFolderMsg::OpenRequest => self.open_dialog.emit(OpenDialogMsg::Open),
            SelectFolderMsg::OpenResponse(path) => {
                self.folder = Some(path.clone());
                let _ = sender.output(SelectFolderOut::FolderSelected(path.clone()));
            }
            SelectFolderMsg::Ignore => {}
        }
    }
}
