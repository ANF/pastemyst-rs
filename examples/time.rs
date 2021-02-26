use pastemyst::time::*;

#[tokio::main]
async fn main() -> TimeResult<()> {
    tokio::task::spawn_blocking(|| call_expires_into_unix().unwrap());
    call_expires_into_unix_async().await?;
    Ok(())
}

fn call_expires_into_unix() -> TimeResult<()> {
    let unix_time: u64 = expires_into_unix(42, expires_in::ONE_DAY)?;
    println!("{}", unix_time.to_string());
    Ok(())
}

async fn call_expires_into_unix_async() -> TimeResult<()> {
    let unix_time: u64 = expires_into_unix_async(1337, expires_in::TWO_DAYS).await?;
    println!("{}", unix_time.to_string());
    Ok(())
}
