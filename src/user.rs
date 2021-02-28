use serde::Deserialize;

/// The type provided by the pastemyst lib. It takes
/// a type `T` and evalutates to that type and a
/// `Result` like so: `Result<T, E>` where `E` has
/// the default value of `reqwest::Error`. Keep note
/// that `E` can be overriden.
pub type UserResult<T, E = reqwest::Error> = Result<T, E>;

const USER_ENDPOINT: &str = "https://paste.myst.rs/api/v2/user/";

/// Gets a user synchronously from [pastemyst](https://paste.myst.rs)
/// This information is stored in the `UserObject` struct. If a user
/// does not exist, it will return nothing and emit a console log
/// warning about the user not existing.
///
/// As of now, there is no specific way to disable this warning
/// unless you compile your own version. This might change in
/// the future, but for now, a warning will be emmited if
/// it returns false.
///
/// It still returns the struct when a user is not found and emit
/// a warning as stated earlier except that this method will return
/// an empty struct i.e, all the strings will be set to `""`, all
/// the booleans set to false and all numerics set to 0.
///
/// ### API Docs
/// The relevent API documentation for this method is:
/// https://paste.myst.rs/api-docs/user
///
/// ## Examples
///
/// ```rust
/// use pastemyst::user::*;
///
/// fn main() -> UserResult<()> {
///     let user_data = get_user("ANF-Studios")?;
///     println!("{:?}", user_data.defaultLang);
///     Ok(())
/// }
/// ```
pub fn get_user(username: &str) -> UserResult<UserObject> {
    let mut result: UserObject = UserObject {
        _id: str!(""),
        username: str!(""),
        avatarUrl: str!(""),
        defaultLang: str!(""),
        publicProfile: false,
        supporterLength: 0,
        contributor: false
    };
    if user_exists(username)? == false {
        print!("[pastemyst] The user '{}' does not exist and an empty object is returned.\n", username);            
    } else {
        result = reqwest::blocking::Client::builder()
            .build()?
            .get(&parse_user(username))
            .send()?
            .json()?;
    }
    Ok(result)
}

/// Gets a user asynchronously from [pastemyst](https://paste.myst.rs)
/// This information is stored in the `UserObject` struct. If a user
/// does not exist, it will return nothing and emit a console log
/// warning about the user not existing.
///
/// As of now, there is no specific way to disable this warning
/// unless you compile your own version. This might change in
/// the future, but for now, a warning will be emmited if
/// it returns false.
///
/// It still returns the struct when a user is not found and emit
/// a warning as stated earlier except that this method will return
/// an empty struct i.e, all the strings will be set to `""`, all
/// the booleans set to false and all numerics set to 0.
///
/// ### API Docs
/// The relevent API documentation for this method is:
/// https://paste.myst.rs/api-docs/user
///
/// ## Examples
///
/// ```rust
/// use pastemyst::user::*;
///
/// #[tokio::main]
/// async fn main() -> UserResult<()> {
///     let user_data = get_user_async("ANF-Studios").await?;
///     println!("{:?}", user_data._id);
///     Ok(())
/// }
/// ```
pub async fn get_user_async(username: &str) -> Result<UserObject, reqwest::Error> {
    let mut result: UserObject = UserObject {
        _id: str!(""),
        username: str!(""),
        avatarUrl: str!(""),
        defaultLang: str!(""),
        publicProfile: false,
        supporterLength: 0,
        contributor: false
    };
    if user_exists_async(username).await? == false {
        print!("[pastemyst] The user '{}' does not exist and an empty object is returned.\n", username);            
    } else {
        result = reqwest::Client::builder()
            .build()?
            .get(&parse_user(username))
            .send().await?
            .json().await?;
    }
    Ok(result)
}

