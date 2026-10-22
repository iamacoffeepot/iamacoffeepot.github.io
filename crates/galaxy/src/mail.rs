//! What the disc and the lens are made with, and the mail between them: the lens asks the disc to move the stars in
//! the window's frame once it is made, and the disc answers with the stars once they move there.

use aethereal_data::Plain;
use aethereal_render::storage::StorageId;
use aethereal_window::WindowId;

/// The context of an ask that needs none.
#[derive(Plain)]
#[kind(name = "aether.example.galaxy.nothing")]
pub struct Nothing;

/// The config of both roots: the window the galaxy is shown on. Each learns the window's size from the window, which
/// sends a new subscriber its surface before anything else, and every resize after.
#[derive(Debug, Clone, Copy, Plain)]
#[kind(name = "aether.example.galaxy.canvas")]
pub struct Canvas {
    pub window: WindowId,
}

/// Asks the disc to move the stars in the compute pass of the window's frame. Answered with [`Moving`].
#[derive(Debug, Clone, Copy, Plain)]
#[kind(name = "aether.example.galaxy.move")]
pub struct Move;

/// The disc's answer to [`Move`]: the stars move, held in `stars`.
#[derive(Debug, Clone, Copy, Plain)]
#[kind(name = "aether.example.galaxy.moving")]
pub struct Moving {
    pub stars: StorageId,
}
