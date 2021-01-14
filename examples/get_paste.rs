// get_paste
use pastemyst_rs::paste;
use pastemyst_rs::paste::PasteObject;

// You can use either the library's provided type or reqwest's
// error and result (which requires it to be installed),
// however, it is recommened to use the former.

#[allow(unused_must_use)]
fn main() -> Result<(), reqwest::Error> /*PasteResult<()>*/ {
    let paste: PasteObject = paste::get_paste("hipfqanx")?;
    println!("{:#?}", paste.pasties[1].language);
    Ok(())
}
