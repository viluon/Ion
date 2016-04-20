extern crate gtk;
use gtk::prelude::*;

fn main() {
	if gtk::init().is_err() {
		println!("Failed to initialize GTK.");
		return;
	}

	let window = gtk::Window::new(gtk::WindowType::Toplevel);
	let header = gtk::HeaderBar::new();
	let open_button = gtk::Button::new_with_label("Open \u{25BE}");
	let save_button = gtk::Button::new_with_label("Save");
	let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

	window.set_title("Ion");
	window.set_position(gtk::WindowPosition::Center);
	window.set_default_size(400, 300);

	header.set_vexpand(false);
	header.set_title(Some("Ion"));
	header.set_subtitle(Some("No file"));
	header.set_show_close_button(true);
	header.pack_start(&open_button);
	header.pack_end(&save_button);
	header.set_decoration_layout(Some("menu:maximize,minimize,close"));

	window.add(&vbox);
	window.set_titlebar(Some(&header));

	window.connect_delete_event(|_, _| {
		gtk::main_quit();
		Inhibit(false)
	});
	let window1 = window.clone();
	open_button.connect_clicked(move |_| {
		let file_chooser = gtk::FileChooserDialog::new(Some("Open File"), Some(&window1), gtk::FileChooserAction::Open);
		file_chooser.add_buttons(&[
			("Open", gtk::ResponseType::Ok as i32),
			("Cancel", gtk::ResponseType::Cancel as i32),
		]);
		if file_chooser.run() == gtk::ResponseType::Ok as i32 {
			let filename = file_chooser.get_filename().unwrap();
			header.set_subtitle(filename.to_str());
			file_chooser.close();
		} else {
			file_chooser.close();
		}
	});

	window.show_all();
	gtk::main();
}
