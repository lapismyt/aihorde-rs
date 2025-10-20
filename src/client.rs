use std::collections::HashMap;

use crate::consts::{DEFAULT_API_KEY, DEFAULT_BASE_URL, PKG_VERSION};
use crate::enums::{ModelState, ModelType};
use crate::errors::AihordeError;
use crate::models::{ActiveModel, GenerationInputStable, RequestAsync, RequestStatusCheck, RequestStatusStable, UserDetails, ValidationError};
use log::{debug, warn};
use reqwest::Client;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone)]
pub struct AihordeClient {
    api_key: String,
    base_url: Url,
    client_agent: String,
    client: Client,
}

impl Default for AihordeClient {
    fn default() -> Self {
        Self {
            api_key: String::from(DEFAULT_API_KEY),
            base_url: Url::parse(DEFAULT_BASE_URL).unwrap_or_else(|_| {
                warn!("Invalid base URL, using default");
                Url::parse(DEFAULT_BASE_URL).unwrap()
            }),
            client_agent: format!(
                "aihorde-rs:{}:https://github.com/lapismyt/aihorde-rs",
                PKG_VERSION
            ),
            client: Client::new(),
        }
    }
}

impl AihordeClient {

    /// ### Create a new AihordeClient instance
    /// #### Arguments
    /// * `api_key` - The API Key corresponding to a registered user.
    /// * `base_url` - The base URL of the AI Horde to connect to.
    /// * `client_agent` - The client name and version.
    pub fn new(
        api_key: Option<String>,
        base_url: Option<Url>,
        client_agent: Option<String>,
    ) -> Self {
        let api_key = api_key.unwrap_or_else(|| String::from(DEFAULT_API_KEY));
        let base_url = base_url.unwrap_or_else(|| Url::parse(DEFAULT_BASE_URL).unwrap());
        let client_agent = client_agent.unwrap_or_else(|| {
            format!(
                "aihorde-rs:{}:https://github.com/lapismyt/aihorde-rs",
                PKG_VERSION
            )
        });
        let client = Client::new();
        Self {
            api_key,
            base_url,
            client_agent,
            client,
        }
    }

    async fn handle_response<T: for<'de> Deserialize<'de>>(
        response: reqwest::Response,
    ) -> Result<T, AihordeError> {
        let status = response.status();
        debug!("Response status: {status}");
        debug!("Response headers: {:?}", response.headers());

        let text = response.text().await?;
        // debug!("Response text: {:?}", text);

        if status.is_success() {
            let result: T = serde_json::from_str(&text)
                .map_err(|e| AihordeError::JsonParseError(e.to_string()))?;
            Ok(result)
        } else {
            // Try to parse as ValidationError first
            if let Ok(validation_error) = serde_json::from_str::<ValidationError>(&text) {
                Err(AihordeError::ApiError {
                    code: validation_error.rc,
                    message: validation_error.message,
                })
            } else {
                Err(AihordeError::UnexpectedHTTPCode {
                    code: status.as_u16(),
                    message: text,
                })
            }
        }
    }

    /// ### Lookup user based on their API key
    /// This can be used to verify a user exists
    pub async fn find_user(&self) -> Result<UserDetails, AihordeError> {
        let url = format!("{}/find_user", self.base_url);
        let response = self
            .client
            .get(url)
            .header("apikey", &self.api_key)
            .header("Client-Agent", &self.client_agent)
            .send()
            .await?;
        let user = AihordeClient::handle_response::<UserDetails>(response).await?;
        Ok(user)
    }

    /// ### Details and statistics about a specific user
    /// #### Arguments
    /// * `user_id` - The ID of the user to retrieve.
    pub async fn get_user(&self, user_id: String) -> Result<UserDetails, AihordeError> {
        let url = format!("{}/users/{}", self.base_url, user_id);
        let response = self
            .client
            .get(url)
            .header("apikey", &self.api_key)
            .header("Client-Agent", &self.client_agent)
            .send()
            .await?;
        let user = AihordeClient::handle_response::<UserDetails>(response).await?;
        Ok(user)
    }

