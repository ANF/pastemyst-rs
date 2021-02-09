use pastemyst::str;
use pastemyst::paste::*;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result<()> {
    tokio::task::spawn_blocking(||call_create_paste().unwrap());
    call_create_paste_async().await?;
    // tokio::task::spawn_blocking(
    //     ||call_create_private_paste(
    //     "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings").unwrap());
    // call_create_private_paste_async(
    //     "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings").await?;
    Ok(())
}

/// Creates a paste synchronously.
fn call_create_paste() -> Result<(), reqwest::Error> /*PasteResult<()>*/ {
    let pasties: Vec<PastyObject> = vec![
        PastyObject {
            _id: str!(""),
            language: str!(pastemyst::paste::language::RUST),
            title: "A pasty title".to_string(),
            code: String::from("fn main() { println!(\"Hello World!\"); }"),
        },
        PastyObject {
            _id: str!(""),
            title: "Another pasty title".to_string(),
            language: str!(pastemyst::paste::language::CLANG),
            code: String::from("#include \"stdio.h\"\n\nint main() {\n\tprintf(\"Hello World!\");\n}"),
        },
    ];
    let data: CreateObject = CreateObject {
        title: String::from("[crates.io/crates/pastemyst] This is a title"),
        expiresIn: String::from("1d"),
        isPrivate: false,
        isPublic: false,
        tags: String::from(""),
        pasties: pasties,
    };
    let paste = create_paste(data)?;
    println!("https://paste.myst.rs/{}", paste._id);
    Ok(())
}

// Creates a paste asynchronously
async fn call_create_paste_async() -> Result<()> {
    let pasties: Vec<PastyObject> = vec![
        PastyObject {
            _id: str!(""),
            language: str!(pastemyst::paste::language::RUST),
            title: "A pasty title".to_string(),
            code: String::from("fn main() { println!(\"Hello World!\"); }"),
        },
        PastyObject {
            _id: str!(""),
            title: "Another pasty title".to_string(),
            language: str!(pastemyst::paste::language::CLANG),
            code: String::from("#include \"stdio.h\"\n\nint main() {\n\tprintf(\"Hello World!\");\n}"),
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
    let paste = create_paste_async(data).await?;
    println!("https://paste.myst.rs/{}", paste._id);
    Ok(())
}

/// Creates a private/owned paste synchronously.
#[allow(dead_code)]
fn call_create_private_paste(auth_token: &str) -> PasteResult<()> {
    let pasties: Vec<PastyObject> = vec![
        PastyObject {
            _id: str!(""),
            language: str!(pastemyst::paste::language::RUST),
            title: "A pasty title".to_string(),
            code: String::from("fn main() { println!(\"Hello World!\"); }"),
        },
        PastyObject {
            _id: str!(""),
            title: "Another pasty title".to_string(),
            language: str!(pastemyst::paste::language::CLANG),
            code: String::from("#include \"stdio.h\"\n\nint main() {\n\tprintf(\"Hello World!\");\n}"),
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
    let paste = create_private_paste(
        data,
        auth_token,
    )?;
    println!("https://paste.myst.rs/{}", paste._id);
    Ok(())
}

/// Creates a private/owned paste asynchronously.
#[allow(dead_code)]
async fn call_create_private_paste_async(auth_token: &str) -> PasteResult<()> {
    let pasties: Vec<PastyObject> = vec![
        PastyObject {
            _id: str!(""),
            language: str!(pastemyst::paste::language::RUST),
            title: "A pasty title".to_string(),
            code: String::from("fn main() { println!(\"Hello World!\"); }"),
        },
        PastyObject {
            _id: str!(""),
            title: "Another pasty title".to_string(),
            language: str!(pastemyst::paste::language::CLANG),
            code: String::from("#include \"stdio.h\"\n\nint main() {\n\tprintf(\"Hello World!\");\n}"),
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
    let paste = create_private_paste(
        data,
        auth_token,
    )?;
    println!("https://paste.myst.rs/{}", paste._id);
    Ok(())
}
