//! The passes of the black hole's one frame, named by types the lens that makes the frame names them by.

use aethereal_data::Plain;
use aethereal_render::pass::Pass;

/// The pass that traces the photons, once, an effect that reads nothing, into a target of floats at the window's size.
#[derive(Plain)]
#[kind(name = "aether.example.blackhole.trace")]
pub struct Trace;

impl Pass for Trace {}

/// The pass that keeps the bright part of what [`Trace`] traced, at a quarter of its size.
#[derive(Plain)]
#[kind(name = "aether.example.blackhole.bright")]
pub struct Bright;

impl Pass for Bright {}

/// The pass that blurs the bright part along each row, at a quarter of the size.
#[derive(Plain)]
#[kind(name = "aether.example.blackhole.across")]
pub struct Across;

impl Pass for Across {}

/// The pass that blurs along each column what [`Across`] left, at a quarter of the size: the glow.
#[derive(Plain)]
#[kind(name = "aether.example.blackhole.down")]
pub struct Down;

impl Pass for Down {}

/// The last pass, which lays the glow over the light and maps it onto the window.
#[derive(Plain)]
#[kind(name = "aether.example.blackhole.composite")]
pub struct Composite;

impl Pass for Composite {}
