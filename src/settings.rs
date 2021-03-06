#[cfg(unix)]
use ui::{UI, SET};
#[cfg(unix)]
use nvim::RedrawEvents;

use ui::Ui;
#[cfg(unix)]
use gio;

#[derive(PartialEq)]
pub enum FontSource {
    Rpc,
    #[cfg(unix)]
    Gnome,
    Default,
}


pub struct Settings {
    font_source: FontSource,

    #[cfg(unix)]
    gnome_interface_settings: gio::Settings,
}

impl Settings {
    #[cfg(unix)]
    pub fn new() -> Settings {
        Settings {
            font_source: FontSource::Default,
            gnome_interface_settings: gio::Settings::new("org.gnome.desktop.interface"),
        }
    }

    #[cfg(target_os = "windows")]
    pub fn new() -> Settings {
        Settings { font_source: FontSource::Default }
    }

    #[cfg(unix)]
    pub fn init(&mut self, ui: &mut Ui) {
        self.gnome_interface_settings.connect_changed(|_, _| monospace_font_changed());
        self.update_font(ui);
    }

    #[cfg(target_os = "windows")]
    pub fn init(&mut self, _: &mut Ui) {}

    #[cfg(unix)]
    fn update_font(&mut self, ui: &mut Ui) {
        // rpc is priority for font
        if self.font_source == FontSource::Rpc {
            return;
        }

        if let Some(ref font_name) =
            self.gnome_interface_settings
                .get_string("monospace-font-name") {
            ui.set_font_desc(font_name);
            self.font_source = FontSource::Gnome;
        }
    }

    pub fn set_font_source(&mut self, src: FontSource) {
        self.font_source = src;
    }
}

#[cfg(unix)]
fn monospace_font_changed() {
    UI.with(|ui_cell| {
        let mut ui = ui_cell.borrow_mut();

        SET.with(|set_cell| {
            let mut set = set_cell.borrow_mut();
            // rpc is priority for font
            if set.font_source != FontSource::Rpc {
                set.update_font(&mut *ui);
                ui.on_redraw();
            }
        });
    });
}
