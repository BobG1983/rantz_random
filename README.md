# rantz_random

A simple Rust library for generating random values using (fastrand)[https://github.com/smol-rs/fastrand]. Basically just a fastrand wrapper for a few additional types at the moment. Likely to grow over time. Designed for use with (Bevy)[https://www.bevyengine.org].

Can generate random values for:

* i8-128
* u8-128
* f32 & f64
* Vec2/3/4, UVec2/3/4, and IVec2/3/4 (with the `bevy` feature)
* Color (with the `bevy` feature)
* Degrees, Radians, Position2D, and all the compasses (with the `spatial2d` feature) - Requires `rantz_spatial2d`

Can generate random values in a range for:

* i8-128
* u8-128
* f32 & f64
* Vec2/3/4, UVec2/3/4, and IVec2/3/4 (with the `bevy` feature)
* Degrees, Radians, and Position2D (with the `spatial2d` feature) - Requires `rantz_spatial2d`

Can shuffle anything `Clone + IntoIterator<T> + FromIterator<T>` as well as returning a random element, or random index.

## Usage

Add to your `Cargo.toml`. Use `rantz_random::*;` to get access to the traits that define random generation.

## Determinism

This crate relies on `fastrand` which is "deterministic". That is to say, for a given seed value,  assuming system order is deterministic, the results will be deterministic. 

For gaurenteed determinism, any system that uses the features of this crate will need to be `.chain()`ed together when added to a Bevy app.
