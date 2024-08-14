#![feature(const_fn_floating_point_arithmetic)]

mod color;

use std::ops::Index;
use std::time::Instant;
use eframe::{egui, Frame};
use eframe::egui::Color32;
use egui_plot::{Line, Plot, PlotPoints};
use parry3d::math::{Isometry, Point, Vector};
use parry3d::query::{Ray, RayCast};
use parry3d::shape::Ball;
use crate::color::{CIE, REC709, Vspd, Xyz};

fn main() -> eframe::Result {
    eframe::run_native(
        "PolyLight",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
            ..Default::default()
        },
        Box::new(|cc| {
            Ok(Box::<App>::default())
        }),
    )
}

struct App {
    output: Option<egui::TextureHandle>,
    output_image: egui::ColorImage,
    update: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            output: None,
            output_image: egui::ColorImage::new([830 - 360, 512], egui::Color32::BLACK),
            update: true,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        let output = self.output.get_or_insert_with(|| ctx.load_texture("output", self.output_image.clone(), Default::default()));
        if self.update {
            let time = Instant::now();
            let camera = Camera::new(Point::origin(), Point::new(1.0, 0.0, 0.0 ), Vector::new(0.0, 1.0, 0.0),20.0, (self.output_image.width() as f32) / (self.output_image.height() as f32));
            let ball = Ball::new(1.0);
            let isom = Isometry::translation(10.0, 0.0, 0.0);

            let mut i = 0;
            for x in 0..self.output_image.height() {
                for y in 0..self.output_image.width() {
                    /*let u = x as f32 / self.output_image.width() as f32;
                    let v = y as f32 / self.output_image.height() as f32;
                    let r = camera.get_ray(u, v);
                    if let Some(c) = ball.cast_ray_and_get_normal(&isom, &r, f32::MAX, true) {
                        self.output_image.pixels[i] = egui::Color32::from_rgb(((c.normal.x + 1.0) * 128.0) as u8, ((c.normal.y + 1.0) * 128.0) as u8, ((c.normal.z + 1.0) * 128.0) as u8);
                    }*/
                    let x = y as f32 + 360.0;
                    let xyz = Xyz::from(Vspd(&[(x, 1.0)]));
                    let rgb = xyz.rgb(REC709);

                    self.output_image.pixels[i] = Color32::from_rgb((rgb.x * 256.0).min(255.0) as u8, (rgb.y * 256.0).min(255.0) as u8, (rgb.z * 256.0).min(255.0) as u8);
                    i += 1;
                }
            }
            output.set(self.output_image.clone(), Default::default());
            println!("{:?}", time.elapsed());

            self.update = false;
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Image::new((output.id(), output.size_vec2())).shrink_to_fit());
            /*let xa: PlotPoints = (360..830).map(|i| {
                let x = i as f32;
                let xyz = Xyz::from(Vspd(&[(x, 1.0)]));
                [x as f64, xyz.rgb(CIE).x as f64]
            }).collect();
            let xb: PlotPoints = (360..830).map(|i| {
                let x = i as f32;
                let xyz = Xyz::from(Vspd(&[(x, 1.0)]));
                [x as f64, xyz.rgb(CIE).y as f64]
            }).collect();
            let xc: PlotPoints = (360..830).map(|i| {
                let x = i as f32;
                let xyz = Xyz::from(Vspd(&[(x, 1.0)]));
                [x as f64, xyz.rgb(CIE).z as f64]
            }).collect();
            let linea = Line::new(xa).color(Color32::RED);
            let lineb = Line::new(xb).color(Color32::GREEN);
            let linec = Line::new(xc).color(Color32::BLUE);
            Plot::new("cie").view_aspect(2.0).show(ui, |pui| {
                pui.line(linea);
                pui.line(lineb);
                pui.line(linec);
            });*/
        });
    }
}

struct Camera {
    origin: Point<f32>,
    corner: Point<f32>,
    horizontal: Vector<f32>,
    vertical: Vector<f32>,
}

impl Camera {
    fn new(origin: Point<f32>, look_at: Point<f32>, vup: Vector<f32>, vfov: f32, aspect: f32) -> Self {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (origin - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let corner = origin - (u * half_width) - (v * half_height) - w;
        let horizontal = u * 2.0 * half_width;
        let vertical = v * 2.0 * half_height;

        Self {
            origin,
            corner,
            horizontal,
            vertical,
        }
    }

    fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.corner + (self.horizontal * u) + (self.vertical * v) - self.origin)
    }
}