    /// ### A List with the details and statistic of all registered users
    /// #### Arguments
    /// * `page` - Which page of results to return. Each page has 25 users.
    /// * `sort` - How to sort the returned list.
    pub async fn get_users(
        &self,
        page: u32,
        sort: Option<String>,
    ) -> Result<Vec<UserDetails>, AihordeError> {
        let sort = sort.unwrap_or("kudos".to_string());
        let url = format!("{}/users?page={}&sort={}", self.base_url, page, sort);
        let response = self
            .client
            .get(url)
            .header("apikey", &self.api_key)
            .header("Client-Agent", &self.client_agent)
            .send()
            .await?;
        let users = AihordeClient::handle_response::<Vec<UserDetails>>(response).await?;
        Ok(users)
    }

    /// ### Initiate an Asynchronous request to generate images
    /// This endpoint will immediately return with the UUID of the request for generation.
    /// This endpoint will always be accepted, even if there are no workers available currently to fulfill this request.
    /// Perhaps some will appear in the next 10 minutes.
    /// Asynchronous requests live for 10 minutes before being considered stale and being deleted.
    pub async fn generate_async(
        &self,
        generation_input: GenerationInputStable,
    ) -> Result<RequestAsync, AihordeError> {
        let url = format!("{}/generate/async", self.base_url);
        let response = self
            .client
            .post(url)
            .header("apikey", &self.api_key)
            .header("Client-Agent", &self.client_agent)
            .json(&generation_input)
            .send()
            .await?;
        let request = AihordeClient::handle_response::<RequestAsync>(response).await?;
        Ok(request)
    }

    /// ### Retrieve the status of an Asynchronous generation request without images
    /// Use this request to check the status of a currently running asynchronous request without consuming bandwidth.
    /// #### Arguments
    /// * `request_id` - The UUID of the request to check.
    pub async fn generation_check(
        &self,
        request_id: String,
    ) -> Result<RequestStatusCheck, AihordeError> {
        let url = format!("{}/generate/check/{}", self.base_url, request_id);
        let response = self
            .client
            .get(url)
            .header("apikey", &self.api_key)
            .header("Client-Agent", &self.client_agent)
            .send()
            .await?;
        let request = AihordeClient::handle_response::<RequestStatusCheck>(response).await?;
        Ok(request)
    }

    /// ### Retrieve the full status of an Asynchronous generation request
    /// This request will include all already generated images in download URL or base64 encoded .webp files.
    /// As such, you are requested to not retrieve this endpoint often. Instead use the /check/ endpoint first.
    /// This endpoint is limited to 10 request per minute.
    /// #### Arguments
    /// * `request_id` - The UUID of the request to check.
    pub async fn generation_status(
        &self,
        request_id: String,
    ) -> Result<RequestStatusStable, AihordeError> {
        let url = format!("{}/generate/status/{}", self.base_url, request_id);
        let response = self
            .client
            .get(url)
            .header("apikey", &self.api_key)
            .header("Client-Agent", &self.client_agent)
            .send()
            .await?;
        let request = AihordeClient::handle_response::<RequestStatusStable>(response).await?;
        Ok(request)
    }

    /// ### Returns a list of models active currently in this horde
    /// #### Arguments
    /// * `model_type` - Filter the models by type (image or text).
    /// * `min_count` - Filter only models that have at least this amount of threads serving.
    /// * `max_count` - Filter the models that have at most this amount of threads serving.
    /// * `model_state` - If 'known', only show stats for known models in the model reference. If 'custom' only show stats for custom models. If 'all' shows stats for all models.
    pub async fn get_active_models(
        &self,
        model_type: Option<ModelType>,
        min_count: Option<u64>,
        max_count: Option<u64>,
        model_state: Option<ModelState>,
    ) -> Result<Vec<ActiveModel>, AihordeError> {
        let url = format!("{}/status/models", self.base_url);
        let mut query = HashMap::new();
        if let Some(model_type) = model_type {
            query.insert("model_type", serde_json::to_string(&model_type).unwrap());
        }
        if let Some(min_count) = min_count {
            query.insert("min_count", min_count.to_string());
        }
        if let Some(max_count) = max_count {
            query.insert("max_count", max_count.to_string());
        }
        if let Some(model_state) = model_state {
            query.insert("state", serde_json::to_string(&model_state).unwrap());
        }
        let response = self
            .client
            .get(url)
            .query(&query)
            .header("apikey", &self.api_key)
            .header("Client-Agent", &self.client_agent)
            .send()
            .await?;
        let models = AihordeClient::handle_response::<Vec<ActiveModel>>(response).await?;
        Ok(models)
    }
}
