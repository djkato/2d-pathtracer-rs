pub mod common;
use eframe::egui;
use glam::*;
use rand::prelude::random;
use rayon::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;
pub struct PathTracer {
    pub texture_handle: egui::TextureHandle,
    texture: egui::ColorImage,
    next_texture: egui::ColorImage,
    render_size: [usize; 2],
    max_bounces: u8,
}
impl PathTracer {
    pub fn new(ui: &mut eframe::egui::Ui, size: [usize; 2]) -> Self {
        let texture = egui::ColorImage::new(size, egui::Color32::DARK_GRAY);
        let texture_handle = ui.ctx().load_texture(
            "render",
            texture.to_owned(),
            egui::TextureOptions::default(),
        );
        Self {
            texture_handle,
            texture,
            next_texture: texture.clone(),
            render_size: size,
            max_bounces: 7,
        }
    }
    pub fn resize(&mut self, size: [usize; 2]) {
        self.render_size = size
    }
    pub fn render(&mut self) {
        tokio::spawn(async move { self.cast_rows() });
    }
    async fn cast_rows(&mut self) {
        let width = self.next_texture.width();
        self.next_texture
            .pixels
            .chunks_exact_mut(width)
            .par_bridge()
            .for_each(|pixel_row| {
                for pixel in pixel_row {
                    pixel.from_pixel()
                }
            })
    }
}
trait RayCast {
    fn from_pixel(&mut self);
}
impl RayCast for egui::Color32 {
    fn from_pixel(&mut self) {}
}
#[derive(Clone, Copy)]
struct Ray {
    origin: Vec2,
    direction: Vec2,
}
#[derive(Clone, Copy)]
struct Material {
    mat_type: MaterialType,
    color: Vec3,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MaterialType {
    Dialectric,
    Lambertian,
    Light,
}
#[derive(Clone, Copy)]
struct Circle {
    origin: Vec2,
    radius: f32,
    material: Material,
}

/* --- INTERSECTION HITS --- */
