// get_private_paste_async
use pastemyst::paste;
use pastemyst::paste::PasteObject;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result<()> {
    let paste: PasteObject = paste::get_private_paste_async(
        "pasteID",
        "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings",
    )
    .await?;
    println!("{}", paste.isPrivate);
    Ok(())
}
