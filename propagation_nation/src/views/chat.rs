use loco_rs::prelude::*;

use crate::models::_entities::chats;

/// Render a list view of `chats`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<chats::Model>) -> Result<Response> {
    format::render().view(v, "chat/list.html", data!({"items": items}))
}

/// Render a single `chat` view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &chats::Model) -> Result<Response> {
    format::render().view(v, "chat/show.html", data!({"item": item}))
}

/// Render a `chat` create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "chat/create.html", data!({}))
}

/// Render a `chat` edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &chats::Model) -> Result<Response> {
    format::render().view(v, "chat/edit.html", data!({"item": item}))
}
