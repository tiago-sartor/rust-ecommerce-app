// Import the standard set of HTML elements provided by the hypertext crate for use in rsx! macros.
pub use hypertext::validation::hypertext_elements::*;

// Define missing elements for type-checking in rsx!
hypertext::define_elements! {
    svg {
        fill
        height
        stroke
        stroke_width
        viewBox
        width
    }
    path {
        clip_rule
        d
        fill
        fill_rule
        stroke
        stroke_linecap
        stroke_linejoin
        stroke_width
    }
}
