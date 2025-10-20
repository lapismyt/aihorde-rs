use crate::enums::ModelState;
use crate::{client::AihordeClient, enums::ModelType};
use crate::models::GenerationInputStable;
use log::{debug, info};
use std::{sync::Once, thread, time::Duration};
use tokio::test;

static INIT: Once = Once::new();

fn test_client() -> AihordeClient {
    INIT.call_once(|| {
        env_logger::init();
    });
    AihordeClient::new(None, None, None)
}

#[test]
async fn test_find_user() {
    let client = test_client();
    let user = client.find_user().await.unwrap();
    info!("{:?}", user);
}

#[test]
async fn test_get_user() {
    let client = test_client();
    let user = client.get_user("0000000000".to_string()).await.unwrap();
    info!("{:?}", user);
}

#[test]
async fn test_get_users() {
    let client = test_client();
    let result = client.get_users(0, None).await;
    match &result {
        Ok(users) => debug!("Success: Found {} users", users.len()),
        Err(e) => {
            debug!("Error: {:?}", e);
            panic!("Failed to get users: {:?}", e);
        }
    }
}

#[test]
async fn test_generate_async_dry_run() {
    let client = test_client();
    let generation_input = GenerationInputStable {
        prompt: "A photo of a cat".to_string(),
        dry_run: Some(true),
        ..Default::default()
    };
    let request = client.generate_async(generation_input).await.unwrap();
    debug!("{:?}", request);
}

#[test]
async fn test_get_active_models() {
    let client = test_client();
    let models = client.get_active_models(Some(ModelType::Image), Some(10), None, Some(ModelState::All)).await.unwrap();
    info!("{:?}", models);
}

#[test]
async fn test_generate_async() {
    let client = test_client();
    let generation_input = GenerationInputStable {
        prompt: "A photo of a cat".to_string(),
        r2: Some(true),
        ..Default::default()
    };
    let request = client.generate_async(generation_input).await.unwrap();
    debug!("{:?}", request);
    loop {
        let status = client.generation_check(request.id.clone().unwrap()).await.unwrap();
        info!("queue position: {:?}", status.queue_position);
        info!("estimated wait time: {:?} secs", status.wait_time);
        if status.done.unwrap_or(false) {
            let full_status = client.generation_status(request.id.unwrap()).await.unwrap();
            info!("{:?}", full_status);
            for generation in full_status.generations.unwrap_or_default() {
                info!("{:?}", generation.img.unwrap_or("null".to_string()));
            }
            break;
        }
        thread::sleep(Duration::from_secs(5));
    }
}