use gtk::prelude::*;
use gtk::{gdk, gio, SearchEntry};

pub fn initialize_root_search(search_entry: SearchEntry) {
	search_entry.connect_changed(move |entry| {
		let text = String::from(entry.text().as_str());
		println!("{}", text);
		// idle_add
		// futureかasyncを使う wrapperを用意する。
	});
}
/*
fn build_ui(application: &gtk::Application) {
	let builder: gtk::Builder = gtk::Builder::from_string(include_str!("ui/main.ui"));
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
	ui::initialize();
	application.connect_activate(move |_| {
		window.show_all();
	});
}
*/
