// Keeps the light of the stars that rises past a threshold, easing in over a soft knee so the glow grows from the
// bright parts without an edge. It draws into a target half the stars' size each way, sampling them at the middle of
// each two by two of their pixels, so the sampler blends the four evenly.

@group(0) @binding(0) var stars: texture_2d<f32>;
@group(1) @binding(0) var blended: sampler;

// The light past which a pixel glows, and the width of the knee it eases in over.
const THRESHOLD: f32 = 0.2;
const KNEE: f32 = 0.3;
// What the light is lifted by before it is weighed against the threshold: the stars were once drawn again for the
// glow at half size, as sprites half as wide again as on the window and a quarter brighter, which the threshold was
// set against. Their light over a stretch of sky is that much over the light of the stars drawn once and blended
// down: (1.3 * 2 / 1.7)² * 0.2 / 0.16.
const LIFT: f32 = 2.92;

@fragment
fn bright(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let light = textureSample(stars, blended, uv).rgb * LIFT;
    let peak = max(light.r, max(light.g, light.b));
    let eased = clamp(peak - THRESHOLD + KNEE, 0.0, 2.0 * KNEE);
    let kept = max(eased * eased / (4.0 * KNEE), peak - THRESHOLD) / max(peak, 0.0001);
    return vec4(light * kept, 1.0);
}
