use pastemyst::data::*;

#[tokio::main]
async fn main() -> DataResult<()> {
    // Get language by name.
    tokio::task::spawn_blocking(|| call_get_language_by_name().unwrap());
    call_get_language_by_name_async().await?;

    // Get language by extension.
    tokio::task::spawn_blocking(|| call_get_language_by_extension().unwrap());
    call_get_language_by_extension_async().await?;
    Ok(())
}

fn call_get_language_by_name() -> DataResult<()> {
    let language: DataObject = get_language_by_name(language::CSHARP)?;
    println!("{:?}", language.mimes[0]);
    Ok(())
}

async fn call_get_language_by_name_async() -> DataResult<()> {
    let language: DataObject = get_language_by_name_async(language::JAVASCRIPT).await?;
    println!("{:?}", language.mode);
    Ok(())
}

fn call_get_language_by_extension() -> DataResult<()> {
    let language: DataObject = get_language_by_extension("c")?;
    println!("{:?}", language.mimes[0]);
    Ok(())
}

async fn call_get_language_by_extension_async() -> DataResult<()> {
    let language: DataObject = get_language_by_extension_async("d").await?;
    println!("{:?}", language.mode);
    Ok(())
}
