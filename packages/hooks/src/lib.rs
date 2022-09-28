pub fn hooks() -> String {
    "hooks".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(hooks(), "hooks".to_string());
    }
}
