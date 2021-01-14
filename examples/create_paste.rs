// create_paste
use pastemyst::paste::*;

fn main() -> Result<(), reqwest::Error> /*PasteResult<()>*/ {
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
        title: String::from("[crates.io/crates/pastemyst] This is a title"),
        expiresIn: String::from("1d"),
        isPrivate: false,
        isPublic: false,
        tags: String::from(""),
        pasties: pasties,
    };
    let paste = create_paste(data)?;
    println!("{:#?}", paste._id);
    Ok(())
}
