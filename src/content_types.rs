use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use contentful::{ContentfulClient, QueryBuilder};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentType {
    pub name: String,
    pub description: Option<String>,
    pub display_field: Option<String>,
    pub fields: Vec<Field>,
    #[serde(rename = "sys")]
    pub system_properties: SystemProperties,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Field {
    pub id: String,
    pub name: String,
    pub type_: String,
    pub required: bool,
    pub localized: bool,
    pub validations: Vec<Validation>,
    pub disabled: bool,
    pub omitted: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Validation {
    pub unique: Option<bool>,
    pub size: Option<Size>,
    pub regex: Option<Regex>,
    pub link_content_type: Option<Vec<String>>,
    pub in_: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Size {
    pub min: Option<i32>,
    pub max: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Regex {
    pub pattern: String,
    pub flags: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemProperties {
    pub id: String,
    pub version: Option<i32>,
    pub revision: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl SystemProperties {
    pub fn new(id: String) -> SystemProperties {
        SystemProperties {
            id,
            version: None,
            revision: None,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn with_version(id: String, version: i32) -> SystemProperties {
        SystemProperties {
            id,
            version: Some(version),
            revision: None,
            created_at: None,
            updated_at: None,
        }
    }
}

// Define a trait to extend ContentfulClient
trait ContentfulClientExt {
    fn get_content_types<T>(&self, query_builder: Option<QueryBuilder>) -> Result<Vec<T>, Box<dyn std::error::Error>>;
    fn get_content_types_by_query_string<T>(&self, query_string: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>;
    fn get_query_string_url_ext(&self, query_string: &str) -> String;
}

// Implement the trait for ContentfulClient
impl ContentfulClientExt for ContentfulClient {
    async fn get_content_types<T>(
        &self,
        query_builder: Option<QueryBuilder>,
    ) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        for<'a> T: Serialize + Deserialize<'a>,
    {
        let query_string = if let Some(query_builder) = query_builder {
            query_builder.build()
        } else {
            "".to_string()
        };

        self.get_content_types_by_query_string::<T>(query_string.as_str()).await
    }

    async fn get_content_types_by_query_string<T>(
        &self,
        query_string: &str,
    ) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        for<'a> T: Serialize + Deserialize<'a>,
    {
        log::debug!("query_string: {:?}", &query_string);
        let url = self.get_query_string_url_ext(query_string);
        if let Some(json) = http_client::get::<Value>(&url, &self.delivery_api_access_token).await? {
            if let Some(items) = json.clone().get_mut("items") {
                if items.is_array() {
                    if let Some(includes) = json.get("includes") {
                        self.resolve_array(items, includes)?;
                    } else {
                        let includes = Value::default();
                        self.resolve_array(items, &includes)?;
                    }

                    let ar_string = items.to_string();
                    let entries = serde_json::from_str::<Vec<T>>(ar_string.as_str())?;
                    Ok(entries)
                } else {
                    unimplemented!();
                }
            } else {
                unimplemented!();
            }
        } else {
            unimplemented!();
        }
    }
    fn get_query_string_url_ext(&self, query_string: &str) -> String {
        let url = format!(
            "{base_url}/{space_id}/environments/{environment}/entries{query_string}",
            base_url = &self.base_url,
            space_id = &self.space_id,
            environment = &self.environment_id,
            query_string = &query_string
        );
        url
    }
}


