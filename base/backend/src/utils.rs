use sam_error::SamError;

pub fn get_host() -> Result<String, SamError> {
    let dev_host = dotenvy::var("DEV_HOST")
        .map_err(|_| SamError::MissingEnviromentVariable("DEV_HOST".to_string()))?;
    let dist_host = dotenvy::var("HOST")
        .map_err(|_| SamError::MissingEnviromentVariable("HOST".to_string()))?;
    let host = match cfg!(debug_assertions) {
        true => dev_host,
        false => dist_host,
    };
    Ok(host)
}
