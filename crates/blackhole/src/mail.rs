//! What the orbit and the lens are made with, and the mail between them: the lens asks the orbit for the view the
//! photons are traced from, and the orbit answers with the uniforms it writes the view into each frame.

use aethereal_data::Plain;
use aethereal_render::uniforms::UniformsId;
use aethereal_window::WindowId;

/// The context of an ask that needs none.
#[derive(Plain)]
#[kind(name = "aether.example.blackhole.nothing")]
pub struct Nothing;

/// The config of both roots: the window the hole is shown on, and its surface's size in pixels as the page set it.
#[derive(Debug, Clone, Copy, Plain)]
#[kind(name = "aether.example.blackhole.canvas")]
pub struct Canvas {
    pub window: WindowId,
    pub width: u32,
    pub height: u32,
}

/// Asks the orbit for the view the photons are traced from. Answered with [`Looking`].
#[derive(Debug, Clone, Copy, Plain)]
#[kind(name = "aether.example.blackhole.look")]
pub struct Look;

/// The orbit's answer to [`Look`]: the uniforms it writes the camera's view into, each frame.
#[derive(Debug, Clone, Copy, Plain)]
#[kind(name = "aether.example.blackhole.looking")]
pub struct Looking {
    pub view: UniformsId,
}
