// Import the standard set of HTML elements provided by the hypertext crate for use in rsx! macros.
pub use hypertext::validation::hypertext_elements::*;

use hypertext::{
    prelude::*,
    validation::{Attribute, AttributeNamespace},
};

// Define missing elements for type-checking in rsx!
hypertext::define_elements! {
    svg {
        width
        height
        fill
        viewBox
        stroke
        stroke_width
    }
    path {
        d
        stroke_linecap
        stroke_linejoin
    }
}

/// Extension trait to provide AlpineJS-specific attributes to the `rsx!` macro.
pub trait AlpineJsAttributes: GlobalAttributes {
    const x_mask: Attribute = Attribute;
    const x_mask_colon: AttributeNamespace = AttributeNamespace;
}

/// Apply AlpineJS attributes to all elements that support global HTML attributes.
impl<T: GlobalAttributes> AlpineJsAttributes for T {}
