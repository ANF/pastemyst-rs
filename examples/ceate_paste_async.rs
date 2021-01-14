// create_paste_async
use pastemyst_rs::paste;
use pastemyst_rs::paste::*;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result<()> {
    let pasties: Vec<PastyObject> = vec![
        PastyObject {
            _id: None,
            language: Some(String::from("autodetect")),
            title: Some(String::from("A pasty title")),
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
        title: String::from("This is a paste title"),
        expiresIn: String::from("1d"),
        isPrivate: false,
        isPublic: false,
        tags: String::from(""),
        pasties,
    };
    let paste = paste::create_paste_async(data).await?;
    println!("{:?}", paste.text().await?);
    Ok(())
}
