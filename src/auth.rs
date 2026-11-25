use url::Url;

use crate::{
    client::HttpClient,
    config::{Config, ILIAS_URL},
    error::{AppError, Result},
    form::FormData,
    html,
};

pub async fn authenticate(config: &Config) -> Result<HttpClient> {
    let client = HttpClient::new()?;

    let ilias_url = Url::parse(ILIAS_URL)?;
    let login_url = ilias_url.join("/shib_login.php")?;

    let (mut current_url, mut document) = client.get_html(&login_url).await?;

    if current_url.host_str() == ilias_url.host_str() {
        return Ok(client);
    }

    if !html::has_saml_response(&document)? {
        if html::has_id(&document, "fudiscr-form")? {
            return Err(AppError::Auth("KIT requires TFA".to_string()));
        }

        let skip = [
            "j_username",
            "j_password",
            "fudis_web_authn_assertion_input",
            "fudis_otp_input",
        ];

        let login_form = html::find_post_form(&document, &skip)?.ok_or_else(|| {
            AppError::Auth("No POST form found on the Shibboleth login page".to_string())
        })?;

        let post_url = current_url.join(&login_form.action)?;

        let mut credentials = FormData::new();

        credentials.push("_eventId_proceed", "");
        credentials.push("j_username", config.username());
        credentials.push("j_password", config.password());
        credentials.push("fudis_web_authn_assertion_input", "");

        credentials.extend(login_form.data);

        let (url, page) = client.post_form(&post_url, &credentials).await?;

        current_url = url;
        document = page;

        if html::has_id(&document, "attributeRelease")? {
            let consent_form = html::find_attribute_release_form(&document)?.ok_or_else(|| {
                AppError::Auth(
                    "Attribute-release page found, but no acceptance form was found".to_string(),
                )
            })?;

            let post_url = current_url.join(&consent_form.action)?;

            let (url, page) = client.post_form(&post_url, &consent_form.data).await?;

            current_url = url;
            document = page;

            if html::has_id(&document, "attributeRelease")? {
                return Err(AppError::Auth(
                    "Still on the attribute-release page after submission".to_string(),
                ));
            }
        }

        if html::has_id(&document, "fudiscr-form")? {
            return Err(AppError::Auth(
                "KIT requires TFA, which is not supported".to_string(),
            ));
        }

        if !html::has_saml_response(&document)? {
            return Err(AppError::Auth(
                "Login failed. Check KIT_USERNAME and KIT_PASSWORD".to_string(),
            ));
        }
    }

    let saml_form = html::find_post_form(&document, &[])?.ok_or_else(|| {
        AppError::Auth("No POST form found for the final SAML response".to_string())
    })?;

    let relay_state = saml_form
        .data
        .get("RelayState")
        .ok_or_else(|| AppError::Auth("Missing RelayState in SAML response".to_string()))?;

    let saml_response = saml_form
        .data
        .get("SAMLResponse")
        .ok_or_else(|| AppError::Auth("Missing SAMLResponse in SAML response".to_string()))?;

    let mut saml_data = FormData::new();
    saml_data.push("RelayState", relay_state);
    saml_data.push("SAMLResponse", saml_response);

    let post_url = current_url.join(&saml_form.action)?;

    let (final_url, _) = client.post_form(&post_url, &saml_data).await?;

    if final_url.host_str() != ilias_url.host_str() {
        return Err(AppError::Auth(format!(
            "SAML response was submitted, but the final URL is unexpected: {final_url}"
        )));
    }

    Ok(client)
}
