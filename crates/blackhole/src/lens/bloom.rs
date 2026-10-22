//! The bloom's effects: the bright part of the traced light kept at a quarter of its size, blurred along each row, then
//! along each column, and laid over the light on the window with the tone map. Every target between them is of floats,
//! so the glow is kept as it is.

use aethereal_actor::BlobRef;
use aethereal_data::Plain;
use aethereal_render::effect::{CreateEffect, EffectId, Read};
use aethereal_render::frame::builder::Float;
use aethereal_render::placement::{Bind, Filter, Resource, Sampling, Wrap};
use aethereal_render::program::{Binding, BindingKind, Blend};
use aethereal_render::shape::{ColourFormat, Shape};

/// The effect that keeps the bright part.
pub const BRIGHT_WGSL: &str = include_str!("../shaders/bright.wgsl");

/// The two effects that blur, across and down.
pub const BLUR_WGSL: &str = include_str!("../shaders/blur.wgsl");

/// The effect that lays the glow over the light and maps it to the screen.
pub const COMPOSITE_WGSL: &str = include_str!("../shaders/composite.wgsl");

/// The frame's effects: the trace, which reads nothing, three that read one target of floats, and the composite,
/// which reads the light and the glow.
#[derive(Debug, Clone, Copy, Plain)]
#[kind(name = "aether.example.blackhole.made")]
pub struct Effects {
    pub trace: EffectId<()>,
    pub bright: EffectId<(Float,)>,
    pub across: EffectId<(Float,)>,
    pub down: EffectId<(Float,)>,
    pub composite: EffectId<(Float, Float)>,
}

/// The effect of `fragment` of the WGSL `wgsl` holds, reading `reads` and binding `bindings` after them, drawing into a
/// target of `colour`.
fn effect(
    wgsl: BlobRef,
    fragment: &str,
    reads: Vec<Read>,
    bindings: Vec<Binding>,
    colour: ColourFormat,
) -> CreateEffect {
    CreateEffect {
        wgsl,
        fragment: String::from(fragment),
        reads,
        bindings,
        blend: Blend::Opaque,
        shape: Shape::flat(colour),
    }
}

/// The effect that keeps the bright part of the traced light, a quarter of its size, into a target of floats.
pub fn bright(wgsl: BlobRef) -> CreateEffect {
    effect(wgsl, "bright", vec![Read::Float], Vec::new(), ColourFormat::Rgba16Float)
}

/// The effect that blurs along each row, into a target of floats.
pub fn across(wgsl: BlobRef) -> CreateEffect {
    effect(wgsl, "across", vec![Read::Float], Vec::new(), ColourFormat::Rgba16Float)
}

/// The effect that blurs along each column, into a target of floats: the glow.
pub fn down(wgsl: BlobRef) -> CreateEffect {
    effect(wgsl, "down", vec![Read::Float], Vec::new(), ColourFormat::Rgba16Float)
}

/// The effect that lays the glow over the light onto a window drawn in `window`, sampling the smaller glow: written in
/// linear light where the window encodes sRGB itself, and encoded where it does not.
pub fn composite(wgsl: BlobRef, window: ColourFormat) -> CreateEffect {
    let encodes = matches!(window, ColourFormat::Rgba8Srgb | ColourFormat::Bgra8Srgb);
    let fragment = if encodes {
        "shown"
    } else {
        "encoded"
    };
    let sampler = vec![Binding { group: 1, binding: 0, kind: BindingKind::Sampler }];

    effect(wgsl, fragment, vec![Read::Float, Read::Float], sampler, window)
}

/// What the composite binds: a sampler that blends the glow's nearest texels and holds to its edge.
pub fn sampled() -> Vec<Bind> {
    vec![Bind { group: 1, binding: 0, to: Resource::Sampler(Sampling { filter: Filter::Linear, wrap: Wrap::Clamp }) }]
}
