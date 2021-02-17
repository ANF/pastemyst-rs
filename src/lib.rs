/// This macro can assist prevention of
/// converting an str to a String and
/// calling that method each time.
///
/// It can be used for other purposes
/// if you wish to, but it's designed
/// to work with any struct field that
/// takes in a String type. Not necessarily
/// recommened to be used however you can
/// if you wish to but it's not compulsory.
///
/// You can also use other macros like
/// methods like `String::from` or even
/// `to_string()` if you wish to.
///
/// This method calls the `String::from` method.
#[macro_export]
macro_rules! str {
    {$value:expr} => (String::from($value));
}

/// This module is extremely useful
/// for time-related operations.
///
/// So far, the only method that the
/// API offers is to convert a date
/// time that expires in pastemyst's
/// time type to unix timestamp.
///
/// This mod also offers pastemyst's
/// "time type" which was mentioned
/// earlier above.
///
/// ### API Docs
/// The relevent API documentation for this
/// module is:
/// https://paste.myst.rs/api-docs/time
#[allow(dead_code, unused_variables)]
pub mod time {
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
}

/// This is the data module that can be
/// used for general purposes
///
/// ### API Docs
/// The relevent API documentation for
/// this module is:
/// https://paste.myst.rs/api-docs/data
pub mod data {
    /// An enum of PasteMyt language constants.
    //#[allow(non_camel_case_types)]
    pub mod language {
        pub const AUTODETECT: &str = "Autodetect";
        pub const PLAIN: &str = "Plain Text";
        pub const APL: &str = "APL";
        pub const PGP: &str = "PGP";
        pub const ASN1: &str = "ASN.1";
        pub const ASTERISK: &str = "Asterisk";
        pub const BRAINFUCK: &str = "Brainfuck";
        pub const CLANG: &str = "C";
        pub const C: &str = "C";
        pub const CPP: &str = "C++";
        pub const COBOL: &str = "Cobol";
        pub const CSHARP: &str = "C#";
        pub const CLOJURE: &str = "Clojure";
        pub const CLOJURE_SCRIPT: &str = "ClojureScript";
        pub const GSS: &str = "Closure Stylesheets (GSS)";
        pub const CMAKE: &str = "CMake";
        pub const COFFEE_SCRIPT: &str = "CoffeeScript";
        pub const LISP: &str = "Common Lisp";
        pub const CYPHER: &str = "Cypher";
        pub const CYTHON: &str = "Cython";
        pub const CRYSTAL: &str = "Crystal";
        pub const CSS: &str = "CSS";
        pub const CQL: &str = "CQL";
        pub const DLANG: &'static str = "D";
        pub const D: &str = "D";
        pub const DART: &str = "Dart";
        pub const DIFF: &str = "diff";
        pub const DJANGO: &str = "Django";
        pub const DOCKER: &str = "Dockerfile";
        pub const DTD: &str = "DTD";
        pub const DYLAN: &str = "Dylan";
        pub const EBNF: &str = "EBNF";
        pub const ECL: &str = "ECL";
        pub const EDN: &str = "edn";
        pub const EIFFEL: &str = "Eiffel";
        pub const ELM: &str = "Elm";
        pub const EJS: &str = "Embedded Javascript";
        pub const ERB: &str = "Embedded Ruby";
        pub const ERLANG: &str = "Erlang";
        pub const ESPER: &str = "Esper";
        pub const FACTOR: &str = "Factor";
        pub const FCL: &str = "FCL";
        pub const FORTH: &str = "Forth";
        pub const FORTRAN: &str = "Fortran";
        pub const FSHARP: &str = "F#";
        pub const GAS: &str = "Gas";
        pub const GHERKIN: &str = "Gherkin";
        pub const GFM: &str = "GitHub Flavored Markdown";
        pub const GITHUB_MARKDOWN: &str = "GitHub Flavored Markdown";
        pub const GO: &str = "Go";
        pub const GROOVY: &str = "Groovy";
        pub const HAML: &str = "HAML";
        pub const HASKELL: &str = "Haskell";
        pub const HASKELL_LITERATE: &str = "Haskell (Literate)";
        pub const HAXE: &str = "Haxe";
        pub const HXML: &str = "HXML";
        pub const ASP_NET: &str = "ASP.NET";
        pub const HTML: &str = "HTML";
        pub const HTTP: &str = "HTTP";
        pub const IDL: &str = "IDL";
        pub const PUG: &str = "Pug";
        pub const JAVA: &str = "Java";
        pub const JSP: &str = "Java Server Pages";
        pub const JAVASCRIPT: &str = "JavaScript";
        pub const JSON: &str = "JSON";
        pub const JSON_LD: &str = "JSON-LD";
        pub const JSX: &str = "JSX";
        pub const JINJA2: &str = "Jinja2";
        pub const JULIA: &str = "Julia";
        pub const KOTLIN: &str = "Kotlin";
        pub const LESS: &str = "LESS";
        pub const LIVESCRIPT: &str = "LiveScript";
        pub const LUA: &str = "Lua";
        pub const MARKDOWN: &str = "Markdown";
        pub const MIRC: &str = "mIRC";
        pub const MARIA_DB: &str = "MariaDB SQL";
        pub const MATHEMATICA: &str = "Mathematica";
        pub const MODELICA: &str = "Modelica";
        pub const MUMPS: &str = "MUMPS";
        pub const MS_SQL: &str = "MS SQL";
        pub const MBOX: &str = "mbox";
        pub const MYSQL: &str = "MySQL";
        pub const NGINX: &str = "Nginx";
        pub const NSIS: &str = "NSIS";
        pub const NTRIPLES: &str = "NTriples";
        pub const OBJ_C: &str = "Objective-C";
        pub const OCAML: &str = "OCaml";
        pub const OCTAVE: &str = "Octave";
        pub const OZ: &str = "Oz";
        pub const PASCAL: &str = "Pascal";
        pub const PEG_JS: &str = "PEG.js";
        pub const PERL: &str = "Perl";
        pub const PHP: &str = "PHP";
        pub const PIG: &str = "Pig";
        pub const PLSQL: &str = "PLSQL";
        pub const POWERSHELL: &str = "PowerShell";
        pub const INI: &str = "Properties files";
        pub const PROTOBUF: &str = "ProtoBuf";
        pub const PYTHON: &str = "Python";
        pub const PUPPET: &str = "Puppet";
        pub const QLANG: &str = "Q";
        pub const RSCRIPT: &str = "R";
        pub const RST: &str = "reStructuredText";
        pub const RPM_CHANGES: &str = "RPM Changes";
        pub const RPM_SPEC: &str = "RPM Spec";
        pub const RUBY: &str = "Ruby";
        pub const RUST: &str = "Rust";
        pub const SAS: &str = "SAS";
        pub const SASS: &str = "Sass";
        pub const SCALA: &str = "Scala";
        pub const SCHEME: &str = "Scheme";
        pub const SCSS: &str = "SCSS";
        pub const SHELL: &str = "Shell";
        pub const SIEVE: &str = "Sieve";
        pub const SLIM: &str = "Slim";
        pub const SMALLTALK: &str = "Smalltalk";
        pub const SMARTY: &str = "Smarty";
        pub const SOLR: &str = "Solr";
        pub const SML: &str = "SML";
        pub const SOY: &str = "Soy";
        pub const SPARQL: &str = "SPARQL";
        pub const SPREADSHEET: &str = "Spreadsheet";
        pub const SQL: &str = "SQL";
        pub const SQLITE: &str = "SQLite";
        pub const SQUIRREL: &str = "Squirrel";
        pub const STYLUS: &str = "Stylus";
        pub const SWIFT: &str = "SWIFT";
        pub const STEX: &str = "sTeX";
        pub const LATEX: &str = "LaTeX";
        pub const SYSTEM_VERILOG: &str = "SystemVerilog";
        pub const TCL: &str = "Tcl";
        pub const TEXTILE: &str = "Textile";
        pub const TIDDLYWIKI: &str = "TiddlyWiki";
        pub const TIKI_WIKI: &str = "Tiki Wiki";
        pub const TOML: &str = "TOML";
        pub const TORNADO: &str = "Tornado";
        pub const TROFF: &str = "troff";
        pub const TTCN: &str = "TTCN";
        pub const TTCN_CFG: &str = "TTCN_CFG";
        pub const TURTLE: &str = "Turtle";
        pub const TYPESCRIPT: &str = "TypeScript";
        pub const TYPESCRIPT_JSX: &str = "TypeScript-JSX";
        pub const TWIG: &str = "Twig";
        pub const WEB_IDL: &str = "Web IDL";
        pub const VB_NET: &str = "VB.NET";
        pub const VBSCRIPT: &str = "VBScript";
        pub const VELOCITY: &str = "Velocity";
        pub const VERILOG: &str = "Verilog";
        pub const VHDL: &str = "VHDL";
        pub const VUE: &str = "Vue.js Component";
        pub const XML: &str = "XML";
        pub const XQUERY: &str = "XQuery";
        pub const YACAS: &str = "Yacas";
        pub const YAML: &str = "YAML";
        pub const Z80: &str = "Z80";
        pub const MSCGEN: &str = "mscgen";
        pub const XU: &str = "xu";
        pub const MSGENNY: &str = "msgenny";
    }
}

/// This is the user module which
/// allows you to communicate with
/// pastemyst's api to send `GET`
/// requests to check if a user exists
/// or not *and* `GET` user information.
///
/// ### API Docs
/// The relevent API documentation for
/// this module is:
/// https://paste.myst.rs/api-docs/user
#[allow(dead_code, unused_variables)]
pub mod user {
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
}

/// The paste namespace which contains
/// every method and struct to
/// `GET` and `POST` (send) a paste
/// to [pastemyst](https://paste.myst.rs).
///
/// ### API Docs
/// The relevent API documentation for this
/// module is:
/// https://paste.myst.rs/api-docs/paste
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
    ///         language: str!(pastemyst::paste::language::PYTHON),
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
    ///         language: str!(pastemyst::paste::language::PYTHON),
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
    ///     language: pastemyst::paste::language::JSON,
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
}
