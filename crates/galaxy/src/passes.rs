//! The passes of the galaxy's two frames, named by types the lens that makes the frames and the disc that dispatches
//! into one share.

use aethereal_data::Plain;
use aethereal_render::pass::Pass;

/// The glow's frame's compute pass, which moves the stars: it runs first in the `Render`, so both frames draw where it
/// left them.
#[derive(Plain)]
#[kind(name = "aether.example.galaxy.simulate")]
pub struct Simulate;

impl Pass for Simulate {}

/// The glow's frame's pass that draws the stars small, into a target of floats at the glow's size.
#[derive(Plain)]
#[kind(name = "aether.example.galaxy.small")]
pub struct Small;

impl Pass for Small {}

/// The glow's frame's pass that keeps the bright part of what [`Small`] drew.
#[derive(Plain)]
#[kind(name = "aether.example.galaxy.bright")]
pub struct Bright;

impl Pass for Bright {}

/// The glow's frame's pass that blurs the bright part along each row.
#[derive(Plain)]
#[kind(name = "aether.example.galaxy.across")]
pub struct Across;

impl Pass for Across {}

/// The glow's frame's last pass, which blurs along each column onto the frame's end.
#[derive(Plain)]
#[kind(name = "aether.example.galaxy.down")]
pub struct Down;

impl Pass for Down {}

/// The window's frame's pass that draws the stars, into a target of floats at the window's size.
#[derive(Plain)]
#[kind(name = "aether.example.galaxy.stars")]
pub struct Stars;

impl Pass for Stars {}

/// The window's frame's last pass, which lays the glow over the stars onto the window.
#[derive(Plain)]
#[kind(name = "aether.example.galaxy.composite")]
pub struct Composite;

impl Pass for Composite {}
