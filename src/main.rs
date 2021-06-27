use gio::prelude::*;
use gtk::prelude::*;
use std::env;

fn main() {
    let app = gtk::Application::new(
        Some("com.danlogs.jojo2021"),
        gio::ApplicationFlags::FLAGS_NONE,
        );

    app.connect_activate(|app| {
        let glade_src = include_str!("../layout.glade");
        let builder = gtk::Builder::from_string(glade_src);
        let window : gtk::Window = builder.object("applicationwindow1").unwrap();
        window.set_application(Some(app));

        let message_input : gtk::Entry = builder.object("jojo_message").unwrap();
        let button : gtk::Button = builder.object("generate_btn").unwrap();
        let message_output : gtk::Label = builder.object("message_out").unwrap();
        let image_output : gtk::Image = builder.object("img_out").unwrap();
        let image_output_clone = image_output.clone();
        let ret_switch : gtk::Switch = builder.object("return_btn").unwrap();
        let brad_switch : gtk::Switch = builder.object("brad_btn").unwrap();

        button.connect_clicked(move |_| {
            let is_ret = ret_switch.is_active();
            let is_brad = brad_switch.is_active();
            let message_gstring = message_input.text();
            let message = if message_gstring.as_str().is_empty() { &"Create by DaemonWhite" } else { message_gstring.as_str() };
            if  is_ret {
                message_output.set_text(&format!("{} ", message,));
                image_output_clone.set_from_file("./img/deat.jpeg");
            } else if is_brad {
                message_output.set_text(&format!("{} ", message,));
                image_output_clone.set_from_file("./img/pedo.png");
            } else {
                message_output.set_text(&format!("{} ", message,));
                image_output_clone.set_from_file("./img/jojo.jpeg");
            }

        image_output_clone.show()
        });

        window.show_all();
        image_output.hide(); 
    });

    app.run();
}