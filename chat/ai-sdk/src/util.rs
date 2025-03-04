use dotenv::dotenv;

pub fn get_deepseek_api_key() -> String {
    dotenv().ok();
    std::env::var("DEEPSEEK_KEY").expect("DEEPSEEK_KEY must be set")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_deepseek_api_key_should_work() {
        let api_key = get_deepseek_api_key();
        assert!(api_key.len() > 0)
    }
}
