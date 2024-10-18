mod models;

use {
    cascade::cascade,
    comfy_table::{modifiers, presets, Table},
    fltk::{
        app,
        enums::{Align, CallbackTrigger, Color, Cursor, Event, Font, FrameType},
        frame::Frame,
        group::{Flex, Wizard},
        image::PngImage,
        input::{Input, InputType},
        menu::{Choice, MenuButton, MenuButtonType},
        misc::Chart,
        prelude::*,
        text::{TextBuffer, TextDisplay, WrapMode},
        valuator::{Counter, CounterType},
        window::Window,
    },
    std::{cell::RefCell, rc::Rc},
};

const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const WIDTH: i32 = HEIGHT * 3;

enum Update {
    NicCal = 41,
}

impl Update {
    const fn event(self) -> Event {
        Event::from_i32(self as i32)
    }
}

fn main() -> Result<(), FltkError> {
    let state = Rc::new(RefCell::new(models::Model::default()));
    const UPDATE: Event = Update::NicCal.event();
    let app = app::App::default();
    cascade!(
        Window::default().with_size(640, 360).center_screen();
        ..size_range(640, 360, 0, 0);
        ..set_label("Niccalc");
        ..set_icon(Some(PngImage::from_data(include_bytes!("../assets/niccalc.png")).unwrap()));
        ..make_resizable(true);
        ..set_callback(move |_| {
            if app::event() == Event::Close {
                app::quit();
            }
        });
        ..add(&cascade!(
            Wizard::default_fill();
            ..add(&cascade!(
                Flex::default_fill().column().with_label("Calculator");
                ..set_frame(FrameType::FlatBox);
                ..set_margin(PAD);
                ..set_pad(PAD);
                ..fixed(&cascade!(
                    Flex::default_fill();
                    ..set_margin(0);
                    ..set_pad(PAD);
                    ..fixed(&Flex::default(), 270);
                    ..add(&cascade!(
                        Flex::default_fill().column();
                        ..set_margin(0);
                        ..set_pad(PAD);
                        ..fixed(&cascade!(
                            Input::default().with_label("Nicotine base strength (mg/ml):");
                            ..set_tooltip("Nicotine base strength must be between 0.0 und 999.9mg/ml");
                            ..set_trigger(CallbackTrigger::Changed);
                            ..set_type(InputType::Float);
                            ..set_value(&state.borrow().shotstr().to_string());
                            ..set_callback(glib::clone!(#[strong] state, move |input| {
                                if let Ok(value) = input.value().parse::<f64>() {
                                    if (0f64..1000f64).contains(&value) {
                                        input.set_color(Color::Background2);
                                        state.borrow_mut().set_shotstr(value);
                                    } else {
                                        input.set_color(Color::Red);
                                    }
                                }
                                app::handle_main(UPDATE).unwrap();
                            }));
                        ), HEIGHT);
                        ..fixed(&cascade!(
                            Input::default().with_label("Nicotine strength wanted (mg/ml):");
                            ..set_tooltip("Nicotine strength wanted must be between  0 and value of nicotine base strength");
                            ..set_trigger(CallbackTrigger::Changed);
                            ..set_type(InputType::Float);
                            ..set_value(&state.borrow().targstr().to_string());
                            ..set_callback(glib::clone!(#[strong] state, move |input| {
                                if let Ok(value) = input.value().parse::<f64>() {
                                    if (0f64..=state.borrow().shotstr()).contains(&value) {
                                        input.set_color(Color::Background2);
                                        state.borrow_mut().set_targstr(value);
                                    } else {
                                        input.set_color(Color::Red);
                                    }
                                }
                                app::handle_main(UPDATE).unwrap();
                            }));
                        ), HEIGHT);
                        ..fixed(&cascade!(
                            Input::default().with_label("Amount wanted (ml):");
                            ..set_tooltip("The amount wanted must be between 0 and 100000!");
                            ..set_trigger(CallbackTrigger::Changed);
                            ..set_type(InputType::Float);
                            ..set_value(&state.borrow().targvol().to_string());
                            ..set_callback(glib::clone!(#[strong] state, move |input| {
                                if let Ok(value) = input.value().parse::<f64>() {
                                    if (0f64..=100000f64).contains(&value) {
                                        input.set_color(Color::Background2);
                                        state.borrow_mut().set_targvol(value);
                                    } else {
                                        input.set_color(Color::Red);
                                    }
                                }
                                app::handle_main(UPDATE).unwrap();
                            }));
                        ), HEIGHT);
                        ..fixed(&cascade!(
                            Input::default().with_label("Flavour amount (ml):");
                            ..set_tooltip("The flavour amount must be between 0 and the base amount minus nicotine base amount!");
                            ..set_trigger(CallbackTrigger::Changed);
                            ..set_type(InputType::Float);
                            ..set_value(&state.borrow().aromavol().to_string());
                            ..set_callback(glib::clone!(#[strong] state, move |input| {
                                if let Ok(value) = input.value().parse::<f64>() {
                                    if (0f64..=state.borrow().limit()).contains(&value) {
                                        input.set_color(Color::Background2);
                                        state.borrow_mut().set_aromavol(value);
                                    } else {
                                        input.set_color(Color::Red);
                                    }
                                }
                                app::handle_main(UPDATE).unwrap();
                            }));
                        ), HEIGHT);
                    ));
                ), HEIGHT * 4 + PAD * 3);
                ..add(&cascade!(
                    Flex::default();
                    ..set_margin(0);
                    ..set_pad(PAD);
                    ..add(&Frame::default());
                    ..end();
                    ..handle(glib::clone!(#[strong] state, move |flex, event| {
                        if event == UPDATE {
                            flex.clear();
                            flex.begin();
                            flex.fixed(&cascade!(
                                TextDisplay::default();
                                ..set_buffer(TextBuffer::default());
                                ..insert({
                                    let mut table = Table::new();
                                    table.load_preset(presets::UTF8_FULL);
                                    table.apply_modifier(modifiers::UTF8_ROUND_CORNERS);
                                    table.set_header(["Ingredient", "Amount(ml)"]);
                                    for (x, y) in state.borrow().output() {
                                        table.add_row([x, &y.to_string()]);
                                    }
                                    &table.to_string()
                                });
                            ), 270);
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
                    }));
                ));
                ..end();
            ));
            ..add(&page_settings());
            ..add(&cascade!(
                Flex::default_fill().with_label("Info");
                ..set_margin(PAD);
                ..add(&cascade!(
                    TextDisplay::default();
                    ..wrap_mode(WrapMode::AtBounds, 0);
                    ..set_scrollbar_size(16);
                    ..set_buffer(TextBuffer::default());
                    ..insert(&(include_str!("../README.md").to_owned() + "\n" + include_str!("../LICENSE.txt")));
                ));
                ..end();
            ));
            ..end();
            ..handle(add_menu);
        ));
        ..end();
    )
    .show();
    app.run()
}

