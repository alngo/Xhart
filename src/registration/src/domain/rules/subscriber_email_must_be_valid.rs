use super::abstract_rule::BusinessRule;
use regex::Regex;

const EMAIL_REGEX: &str =
    r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";

pub struct SubscriberEmailMustBeValid {
    email: String,
}

impl SubscriberEmailMustBeValid {
    pub fn new(email: String) -> Self {
        SubscriberEmailMustBeValid { email }
    }
}

impl BusinessRule for SubscriberEmailMustBeValid {
    fn is_broken(&self) -> bool {
        self.email.is_empty() || !Regex::new(EMAIL_REGEX).unwrap().is_match(&self.email)
    }

    fn message(&self) -> String {
        "User email must be valid.".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_email_must_be_valid_is_broken() {
        let test_cases = vec![
            "".to_string(),
            "a".to_string(),
            "a.com".to_string(),
            "@".to_string(),
            "a@".to_string(),
            "a@.com".to_string(),
        ];

        for test_case in test_cases {
            assert_eq!(true, SubscriberEmailMustBeValid::new(test_case).is_broken());
        }
    }
}
