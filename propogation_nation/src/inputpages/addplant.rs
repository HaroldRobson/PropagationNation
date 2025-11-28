use crate::prelude::client::*;
#[component]
pub fn AddPlant() -> impl IntoView {
    let (state, set_state) = use_cookie::<UserStateCS, JsonSerdeCodec>("state");
    let session_id = move || match state.get() {
        None => "".to_string(),
        Some(uscs) => uscs.session_id.unwrap_or("".to_string()),
    };
    let (location, set_location) = signal("".to_string());
    let upload_action = Action::new_local(move |data: &FormData| {
        let dat = data.clone().into();
        // `MultipartData` implements `From<FormData>`
        async move { add_plant(dat).await }
    });

    view! {
        <fieldset>
        <label>
        "enter your borough"
        <input type="text"

        placeholder="Hackney".to_string()
        on:input:target=move |ev| {
                // .value() returns the current value of an HTML input element
                set_location.set(ev.target().value());
            }
        prop:value=location
        />
        </label>
        <p>Upload a selfie!</p>
        <form on:submit=move |ev: SubmitEvent| {
            ev.prevent_default();
            let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
            let form_data = FormData::new_with_form(&target).unwrap();
            let _ = form_data.append_with_str("location", location.get().as_str());
            let _ = form_data.append_with_str("session_id", session_id().as_str());
            upload_action.dispatch_local(form_data);
        }>
            <input type="file" name="file_to_upload" />
            <input type="submit" />
        </form>
        <p>
            {move || {
                if upload_action.input().read().is_none() && upload_action.value().read().is_none()
                {
                    "Upload a file.".to_string()
                } else if upload_action.pending().get() {
                    "Uploading...".to_string()
                } else if let Some(Ok(val)) = upload_action.value().get() {
                    "Success!".to_string()
                } else {
                    format!("{:?}", upload_action.value().get())
                }
            }}

        </p>

        </fieldset>
    }
}

#[server(input = MultipartFormData)]
pub async fn add_plant(data: MultipartData) -> Result<String, ServerFnError> {
    use crate::prelude::server::*;
    Ok("".to_string())
}
