use crate::server::backend_handlers::Type;
use crate::utils::hypertext_elements;
use hypertext::validation::attributes::*;
use hypertext::{Renderable, rsx};
use std::collections::HashMap;

pub fn admin_customers_template(context: &HashMap<String, Type>) -> impl Renderable {
    rsx! {}
}
