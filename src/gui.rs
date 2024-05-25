use std::sync::Arc;

use chrono::{NaiveDate, Datelike};
use eframe::egui;
use eframe::egui::{Color32, Pos2, Shape, Stroke, Vec2};
use egui::{Rounding, RichText, FontId, Label, Window, IconData, ViewportBuilder};
use egui::epaint::RectShape;
use rand::Rng;

use crate::{read_file, get_filepath};

pub fn gui() -> Result<(), eframe::Error> {

    let icon = eframe::icon_data::from_png_bytes(include_bytes!("icon.png")).unwrap();

    let options = eframe::NativeOptions{
        viewport : ViewportBuilder::default().with_icon(Arc::new(IconData {
            rgba: icon.rgba,
            height: icon.height,
            width: icon.width,
        })),
        ..Default::default()
    };


    eframe::run_native(
        "Digital Wellbeing Monitor",
        options,
        Box::new(|_cc| Box::new(WellbeingChart::default())),
    )
}

fn get_format_time(time: &f32) -> String {
    let seconds = time % 60.0;
    let minutes = ((time / 60.0) % 60.0).floor();
    let hours = ((time / 60.0) / 60.0).floor();

    let mut message = String::new();

    if minutes < 1.0 {
        message = format!(": {seconds}sec\t");
    }else if hours < 1.0 {
        message = format!(": {minutes}min {seconds}sec\t");
    }else{
        message = format!(": {hours}hr {minutes}min {seconds}sec\t");
    }
    return message;
}

struct WellbeingChart {
    data: Vec<(String, f32, Color32)>,
    date: NaiveDate,
    show_error_window: bool,
    show_future_date_window: bool,
}

impl Default for WellbeingChart {
    fn default() -> Self {
        let mut data_vec: Vec<(String,f32,Color32)> = vec![];
        let curr_date = chrono::Local::now().date_naive();
        let hashmap_data = read_file(&get_filepath());

        for (name, time) in hashmap_data {
            let r = rand::thread_rng().gen_range(0..255);
            let g = rand::thread_rng().gen_range(0..255);
            let b = rand::thread_rng().gen_range(0..255);
            if time > 0{
                data_vec.push((name,time as f32,Color32::from_rgb(r, g, b)));
            }
        }
        Self {
            data: data_vec,
            date: curr_date,
            show_error_window: false,
            show_future_date_window: false,
        }
    }
}

impl eframe::App for WellbeingChart {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let mut total_time = 0.0;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("Digital Wellbeing Monitor").font(FontId::monospace(20.0)).color(Color32::WHITE));

                let (response, painter) = ui.allocate_painter(Vec2::new(300.0,300.0), egui::Sense::hover());
                let rect = response.rect;
                let center = rect.center();
                let radius = rect.width().min(rect.height()) / 3.0;

                let total: f32 = self.data.iter().map(|(_, value, _)| *value).sum();
                let mut start_angle = 0.0;

                for (label, value, color) in &self.data {
                    let end_angle = start_angle + 2.0 * std::f32::consts::PI * value / total;
                    let points: Vec<Pos2> = (0..=100).map(|i| {
                        let angle = start_angle + (end_angle - start_angle) * (i as f32 / 100.0);
                        Pos2::new(
                            center.x + angle.cos() * radius,
                            center.y + angle.sin() * radius,
                        )
                    }).collect();

                    let path = Shape::convex_polygon(
                        std::iter::once(center).chain(points.iter().cloned()).collect(),
                        *color,
                        Stroke::new(2.0, Color32::WHITE),
                    );

                    painter.add(path.clone());

                    start_angle = end_angle;
                }
            });
            ui.horizontal_wrapped(|ui2| {
                for (name,time,color) in &self.data {

                    let (mut resp, pain) = ui2.allocate_painter(Vec2::new(20.0, 20.0), egui::Sense::hover());
                    resp.rect.set_width(20.0);
                    let path2 = Shape::Rect(RectShape::new(resp.rect, Rounding::from(7.0), *color, Stroke::NONE));

                    pain.add(path2);

                    total_time += time;
                    let message = name.to_owned() + get_format_time(time).as_str();

                    ui2.add_sized(Vec2::new(20.0, 20.0), Label::new(RichText::new(message).font(FontId::monospace(13.0)).color(Color32::WHITE)));
                }
            });
            ui.vertical_centered(|ui3|{
                let tot_time_text = format!("\nTotal Time on {}/{}/{}",&self.date.day(),&self.date.month(),&self.date.year()) + get_format_time(&total_time).as_str();
                ui3.label(RichText::new(tot_time_text).font(FontId::monospace(22.5)).color(Color32::LIGHT_GRAY));
            });

            ui.horizontal(|ui|{
                if ui.button(RichText::new("Previous Day").size(20.0)).clicked() {

                    let path = std::env::var("APPDATA").map( |path| path.to_string()).unwrap();
                    self.date = self.date.pred_opt().unwrap();

                    let mut file_path = path.to_owned().to_string() + "\\digital-wellbeing\\Data\\" + self.date.to_string().as_str() + ".log";
                    let mut prev_date_vec = vec![];
                    let mut prev_date_data = read_file(&file_path);

                    if prev_date_data.is_empty() {
                        self.show_error_window = true;
                        self.date = self.date.succ_opt().unwrap();
                        file_path = path + "\\digital-wellbeing\\Data\\" + self.date.to_string().as_str() + ".log";
                        prev_date_data = read_file(&file_path);
                        ui.set_enabled(false);
                    }else{

                        for (name, time) in prev_date_data {
                            let r = rand::thread_rng().gen_range(0..255);
                            let g = rand::thread_rng().gen_range(0..255);
                            let b = rand::thread_rng().gen_range(0..255);
                            if time > 0{
                                prev_date_vec.push((name,time as f32,Color32::from_rgb(r, g, b)));
                            }
                        }
                        self.data = prev_date_vec;
                    }
                }
                if self.show_error_window == true {
                    Window::new("Error").resizable(false).collapsible(false).movable(false).show(ctx, |box_ui|{
                        box_ui.label("There is no data available for the previous date");
                        if box_ui.button("CLOSE").clicked() {
                            self.show_error_window = false;
                        }
                    });
                }

                if ui.button(RichText::new("Next Day").size(20.0)).clicked() {
                   if self.date == chrono::Local::now().date_naive() {
                       self.show_future_date_window = true;
                   } else {
                       let path = std::env::var("APPDATA").map( |path| path.to_string()).unwrap();
                       self.date = self.date.succ_opt().unwrap();

                        let file_path = path.to_owned().to_string() + "\\digital-wellbeing\\Data\\" + self.date.to_string().as_str() + ".log";
                        let mut prev_date_vec = vec![];
                        let prev_date_data = read_file(&file_path);

                        for (name, time) in prev_date_data {
                            let r = rand::thread_rng().gen_range(0..255);
                            let g = rand::thread_rng().gen_range(0..255);
                            let b = rand::thread_rng().gen_range(0..255);
                            if time > 0{
                                prev_date_vec.push((name,time as f32,Color32::from_rgb(r, g, b)));
                            }
                        }
                        self.data = prev_date_vec;
                   }
                }
                if self.show_future_date_window == true {
                    Window::new("Error").resizable(false).collapsible(false).movable(false).show(ctx, |box_ui|{
                        box_ui.label("You are at the latest date!");
                        if box_ui.button("CLOSE").clicked() {
                            self.show_future_date_window = false;
                        }
                    });
                }
            });
        });
    }
}
