use reqwest::{Client, Error};

pub struct ContentfulClient {
    api_token: String,
    client: Client,
}

impl ContentfulClient {
    pub fn new(api_token: String) -> Self {
        ContentfulClient {
            api_token,
            client: Client::new(),
        }
    }

    pub async fn get_spaces(&self) -> Result<String, Error> {
        let url = "https://cdn.contentful.com/spaces";
        let response = self.client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }

    pub async fn get_entries(&self, space_id: &str, env: &str) -> Result<String, Error> {
        let url = format!("https://cdn.contentful.com/spaces/{}/environments/{}/entries", space_id, env);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }

    pub async fn get_content_types(&self, space_id: &str, env: &str) -> Result<String, Error> {
        let url = format!("https://cdn.contentful.com/spaces/{}/environments/{}/content_types", space_id, env);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
           .await?
            .text()
            .await?;
        Ok(response)
    }
}
