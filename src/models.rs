use std::collections::HashMap;

use crate::enums::{
    ControlType, GenerationState, InjectTi, MetadataType, MetadataValue, ModelType, PostProcessing, RequestErrorCode, RequestWarningCode, SamplerName, SourceProcessing, StyleType, Workflow
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub type ModelSpecialPayloadStable = HashMap<String, Map<String, Value>>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ValidationError {
    pub message: Option<String>,
    pub rc: RequestErrorCode,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserKudosDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulated: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gifted: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub donated: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub received: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub awarded: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub styled: Option<f64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ResponseModelStylesUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_type: Option<StyleType>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserActiveGenerations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alchemy: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MonthlyKudos {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_received: Option<DateTime<Utc>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UsageDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub megapixelsteps: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ContributionsDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub megapixelsteps: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillments: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserThingRecords {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub megapixelsteps: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserAmountRecords {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interrogation: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserRecords {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<UserThingRecords>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contribution: Option<UserThingRecords>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment: Option<UserAmountRecords>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<UserAmountRecords>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<UserAmountRecords>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kudos: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluating_kudos: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_invited: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderator: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kudos_details: Option<UserKudosDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_ids: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<Vec<ResponseModelStylesUser>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharedkey_ids: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_generations: Option<UserActiveGenerations>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_kudos: Option<MonthlyKudos>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flagged: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub education: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customizer: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub special: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspicious: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudonymous: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_comment: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_passkey: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_age: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<UsageDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributions: Option<ContributionsDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<UserRecords>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserDetailsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ModelPayloadLorasStable {
    /// The exact name or CivitAI Model Page ID of the LoRa. If is_version is true, this should be the CivitAI version ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The strength of the LoRa to apply to the SD model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<f64>,

    /// The strength of the LoRa to apply to the clip model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip: Option<f64>,

    /// If set, will try to discover a trigger for this LoRa which matches or is similar to this string and inject it into the prompt. If 'any' is specified it will be pick the first trigger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inject_trigger: Option<String>,

    /// If true, will consider the LoRa ID as a CivitAI version ID and search accordingly. Ensure the name is an integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_version: Option<bool>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ModelPayloadTextualInversionsStable {
    /// The exact name or CivitAI ID of the Textual Inversion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// If set, Will automatically add this TI filename to the prompt or negative prompt accordingly using the provided strength. If this is set to None, then the user will have to manually add the embed to the prompt themselves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inject_ti: Option<InjectTi>,

    /// The strength with which to apply the TI to the prompt. Only used when inject_ti is not None.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f32>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ExtraText {
    /// The extra text to send along with this generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// The reference which points how and where this text should be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ExtraSourceImage {
    /// The Base64-encoded webp to use for further processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// Optional field, determining the strength to use for the processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f32>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ModelGenerationInputStable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampler_name: Option<SamplerName>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfg_scale: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub denoising_strength: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hires_fix_denoising_strength: Option<f32>,

    /// The height of the image to generate. Must be multiple of 64.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u16>,

    /// The width of the image to generate. Must be multiple of 64.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u16>,

    /// The list of post-processors to apply to the image, in the order to be applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_processing: Option<Vec<PostProcessing>>,

    /// Set to True to enable karras noise scheduling tweaks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub karras: Option<bool>,

    /// Set to True to create images that stitch together seamlessly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiling: Option<bool>,

    /// Set to True to process the image at base resolution before upscaling and re-processing or to use Stable Cascade 2-pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hires_fix: Option<bool>,

    /// The number of CLIP language processor layers to skip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_skip: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub facefixer_strength: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loras: Option<Vec<ModelPayloadLorasStable>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tis: Option<Vec<ModelPayloadTextualInversionsStable>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub special: Option<ModelSpecialPayloadStable>,

    /// Explicitly specify the horde-engine workflow to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<Workflow>,

    /// Set to True to generate the image using Layer Diffuse, creating an image with a transparent background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparent: Option<bool>,

    /// The seed to use to generate this request. You can pass text as well as numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<String>,

    /// If passed with multiple n, the provided seed will be incremented every time by this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_variation: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_type: Option<ControlType>,

    /// Set to True if the image submitted is a pre-generated control map for ControlNet use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_is_control: Option<bool>,

    /// Set to True if you want the ControlNet map returned instead of a generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_control_map: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_texts: Option<Vec<ExtraText>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<u16>,

    /// The amount of images to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GenerationInputStable {
    /// The prompt which will be sent to Stable Diffusion to generate an image.
    pub prompt: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<ModelGenerationInputStable>,

    /// Set to true if this request is NSFW. This will skip workers which censor images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,

    /// When true, only trusted workers will serve this request. When False, Evaluating workers will also be used which can increase speed but adds more risk!
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_workers: Option<bool>,

    /// When true, only inference backends that are validated by the AI Horde devs will serve this request. When False, non-validated backends will also be used which can increase speed but you may end up with unexpected results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validated_backends: Option<bool>,

    /// When True, allows slower workers to pick up this request. Disabling this incurs an extra kudos cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_workers: Option<bool>,

    /// When True, allows very slower workers to pick up this request. Use this when you don't mind waiting a lot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_slow_workers: Option<bool>,

    /// If the request is SFW, and the worker accidentally generates NSFW, it will send back a censored image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub censor_nsfw: Option<bool>,

    /// Specify up to 5 workers which are allowed to service this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workers: Option<Vec<String>>,

    /// If true, the worker list will be treated as a blacklist instead of a whitelist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_blacklist: Option<Vec<String>>,

    /// Specify which models are allowed to be used for this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<String>>,

    /// The Base64-encoded webp to use for img2img.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_image: Option<String>,

    /// If source_image is provided, specifies how to process it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_processing: Option<SourceProcessing>,

    /// If source_processing is set to 'inpainting' or 'outpainting', this parameter can be optionally provided as the Base64-encoded webp mask of the areas to inpaint. If this arg is not passed, the inpainting/outpainting mask has to be embedded as alpha channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_mask: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_source_images: Option<Vec<ExtraSourceImage>>,

    /// If True, the image will be sent via cloudflare r2 download link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r2: Option<bool>,

    /// If True, The image will be shared with LAION for improving their dataset. This will also reduce your kudos consumption by 2. For anonymous users, this is always True.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,

    /// If enabled, suspicious prompts are sanitized through a string replacement filter instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_filter: Option<bool>,

    /// When true, the endpoint will simply return the cost of the request in kudos and exit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,

    /// If using a service account as a proxy, provide this value to identify the actual account from which this request is coming from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxied_account: Option<String>,

    /// When true, This request will not use batching. This will allow you to retrieve accurate seeds. Feature is restricted to Trusted users and Patreons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_batching: Option<bool>,

    /// When true and the request requires upfront kudos and the account does not have enough The request will be downgraded in steps and resolution so that it does not need upfront kudos.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_downgrade: Option<bool>,

    /// Provide a URL where the AI Horde will send a POST call after each delivered generation. The request will include the details of the job as well as the request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,

    /// A horde style ID or name to use for this generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RequestSingleWarning {
    /// A unique identifier for this warning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<RequestWarningCode>,

    /// Something that you should be aware about this request, in plain text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RequestAsync {
    /// The UUID of the request. Use this to retrieve the request status in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The expected kudos consumption for this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kudos: Option<f64>,

    /// Any extra information from the horde about this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<RequestSingleWarning>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GenerationMetadataStable {
    /// The relevance of the metadata field
    #[serde(rename = "type")]
    pub metadata_type: MetadataType,

    /// The value of the metadata field
    #[serde(rename = "value")]
    pub metadata_value: MetadataValue,

    /// Optionally a reference for the metadata (e.g. a lora ID)
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub metadata_ref: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GenerationStable {
    /// The UUID of the worker which generated this image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,

    /// The name of the worker which generated this image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_name: Option<String>,

    /// The model which generated this image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// OBSOLETE (Use the gen_metadata field). The state of this generation.
    pub state: GenerationState,

    /// The generated image as a Base64-encoded .webp file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<String>,

    /// The seed which generated this image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<String>,

    /// The ID for this image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// When true this image has been censored by the worker's safety filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub censored: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gen_metadata: Option<Vec<GenerationMetadataStable>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RequestStatusCheck {
    /// The amount of finished jobs in this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<u8>,

    /// The amount of still processing jobs in this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing: Option<u8>,

    /// The amount of jobs that timed out and had to be restarted or were reported as failed by a worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restarted: Option<u8>,

    /// The amount of jobs waiting to be picked up by a worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiting: Option<u8>,

    /// True when all jobs in this request are done. Else False.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,

    /// True when this request caused an internal server error and could not be completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faulted: Option<bool>,

    /// The expected amount to wait (in seconds) to generate all jobs in this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_time: Option<u16>,

    /// The position in the requests queue. This position is determined by relative Kudos amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_position: Option<u16>,

    /// The amount of total Kudos this request has consumed until now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kudos: Option<f32>,

    /// If False, this request will not be able to be completed with the pool of workers currently available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_possible: Option<bool>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RequestStatusStable {
    /// The amount of finished jobs in this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<u8>,

    /// The amount of still processing jobs in this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing: Option<u8>,

    /// The amount of jobs that timed out and had to be restarted or were reported as failed by a worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restarted: Option<u8>,

    /// The amount of jobs waiting to be picked up by a worker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiting: Option<u8>,

    /// True when all jobs in this request are done. Else False.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,

    /// True when this request caused an internal server error and could not be completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faulted: Option<bool>,

    /// The expected amount to wait (in seconds) to generate all jobs in this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_time: Option<u16>,

    /// The position in the requests queue. This position is determined by relative Kudos amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_position: Option<u16>,

    /// The amount of total Kudos this request has consumed until now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kudos: Option<f32>,

    /// If False, this request will not be able to be completed with the pool of workers currently available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_possible: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub generations: Option<Vec<GenerationStable>>,

    /// If True, These images have been shared with LAION.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ActiveModel {

    /// The Name of a model available by workers in this horde.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// How many of workers in this horde are running this model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,

    /// The average speed of generation for this model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance: Option<f64>,

    /// The amount waiting to be generated by this model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued: Option<f64>,

    /// The job count waiting to be generated by this model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<f64>,

    /// Estimated time in seconds for this model's queue to be cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eta: Option<u64>,

    /// The model type (text or image).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<ModelType>,
}