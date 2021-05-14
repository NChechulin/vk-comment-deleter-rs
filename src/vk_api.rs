use crate::comment_reading::Comment;
use std::collections::HashMap;

impl Comment {
    fn serialize(&self, token: &String) -> HashMap<&str, String> {
        let mut result = HashMap::new();

        result.insert("owner_id", self.owner_id.to_string());
        result.insert("comment_id", self.comment_id.to_string());
        result.insert("access_token", token.to_string());
        result.insert("v", String::from("5.120"));

        result
    }
}
