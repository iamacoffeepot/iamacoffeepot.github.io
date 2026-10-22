// Keeps the part of the traced light that rises past a threshold, easing in over a soft knee, so the glow grows from
// the bright disk and the photon ring without an edge. A camera's lens and the eye both scatter a little of a bright
// source's light around it; this is that scatter's source.
//
// It draws into a target a quarter of the light's size each way, so each of its pixels covers four by four of the
// light's: it keeps the bright part of each of the sixteen and takes their mean, which is what keeping the bright part
// of one photon traced somewhere in the sixteen gives on average, without the flicker of picking one.

@group(0) @binding(0) var light: texture_2d<f32>;

// The light past which a pixel glows, and the width of the knee it eases in over.
const THRESHOLD: f32 = 0.9;
const KNEE: f32 = 0.6;
// The light's pixels along each side of one of this target's: the frame draws it at `scale::Quarter` of the light.
const SPAN: i32 = 4;

// The part of `seen` past the threshold, eased in over the knee.
fn kept(seen: vec3<f32>) -> vec3<f32> {
    let peak = max(seen.r, max(seen.g, seen.b));
    let eased = clamp(peak - THRESHOLD + KNEE, 0.0, 2.0 * KNEE);
    return seen * max(eased * eased / (4.0 * KNEE), peak - THRESHOLD) / max(peak, 0.0001);
}

@fragment
fn bright(@builtin(position) at: vec4<f32>) -> @location(0) vec4<f32> {
    let first = vec2<i32>(at.xy) * SPAN;
    let last = vec2<i32>(textureDimensions(light)) - 1;
    var sum = vec3(0.0);
    for (var down = 0; down < SPAN; down++) {
        for (var across = 0; across < SPAN; across++) {
            sum += kept(textureLoad(light, min(first + vec2(across, down), last), 0).rgb);
        }
    }
    return vec4(sum / f32(SPAN * SPAN), 1.0);
}
