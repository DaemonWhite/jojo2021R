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
        let ret_switch : gtk::Switch = builder.object("ret_btn").unwrap();
        let brad_switch : gtk::Switch = builder.object("brad_btn").unwrap();
        let math_switch : gtk::Switch = builder.object("math_btn").unwrap();
        let fire_switch : gtk::Switch = builder.object("fire_btn").unwrap();

        button.connect_clicked(move |_| {
            let is_ret = ret_switch.is_active();
            let is_brad = brad_switch.is_active();
            let is_math = math_switch.is_active();
            let is_fire = fire_switch.is_active();;
            let message_gstring = message_input.text();
            let message = if message_gstring.as_str().is_empty() { &"Create by DaemonWhite" } else { message_gstring.as_str() };

            if  is_brad && is_math && is_ret && is_fire {
                message_output.set_text(&format!("{} ", message,));
                image_output_clone.set_from_file("./img/1.jpeg");
            } else if is_brad && is_math && is_ret {
                message_output.set_text(&format!("{} ", message,));
                image_output_clone.set_from_file("./img/2.jpg");
            } else if is_brad && is_math && is_fire {
                message_output.set_text(&format!("{} ", message,));
                image_output_clone.set_from_file("./img/4.png");
            } else if is_brad && is_ret && is_fire {
                message_output.set_text(&format!("{} ", message,));
                image_output_clone.set_from_file("./img/7.jpg");
            } else if is_math && is_ret && is_fire {
                message_output.set_text(&format!("{} ", message,));
                image_output_clone.set_from_file("./img/10.jpeg");
            } else if is_brad && is_math {
                message_output.set_text(&format!("{} ", message,));
                image_output_clone.set_from_file("./img/3.png");
            } else if is_brad && is_fire {
                message_output.set_text(&format!("{} ", message,));
                image_output_clone.set_from_file("./img/5.png");
            } else if is_math && is_fire {
                message_output.set_text(&format!("{} ", message,));
                image_output_clone.set_from_file("./img/6.jpeg");
            } else if is_ret && is_fire {
                message_output.set_text(&format!("{} ", message,));
                image_output_clone.set_from_file("./img/8.jpeg");
            } else if is_brad && is_ret {
                message_output.set_text(&format!("{} ", message,));
                image_output_clone.set_from_file("./img/9.jpeg");
            } else if is_math && is_ret {
                message_output.set_text(&format!("{} ", message,));
                image_output_clone.set_from_file("./img/11.png");
            } else if is_brad {
                message_output.set_text(&format!("{} ", message,));
                image_output_clone.set_from_file("./img/brad.png");
            } else if is_math {
                message_output.set_text(&format!("{} ", message,));
                image_output_clone.set_from_file("./img/math.jpeg");
            } else if is_ret {
                message_output.set_text(&format!("{} ", message,));
                image_output_clone.set_from_file("./img/deat.jpeg");
            } else if is_fire {
		        message_output.set_text(&format!("{} ", message,));
                image_output_clone.set_from_file("./img/fire.jpeg");
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
