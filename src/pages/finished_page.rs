use gettextrs::gettext;
use relm4::{
    adw,
    gtk::{
        self,
        prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt},
    },
    ComponentParts, ComponentSender, SimpleComponent,
};

pub struct FinishedPage;

#[derive(Debug)]
pub enum FinishedPageMsg {
    StartOver,
    Quit,
}

#[relm4::component(pub)]
impl SimpleComponent for FinishedPage {
    type Init = ();
    type Input = ();
    type Output = FinishedPageMsg;

    view! {
        adw::StatusPage {
            set_hexpand: true,
            set_vexpand: true,
            set_title: &gettext("Conversion Complete"),
            set_description: Some(&gettext("The conversion was successful")),
            set_icon_name: Some("test-pass"),
            gtk::Box {
                set_halign: gtk::Align::Center,
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 24,

                gtk::Button {
                    set_label: "Close",
                    add_css_class: "suggested-action",
                    add_css_class: "pill",
                    connect_clicked[sender] => move |_| {
                        let _ = sender.output(FinishedPageMsg::Quit);
                    }
                },
                gtk::Button {
                    set_label: "Restart",
                    connect_clicked[sender] => move |_| {
                        let _ = sender.output(FinishedPageMsg::StartOver);
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
        let model = Self {};

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
