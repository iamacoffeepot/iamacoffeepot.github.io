//! The passes of the galaxy's one frame, named by types the lens that makes the frame and the disc that dispatches
//! into it share.

use aethereal_data::Plain;
use aethereal_render::pass::Pass;

/// The compute pass, which moves the stars: it runs first, so the passes after draw them where it left them.
#[derive(Plain)]
#[kind(name = "aether.example.galaxy.simulate")]
pub struct Simulate;

impl Pass for Simulate {}

/// The pass that draws the stars, once, into a target of floats at the window's size.
#[derive(Plain)]
#[kind(name = "aether.example.galaxy.stars")]
pub struct Stars;

impl Pass for Stars {}

/// The pass that keeps the bright part of what [`Stars`] drew, at half its size.
#[derive(Plain)]
#[kind(name = "aether.example.galaxy.bright")]
pub struct Bright;

impl Pass for Bright {}

/// The pass that blurs the bright part along each row, at half size.
#[derive(Plain)]
#[kind(name = "aether.example.galaxy.across")]
pub struct Across;

impl Pass for Across {}

/// The pass that blurs along each column what [`Across`] left, at half size: the glow.
#[derive(Plain)]
#[kind(name = "aether.example.galaxy.down")]
pub struct Down;

impl Pass for Down {}

/// The last pass, which lays the glow over the stars onto the window.
#[derive(Plain)]
#[kind(name = "aether.example.galaxy.composite")]
pub struct Composite;

impl Pass for Composite {}