fn add_menu(wizard: &mut Wizard, event: Event) -> bool {
    match event {
        Event::Push => match app::event_mouse_button() {
            app::MouseButton::Right => {
                cascade!(
                    MenuButton::default();
                    ..add_choice(
                        &(0..wizard.children()).map(|x| {
                            let label = wizard.child(x).unwrap().label();
                            if wizard.try_current_widget().unwrap().label() == label {
                                format!("@->  {}", label)
                            } else {
                                format!("@-  {}", label)
                            }
                        })
                        .collect::<Vec<String>>()
                        .join("|")
                    );
                    ..set_type(MenuButtonType::Popup3);
                    ..set_callback({
                        let mut wizard = wizard.clone();
                        move |menu| {
                            wizard.set_current_widget(&wizard.child(menu.value()).unwrap());
                        }
                    });
                )
                .popup();
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

fn page_settings() -> Flex {
    cascade!(
        Flex::default_fill().with_label("Settings");
        ..set_frame(FrameType::FlatBox);
        ..set_margin(PAD);
        ..set_pad(PAD);
        ..add(&Frame::default());
        ..fixed(&cascade!(
            Flex::default_fill().column();
            ..set_pad(PAD);
            ..set_margin(PAD);
            ..add(&Frame::default());
            ..add(&cascade!(
                Flex::default_fill();
                ..fixed(&Frame::default(), WIDTH);
                ..add(&cascade!(
                    Flex::default_fill().column();
                    ..set_color(Color::Foreground);
                    ..set_pad(PAD);
                    ..fixed(&cascade!(
                        Choice::default().with_label("Theme");
                        ..add_choice("Light|Dark");
                        ..set_value(0);
                        ..set_callback(move |choice| {
                            let color = [
                                [
                                    0xF6F5F4, //set_background_color
                                    0xFCFCFC, //set_background2_color
                                    0x323232, //set_foreground_color
                                    0x3584E4, //set_selection_color
                                ],
                                [
                                    0x353535, //set_background_color
                                    0x303030, //set_background2_color
                                    0xD6D6D6, //set_foreground_color
                                    0x15539E, //set_selection_color
                                ],
                            ][choice.value() as usize];
                            app::set_scheme(match choice.value() {
                                0 => app::Scheme::Oxy,
                                _ => app::Scheme::Gtk,
                            });
                            let (r, g, b) = Color::from_hex(color[0]).to_rgb();
                            app::set_background_color(r, g, b);
                            let (r, g, b) = Color::from_hex(color[1]).to_rgb();
                            app::set_background2_color(r, g, b);
                            let (r, g, b) = Color::from_hex(color[2]).to_rgb();
                            app::set_foreground_color(r, g, b);
                            let (r, g, b) = Color::from_hex(color[3]).to_rgb();
                            app::set_selection_color(r, g, b);
                            app::set_color(Color::Blue, r, g, b);
                            for (color, hex) in [
                                (Color::Yellow, 0xb58900),
                                (Color::Red, 0xdc322f),
                                (Color::Magenta, 0xd33682),
                                (Color::Cyan, 0x2aa198),
                                (Color::Green, 0x859900),
                            ] {
                                let (r, g, b) = Color::from_hex(hex).to_rgb();
                                app::set_color(color, r, g, b);
                            }
                            app::set_visible_focus(false);
                            app::redraw();
                        });
                        ..do_callback();
                    ), HEIGHT);
                    ..fixed(&cascade!(
                        Choice::default().with_label("Font name");
                        ..add_choice(&app::fonts().join("|"));
                        ..set_value(5);
                        ..set_callback(move |choice| {
                            app::set_font(Font::by_index(choice.value() as usize));
                        });
                        ..do_callback();
                    ), HEIGHT);
                    ..fixed(&cascade!(
                        Counter::default().with_label("Font size");
                        ..set_type(CounterType::Simple);
                        ..set_align(Align::Left);
                        ..set_range(1_f64, 14f64);
                        ..set_precision(0);
                        ..set_value(14f64);
                        ..set_callback(move |counter| {
                            app::set_font_size(counter.value() as i32);
                        });
                        ..do_callback();
                    ), HEIGHT);
                    ..end();
                ));
                ..end();
            ));
            ..add(&Frame::default());
            ..end();
        ), WIDTH * 3);
        ..add(&Frame::default());
        ..end();
    )
}
