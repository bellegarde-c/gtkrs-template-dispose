// Copyright (c) 2021-2022 Cedric Bellegarde <cedric.bellegarde@adishatz.org>

use adw::subclass::prelude::*;
use gtk::{self, prelude::*};
use gtk::{glib, CompositeTemplate};

mod imp {
    use super::*;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/org/gnome/gtkrs_test/row.ui")]
    pub struct Row {
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Row {
        const NAME: &'static str = "Row";
        type Type = super::Row;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Row {
        fn dispose(&self) {
            self.obj().dispose_template(Self::Type::static_type());
        }
    }

    impl WidgetImpl for Row {}
}

glib::wrapper! {
    pub struct Row(ObjectSubclass<imp::Row>)
        @extends gtk::Widget;
}

#[gtk::template_callbacks]
impl Row {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }
}
