use gettextrs::gettext;
use relm4::{
    adw,
    gtk::{
        self,
        prelude::{BoxExt, OrientableExt, WidgetExt},
    },
    ComponentParts, ComponentSender, SimpleComponent,
};

pub struct ProgressingPage {
    file_count: usize,
    progress: f64,
}

#[derive(Debug)]
pub enum ProgressingPageMsg {
    SetFileCount(usize),
    SetProgress(f64),
}

#[relm4::component(pub)]
impl SimpleComponent for ProgressingPage {
    type Init = ();
    type Input = ProgressingPageMsg;
    type Output = ();

    view! {
        adw::StatusPage {
            set_hexpand: true,
            set_vexpand: true,
            set_title: &gettext("Converting"),
            set_description: Some(&gettext("Please wait while the conversion is in progress")),
            set_icon_name: Some("pocket-knife"),
            gtk::Box {
                set_halign: gtk::Align::Center,
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 8,

                gtk::Label {
                    set_xalign: 0.5,
                    #[watch]
                    set_visible: model.file_count > 0,
                    #[watch]
                    set_label: &format!(
                        "{} / {}",
                        model.file_count as u32 * model.progress as u32,
                        model.file_count
                    ),
                },
                gtk::ProgressBar {
                    set_hexpand: true,
                    #[watch]
                    set_fraction: model.progress,
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            file_count: 0,
            progress: 0.0,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            ProgressingPageMsg::SetFileCount(file_count) => {
                self.file_count = file_count;
            }
            ProgressingPageMsg::SetProgress(progress) => {
                self.progress = progress;
            }
        }
    }
}
