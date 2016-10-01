#![forbid(missing_docs, warnings)]
#![deny(deprecated, drop_with_repr_extern, improper_ctypes,
        non_shorthand_field_patterns, overflowing_literals, plugin_as_library,
        private_no_mangle_fns, private_no_mangle_statics, stable_features, unconditional_recursion,
        unknown_lints, unused, unused_allocation, unused_attributes,
        unused_comparisons, unused_features, unused_parens, while_true)]
#![warn(trivial_casts, trivial_numeric_casts, unused, unused_extern_crates,
        unused_import_braces, unused_qualifications, unused_results, variant_size_differences)]

pub mod vsop87a;
pub mod vsop87b;
pub mod vsop87c;
pub mod vsop87d;
pub mod vsop87e;

mod mercury;
mod venus;
mod earth_moon;
mod mars;
mod jupiter;
mod saturn;
mod uranus;
mod neptune;

use std::f32::consts::PI;

#[cfg(feature = "f32")]
pub fn keplerian_elements_from_vsop87(a: f32,
                                      l: f32,
                                      k: f32,
                                      h: f32,
                                      q: f32,
                                      p: f32)
                                      -> (f32, f32, f32, f32, f32, f32) {

    let e = (h * h + k * k).sqrt();
    let i = (1_f64 - 2_f64 * (p * p + q * q)).acos();
    let lan = (p / q).atan();
    let lper = (h / e).asin();

    (a, e, i, lan, lper, l)
}
