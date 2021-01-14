/// The paste namespace which contains
/// every method and struct to
/// `GET` and `POST` (send) a paste
// to [pastemyst](https://paste.myst.rs).
#[allow(dead_code, unused_variables)]
pub mod paste {
    use serde::Deserialize;

    /// The PasteResult type provided
    /// by this library for ease. It
    /// has a return value and error.
    pub type PasteResult<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

    const ENDPOINT: &str = "https://paste.myst.rs/";
    const BASE_ENDPOINT: &str = "https://paste.myst.rs/api/v2/";
    /// This endpoint is temporarily here due to a bug in pastemyst
    /// which does not allow the paste to be end when the last
    /// slash is present.
    const SEND_ENDPOINT: &str = "https://paste.myst.rs/api/v2/paste";
    const PASTE_ENDPOINT: &str = "https://paste.myst.rs/api/v2/paste/";

    /// Gets a paste's data in json format
    /// from [pastemyst](https://paste.myst.rs)
    /// synchronously. It returns a `Result`
    /// with a `PasteObject` and error.
    pub fn get_paste(id: &str) -> Result<PasteObject, reqwest::Error> {
        let info: PasteObject = reqwest::blocking::get(&parse_url(id))?.json()?;
        Ok(info)
    }

    /// Gets a paste's data in json format
    /// from [pastemyst](https://paste.myst.rs)
    /// asynchronously. It returns a `Result`
    /// with a `PasteObject` and error.
    pub async fn get_paste_async(id: &str) -> Result<PasteObject, reqwest::Error> {
        let info: PasteObject = reqwest::get(&parse_url(id)).await?.json().await?;
        Ok(info)
    }

    /// Gets a private paste's data in json format
    /// from [pastemyst](https://paste.myst.rs)
    /// synchronously. It returns a `Result`
    /// with a `PasteObject` and error.
    pub fn get_private_paste(id: &str, auth_token: &str) -> Result<PasteObject, reqwest::Error> {
        let info: PasteObject = reqwest::blocking::Client::builder()
            .build()?
            .get(&parse_url(id))
            .header("Authorization", auth_token)
            .send()?
            .json()?;
        Ok(info)
    }

    /// Gets a private paste's data in json format
    /// from [pastemyst](https://paste.myst.rs)
    /// asynchronously. It returns a `Result`
    /// with a `PasteObject` and error.
    pub async fn get_private_paste_async(
        id: &str,
        auth_token: &str,
    ) -> Result<PasteObject, reqwest::Error> {
        let info: PasteObject = reqwest::Client::builder()
            .build()?
            .get(&parse_url(&id))
            .header("Authorization", auth_token)
            .send()
            .await?
            .json()
            .await?;
        Ok(info)
    }

    /// Parses the url by combining
    /// the `GET_ENDPOINT` with a
    // provided id.
    fn parse_url(id: &str) -> String {
        return GET_ENDPOINT.to_owned() + &id;
    }

    /// The paste object recieved when
    /// getting a paste. It contains
    /// both the `PastyObject` and
    /// `EditObject` in an array.
    #[derive(Deserialize)]
    #[allow(non_snake_case, dead_code)]
    pub struct PasteObject {
        /// Id of the paste.
        pub _id: String,
        /// Id of the owner, if it doesn't
        ///  have an owner it's set to "".
        pub ownerId: String,
        /// Title of the paste.
        pub title: String,
        /// Unix time of when
        /// the paste is created.
        pub createdAt: u64,
        /// When the paste will expire,
        /// possible values are
        /// `never`, `1h`, `2h`, `10h`,
        /// `1d`, `2d`, `1w`, `1m`, `1y`.
        pub expiresIn: String,
        /// When the paste will be deleted, if
        /// it has no expiry time it's set to 0.
        pub deletesAt: u64,
        /// Number of stars the paste received.
        pub stars: u64,
        /// If it's private it's only
        /// accessible by the owner.
        pub isPrivate: bool,
        /// Is it displayed on the
        /// owner's public profile.
        pub isPublic: bool,
        /// List of tags.
        pub tags: Vec<String>,
        /// List of pasties/files in
        /// the paste, can't be empty.
        pub pasties: Vec<PastyObject>,
        /// List of edits.
        pub edits: Vec<EditObject>,
    }

    /// Information about a specific pasty in a paste.
    #[derive(Deserialize)]
    #[allow(non_snake_case, dead_code)]
    pub struct PastyObject {
        /// Id of the pasty.
        pub _id: String,
        /// Language of the pasty.
        pub language: String,
        /// title of the pasty.
        pub Title: String,
        /// contents of the pasty.
        pub Code: String,
    }

    /// Infomation about edits in a pasty in a paste.
    #[derive(Deserialize)]
    #[allow(non_snake_case, dead_code)]
    pub struct EditObject {
        /// Unique id of the edit.
        pub _id: String,
        /// Id of the edit, multiple edits can
        /// share the same id showing that multiple
        /// fields were changed at the same time.
        pub editId: String,
        /// Type of edit, possible values are
        /// title(0), pastyTitle(1), pastyLanguage(2),
        /// pastyContent(3), pastyAdded(4), pastyRemoved(5).
        pub editType: i32,
        /// Various metadata used internally,
        /// biggest usecase is storing exactly which
        /// pasty was edited.
        pub metadata: Vec<String>,
        /// Actual paste edit, it stores old data
        /// before the edit as the current paste
        /// stores the new data
        pub edit: String,
        /// Unix time of when the edit was made
        pub editedAt: i32,
    }
}