/// Sends a request to [pastemyst](https://paste.myst.rs)
/// to check if a user exists. If a user *does exist*,
/// it returns a value of `200` i.e, `true` else `false`.
/// This methods runs synchronously.
///
/// The return value of this function is not to be confused
/// with an integer -- this method returns a boolean.
///
/// ### API Docs
/// The relevent API documentation for this method is:
/// https://paste.myst.rs/api-docs/user
///
/// ## Examples
///
/// ```rust
/// use pastemyst::user::*;
///
/// fn main() -> UserResult<()> {
///     const USERNAME: &str = "ANF-Studios";
///     let exists: bool = user_exists(USERNAME)?;
///     print!("The user '{}' exists: {}", USERNAME, exists);
///     Ok(())
/// }
/// ```
///
/// ```rust
/// use pastemyst::user::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     const USERNAME: &str = "ANF-Studios";
///     if user_exists(USERNAME)? == true {
///         println!("{} does indeed exist!", USERNAME);
///     } else { println!("{} was not found and does not exist.", USERNAME); }
///     Ok(())
/// }
/// ```
pub fn user_exists(username: &str) -> UserResult<bool> {
    let result = reqwest::blocking::Client::builder()
        .build()?
        .get(&parse_user_get(username))
        .send()?;
    let mut user_exists: bool = false;
    if result.status().as_u16() == 200 { user_exists = true; }
    else if result.status().as_u16() == 404 { user_exists = false; }
    Ok(user_exists)
}

/// Sends a request to [pastemyst](https://paste.myst.rs)
/// to check if a user exists. If a user *does exist*,
/// it returns a value of `200` i.e, `true` else `false`.
/// This methods runs asynchronously.
///
/// The return value of this function is not to be confused
/// with an integer -- this method returns a boolean.
///
/// ### API Docs
/// The relevent API documentation for this method is:
/// https://paste.myst.rs/api-docs/user
///
/// ## Examples
///
/// ```rust
/// use pastemyst::user::*;
///
/// #[tokio::main]
/// async fn main() -> UserResult<()> {
///     const USERNAME: &str = "ANF-Studios";
///     let exists: bool = user_exists(USERNAME).await?;
///     print!("The user '{}' exists: {}", USERNAME, exists);
///     Ok(())
/// }
/// ```
///
/// ```rust
/// use pastemyst::user::*;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     const USERNAME: &str = "ANF-Studios";
///     if user_exists(USERNAME).await? == true {
///         println!("{} does indeed exist!", USERNAME);
///     } else { println!("{} was not found and does not exist.", USERNAME); }
///     Ok(())
/// }
/// ```
pub async fn user_exists_async(username: &str) -> UserResult<bool> {
    let result = reqwest::Client::builder()
        .build()?
        .get(&parse_user_get(username))
        .send().await?;
    let mut user_exists: bool = false;
    if result.status().as_u16() == 200 { user_exists = true; }
    else if result.status().as_u16() == 404 { user_exists = false; }
    Ok(user_exists)
}

/// Parses a user `GET` url endpoint.
fn parse_user(username: &str) -> String { return format!("{}{}", USER_ENDPOINT, username) }
/// Parses a user exists url endpoint.
fn parse_user_get(username: &str) -> String { return format!("{}{}/exists", USER_ENDPOINT, username) }

/// The user object that pastemyst provides.
/// It has all the public details of a user.
///
/// ### API Docs
/// The relevent API documentation for this method is:
/// https://paste.myst.rs/api-docs/user
#[derive(Deserialize)]
#[allow(non_snake_case, dead_code, unused_doc_comments)]
pub struct UserObject {
    /// Id of the user.
    pub _id: String,
    /// The username of the user.
    pub username: String,
    /// URL of the avatar image.
    pub avatarUrl: String,
    /// The default pasty language
    /// of the user.
    pub defaultLang: String,
    /// If their profile is public
    /// or not.
    pub publicProfile: bool,
    /// How long has the user
    /// been a supporter for,
    /// 0 if not a supporter
    pub supporterLength: u32,
    /// If the user has contributed
    /// to pastemyst.
    pub contributor: bool,
}
