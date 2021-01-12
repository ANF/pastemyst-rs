// get_paste
use pastemyst_rs::paste;
use pastemyst_rs::paste::PasteInfo;

#[allow(unused_must_use)]
fn main() -> Result<(), reqwest::Error> {
    let paste: PasteInfo = paste::get_paste("hipfqanx")?;
    println!("{}", paste.pasties[1].language);
    Ok(())
}
