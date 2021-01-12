// get_private_paste
use pastemyst_rs::paste;
use pastemyst_rs::paste::PasteInfo;

#[allow(unused_must_use)]
fn main() -> Result<(), reqwest::Error> {
    let paste: PasteInfo = paste::get_private_paste("pasteID", "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings")?;
    println!("{}", paste.ownerId);
    Ok(())
}
