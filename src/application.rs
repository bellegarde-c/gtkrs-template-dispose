/* application.rs
 *
 * Copyright 2022 Cédric Bellegarde
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use glib::clone;
use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::config::VERSION;
use crate::GtkrsTestWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct GtkrsTestApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for GtkrsTestApplication {
        const NAME: &'static str = "GtkrsTestApplication";
        type Type = super::GtkrsTestApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for GtkrsTestApplication {
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().setup_gactions();
            self.obj().set_accels_for_action("app.quit", &["<primary>q"]);
        }
    }

    impl ApplicationImpl for GtkrsTestApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            // Get the current window or create one if necessary
            let window = if let Some(window) = self.obj().active_window() {
                window
            } else {
                let window = GtkrsTestWindow::new(&*self.obj());
                window.upcast()
            };

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for GtkrsTestApplication {}
    impl AdwApplicationImpl for GtkrsTestApplication {}
}

glib::wrapper! {
    pub struct GtkrsTestApplication(ObjectSubclass<imp::GtkrsTestApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl GtkrsTestApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::new(&[
            ("application-id", &application_id),
            ("flags", flags),
        ])
    }

    fn setup_gactions(&self) {
        let quit_action = gio::SimpleAction::new("quit", None);
        quit_action.connect_activate(clone!(@weak self as app => move |_, _| {
            app.quit();
        }));
        self.add_action(&quit_action);

        let about_action = gio::SimpleAction::new("about", None);
        about_action.connect_activate(clone!(@weak self as app => move |_, _| {
            app.show_about();
        }));
        self.add_action(&about_action);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutWindow::builder()
            .transient_for(&window)
            .application_name("gtkrs-test")
            .application_icon("org.gnome.gtkrs_test")
            .developer_name("Cédric Bellegarde")
            .version(VERSION)
            .developers(vec!["Cédric Bellegarde".into()])
            .copyright("© 2022 Cédric Bellegarde")
            .build();

        about.present();
    }
}
