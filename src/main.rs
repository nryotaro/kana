use gtk::glib;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Box, Button, Label, Paned};

fn main() {
    let application = gtk::Application::new(Some("nryotaro.dev.kana"), Default::default());
    application.connect_activate(build_ui);

    application.run();
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("main.ui");
    let builder = gtk::Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let button: Button = builder.object("button").expect("Couldn't get button");

    let menu_builder = gtk::Builder::from_string(include_str!("menu.ui"));
    let gtk_box: Box = menu_builder.object("box").expect("Couldn't get doge");
    let label: Label = builder.object("label").expect("Couldn't get label");

    let paned: Paned = builder.object("paned").expect("Couldn't get paned");
    button.connect_clicked(move |_| {
        &paned.remove(&label);
        &paned.add2(&gtk_box);
        gtk_box.show_all();
    });
    /*
    button.connect_clicked(glib::clone!(@weak paned =>  move |_| {
        &paned.remove(&label);
    }));
    */
    window.show_all();
}
