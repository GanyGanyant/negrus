extern crate google_youtube3 as youtube3;
use youtube3::api::SearchListResponse;
use youtube3::Error;
use youtube3::{hyper, hyper_rustls, oauth2, YouTube};

mod key;
pub async fn search(key_word: &str) -> Result<String, String> {
    let authenticator = oauth2::ServiceAccountAuthenticator::builder(key::get_key())
        .subject("")
        .build()
        .await
        .expect("failed to create authenticator");

    let hub = YouTube::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .enable_http2()
                .build(),
        ),
        authenticator,
    );
    // You can configure optional parameters by calling the respective setters at will, and
    // execute the final call using `doit()`.
    // Values shown here are possibly random and not representative !
    let result = hub
        .search()
        .list(&vec![])
        //.video_caption(key_word)
        .q(key_word)
        .add_type("video")
        .max_results(1)
        .doit()
        .await;

    let slr: SearchListResponse = match result {
        Err(e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
            Error::HttpError(_)
            | Error::Io(_)
            | Error::MissingAPIKey
            | Error::MissingToken(_)
            | Error::Cancelled
            | Error::UploadSizeLimitExceeded(_, _)
            | Error::Failure(_)
            | Error::BadRequest(_)
            | Error::FieldClash(_)
            | Error::JsonDecodeError(_, _) => return Err(e.to_string()),
        },
        Ok(res) => res.1,
    };

    if let Some(items) = slr.items {
        if let Some(sr) = items.first() {
            if let Some(rid) = sr.id.clone() {
                if let Some(id) = rid.video_id {
                    return Ok(id);
                }
            }
        }
    };
    Err("Video not found".into())
}
