use gtk::*;
use gtk::prelude::*;
use glib::clone;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.widget_subclass"),
        Default::default(),
    );

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title("Set button's margin");
        let button_1 = Button::with_label("Button №1");

        button_1.set_margin_top(18);
        button_1.set_margin_bottom(18);
        button_1.set_margin_start(18);
        button_1.set_margin_end(18);

        // 2. Then this one would print in terminal
        button_1.connect_clicked(move |_| println!("Button №1"));

        // 1. This one first would close the window
        button_1.connect_clicked(clone!(@weak window => move |_| 
            unsafe {
                window.destroy()
            }
        ));

        window.set_child(Some(&button_1));
        window.show_all();
    });

    application.run();
}
