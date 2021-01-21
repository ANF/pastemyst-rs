<h1 align="center" style="position: relative;">
    <img width="200" style="border-radius: 50%;"
    src="https://raw.githubusercontent.com/ANF/pastemyst-rs/main/images/RustMyst.png" /><br>
    PasteMyst.RS
</h1>
<h3 align="center">pastemyst-rs is an api wrapper for <a href="https://paste.myst.rs">pastemyst</a> written in Rust.</h3>
<h2 align="center">⚠ This package is under development ⚠</h2>


### Sample usage

To get a paste from pastemyst synchronously:
```rust
use pastemyst::paste;
use pastemyst::paste::PasteObject;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paste: PasteObject = paste::get_paste("hipfqanx")?;
    println!("{}", paste.pasties[1].language);
    Ok(())
}
```

To create paste synchronously:
```rust
use pastemyst::paste;
use pastemyst::paste::PasteObject;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pasties: Vec<PastyObject> = vec![
        PastyObject {
            _id: None,
            title: Some(String::from("A pasty title")),
            language: Some(String::from("autodetect")),
            code: Some(String::from("fn main() { println!(\"Hello World!\"); }")),
        },
        PastyObject {
            _id: None,
            title: Some(String::from("Another pasty title")),
            language: Some(String::from("autodetect")),
            code: Some(String::from("#include \"stdio.h\"\n\nint main() {\n\tprintf(\"Hello World!\");\n}")),
        },
    ];
    let data: CreateObject = CreateObject {
        title: String::from("[crates.io/crates/pastemyst] This is a title"),
        expiresIn: String::from("1d"),
        isPrivate: false,
        isPublic: false,
        tags: String::from(""),
        pasties,
    };
    let paste: /*reqwest::Response*/ = paste::create_paste(data).unwrap(); // You don't need to add the commented part, that's jut for your information.
    println!("{}", paste._id);
    Ok(())
}
```

More from the [examples](./examples/) and [documentation](https://docs.rs/pastemyst-rs/*/).

### Feature support
| Feature | Support | Async |
| :----------- | :-----------: | -----------: |
| API v1 | No | N/A |
| API v2 | Yes | N/A |
| Get pastes | Yes | Yes |
| Get private pastes | Yes | Yes |
| Create pastes | Yes | Yes |
| Create Private pastes* | Yes | Yes |
| Edit pastes | No | No |
| Delete pastes | Yes | Yes |
| Get Users | No | No |
| Check if a user exists | No | No |
| Get a language by name | No | No |
| Get a language by extension | No | No |

*This also includes a paste to be tied to your account, or create a private/public paste, or with tags.

### Repository structure
This is the current structure of the code:
```
./
├───.github/
│   ├─.workflows/
│   │   └─ rust.yml
│   └─ISSUE_TEMPLATES/
│       ├─ bug_report.md
│       ├─ feature_request.md
│       ├─ documentation.md
│       └ question.md
├───examples/
│   ├─ create_paste_async.rs
│   ├─ create_paste.rs
│   ├─ get_paste_async.rs
│   ├─ get_paste.rs
│   ├─ get_private_paste_async.rs
│   └─ get_private_paste.rs
├───src/
│   └─ lib.rs
├─── .gitattributes 
├─── .gitignore
├─── Cargo.toml
├─── LICENSE
└─── README.MD
```

### Building and Running
Being a Rust library, pastemyst-rs requires the Rust compiler installed. To check if it's installed, run: `rustc --version` and `cargo --version` to verify it. If it's not installed, install it from their [site](https://rust-lang.org). Once that's cleared out; run `cargo install` to get the packages. To test it on-hand, either
1. Create a main.rs with the main method and run those tests (`cargo run`).
2. Run from the examples using `cargo run --example example_name`, for example `cargo run --example get_paste`.

#### Installation
If you want to use it in your rust application, it is recommended to get the crate from https://crates.io/crates/pastemyst-rs.
In your `Cargo.toml` file, under `[dependencies]` paste this:
```toml
pastemyst-rs = "<Replace this text with the latest version>"
# OR
pastemyst-rs = { version = "<Replace this text with the latest version>" }
```

### Versioning
pastemyst-rs uses [SemVer](https://semver.org/).
> Given a version number MAJOR.MINOR.PATCH, increment the
>
> `MAJOR` version when you make incompatible API changes,
>
> `MINOR` version when you add functionality in a backwards compatible manner, and
>
> `PATCH` version when you make backwards compatible bug fixes.
>
> Additional labels for pre-release and build metadata are available as extensions to the MAJOR.MINOR.PATCH format.

### Help/Assistance
You can [create an issue](https://github.com/ANF-Studios/BotANF/issues/new) or just join the support (discord) server.


<a href="https://discord.gg/fKWpK7A"><img
        src="https://discord.com/api/guilds/732064655396044840/embed.png?style=banner3"
        alt="Discuss in the server"></img></a>
