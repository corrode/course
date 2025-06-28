use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct User {
    id: u32,
    name: String,
    email: String,
    active: bool,
}

fn user_to_json(user: &User) -> String {
    todo!()
}

fn user_from_json(json: &str) -> Result<User, serde_json::Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_serde() {
        let user = User {
            id: 1,
            name: "John".to_string(),
            email: "john@example.com".to_string(),
            active: true,
        };
        
        let json = user_to_json(&user);
        let parsed = user_from_json(&json).unwrap();
        assert_eq!(user, parsed);
    }
}