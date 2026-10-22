//! The black hole's one frame, on the window: it traces the photons once at the window's size, keeps their bright part
//! at a quarter of that size and blurs it across and down, and lays the glow over the light. A frame that ends on the
//! window follows its size, so every target is made again at each size the window is drawn at.

use super::bloom::{Effects, sampled};
use super::trace::viewed;
use crate::passes::{Across, Bright, Composite, Down, Trace};
use aethereal_render::frame::builder::{Effect, Float, FrameBuilder, scale};
use aethereal_render::frame::{CreateFrame, Ends, FrameRefused};
use aethereal_render::uniforms::UniformsId;
use aethereal_window::WindowId;

/// The frame on `window`, its photons traced from the orbit's `view` and its bloom drawn by `effects`.
pub fn shown(window: WindowId, effects: &Effects, view: UniformsId) -> Result<CreateFrame, FrameRefused> {
    let mut frame = FrameBuilder::on(Ends::Window(window));
    let light = frame.effect(Trace, Effect::new(&effects.trace, ()).bound(viewed(view)), Float, scale::Full);
    let bright = frame.effect(Bright, Effect::new(&effects.bright, (light,)), Float, scale::Quarter);
    let across = frame.effect(Across, Effect::new(&effects.across, (bright,)), Float, scale::Quarter);
    let glow = frame.effect(Down, Effect::new(&effects.down, (across,)), Float, scale::Quarter);

    frame.finish(Composite, Effect::new(&effects.composite, (light, glow)).bound(sampled()))
}
