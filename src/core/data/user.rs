use crate::core::data::country_code::CountryCode;

#[derive(Debug, PartialEq)]
pub enum UserError {
    InvalidCountryCode(String),
    InvalidId(String),
    InvalidUsername(String),
}

pub struct UserFields {
    pub authenticated: bool,
    pub country_code_str: String,
    pub email: String,
    pub email_verified: bool,
    pub first_name: String,
    pub id: String,
    pub last_name: String,
    pub nostr_public_key: String,
    pub username: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    authenticated: bool,
    country: CountryCode,
    email: String,
    email_verified: bool,
    first_name: String,
    id: String,
    last_name: String,
    nostr_public_key: String,
    username: String,
}

impl User {
    #[allow(dead_code)]
    fn new(fields: UserFields) -> Result<Self, UserError> {
        let country: CountryCode = match fields.country_code_str.parse() {
            Ok(country) => country,
            Err(err) => return Err(UserError::InvalidCountryCode(err.to_string())),
        };

        if fields.id.is_empty() {
            return Err(UserError::InvalidId(String::from("")));
        }

        if fields.username.is_empty() {
            return Err(UserError::InvalidUsername(fields.username));
        }

        Ok(Self {
            authenticated: fields.authenticated,
            country,
            email: fields.email,
            email_verified: fields.email_verified,
            first_name: fields.first_name,
            id: fields.id,
            last_name: fields.last_name,
            nostr_public_key: fields.nostr_public_key,
            username: fields.username,
        })
    }

    pub fn authenticated(&self) -> bool {
        self.authenticated
    }

    pub fn country(&self) -> CountryCode {
        self.country
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn email_verified(&self) -> bool {
        self.email_verified
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    pub fn nostr_public_key(&self) -> &str {
        &self.nostr_public_key
    }

    pub fn username(&self) -> &str {
        &self.username
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_user_fields() -> UserFields {
        UserFields {
            authenticated: false,
            country_code_str: String::from("WF"),
            email: String::from(""),
            email_verified: false,
            first_name: String::from(""),
            id: String::from("12345"),
            last_name: String::from(""),
            nostr_public_key: String::from(""),
            username: String::from("testuser"),
        }
    }

    #[test]
    fn test_initialise_user() {
        let user = User::new(generate_user_fields());

        assert!(user.is_ok());
    }

    #[test]
    fn test_user_with_empty_id_fails() {
        let mut user_fields = generate_user_fields();

        user_fields.id = String::from("");

        let user = User::new(user_fields);

        assert!(user.is_err_and(|x| matches!(x, UserError::InvalidId(_))));
    }

    #[test]
    fn test_user_with_invalid_country_code_fails() {
        let mut user_fields = generate_user_fields();

        user_fields.country_code_str = String::from("ABC");

        let user = User::new(user_fields);

        assert!(user.is_err_and(|x| matches!(x, UserError::InvalidCountryCode(_))));
    }

    #[test]
    fn test_user_with_empty_username_fails() {
        let mut user_fields = generate_user_fields();

        user_fields.username = String::from("");

        let user = User::new(user_fields);

        assert!(user.is_err_and(|x| matches!(x, UserError::InvalidUsername(_))));
    }

    #[test]
    fn test_user_authenticated_getter() {
        let mut user_fields = generate_user_fields();

        user_fields.authenticated = true;

        let user = User::new(user_fields).unwrap();

        assert!(user.authenticated());
    }

    #[test]
    fn test_user_country_getter() {
        let mut user_fields = generate_user_fields();
        let country_code_str = String::from("US");

        user_fields.country_code_str = country_code_str.clone();

        let country: CountryCode = country_code_str.parse().unwrap();
        let user = User::new(user_fields).unwrap();

        assert_eq!(user.country(), country);
    }

    #[test]
    fn test_user_email_getter() {
        let mut user_fields = generate_user_fields();
        let email = String::from("test@test.com");

        user_fields.email = email.clone();

        let user = User::new(user_fields).unwrap();

        assert_eq!(user.email(), email);
    }

    #[test]
    fn test_user_email_verified_getter() {
        let mut user_fields = generate_user_fields();

        user_fields.email_verified = true;

        let user = User::new(user_fields).unwrap();

        assert!(user.email_verified());
    }

    #[test]
    fn test_user_first_name_getter() {
        let mut user_fields = generate_user_fields();
        let first_name = String::from("Lorem");

        user_fields.first_name = first_name.clone();

        let user = User::new(user_fields).unwrap();

        assert_eq!(user.first_name(), first_name);
    }

    #[test]
    fn test_user_id_getter() {
        let mut user_fields = generate_user_fields();
        let id = String::from("123456");

        user_fields.id = id.clone();

        let user = User::new(user_fields).unwrap();

        assert_eq!(user.id(), id);
    }

    #[test]
    fn test_user_last_name_getter() {
        let mut user_fields = generate_user_fields();
        let last_name = String::from("Lorem");

        user_fields.last_name = last_name.clone();

        let user = User::new(user_fields).unwrap();

        assert_eq!(user.last_name(), last_name);
    }

    #[test]
    fn test_user_nostr_public_key_getter() {
        let mut user_fields = generate_user_fields();
        let nostr_public_key = String::from("halwerkgjharklh");

        user_fields.nostr_public_key = nostr_public_key.clone();

        let user = User::new(user_fields).unwrap();

        assert_eq!(user.nostr_public_key(), nostr_public_key);
    }

    #[test]
    fn test_user_username_getter() {
        let mut user_fields = generate_user_fields();
        let username = String::from("testuser");

        user_fields.username = username.clone();

        let user = User::new(user_fields).unwrap();

        assert_eq!(user.username(), username);
    }
}
