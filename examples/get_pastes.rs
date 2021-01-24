use pastemyst::paste::*;

// You can use either the library's provided type or reqwest's
// error and result (which requires it to be installed),
// however, it is recommened to use the former.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::task::spawn_blocking(||call_get_paste().unwrap());
    call_get_paste_async().await?;
    // tokio::task::spawn_blocking(
    //     ||call_get_private_paste(
    //     "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings").unwrap());
    // call_get_private_paste_async(
    //     "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings").await?;
    Ok(())
}

/// Gets a paste from pastemyst synchronously.
fn call_get_paste() -> PasteResult<()> {
    let paste: PasteObject = get_paste("hipfqanx")?;
    println!("{:#?}", paste.pasties[1].language);
    Ok(())
}

/// Gets a paste from pastemyst asynchronously.
async fn call_get_paste_async() -> PasteResult<()> {
    let paste: PasteObject = get_paste_async("hipfqanx").await?;
    println!("{:#?}", paste.pasties[0].language);
    Ok(())
}

// You can use either the library's provided type or reqwest's
// error and result (which requires it to be installed),
// however, it is recommened to use the former.

/// Gets a private paste from pastemyst synchronously.
#[allow(dead_code)]
fn call_get_private_paste() -> PasteResult<()> {
    let paste: PasteObject = get_private_paste(
        "pasteID",
        "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings")?;
    println!("{}", paste.ownerId);
    Ok(())
}

/// Gets a private paste from pastemyst asynchronously.
#[allow(dead_code)]
async fn call_get_private_paste_async(auth_token: &str) -> PasteResult<()> {
    let paste: PasteObject = get_private_paste_async(
        "pasteID",
        auth_token,
    )
    .await?;
    println!("{}", paste.isPrivate);
    Ok(())
}
