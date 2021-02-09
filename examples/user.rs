use pastemyst::user::*;

#[tokio::main]
async fn main() -> UserResult<()> {
    // Get a user.
    tokio::task::spawn_blocking(||call_get_user().unwrap());
    call_get_user_async().await?;

    // Check if a user exists.
    tokio::task::spawn_blocking(||call_user_exists().unwrap());
    call_user_exists_async().await?;
    Ok(())
}

fn call_user_exists() -> UserResult<()> {
    const USERNAME: &str = "ANF-Studios";
    let exists: bool = user_exists(USERNAME)?;
    println!("The user '{}' exists: {}", USERNAME, exists);
    Ok(())
}

async fn call_user_exists_async() -> UserResult<()> {
    const USERNAME: &str = "ANF-Studios";
    let exists: bool = user_exists_async(USERNAME).await?;
    println!("The user '{}' exists: {}", USERNAME, exists);
    Ok(())
}

fn call_get_user() -> UserResult<()> {
    const USERNAME: &str = "ANF-Studios";
    let user = get_user(USERNAME)?;
    println!("{}", user.publicProfile);
    Ok(())
}

async fn call_get_user_async() -> UserResult<()> {
    const USERNAME: &str = "ANF-Studios";
    let user = get_user_async(USERNAME).await?;
    println!("{}", user.publicProfile);
    Ok(())
}
