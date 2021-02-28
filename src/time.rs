use serde::Deserialize;

/// The type provided by the pastemyst lib. It takes
/// a type `T` and evalutates to that type and a
/// `Result` like so: `Result<T, E>` where `E` has
/// the default value of `reqwest::Error`. Keep note
/// that `E` can be overriden.
pub type TimeResult<T, E = reqwest::Error> = Result<T, E>;

const TIME_ENDPOINT: &str = "https://paste.myst.rs/api/v2/time/expiresInToUnixTime";

/// All the possible values of the
/// expiration of a paste provided
/// by PasteMyst's API v2.
pub mod expires_in {
    /// The paste will never expire.
    /// Granted that this will only
    /// not be deleted by the code
    /// base, the data may get lost
    /// from the physical drive
    /// though chances are extremely
    /// slim.
    pub const NEVER: &str = "never";
    /// The paste will expire in
    /// an hour.
    pub const ONE_HOUR: &str = "1h";
    /// The paste will expire in
    /// two hours.
    pub const TWO_HOURS: &str = "2h";
    /// The paste will expire in
    /// ten hours.
    pub const TEN_HOURS: &str = "10h";
    /// The paste will expire in
    /// one day (24 hours).
    pub const ONE_DAY: &str = "1d";
    /// The paste will expire in
    /// two days (48 hours).
    pub const TWO_DAYS: &str = "2d";
    /// The paste will expire in
    /// a week (168 hours).
    pub const ONE_WEEK: &str = "1w";
    /// The paste will expire in
    /// a month (~672 hours).
    pub const ONE_MONTH: &str = "1m";
    /// The paste will expire in
    /// a whole year (~8736 hours,
    /// followed by solar calander).
    pub const ONE_YEAR: &str = "1y";
}

/// Synchronously sends a request to pastemyst's time
/// module to convert the `expires_in` field to a unix
/// timestamp. This method is really useful for time
/// related operations where this kind of data is cruicial
/// for certian functionality.
///
/// If the `expires_in` field is not valid or recognized,
/// it does **NOT** send any web requests and emits a warning.
/// To prevent runtime exceptions, the return value, when unset
/// i.e, when web requests aren't send, sets the value of the
/// return integer to 0 regardless.
///
/// The `created_at` value is an unsigned 64 bit integer meanwhile
/// `expires_at` is a string. It's recommended to use provided strings
/// by the library under `pastemyst::time::expires_in`.
///
/// ### API Docs
/// The relevent API documentation for this method is:
/// https://paste.myst.rs/api-docs/time
///
/// ## Examples
///
/// ```rust
/// use pastemyst::time::*;
///
/// fn main() -> TimeResult<()> {
///     let unix_time: u64 = expires_into_unix(42, expires_in::ONE_DAY)?;
///     println!("{}", unix_time.to_string());
///     Ok(())
/// }
/// ```
pub fn expires_into_unix(created_at: u64, expires_in: &str) -> TimeResult<u64> {
    let mut response: TimeObject = TimeObject { result: 0 };
    #[allow(unused_assignments)] let mut valid_time: bool = false;
    match expires_in {
        expires_in::NEVER => valid_time = true,
        expires_in::ONE_HOUR => valid_time = true,
        expires_in::TWO_HOURS => valid_time = true,
        expires_in::ONE_DAY => valid_time = true,
        expires_in::TWO_DAYS => valid_time = true,
        expires_in::ONE_WEEK => valid_time = true,
        expires_in::ONE_MONTH => valid_time = true,
        expires_in::ONE_YEAR => valid_time = true,
        _ => valid_time = false
    }
    if valid_time {
        response = reqwest::blocking::get(&parse_time(created_at, expires_in))?
            .json().unwrap();
    } else { println!("[pastemyst] The given expires timestamp is not valid and 0 will be returned."); }
    Ok(response.result)
}

/// Asynchronously sends a request to pastemyst's time
/// module to convert the `expires_in` field to a unix
/// timestamp. This method is really useful for time
/// related operations where this kind of data is cruicial
/// for certian functionality.
///
/// If the `expires_in` field is not valid or recognized,
/// it does **NOT** send any web requests and emits a warning.
/// To prevent runtime exceptions, the return value, when unset
/// i.e, when web requests aren't send, sets the value of the
/// return integer to 0 regardless.
///
/// The `created_at` value is an unsigned 64 bit integer meanwhile
/// `expires_at` is a string. It's recommended to use provided strings
/// by the library under `pastemyst::time::expires_in`.
///
/// ### API Docs
/// The relevent API documentation for this method is:
/// https://paste.myst.rs/api-docs/time
///
/// ## Examples
///
/// ```rust
/// use pastemyst::time::*;
///
/// #[tokio::main]
/// async fn main() -> TimeResult<()> {
///     let unix_time: u64 = expires_into_unix_async(1337, expires_in::TWO_DAYS).await?;
///     println!("{}", unix_time.to_string());
///     Ok(())
/// }
/// ```
pub async fn expires_into_unix_async(created_at: u64, expires_in: &str) -> TimeResult<u64> {
    let mut response: TimeObject = TimeObject { result: 0 };
    #[allow(unused_assignments)] let mut valid_time: bool = false;
    match expires_in {
        expires_in::NEVER => valid_time = true,
        expires_in::ONE_HOUR => valid_time = true,
        expires_in::TWO_HOURS => valid_time = true,
        expires_in::ONE_DAY => valid_time = true,
        expires_in::TWO_DAYS => valid_time = true,
        expires_in::ONE_WEEK => valid_time = true,
        expires_in::ONE_MONTH => valid_time = true,
        expires_in::ONE_YEAR => valid_time = true,
        _ => valid_time = false
    }
    if valid_time {
        response = reqwest::Client::builder()
            .build()?
            .get(&parse_time(created_at, expires_in))
            .send().await?
            .json().await?;
    } else { println!("[pastemyst] The given expires timestamp is not valid and 0 will be returned."); }
    Ok(response.result)
}

/// This struct is only here so
/// that the json object recieved
/// from the API can be serialized
/// for the variable to be extracted
/// and be returned directly instead
/// of the struct itself for ease.
/// This is the main reason why this
/// struct has been kept private.
#[derive(Deserialize)]
struct TimeObject { result: u64 }

/// Parses the time module's API path
fn parse_time(created_at: u64, expires_in: &str) -> String {
    return format!(
        "{}?createdAt={}&expiresIn={}",
        TIME_ENDPOINT, created_at, expires_in
    )
}
