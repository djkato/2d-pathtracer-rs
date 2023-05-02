use eframe::{
    egui::{self, Frame},
    CreationContext,
};
use path_tracer::PathTracer;
use tokio::runtime;
mod path_tracer;
fn main() {
    let mut options = eframe::NativeOptions::default();
    eframe::run_native(
        "2D Pathtracer",
        options,
        Box::new(|ctx| Box::new(PathTracerWindow::new(ctx))),
    )
    .unwrap();
}
struct PathTracerWindow {
    runtime: runtime::Runtime,
    started: bool,
    path_tracer: Option<PathTracer>,
}

impl PathTracerWindow {
    fn new(ctx: &CreationContext) -> Self {
        Self {
            runtime: runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            started: false,
            path_tracer: None,
        }
    }
}
impl eframe::App for PathTracerWindow {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Render!").clicked() {
                    self.started = true;
                }
            });
            Frame::canvas(ui.style()).show(ui, |ui| {
                if !self.started {
                    return;
                }
                ui.ctx().request_repaint();
                let time = ui.input(|i| i.time);
                let (_rect_id, rect) = ui.allocate_space(ui.available_size());

                if let Some(path_tracer) = &mut self.path_tracer {
                    ui.image(
                        &path_tracer.texture_handle,
                        path_tracer.texture_handle.size_vec2(),
                    );
                } else {
                    self.path_tracer = Some(PathTracer::new(
                        ui,
                        [rect.size().x as usize, rect.size().y as usize],
                    ));
                }
            });
        });
    }
}
