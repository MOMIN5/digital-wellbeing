use eframe::egui;
use eframe::egui::{Color32, Pos2, Shape, Stroke, Vec2};
use egui::{Rounding, Label};
use egui::epaint::RectShape;
use rand::Rng;

use crate::read_file;

pub fn gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Digital Wellbeing Chart",
        options,
        Box::new(|_cc| Box::new(WellbeingChart::default())),
    )
}

struct WellbeingChart {
    data: Vec<(String, f32, Color32)>,
}

impl Default for WellbeingChart {
    fn default() -> Self {
        let mut data_vec: Vec<(String,f32,Color32)> = vec![];
        let hashmap_data = read_file();
        println!("{:?}",hashmap_data);
        for (name, time) in hashmap_data {
            let r = rand::thread_rng().gen_range(0..255);
            let g = rand::thread_rng().gen_range(0..255);
            let b = rand::thread_rng().gen_range(0..255);
            data_vec.push((name,time as f32,Color32::from_rgb(r, g, b)));
            //println!("data")
        }
        Self {
            data: data_vec,
        }
    }
}

impl eframe::App for WellbeingChart {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Digital Wellbeing Chart");

                let (response, painter) =
                    ui.allocate_painter(Vec2::new(300.0,300.0), egui::Sense::hover());

                let rect = response.rect;
                let center = rect.center();
                let radius = rect.width().min(rect.height()) / 3.0;

                let total: f32 = self.data.iter().map(|(_, value, _)| *value).sum();
                let mut start_angle = 0.0;
                //println!("{:?}",self.data);
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
                        Stroke::new(5.0, Color32::BLACK),
                    );

                    painter.add(path.clone());

                    start_angle = end_angle;
                    ui.label(format!("{}: {}\t",label,color.to_hex()));
                    
                    let (mut resp, pain) = ui.allocate_painter(Vec2::new(10.0, 10.0), egui::Sense::hover());
                    resp.rect.set_width(10.0);
                    let path2 = Shape::Rect(RectShape::new(resp.rect, Rounding::from(10.0), *color, Stroke::NONE));
                   // println!("{}, {}",resp.rect.width(),resp.rect.height());
                    
                    pain.add(path2);
                }
            });
        });
    }
}
