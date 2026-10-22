//! A galaxy, on a page: sixty-five thousand stars in a spiral-armed disc, moved each frame by a compute program in
//! storage the GPU alone writes, drawn once as soft sprites that add their light into a target of floats, and given a
//! glow in the same frame: the bright part kept at half size, and blurred across then down. The frame lays the glow
//! over the stars, maps the light to the screen with a filmic curve, and darkens the corners. Held down, the pointer
//! pulls the stars near it, which stream back into the arms once it lets go. The frame follows the window's size.
//!
//! Two roots draw it, talking only by mail: the disc seeds the stars, moves them, and feeds the pointer to the GPU,
//! and the lens makes the frame that simulates, glows, and shows them, and places the stars in it.
//! `scripts/build.sh` at the repository's root builds the page; `site/demos/galaxy/index.html` runs it.

#[cfg(target_arch = "wasm32")]
mod bytes;
#[cfg(target_arch = "wasm32")]
mod camera;
#[cfg(target_arch = "wasm32")]
mod disc;
#[cfg(target_arch = "wasm32")]
mod lens;
#[cfg(target_arch = "wasm32")]
mod mail;
#[cfg(target_arch = "wasm32")]
mod page;
#[cfg(target_arch = "wasm32")]
mod passes;
