/// The paste namespace which contains
/// every method and struct to
/// `GET` and `POST` (send) a paste
/// to [pastemyst](https://paste.myst.rs).
///
/// ### [API Docs](https://paste.myst.rs/api-docs/index)
#[allow(dead_code, unused_variables)]
pub mod paste {
    use serde::Deserialize;
    use serde::Serialize;

    /// The PasteResult type provided
    /// by this library for ease. It
    /// has a return value and error.
    ///
    /// ## Examples
    /// ```rust
    /// use pastemyst::paste::PasteResult;
    ///
    /// fn main() -> PasteResult<()> {
    ///     Ok(())
    /// }
    /// ```
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
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use pastemyst::paste::get_paste;
    /// use pastemyst::paste::PasteResult;
    ///
    /// fn main() -> PasteResult<()> {
    ///     let foo = get_paste("hipfqanx");
    ///     println!("{:?}", foo.title);
    ///     Ok(())
    /// }
    /// ```
    ///
    pub fn get_paste(id: &str) -> Result<PasteObject, reqwest::Error> {
        let info: PasteObject = reqwest::blocking::get(&parse_url(id))?.json()?;
        Ok(info)
    }

    /// Gets a paste's data in json format
    /// from [pastemyst](https://paste.myst.rs)
    /// asynchronously. It returns a `Result`
    /// with a `PasteObject` and error.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use pastemyst::paste::get_paste_async;
    /// use pastemyst::paste::PasteResult;
    ///
    /// #[tokio::main]
    /// async fn main() -> PasteResult<()> {
    ///     let foo = get_paste_async("hipfqanx").await?;
    ///     println!("{:?}", foo._id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_paste_async(id: &str) -> Result<PasteObject, reqwest::Error> {
        let info: PasteObject = reqwest::get(&parse_url(id)).await?.json().await?;
        Ok(info)
    }

    /// Gets a private paste's data in json format
    /// from [pastemyst](https://paste.myst.rs)
    /// synchronously. It returns a `Result`
    /// with a `PasteObject` and error.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use pastemyst::paste::get_private_paste;
    /// use pastemyst::paste::PasteResult;
    ///
    /// fn main() -> PasteResult<()> {
    ///     let foo = get_private_paste("pasteID", "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings");
    ///     println!("{:?}", foo._id);
    ///     Ok(())
    /// }
    /// ```
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
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use pastemyst::paste::get_private_paste_async;
    /// use pastemyst::paste::PasteResult;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let foo = paste::get_private_paste_async("pasteID", "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings").await?;
    ///     println!("{}", paste.isPrivate);
    ///     Ok(())
    /// }
    /// ```
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

    /// Uses the `CreateObject` struct as a parameter for paste data
    /// to be contructed and sent to [pastemyst](https://paste.myst.rs).
    /// in a synchronous manner.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use pastemyst::paste::PastyObject;
    /// use pastemyst::paste::*;
    ///
    /// fn main() -> PasteResult<()> {
    ///     let pasties: Vec<PastyObject> = vec![
    ///             PastyObject {
    ///             _id: None,
    ///             language: Some(String::from("autodetect")),
    ///             title: Some(String::from("Pasty1")),
    ///             code: Some(String::from("Code")),
    ///         },
    ///         PastyObject {
    ///             _id: None,
    ///             language: Some(String::from("autodetect")),
    ///             title: Some(String::from("Pasty2")),
    ///             code: Some(String::from("Code")),
    ///         },
    ///     ];
    ///     let data: CreateObject = CreateObject {
    ///         title: String::from("[crates.io/crates/pastemyst] This is a title"),
    ///         expiresIn: String::from("1d"),
    ///         isPrivate: false,
    ///         isPublic: false,
    ///         tags: String::from(""),
    ///         pasties: pasties,
    ///     };
    ///     let paste = create_paste(data)?;
    ///     println!("{:#?}", paste._id);
    ///     Ok(())
    /// }
    /// ```
    pub fn create_paste(contents: CreateObject) -> Result<PasteObject, reqwest::Error> {
        let content_type = reqwest::header::HeaderValue::from_static("application/json");
        let result = reqwest::blocking::Client::builder()
            .build()?
            .post(SEND_ENDPOINT)
            .header(reqwest::header::CONTENT_TYPE, content_type)
            .body(serde_json::to_string(&contents).unwrap())
            .send()
            .unwrap();
        Ok(result.json()?)
    }

    /// Uses the `CreateObject` struct as a parameter for paste
    /// data to be contructed and sent to [pastemyst](https://paste.myst.rs)
    /// in an asynchronous manner.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use pastemyst::paste::PastyObject;
    /// use pastemyst::paste::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> PasteResult<()> {
    ///     let pasties: Vec<PastyObject> = vec![
    ///             PastyObject {
    ///             _id: None,
    ///             language: Some(String::from("autodetect")),
    ///             title: Some(String::from("Pasty1")),
    ///             code: Some(String::from("Code")),
    ///         },
    ///         PastyObject {
    ///             _id: None,
    ///             language: Some(String::from("autodetect")),
    ///             title: Some(String::from("Pasty2")),
    ///             code: Some(String::from("Code")),
    ///         },
    ///     ];
    ///     let data: CreateObject = CreateObject {
    ///         title: String::from("[crates.io/crates/pastemyst] This is a title"),
    ///         expiresIn: String::from("1d"),
    ///         isPrivate: false,
    ///         isPublic: false,
    ///         tags: String::from(""),
    ///         pasties: pasties,
    ///     };
    ///     let paste = paste::create_paste_async(data).await?;
    ///     println!("{:?}", paste._id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_paste_async(contents: CreateObject) -> Result<PasteObject, reqwest::Error> {
        let content_type = reqwest::header::HeaderValue::from_static("application/json");
        let result = reqwest::Client::builder()
            .build()?
            .post(SEND_ENDPOINT)
            .header(reqwest::header::CONTENT_TYPE, content_type)
            .body(serde_json::to_string(&contents).unwrap())
            .send()
            .await?;
        Ok(result.json().await?)
    }

    pub fn create_private_paste(contents: CreateObject, auth_token: &str) -> Result<PasteObject, reqwest::Error> {
        let content_type = reqwest::header::HeaderValue::from_static("application/json");
        let result = reqwest::blocking::Client::builder()
            .build()?
            .post(SEND_ENDPOINT)
            .header("Authorization", auth_token)
            .header(reqwest::header::CONTENT_TYPE, content_type)
            .body(serde_json::to_string(&contents).unwrap())
            .send()?;
        Ok(result.json()?)
    }

    pub async fn create_private_paste_async(contents: CreateObject, auth_token: &str) -> Result<PasteObject, reqwest::Error> {
        let content_type = reqwest::header::HeaderValue::from_static("application/json");
        let result = reqwest::Client::builder()
            .build()?
            .post(SEND_ENDPOINT)
            .header("Authorization", auth_token)
            .header(reqwest::header::CONTENT_TYPE, content_type)
            .body(serde_json::to_string(&contents).unwrap())
            .send().await?;
        Ok(result.json().await?)
    }

    /// Parses the url by combining
    /// the `PASTE_ENDPOINT` with a
    /// provided id.
    fn parse_url(id: &str) -> String { return PASTE_ENDPOINT.to_owned() + &id }

    /// The paste object recieved when
    /// getting a paste. It contains
    /// both the `PastyObject` and
    /// `EditObject` in an array.
    ///
    /// ### API Docs
    /// The relevent link to the API documentation
    /// is: https://paste.myst.rs/api-docs/objects
    ///
    /// ## Examples
    ///
    /// ```rust
    /// let _foo: PasteObject = get_paste("hipfqanx");
    /// ```
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
    ///
    /// ### API Docs
    /// The relevent link to the API documentation
    /// is: https://paste.myst.rs/api-docs/objects
    ///
    /// ## Examples
    ///
    /// ```rust
    /// let pasty: PastyObject = PastyObject {
    ///     _id: None,
    ///     language: Some(String::from("autodetect")),
    ///     title: Some(String::from("This is a pasty title")),
    ///     code: Some(String::from("{\"This_Is\": \"JSON_Code\"}")),
    /// };
    /// ```
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case, dead_code)]
    pub struct PastyObject {
        /// Id of the pasty.
        pub _id: Option<String>,
        /// Language of the pasty.
        pub language: Option<String>,
        /// title of the pasty.
        pub title: Option<String>,
        /// contents of the pasty.
        pub code: Option<String>,
    }

    /// Infomation about edits in a pasty in a paste.
    ///
    /// ### API Docs
    /// The relevent link to the API documentation
    /// is: https://paste.myst.rs/api-docs/objects
    ///
    /// ## Examples
    ///
    /// ```rust
    /// // Get paste from pastemyst
    /// let edits: EditObject = paste.edits;
    /// ```
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

    /// The structure object that holds
    /// the base to create a paste. This
    /// is then sent to pastemyst. All
    /// fields are optional *except* the
    /// `pasties` array which uses `PastyObject`.
    ///
    /// ### API Docs
    /// The relevent link to the API documentation
    /// is: https://paste.myst.rs/api-docs/objects
    ///
    /// ## Examples
    ///
    /// ```rust
    /// let _data: CreateObject = CreateObject {
    ///     title: String::from("[crates.io/crates/pastemyst] This is a title"),
    ///     expiresIn: String::from("1d"),
    ///     isPrivate: false,
    ///     isPublic: false,
    ///     tags: String::from(""),
    ///     pasties: var_pasties,
    /// };
    /// ```
    #[derive(Serialize)]
    #[allow(non_snake_case, dead_code)]
    pub struct CreateObject {
        /// Title of the paste.
        pub title: String,
        /// When the paste will expire,
        /// possible values are never, 1h,
        /// 2h, 10h, 1d, 2d, 1w, 1m, 1y.
        pub expiresIn: String,
        /// If it's private it's only
        /// accessible by the owner.
        pub isPrivate: bool,
        /// Is it displayed on the
        /// owner's public profile.
        pub isPublic: bool,
        /// List of tags, comma separated.
        pub tags: String,
        /// List of pasties.
        pub pasties: Vec<PastyObject>,
    }
}
