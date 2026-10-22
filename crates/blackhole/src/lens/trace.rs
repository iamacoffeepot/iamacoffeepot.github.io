//! The photons as the lens traces them: an effect that reads nothing, drawn over the whole of a target at the window's
//! size, whose fragment stage follows each pixel's photon through the curved spacetime, bound to the orbit's view.

use aethereal_actor::BlobRef;
use aethereal_render::effect::CreateEffect;
use aethereal_render::placement::{Bind, Resource};
use aethereal_render::program::{Binding, BindingKind, Blend};
use aethereal_render::shape::{ColourFormat, Shape};
use aethereal_render::uniforms::UniformsId;

/// The effect that traces the photons.
pub const TRACE_WGSL: &str = include_str!("../shaders/trace.wgsl");

/// The bytes of the view's uniforms.
const VIEW_BYTES: u32 = 32;

/// The effect of the WGSL the blob `wgsl` holds, which reads no target and writes each pixel's traced light into a
/// target of floats, bound to the view in group 1.
pub fn effect(wgsl: BlobRef) -> CreateEffect {
    CreateEffect {
        wgsl,
        fragment: String::from("shade"),
        reads: Vec::new(),
        bindings: vec![Binding { group: 1, binding: 0, kind: BindingKind::Uniform { bytes: VIEW_BYTES } }],
        blend: Blend::Opaque,
        shape: Shape::flat(ColourFormat::Rgba16Float),
    }
}

/// What the trace binds: the orbit's view, `view`.
pub fn viewed(view: UniformsId) -> Vec<Bind> {
    vec![Bind { group: 1, binding: 0, to: Resource::Uniforms(view) }]
}
