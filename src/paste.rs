#[allow(dead_code)]

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
///     let foo = get_private_paste_async("pasteID", "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings").await?;
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

/// Uses the `CreateObject` struct as a parameter for paste
/// data to be constructed into json format and sent to
/// [pastemyst](https://paste.myst.rs) in a synchronous manner.
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
/// data to be constructed into json format and sent to
/// [pastemyst](https://paste.myst.rs) in an asynchronous manner.
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

/// Uses the `CreateObject` and `&str` (`auth_token`) to
/// send a paste to [pastemyst](https://paste.myst.rs)
/// held under your account which you can configure
/// to be private/public or not. You also get the
/// authority to delete that paste. This is a 
/// synchronous method.
///
/// ## Examples
///
/// ```rust
/// use pastemyst::paste::create_private_paste;
/// use pastemyst::paste::get_paste;
/// use pastemyst::paste::PasteResult;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let contents = get_paste("hipfqanx");
///     let paste = create_private_paste(contents, "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings").await?;
///     println!("{}", paste.isPrivate);
///     Ok(())
/// }
/// ```
pub fn create_private_paste(
    contents: CreateObject,
    auth_token: &str,
) -> Result<PasteObject, reqwest::Error> {
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

/// Uses the `CreateObject` struct and a `&str` authorization
/// key as a parameter which you can get from user settings
/// on [pastemyst](https://paste.myst.rs/user/settings).
/// This data is constructed into json format and sent to
/// [pastemyst](https://paste.myst.rs)
/// in an asynchronous manner. The paste is send under
/// the ownership of the account the auth key belongs to.
///
/// ## Examples
///
/// ```rust
/// use pastemyst::paste::*;
/// 
/// fn main() -> Result<(), reqwest::Error> /*PasteResult<()>*/ {
///     let pasties: Vec<PastyObject> = vec![
///         PastyObject {
///             _id: None,
///             language: Some(String::from("autodetect")),
///             title: Some(String::from("A pasty title")),
///             code: Some(String::from("fn main() { println!(\"Hello World!\"); }")),
///         },
///         PastyObject {
///             _id: None,
///             title: Some(String::from("Another pasty title")),
///             language: Some(String::from("autodetect")),
///             code: Some(String::from(
///                 "#include \"stdio.h\"\n\nint main() {\n\tprintf(\"Hello World!\");\n}",
///             )),
///         },
///     ];
///     let data: CreateObject = CreateObject {
///         title: String::from("[crates.io/crates/pastemyst] This is a title"),
///         expiresIn: String::from("1d"),
///         isPrivate: false,
///         isPublic: false,
///         tags: String::from(""),
///         pasties,
///     };
///     let paste = create_private_paste(
///         data,
///         "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings",
///     )?;
///     println!("{:#?}", paste.ownerId);
///     Ok(())
/// }
/// ```
pub async fn create_private_paste_async(
    contents: CreateObject,
    auth_token: &str,
) -> Result<PasteObject, reqwest::Error> {
    let content_type = reqwest::header::HeaderValue::from_static("application/json");
    let result = reqwest::Client::builder()
        .build()?
        .post(SEND_ENDPOINT)
        .header("Authorization", auth_token)
        .header(reqwest::header::CONTENT_TYPE, content_type)
        .body(serde_json::to_string(&contents).unwrap())
        .send()
        .await?;
    Ok(result.json().await?)
}

/// Sends a request to pastemyst to edit a
/// specific paste. You need to provide the
/// `EditObject` struct i.e, whatever you
/// want to edit. This is a synchronous method.
///
/// An important note, the pasty will **NOT**
/// be edited if you do not supply the id
/// (or the correct id) of the pasty. PasteMyst
/// needs to know which pasty to edit exactly.
///
/// The API does not allow you to append more
/// pastes as of this version writing this,
/// you can only append pastes when editing
/// within the site itself as the user.
/// 
/// ## Examples
///
/// ```rust
/// use pastemyst::str;
/// use pastemyst::paste;
///
/// fn main() {
///     let pasties = vec![pastemyst::paste::PastyObject {
///         _id: str!("PastyID"),
///         code: String::from("print('Hello World!')"),
///         language: str!(pastemyst::data::language::PYTHON),
///         title: "Pasty Title".to_string(),
///     }];
///     let edit_object = pastemyst::paste::EditObject {
///         isPrivate: false,
///         isPublic: false,
///         pasties: pasties,
///         tags: str!("Hello, World"),
///         title: str!("My title")
///     };
///     let paste_result: PasteObject = paste::edit_paste(edit_object,
///         "PasteID",
///         "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings",
///     )?;
/// }
/// ```
pub fn edit_paste(edit_info: EditObject, id: &str, auth_token: &str) -> Result<PasteObject, reqwest::Error> {
    let content_type = reqwest::header::HeaderValue::from_static("application/json");
    let result = reqwest::blocking::Client::builder()
        .build()?
        .patch(&parse_url(&id))
        .header("Authorization", auth_token)
        .header(reqwest::header::CONTENT_TYPE, content_type)
        .body(serde_json::to_string(&edit_info).unwrap())
        .send()?;
    Ok(result.json()?)
}

/// Sends a request to pastemyst to edit a
/// specific paste. You need to provide the
/// `EditObject` struct i.e, whatever you
/// want to edit. This is a asynchronous method.
///
/// An important note, the pasty will **NOT**
/// be edited if you do not supply the id
/// (or the correct id) of the pasty. PasteMyst
/// needs to know which pasty to edit exactly.
///
/// The API does not allow you to append more
/// pastes as of this version writing this,
/// you can only append pastes when editing
/// within the site itself as the user.
/// 
/// ## Examples
///
/// ```rust
/// use pastemyst::str;
/// use pastemyst::paste;
///
/// #[tokio::main]
/// async fn main() {
///     let pasties = vec![pastemyst::paste::PastyObject {
///         _id: str!("PastyID"),
///         code: String::from("print('Hello World!')"),
///         language: str!(pastemyst::data::language::PYTHON),
///         title: "Pasty Title".to_string(),
///     }];
///     let edit_object = pastemyst::paste::EditObject {
///         isPrivate: false,
///         isPublic: false,
///         pasties: pasties,
///         tags: str!("Hello, World"),
///         title: str!("My title")
///     };
///     let paste_result: PasteObject = paste::edit_paste(edit_object,
///         "PasteID",
///         "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings",
///     ).await?;
/// }
/// ```
pub async fn edit_paste_async(edit_info: EditObject, id: &str, auth_token: &str) -> Result<PasteObject, reqwest::Error> {
    let content_type = reqwest::header::HeaderValue::from_static("application/json");
    let result = reqwest::Client::builder()
        .build()?
        .patch(&parse_url(&id))
        .header("Authorization", auth_token)
        .header(reqwest::header::CONTENT_TYPE, content_type)
        .body(serde_json::to_string(&edit_info).unwrap())
        .send().await?;
    Ok(result.json().await?)
}

/// You can only delete pastes on your account, which
/// means you must also provide the authorization key.
/// This action is irreversible can the paste cannot
/// be restored in any way. This methods sends the
/// request synchronously.
///
/// This method returns an unsigned 16 bit integer
/// which is a status code recieved by the PasteMyst
/// server. If a paste deletes successfully, you
/// should recieve a status code of `200`. For
/// a list of all the web status codes, refer to:
/// https://en.wikipedia.org/wiki/List_of_HTTP_status_codes
///
/// ### API Docs
/// The relevent link to the API Documentation
/// is: https://paste.myst.rs/api-docs/paste
///
/// ```rust
/// use pastemyst::paste::*;
///
/// fn main() -> PasteResult<()> {
///     let paste_del_result = delete_paste(
///         "PasteID",
///         "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings",
///     )?;
///     if (paste_del_result == 200) { println!("Paste has been deleted successfully."); }
///     else { println!("Something went wrong and we recieved a status code of {}", paste_del_result); }
///     Ok(())
/// }
/// ```
pub fn delete_paste(id: &str, auth_token: &str) -> Result<u16, reqwest::Error> {
    let result = reqwest::blocking::Client::builder()
        .build()?
        .delete(&parse_url(&id))
        .header("Authorization", auth_token)
        .send()?;
    Ok(result.status().as_u16())
}

/// You can only delete pastes on your account, which
/// means you must also provide the authorization key.
/// This action is irreversible can the paste cannot
/// be restored in any way. This methods sends the
/// request asynchronously.
///
/// This method returns an unsigned 16 bit integer
/// which is a status code recieved by the PasteMyst
/// server. If a paste deletes successfully, you
/// should recieve a status code of `200`. For
/// a list of all the web status codes, refer to:
/// https://en.wikipedia.org/wiki/List_of_HTTP_status_codes
///
/// ### API Docs
/// The relevent link to the API Documentation
/// is: https://paste.myst.rs/api-docs/paste
///
/// ```rust
/// use pastemyst::paste::*;
///
/// #[tokio::main]
/// async fn main() -> PasteResult<()> {
///     let paste_del_result = delete_paste(
///         "PasteID",
///         "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings",
///     ).await?;
///     if (paste_del_result == 200) { println!("Paste has been deleted successfully."); }
///     else { println!("Something went wrong and we recieved a status code of {}", paste_del_result); }
///     Ok(())
/// }
/// ```
pub async fn delete_paste_async(id: &str, auth_token: &str) -> Result<u16, reqwest::Error> {
    let result = reqwest::Client::builder()
        .build()?
        .delete(&parse_url(&id))
        .header("Authorization", auth_token)
        .send().await?;
    Ok(result.status().as_u16())
}

/// Parses the url by combining
/// the `PASTE_ENDPOINT` with a
/// provided id.
fn parse_url(id: &str) -> String { return PASTE_ENDPOINT.to_owned() + &id }

/// The paste object recieved when
/// getting a paste. It contains
/// both the `PastyObject` and
/// `EditHistory` in an array.
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
    pub edits: Vec<EditHistory>,
}

