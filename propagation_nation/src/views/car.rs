use loco_rs::prelude::*;

use crate::models::_entities::cars;

/// Render a list view of `cars`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<cars::Model>) -> Result<Response> {
    format::render().view(v, "car/list.html", data!({"items": items}))
}

/// Render a single `car` view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &cars::Model) -> Result<Response> {
    format::render().view(v, "car/show.html", data!({"item": item}))
}

/// Render a `car` create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "car/create.html", data!({}))
}

/// Render a `car` edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &cars::Model) -> Result<Response> {
    format::render().view(v, "car/edit.html", data!({"item": item}))
}
