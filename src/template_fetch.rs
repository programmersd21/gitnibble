#[cfg(feature = "fetch")]
pub async fn fetch_template(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://raw.githubusercontent.com/toptal/gitignore/master/templates/{}.gitignore",
        name
    );
    let resp = reqwest::get(&url).await?.text().await?;
    Ok(resp)
}

#[cfg(not(feature = "fetch"))]
pub fn fetch_disabled_notice() -> &'static str {
    "fetch feature is disabled in default build."
}
