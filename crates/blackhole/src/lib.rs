//! A Schwarzschild black hole, on a page, traced through the curved spacetime around it one photon a pixel. Each
//! pixel's light is followed backward from the camera along a null geodesic, falls into the hole or bends around it,
//! and lands on the thin accretion disk or on the sky. The disk glows at the Novikov–Thorne temperature, shifted by
//! the redshift of the gravity it climbs out of and the Doppler shift of the gas's orbit, and its far side is lensed
//! over and under the shadow. The photons are traced once, by an effect that begins the frame, and a glow is bloomed
//! from the bright parts at a quarter of the size in the same frame, then laid over the light and mapped to the screen
//! with a filmic curve. Dragged, the camera orbits the hole; left alone, it drifts; the wheel draws it nearer or away.
//!
//! Two roots draw it, talking only by mail: the orbit keeps the camera, following the pointer and the clock, and
//! writes its view into uniforms the GPU reads, for the window as it was last resized, and the lens makes the frame
//! that traces and shows the hole, its trace bound to the orbit's view. `scripts/build.sh` at the repository's root
//! builds the page; `site/demos/blackhole/index.html` runs it.

#[cfg(target_arch = "wasm32")]
mod bytes;
#[cfg(target_arch = "wasm32")]
mod lens;
#[cfg(target_arch = "wasm32")]
mod mail;
#[cfg(target_arch = "wasm32")]
mod orbit;
#[cfg(target_arch = "wasm32")]
mod page;
#[cfg(target_arch = "wasm32")]
mod passes;
