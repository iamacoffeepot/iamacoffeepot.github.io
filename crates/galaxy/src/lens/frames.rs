//! The galaxy's one frame, on the window: it moves the stars, draws them once at the window's size, keeps their bright
//! part at half that size and blurs it across and down, and lays the glow over the stars. A frame that ends on the
//! window follows its size, so every target is made again at each size the window is drawn at.

use super::bloom::{Effects, sampled};
use crate::passes::{Across, Bright, Composite, Down, Simulate, Stars};
use aethereal_render::frame::builder::{Effect, Float, FrameBuilder, depth, scale};
use aethereal_render::frame::{CreateFrame, Ends, FrameRefused};
use aethereal_window::WindowId;

/// The frame on `window`, its bloom drawn by `effects`.
pub fn shown(window: WindowId, effects: &Effects) -> Result<CreateFrame, FrameRefused> {
    let mut frame = FrameBuilder::on(Ends::Window(window));
    frame.compute(Simulate);
    let stars = frame.draw(Stars, Float, scale::Full, depth::Off);
    let bright = frame.effect(Bright, Effect::new(&effects.bright, (stars,)).bound(sampled()), Float, scale::Half);
    let across = frame.effect(Across, Effect::new(&effects.across, (bright,)), Float, scale::Half);
    let glow = frame.effect(Down, Effect::new(&effects.down, (across,)), Float, scale::Half);

    frame.finish(Composite, Effect::new(&effects.composite, (stars, glow)).bound(sampled()))
}
