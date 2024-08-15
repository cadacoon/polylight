#![feature(const_fn_floating_point_arithmetic, portable_simd)]

mod color;

use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "PolyLight",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([512.0, 512.0]),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::<App>::default())),
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
            output_image: egui::ColorImage::new([512, 512], egui::Color32::BLACK),
            update: true,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let output = self.output.get_or_insert_with(|| {
            ctx.load_texture("output", self.output_image.clone(), Default::default())
        });
        if self.update {
            output.set(self.output_image.clone(), Default::default());
            self.update = false;
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Image::new((output.id(), output.size_vec2())).shrink_to_fit());
        });
    }
}
