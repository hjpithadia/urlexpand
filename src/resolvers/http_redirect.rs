// HTTP 3xx Redirect Resolver
// For shorteners that use standard HTTP redirects (301, 302, etc.)
use std::time::Duration;

use reqwest::redirect::Policy;

use super::get_client_builder;
use crate::Result;

/// Follow HTTP redirects and return the final URL
pub(crate) async fn unshort(url: &str, timeout: Option<Duration>) -> Result<String> {
    let client = get_client_builder(timeout)
        .redirect(Policy::limited(10)) // Follow up to 10 redirects
        .build()?;

    let response = client.get(url).send().await?;

    // Return the final URL after all redirects
    Ok(response.url().as_str().into())
}
