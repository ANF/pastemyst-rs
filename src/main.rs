use pastemyst::time::*;

fn main() {
    expires_into_unix(42, expires_in::NEVER);
    expires_into_unix(42, expires_in::ONE_DAY);
}
