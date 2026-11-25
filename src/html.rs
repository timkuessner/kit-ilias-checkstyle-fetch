use scraper::{ElementRef, Html, Selector};

use crate::{
    error::{AppError, Result},
    http::FormData,
};

pub struct PostForm {
    pub action: String,
    pub data: FormData,
}

fn selector(query: &str) -> Result<Selector> {
    Selector::parse(query)
        .map_err(|error| AppError::Ilias(format!("invalid HTML selector '{query}': {error}")))
}

pub fn has_id(document: &Html, id: &str) -> Result<bool> {
    let selector = selector("[id]")?;

    Ok(document
        .select(&selector)
        .any(|element| element.value().attr("id") == Some(id)))
}

pub fn has_saml_response(document: &Html) -> Result<bool> {
    let relay_selector = selector("input[name='RelayState']")?;
    let saml_selector = selector("input[name='SAMLResponse']")?;

    Ok(document.select(&relay_selector).next().is_some()
        && document.select(&saml_selector).next().is_some())
}

pub fn find_post_form(document: &Html, skip_names: &[&str]) -> Result<Option<PostForm>> {
    let form_selector = selector("form")?;

    for form in document.select(&form_selector) {
        let method = form.value().attr("method").unwrap_or("get");

        if !method.eq_ignore_ascii_case("post") {
            continue;
        }

        let action = form.value().attr("action").unwrap_or("").to_string();
        let data = hidden_fields(form, skip_names)?;

        return Ok(Some(PostForm { action, data }));
    }

    Ok(None)
}

fn hidden_fields(form: ElementRef, skip_names: &[&str]) -> Result<FormData> {
    let input_selector = selector("input[name]")?;
    let mut data = FormData::new();

    for input in form.select(&input_selector) {
        let name = input.value().attr("name").unwrap_or("");

        if name.is_empty() || skip_names.contains(&name) {
            continue;
        }

        let input_type = input
            .value()
            .attr("type")
            .unwrap_or("text")
            .to_ascii_lowercase();

        if matches!(input_type.as_str(), "submit" | "button" | "image") {
            continue;
        }

        let value = input.value().attr("value").unwrap_or("");
        data.push(name, value);
    }

    Ok(data)
}

pub fn find_attribute_release_form(document: &Html) -> Result<Option<PostForm>> {
    let form_selector = selector("form")?;
    let input_selector = selector("input[name]")?;
    let button_selector = selector("button[name]")?;

    for form in document.select(&form_selector) {
        let method = form.value().attr("method").unwrap_or("get");

        if !method.eq_ignore_ascii_case("post") {
            continue;
        }

        let mut submit_buttons = Vec::new();

        for button in form.select(&button_selector) {
            let button_type = button.value().attr("type").unwrap_or("submit");

            if button_type.eq_ignore_ascii_case("submit") {
                let name = button.value().attr("name").unwrap_or("");
                let value = button.value().attr("value").unwrap_or("");

                if !name.is_empty() {
                    submit_buttons.push((name.to_string(), value.to_string()));
                }
            }
        }

        let accept_button = submit_buttons
            .iter()
            .find(|(name, _)| name.to_ascii_lowercase().contains("consentyes"))
            .cloned()
            .or_else(|| {
                submit_buttons
                    .iter()
                    .find(|(name, value)| {
                        let name = name.to_ascii_lowercase();
                        let value = value.to_ascii_lowercase();

                        !(name.contains("consentno")
                            || name.contains("cancel")
                            || value.contains("cancel")
                            || value.contains("decline")
                            || value.contains("abbrechen"))
                    })
                    .cloned()
            });

        let Some((accept_name, accept_value)) = accept_button else {
            continue;
        };

        let mut data = FormData::new();

        for input in form.select(&input_selector) {
            let name = input.value().attr("name").unwrap_or("");

            if name.is_empty() {
                continue;
            }

            let input_type = input
                .value()
                .attr("type")
                .unwrap_or("text")
                .to_ascii_lowercase();

            let value = input.value().attr("value").unwrap_or("");

            match input_type.as_str() {
                "hidden" => data.push(name, value),
                "checkbox" | "radio" => {
                    if input.value().attr("checked").is_some() {
                        data.push(name, if value.is_empty() { "on" } else { value });
                    }
                }
                _ => {}
            }
        }

        data.push(accept_name, accept_value);

        let action = form.value().attr("action").unwrap_or("").to_string();

        return Ok(Some(PostForm { action, data }));
    }

    Ok(None)
}
