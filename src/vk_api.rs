use crate::comment_reading::Comment;
use reqwest::Client;
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

    pub async fn send_delete_comment_request(
        &self,
        token: &String,
        client: &mut Client,
    ) -> Result<(), String> {
        let params = self.serialize(token);
        let res = client
            .get("https://api.vk.com/method/wall.deleteComment")
            .query(&params)
            .send()
            .await;

        match res {
            Ok(resp) => match resp.status().is_success() {
                true => Ok(()),
                false => Err(resp.status().to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}
