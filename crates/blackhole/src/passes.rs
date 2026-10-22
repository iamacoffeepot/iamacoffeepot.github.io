//! The passes of the black hole's two frames, named by types the lens that makes the frames and the placement of the
//! photons' quad share.

use aethereal_data::Plain;
use aethereal_render::pass::Pass;

/// The glow's frame's pass that traces the photons small, into a target of floats at the glow's size.
#[derive(Plain)]
#[kind(name = "aether.example.blackhole.small")]
pub struct Small;

impl Pass for Small {}

/// The glow's frame's pass that keeps the bright part of what [`Small`] traced.
#[derive(Plain)]
#[kind(name = "aether.example.blackhole.bright")]
pub struct Bright;

impl Pass for Bright {}

/// The glow's frame's pass that blurs the bright part along each row.
#[derive(Plain)]
#[kind(name = "aether.example.blackhole.across")]
pub struct Across;

impl Pass for Across {}

/// The glow's frame's last pass, which blurs along each column onto the frame's end.
#[derive(Plain)]
#[kind(name = "aether.example.blackhole.down")]
pub struct Down;

impl Pass for Down {}

/// The window's frame's pass that traces the photons, into a target of floats at the window's size.
#[derive(Plain)]
#[kind(name = "aether.example.blackhole.trace")]
pub struct Trace;

impl Pass for Trace {}

/// The window's frame's last pass, which lays the glow over the light and maps it onto the window.
#[derive(Plain)]
#[kind(name = "aether.example.blackhole.composite")]
pub struct Composite;

impl Pass for Composite {}
