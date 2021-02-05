#![windows_subsystem = "windows"]
mod calculate_nic;
pub use crate::calculate_nic::calculate_nic;
use fltk::{image, app::*, group::*, input::*, output::*, text::*, window::*};
fn main() {
    let icon_data = include_bytes!("niccalc.png");
    let img = image::PngImage::from_data(icon_data).unwrap();
    let app = App::default().with_scheme(AppScheme::Gleam);
    let mut wind = Window::new(0, 0, 445, 300, "Niccalc");
    let mut tabs = Tabs::new(0, 0, 445, 300, "");
    tabs.end();
    let mut calc_tab = Group::new(0, 27, 400, 283, "Calculator");
    calc_tab.end();
    tabs.add(&calc_tab);
    let mut input_shotstr = FloatInput::new(255, 45, 185, 25, "Nicotine base strenght (mg/ml):");
    calc_tab.add(&input_shotstr);
    let mut input_targstr = FloatInput::new(255, 70, 185, 25, "Nicotine strength wanted (mg/ml):");
    calc_tab.add(&input_targstr);
    let mut input_targvol = FloatInput::new(255, 95, 185, 25, "Amount wanted (ml):");
    calc_tab.add(&input_targvol);
    let mut input_aromavol = FloatInput::new(255, 120, 185, 25, "Flavour amount (ml):");
    calc_tab.add(&input_aromavol);
    let mut output_result = MultilineOutput::new(255, 150, 185, 140, "");
    calc_tab.add(&output_result);
    let mut chart = fltk::misc::Chart::new(5, 150, 245, 140, "");
    calc_tab.add(&chart);
    let mut info_tab = Group::new(0, 27, 410, 273, "Info");
    info_tab.end();
    info_tab.hide();
    tabs.add(&info_tab);
    let mut info_display = TextDisplay::new(5, 30, 435, 265, "");
    info_tab.add(&info_display);
    let help_and_license =
        include_str!("README.md").to_owned() + "\n" + include_str!("LICENSE.txt");
    wind.set_icon(Some(img));
    let buffer = TextBuffer::default();
    info_display.set_buffer(Some(buffer));
    info_display.wrap_mode(WrapMode::AtColumn, 55);
    info_display.set_scrollbar_size(15);
    info_display.insert(&help_and_license);
    output_result.set_wrap(true);
    wind.end();
    wind.show();
    input_aromavol.set_value(&format!("{}", 0.0));
    input_shotstr.set_value(&format!("{}", 20.0));
    input_targstr.set_value(&format!("{}", 0.0));
    input_targvol.set_value(&format!("{}", 0.0));

    input_aromavol.set_trigger(CallbackTrigger::Changed);
    input_shotstr.set_trigger(CallbackTrigger::Changed);
    input_targstr.set_trigger(CallbackTrigger::Changed);
    input_targvol.set_trigger(CallbackTrigger::Changed);

    let (s, r) = fltk::app::channel::<bool>();
    input_aromavol.emit(s, true);
    input_shotstr.emit(s, true);
    input_targstr.emit(s, true);
    input_targvol.emit(s, true);
    while app.wait() {
        let inp_val_a: f64 = if input_aromavol.value().is_empty() {
            0.0
        } else {
            input_aromavol.value().parse().unwrap_or(0.0)
        };
        let inp_val_s: f64 = if input_shotstr.value().is_empty() {
            0.0
        } else {
            input_shotstr.value().parse().unwrap_or(0.0)
        };
        let inp_val_ts: f64 = if input_targstr.value().is_empty() {
            0.0
        } else {
            input_targstr.value().parse().unwrap_or(0.0)
        };
        let inp_val_tv: f64 = if input_targvol.value().is_empty() {
            0.0
        } else {
            input_targvol.value().parse().unwrap_or(0.0)
        };
        if let Some(_msg) = r.recv() {
            chart.clear();
            output_result.set_value("");
            if inp_val_s < 0.0 || inp_val_s >= 1000.0 {
                output_result
                    .set_value("Nicotine base strenght must be between 0.0 und 999.9mg/ml");
            } else if inp_val_ts < 0.0 || inp_val_ts > inp_val_s {
                output_result.set_value("Nicotine strenght wanted must be between  0 and value of nicotine base strenght");
            } else if inp_val_tv < 0.0 || inp_val_tv >= 100000.00 {
                output_result.set_value("The amount wanted must be between 0 and 100000!");
            } else if inp_val_a < 0.0
                || inp_val_a > (inp_val_tv - calculate_nic(inp_val_tv, inp_val_ts, inp_val_s))
            {
                output_result.set_value(
                    "The flavour amount must be between 0 and the base amount minus nicotine base amount!",
                );
            } else {
                chart.set_type(fltk::misc::ChartType::Bar);
                if inp_val_a == 0.0 {
                    let shots = calculate_nic(inp_val_tv, inp_val_ts, inp_val_s);
                    output_result.set_value(&format!("Base: {:.2}ml \nFlavour: {:.2}ml \nNicotine base: {:.2}ml ({:.2}mg/ml)\n\n{:.2}ml Base {:.2}mg/ml Nicotine",
                inp_val_tv - (shots  + inp_val_a), inp_val_a, shots, inp_val_s, inp_val_tv, inp_val_ts
            ));
                    chart.insert(1, shots, "Nicotine base", Color::DarkYellow);
                    chart.insert(2, inp_val_tv - (shots + inp_val_a), "Base", Color::DarkBlue);
                    chart.insert(3, inp_val_a, "Flavour", Color::Cyan);
                } else {
                    let shots = calculate_nic(inp_val_tv, inp_val_ts, inp_val_s);
                    output_result.set_value(&format!("Base: {:.2}ml \nFlavour: {:.2}ml \nNicotine Base: {:.2}ml ({:.2}mg/ml)\n\n{:.2}ml Liquid {:.2}mg/ml Nicotine",
                inp_val_tv - (shots + inp_val_a), inp_val_a, shots, inp_val_s, inp_val_tv, inp_val_ts
            ));
                    chart.insert(1, shots, "Nicotine Base", Color::DarkYellow);
                    chart.insert(2, inp_val_tv - (shots + inp_val_a), "Base", Color::DarkBlue);
                    chart.insert(3, inp_val_a, "Flavour", Color::Cyan);
                }
            }
        }
    }
}
