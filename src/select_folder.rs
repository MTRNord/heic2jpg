use gtk::prelude::*;
use relm4::{
    adw, gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};
use relm4_components::{open_dialog::*, save_dialog::*};
use std::path::PathBuf;

pub(super) struct SelectFolder {
    open_dialog: Controller<OpenDialog>,
    folder: Option<PathBuf>,
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
}

#[relm4::component(pub)]
impl SimpleComponent for SelectFolder {
    type Init = ();
    type Input = SelectFolderMsg;
    type Output = SelectFolderOut;

    view! {
        adw::StatusPage {
            set_hexpand: true,
            set_vexpand: true,
            set_title: "Select Folder",
            set_description: Some("Select the folder containing the HEIC files you want to convert"),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 24,

                gtk::Button {
                    set_halign: gtk::Align::Center,
                    set_label: "Select input directory",
                    connect_clicked[sender] => move |_| {
                        sender.input(SelectFolderMsg::OpenRequest);
                    }
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
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