/// Information about a specific pasty in a paste.
///
/// All fields except `language` are optional but due
/// to Rust's nature, so you must provide them. The
/// _id field should always be set to `None` though
/// if it's not, it is ignored by PasteMyst's API.
/// 
/// The design choice of the language field not being
/// optional was because auto detect isn't perfect
/// and you generally should not rely on it especially
/// with close bonded languages like C++ and C# which is
/// sometimes confused by the language detector. However,
/// you do not need to and can change set it to auto detect.
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
///     language: pastemyst::data::language::JSON,
///     title: Some(String::from("This is a pasty title")),
///     code: Some(String::from("{\"This_Is\": \"JSON_Code\"}")),
/// };
/// ```
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case, dead_code)]
pub struct PastyObject {
    /// Id of the pasty.
    pub _id: String,
    /// Language of the pasty.
    pub language: String,
    /// title of the pasty.
    pub title: String,
    /// contents of the pasty.
    pub code: String,
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
/// let edits: EditHistory = paste.edits[0];
/// ```
#[derive(Deserialize)]
#[allow(non_snake_case, dead_code)]
pub struct EditHistory {
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
///     pasties: pasties,
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
    /// If it"s private it"s only
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

/// The same as `CreateObject` except
/// that it does not have the `expiresIn`
/// field which has been removed for
/// convenience. This may change in
/// the future, but for the current
/// moment, this shall remain.
///
/// You can only edit pastes on your account,
/// so you must provide the Authorization header.
/// it returns a full paste object. To edit a paste
/// you need to provide only the values you are
/// editing in the JSON body.
///
/// To edit a single pasty you will need to provide
/// all of the original pasties changing the fields
//// you want. it"s not possible to update a single
/// pasty without providing all of the pasties.
///
/// ### API Docs
/// The relevent link to the API documentation
/// is: https://paste.myst.rs/api-docs/paste#edit-a-paste
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
pub struct EditObject {
    /// Title of the paste.
    pub title: String,
    /// If it"s private it"s only
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
