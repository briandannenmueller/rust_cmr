#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get() {
        // Create a Query instance with a mock route and mode
        let mut query = Query::new("mock_route", "mock_mode");

        // Ensure that get method returns a Result<Vec<Value>, Error>
        let results = query.get(2000).await;
        assert!(results.is_ok());

        // Extract the Vec<Value> from the Result
        let results = results.unwrap();

        // Ensure that the results are not empty
        assert!(!results.is_empty());

        // Additional assertions can be made based on the structure of the results
    }

    #[test]
    fn test_build_url() {
        // Create a Query instance with a mock route and mode
        let query = Query::new("mock_route", "mock_mode");

        // Call build_url to get the URL string
        let url_result = query.build_url();

        // Ensure that build_url returns a valid URL string
        assert!(url_result.is_ok());

        // Additional assertions can be made based on the structure of the URL
    }
}