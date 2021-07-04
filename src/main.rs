use gtk::prelude::*;
mod ui;

fn main() {
    let application = ui::initialize();
    application.run();
}
