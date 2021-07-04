use gtk::prelude::*;
use gtk::{gdk, gio, ApplicationWindow};
mod ui;

fn main() {
    let application =
        gtk::Application::new(Some("dev.nryotaro.kana"), gio::ApplicationFlags::empty());

    application.connect_startup(|app| {
        let provider = gtk::CssProvider::new();
        // Load the CSS file
        let style = include_bytes!("ui/style.css");
        provider.load_from_data(style).expect("Failed to load CSS");
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // We build the application UI.
        build_ui(app);
    });

    application.run();
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("ui/main.ui");
    let builder = gtk::Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    /*
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
    */
    application.connect_activate(move |_| {
        window.show_all();
    });
}
