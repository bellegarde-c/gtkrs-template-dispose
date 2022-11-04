/* window.rs
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

use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};

use crate::row::Row;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/org/gnome/gtkrs_test/window.ui")]
    pub struct GtkrsTestWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<gtk::HeaderBar>,
        #[template_child]
        pub listbox: TemplateChild<gtk::ListBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GtkrsTestWindow {
        const NAME: &'static str = "GtkrsTestWindow";
        type Type = super::GtkrsTestWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GtkrsTestWindow {}
    impl WidgetImpl for GtkrsTestWindow {
        fn map(&self) {
            let obj = self.obj();
            obj.add_label();
            obj.add_label();
            obj.add_label();
            self.parent_map();
        }
    }
    impl WindowImpl for GtkrsTestWindow {}
    impl ApplicationWindowImpl for GtkrsTestWindow {}
    impl AdwApplicationWindowImpl for GtkrsTestWindow {}
}

glib::wrapper! {
    pub struct GtkrsTestWindow(ObjectSubclass<imp::GtkrsTestWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl GtkrsTestWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::new::<Self>(&[("application", application)])
    }

    pub fn add_label(&self) {
        let this = self.imp();
        match this.listbox.row_at_index(0) {
            Some(widget) => {
                this.listbox.remove(&widget);
            }
            None => {}
        }
        let row = Row::new();
        this.listbox.append(&row);
    }
}
