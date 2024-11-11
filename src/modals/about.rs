use gtk::prelude::GtkWindowExt;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::config::{APP_ID, VERSION};

pub struct AboutDialog {}

impl SimpleComponent for AboutDialog {
    type Init = ();
    type Widgets = adw::AboutWindow;
    type Input = ();
    type Output = ();
    type Root = adw::AboutWindow;

    fn init_root() -> Self::Root {
        adw::AboutWindow::builder()
            .application_icon(APP_ID)
            .license_type(gtk::License::Agpl30)
            .website("https://github.com/MTRNord/heic2jpg")
            .issue_url("https://github.com/MTRNord/heic2jpg/issues")
            .application_name("Heic2JPG")
            .version(VERSION)
            .translator_credits("translator-credits")
            .copyright("© 2024 MTRNord")
            .developers(vec!["MTRNord"])
            .designers(vec!["MTRNord"])
            .hide_on_close(true)
            .build()
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        let widgets = root.clone();

        ComponentParts { model, widgets }
    }

    fn update_view(&self, dialog: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        dialog.present();
    }
}
