// Import the standard set of HTML elements provided by the hypertext crate for use in rsx! macros.
pub use hypertext::validation::hypertext_elements::*;

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
        stroke_width
        stroke_linecap
        stroke_linejoin
    }
}
