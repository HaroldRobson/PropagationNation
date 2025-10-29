use loco_rs::prelude::*;

use crate::models::_entities::species_of_plants;

/// Render a list view of `species_of_plants`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<species_of_plants::Model>) -> Result<Response> {
    format::render().view(v, "species_of_plant/list.html", data!({"items": items}))
}

/// Render a single `species_of_plant` view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &species_of_plants::Model) -> Result<Response> {
    format::render().view(v, "species_of_plant/show.html", data!({"item": item}))
}

/// Render a `species_of_plant` create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "species_of_plant/create.html", data!({}))
}

/// Render a `species_of_plant` edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &species_of_plants::Model) -> Result<Response> {
    format::render().view(v, "species_of_plant/edit.html", data!({"item": item}))
}
