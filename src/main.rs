mod models;

use {
    fltk::{
        app,
        browser::Browser,
        enums::{CallbackTrigger, Color, Cursor, Event, Font, FrameType},
        frame::Frame,
        group::{Flex, Wizard},
        image::PngImage,
        input::{Input, InputType},
        menu::{MenuButton, MenuButtonType},
        misc::{Chart, Tooltip},
        prelude::*,
        text::{TextBuffer, TextDisplay, WrapMode},
        window::Window,
    },
    std::{cell::RefCell, rc::Rc},
};

const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;

enum Update {
    NicCal = 41,
}

impl Update {
    const fn event(self) -> Event {
        Event::from_i32(self as i32)
    }
}

fn main() -> Result<(), FltkError> {
    pub const LINE: i32 = PAD / 2;
    let state = Rc::new(RefCell::new(models::Model::default()));
    const UPDATE: Event = Update::NicCal.event();
    let app = app::App::default().load_system_fonts();
    app::set_scheme(app::Scheme::Base);
    app::set_frame_type2(FrameType::UpBox, FrameType::ThinUpBox);
    app::set_frame_type2(FrameType::DownBox, FrameType::ThinDownBox);
    app::set_background_color(238, 232, 213);
    app::set_background2_color(253, 246, 227);
    app::set_foreground_color(7, 54, 66);
    app::set_selection_color(203, 75, 22);
    app::set_inactive_color(181, 137, 0);
    Tooltip::set_color(Color::Background2);
    Tooltip::set_text_color(Color::Foreground);
    for (color, (r, g, b)) in [
        (Color::Red, (220, 50, 47)),
        (Color::Magenta, (211, 54, 130)),
        (Color::Blue, (38, 139, 210)),
        (Color::Cyan, (42, 161, 152)),
        (Color::Green, (133, 153, 0)),
        (Color::Yellow, (181, 137, 0)),
    ] {
        app::set_color(color, r, g, b);
    }
    app::set_visible_focus(false);
    app::set_font(match cfg!(target_os = "windows") {
        true => Font::by_name("BCascadia Mono"),
        false => Font::CourierBold,
    });
    let mut wnd = Window::default().with_size(640, 360).center_screen();
    wnd.size_range(640, 360, 0, 0);
    wnd.set_label("Niccalc");
    wnd.set_icon(Some(
        PngImage::from_data(include_bytes!("../assets/niccalc.png")).unwrap(),
    ));
    wnd.make_resizable(true);
    wnd.set_callback(move |_| {
        if app::event() == Event::Close {
            app::quit();
        }
    });
    wnd.add(&{
        let mut wzd = Wizard::default_fill();
        wzd.add(&{
            let mut vbox = Flex::default_fill().column().with_label("Calculator");
            vbox.set_frame(FrameType::FlatBox);
            vbox.set_margin(PAD);
            vbox.set_pad(PAD);
            vbox.fixed(&{
                let mut hbox = Flex::default_fill();
                hbox.set_margin(0);
                hbox.set_pad(PAD);
                hbox.fixed(&Flex::default(), 270);
                hbox.add(&{
                    let mut vbox = Flex::default_fill().column();
                    vbox.set_margin(0);
                    vbox.set_pad(PAD);
                    vbox.fixed(&{
                        let mut wgt = Input::default().with_label("Nicotine base strength (mg/ml):");
                        wgt.set_tooltip("Nicotine base strength must be between 0.0 und 999.9mg/ml");
                        wgt.set_trigger(CallbackTrigger::Changed);
                        wgt.set_type(InputType::Float);
                        wgt.set_value(&state.borrow().shotstr().to_string());
                        wgt.set_callback({
                            let state = state.clone();
                            move |input| {
                                if let Ok(value) = input.value().parse::<f64>() {
                                    if (0f64..1000f64).contains(&value) {
                                        input.set_color(Color::Background2);
                                        state.borrow_mut().set_shotstr(value);
                                    } else {
                                        input.set_color(Color::Red);
                                    }
                                }
                                app::handle_main(UPDATE).unwrap();
                            }
                        });
                        wgt
                    }, HEIGHT);
                    vbox.fixed(&{
                        let mut wgt = Input::default().with_label("Nicotine strength wanted (mg/ml):");
                        wgt.set_tooltip("Nicotine strength wanted must be between  0 and value of nicotine base strength");
                        wgt.set_trigger(CallbackTrigger::Changed);
                        wgt.set_type(InputType::Float);
                        wgt.set_value(&state.borrow().targstr().to_string());
                        wgt.set_callback({
                            let state = state.clone();
                            move |input| {
                                if let Ok(value) = input.value().parse::<f64>() {
                                    if (0f64..=state.borrow().shotstr()).contains(&value) {
                                        input.set_color(Color::Background2);
                                        state.borrow_mut().set_targstr(value);
                                    } else {
                                        input.set_color(Color::Red);
                                    }
                                }
                                app::handle_main(UPDATE).unwrap();
                            }
                        });
                        wgt
                    }, HEIGHT);
                    vbox.fixed(&{
                        let mut wgt = Input::default().with_label("Amount wanted (ml):");
                        wgt.set_tooltip("The amount wanted must be between 0 and 100000!");
                        wgt.set_trigger(CallbackTrigger::Changed);
                        wgt.set_type(InputType::Float);
                        wgt.set_value(&state.borrow().targvol().to_string());
                        wgt.set_callback({
                            let state = state.clone();
                            move |input| {
                                if let Ok(value) = input.value().parse::<f64>() {
                                    if (0f64..=100000f64).contains(&value) {
                                        input.set_color(Color::Background2);
                                        state.borrow_mut().set_targvol(value);
                                    } else {
                                        input.set_color(Color::Red);
                                    }
                                }
                                app::handle_main(UPDATE).unwrap();
                            }
                        });
                        wgt
                    }, HEIGHT);
                    vbox.fixed(&{
                        let mut wgt = Input::default().with_label("Flavour amount (ml):");
                        wgt.set_tooltip("The flavour amount must be between 0 and the base amount minus nicotine base amount!");
                        wgt.set_trigger(CallbackTrigger::Changed);
                        wgt.set_type(InputType::Float);
                        wgt.set_value(&state.borrow().aromavol().to_string());
                        wgt.set_callback({
                            let state = state.clone();
                            move |input| {
                                if let Ok(value) = input.value().parse::<f64>() {
                                    if (0f64..=state.borrow().limit()).contains(&value) {
                                        input.set_color(Color::Background2);
                                        state.borrow_mut().set_aromavol(value);
                                    } else {
                                        input.set_color(Color::Red);
                                    }
                                }
                                app::handle_main(UPDATE).unwrap();
                            }
                        });
                        wgt
                    }, HEIGHT);
                    vbox
                });
                hbox.end();
                hbox
            }, HEIGHT * 4 + PAD * 3);
            vbox.add(&{
                let mut hbox = Flex::default();
                hbox.set_margin(0);
                hbox.set_pad(PAD);
                hbox.add(&Frame::default());
                hbox.end();
                hbox.handle({
                    let state = state.clone();
                    move |flex, event| {
                        if event == UPDATE {
                            flex.clear();
                            flex.begin();
                            flex.fixed(&{
                                let mut tbl = Browser::default();
                                tbl.set_column_widths(&[115, 155]);
                                tbl.add("@uIngredient\t@uAmount(ml)");
                                for (x, y) in state.borrow().output() {
                                    tbl.add(&format!("{x}\t{y}"));
                                };
                                tbl
                            }, 270);
                            flex.add(&{
                                let mut color = [Color::Blue, Color::Green, Color::Yellow, Color::Red].iter();
                                let mut chart = Chart::default();
                                chart.set_frame(FrameType::DownBox);
                                chart.set_color(Color::Background2);
                                for (x, y) in state.borrow().output() {
                                    chart.add(y, x, *color.next().unwrap());
                                };
                                chart
                            });
                            flex.end();
                            return true;
                        };
                        false
                    }
                });
                hbox
            });
            vbox.end();
            vbox
        });
        wzd.add(&{
            let mut hbox = Flex::default_fill().with_label("Info");
            hbox.set_margin(PAD);
            hbox.add(&{
                let mut wgt = TextDisplay::default();
                wgt.wrap_mode(WrapMode::AtBounds, 0);
                wgt.set_scrollbar_size(16);
                wgt.set_buffer(TextBuffer::default());
                wgt.set_scrollbar_size(LINE);
                wgt.insert(&(include_str!("../README.md").to_owned() + "\n" + include_str!("../LICENSE.txt")));
                wgt
            });
            hbox.end();
            hbox
        });
        wzd.end();
        wzd.handle(add_menu);
        wzd
    });
    wnd.end();
    wnd.show();
    app.run()
}

fn add_menu(wizard: &mut Wizard, event: Event) -> bool {
    match event {
        Event::Push => match app::event_mouse_button() {
            app::MouseButton::Right => {
                let mut wgt = MenuButton::default();
                wgt.add_choice(
                    &(0..wizard.children())
                        .map(|x| {
                            let label = wizard.child(x).unwrap().label();
                            if wizard.try_current_widget().unwrap().label() == label {
                                format!("@->  {}", label)
                            } else {
                                format!("@-  {}", label)
                            }
                        })
                        .collect::<Vec<String>>()
                        .join("|"),
                );
                wgt.set_type(MenuButtonType::Popup3);
                wgt.set_callback({
                    let mut wizard = wizard.clone();
                    move |menu| {
                        wizard.set_current_widget(&wizard.child(menu.value()).unwrap());
                    }
                });
                wgt.popup();
                true
            }
            _ => false,
        },
        Event::Enter => {
            wizard.window().unwrap().set_cursor(Cursor::Hand);
            true
        }
        Event::Leave => {
            wizard.window().unwrap().set_cursor(Cursor::Arrow);
            true
        }
        _ => false,
    }
}
