use loco_rs::prelude::*;

use crate::models::_entities::plants;

/// Render a list view of `plants`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<plants::Model>) -> Result<Response> {
    format::render().view(v, "plant/list.html", data!({"items": items}))
}

/// Render a single `plant` view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &plants::Model) -> Result<Response> {
    format::render().view(v, "plant/show.html", data!({"item": item}))
}

/// Render a `plant` create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "plant/create.html", data!({}))
}

/// Render a `plant` edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &plants::Model) -> Result<Response> {
    format::render().view(v, "plant/edit.html", data!({"item": item}))
}
