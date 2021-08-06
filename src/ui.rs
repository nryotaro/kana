use crate::core::document;
use gtk::prelude::*;
use gtk::{gdk, gio};
use std::sync::mpsc;
mod settings;

pub fn initialize(document_sender: mpsc::Sender<document::DocumentMessage>) -> gtk::Application {
	let application =
		gtk::Application::new(Some("dev.nryotaro.kana"), gio::ApplicationFlags::empty());

	application.connect_startup(move |app| {
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
		build_ui(app, &document_sender);
	});
	application
}

fn build_ui(
	application: &gtk::Application,
	document_sender: &mpsc::Sender<document::DocumentMessage>,
) {
	let builder: gtk::Builder = gtk::Builder::from_string(include_str!("ui/main.ui"));
	let window: gtk::ApplicationWindow = builder.object("window").expect("Couldn't get window");
	window.set_application(Some(application));
	settings::initialize(builder, &document_sender);
	application.connect_activate(move |_| {
		window.show_all();
	});
}
