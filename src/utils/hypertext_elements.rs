/**
 * Define missing elements for type-checking in rsx!
 */
pub use hypertext::validation::hypertext_elements::*;

hypertext::define_elements! {
    svg {}
    path {}
}
