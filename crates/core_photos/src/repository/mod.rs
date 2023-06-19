
/// MockPhotosRepositoryTrait is created by automock macro
#[cfg_attr(test, automock)]
#[async_trait]
trait PhotosRepositoryTrait {
    /// Gets a list of heroes from the DB filted by name
    async fn get_photos_for_user(&self, user_id: &str) -> Result<Vec<Photo>, DataAccessError>;
}

struct PhotosRepository();

#[async_trait]
impl PhotosRepositoryTrait for PhotosRepository {
    async fn get_photos_for_user(&self, user_id: &str) -> Result<Vec<Photo>, DataAccessError> {
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    async fn get_photos_for_user_success(#[case] uri: &'static str, #[case] expected_filter: &'static str);
}
