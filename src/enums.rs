use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SamplerName {
    KDpmAdaptive,
    KEulerA,
    KDpmpp2M,
    #[serde(rename = "DDIM")]
    Ddim,
    KDpm2,
    KEuler,
    KDpmpp2SA,
    KLms,
    KDpmppSde,
    KDpm2A,
    KDpmFast,
    KHeun,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostProcessing {
    #[serde(rename = "GFPGAN")]
    Gfpgan,
    #[serde(rename = "RealESRGAN_x4plus")]
    RealEsrganX4plus,
    #[serde(rename = "RealESRGAN_x2plus")]
    RealEsrganX2plus,
    #[serde(rename = "RealESRGAN_x4plus_anime_6B")]
    RealEsrganX4plusAnime6B,
    #[serde(rename = "NMKD_Siax")]
    NmkdSiax,
    #[serde(rename = "4x_AnimeSharp")]
    FourXAnimeSharp,
    #[serde(rename = "CodeFormers")]
    Codeformers,
    StripBackground,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum RequestErrorCode {
    MissingPrompt,
    CorruptPrompt,
    KudosValidationError,
    NoValidActions,
    InvalidSize,
    InvalidPromptSize,
    TooManySteps,
    Profanity,
    ProfaneWorkerName,
    ProfaneBridgeAgent,
    ProfaneWorkerInfo,
    ProfaneUserName,
    ProfaneUserContact,
    ProfaneAdminComment,
    ProfaneTeamName,
    ProfaneTeamInfo,
    TooLong,
    TooLongWorkerName,
    TooLongUserName,
    NameAlreadyExists,
    WorkerNameAlreadyExists,
    TeamNameAlreadyExists,
    PolymorphicNameConflict,
    ImageValidationFailed,
    SourceImageResolutionExceeded,
    SourceImageSizeExceeded,
    SourceImageUrlInvalid,
    SourceImageUnreadable,
    InpaintingMissingMask,
    SourceMaskUnnecessary,
    UnsupportedSampler,
    UnsupportedModel,
    ControlNetUnsupported,
    ControlNetSourceMissing,
    ControlNetInvalidPayload,
    SourceImageRequiredForModel,
    UnexpectedModelName,
    TooManyUpscalers,
    ProcGenNotFound,
    InvalidAestheticAttempt,
    AestheticsNotCompleted,
    AestheticsNotPublic,
    AestheticsDuplicate,
    AestheticsMissing,
    AestheticsSolo,
    AestheticsConfused,
    AestheticsAlreadyExist,
    AestheticsServerRejected,
    AestheticsServerError,
    AestheticsServerDown,
    AestheticsServerTimeout,
    InvalidAPIKey,
    WrongCredentials,
    NotAdmin,
    NotModerator,
    NotOwner,
    NotPrivileged,
    AnonForbidden,
    AnonForbiddenWorker,
    AnonForbiddenUserMod,
    NotTrusted,
    UntrustedTeamCreation,
    UntrustedUnsafeIP,
    WorkerMaintenance,
    WorkerFlaggedMaintenance,
    TooManySameIPs,
    WorkerInviteOnly,
    UnsafeIP,
    TimeoutIP,
    TooManyNewIPs,
    KudosUpfront,
    SharedKeyEmpty,
    InvalidJobID,
    RequestNotFound,
    WorkerNotFound,
    TeamNotFound,
    FilterNotFound,
    UserNotFound,
    DuplicateGen,
    AbortedGen,
    RequestExpired,
    TooManyPrompts,
    NoValidWorkers,
    MaintenanceMode,
    TargetAccountFlagged,
    SourceAccountFlagged,
    FaultWhenKudosReceiving,
    FaultWhenKudosSending,
    TooFastKudosTransfers,
    KudosTransferToAnon,
    KudosTransferToSelf,
    KudosTransferNotEnough,
    NegativeKudosTransfer,
    KudosTransferFromAnon,
    InvalidAwardUsername,
    KudosAwardToAnon,
    NotAllowedAwards,
    NoWorkerModSelected,
    NoUserModSelected,
    NoHordeModSelected,
    NoTeamModSelected,
    NoFilterModSelected,
    NoSharedKeyModSelected,
    BadRequest,
    Forbidden,
    Locked,
    ControlNetInpaintingMismatch,
    HiResFixMismatch,
    TooManyLoras,
    BadLoraVersion,
    TooManyTIs,
    BetaAnonForbidden,
    BetaComparisonFault,
    BadCFGDecimals,
    BadCFGNumber,
    BannedClientAgent,
    SpecialMissingPayload,
    SpecialForbidden,
    SpecialMissingUsername,
    SpecialModelNeedsSpecialUser,
    SpecialFieldNeedsSpecialUser,
    Img2ImgMismatch,
    TilingMismatch,
    ControlNetMismatch,
    HiResMismatch,
    EducationCannotSendKudos,
    InvalidPriorityUsername,
    SharedKeyExpired,
    SharedKeyInsufficientKudos,
    OnlyServiceAccountProxy,
    RequiresTrust,
    #[default]
    #[serde(other)]
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ControlType {
    Canny,
    Hed,
    Depth,
    Normal,
    Openpose,
    Seg,
    Scribble,
    Fakescribbles,
    Hough,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Workflow {
    QrCode,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceProcessing {
    Img2img,
    Inpainting,
    Outpainting,
    Remix,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RequestWarningCode {
    NoAvailableWorker,
    ClipSkipMismatch,
    StepsTooFew,
    StepsTooMany,
    CfgScaleMismatch,
    CfgScaleTooSmall,
    CfgScaleTooLarge,
    SamplerMismatch,
    SchedulerMismatch,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InjectTi {
    Prompt,
    Negprompt,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum GenerationState {
    #[default]
    Ok,
    Censored,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetadataType {
    Lora,
    Ti,
    Censorship,
    SourceImage,
    SourceMask,
    ExtraSourceImages,
    BatchIndex,
    Information,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetadataValue {
    DownloadFailed,
    ParseFailed,
    BaselineMismatch,
    Csam,
    Nsfw,
    SeeRef,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterrogationType {
    Caption,
    Interrogation,
    Nsfw,
    StripBackground,
    #[serde(rename = "RealESRGAN_x4plus")]
    RealEsrganX4plus,
    #[serde(rename = "RealESRGAN_x2plus")]
    RealEsrganX2plus,
    #[serde(rename = "GFPGAN")]
    Gfpgan,
    #[serde(rename = "CodeFormers")]
    Codeformers,
    #[serde(rename = "RealESRGAN_x4plus_anime_6B")]
    RealEsrganX4plusAnime6B,
    #[serde(rename = "NMKD_Siax")]
    NmkdSiax,
    #[serde(rename = "4x_AnimeSharp")]
    FourXAnimeSharp,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelType {
    Image,
    Text,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StyleType {
    Image,
    Text,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelState {
    Known,
    Custom,
    All,
}
