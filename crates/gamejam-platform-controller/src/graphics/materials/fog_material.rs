use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef}
};
use bevy::sprite::{AlphaMode2d, Material2d};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FogMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Handle<Image>,
}

impl Material2d for FogMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fog.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}