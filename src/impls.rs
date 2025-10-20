use std::fmt;

use crate::enums::{
    ControlType, InjectTi, InterrogationType, MetadataType, MetadataValue, ModelState, ModelType,
    PostProcessing, RequestErrorCode, SamplerName, SourceProcessing, StyleType, Workflow,
};

impl Default for SamplerName {
    fn default() -> Self {
        SamplerName::KEulerA
    }
}

impl Default for Workflow {
    fn default() -> Self {
        Workflow::QrCode
    }
}

impl Default for SourceProcessing {
    fn default() -> Self {
        SourceProcessing::Img2img
    }
}

impl Default for PostProcessing {
    fn default() -> Self {
        PostProcessing::Codeformers
    }
}

impl Default for InjectTi {
    fn default() -> Self {
        InjectTi::Prompt
    }
}

impl Default for ControlType {
    fn default() -> Self {
        ControlType::Normal
    }
}

impl Default for InterrogationType {
    fn default() -> Self {
        InterrogationType::Caption
    }
}

impl Default for ModelType {
    fn default() -> Self {
        ModelType::Image
    }
}

impl Default for MetadataType {
    fn default() -> Self {
        MetadataType::Lora
    }
}

impl Default for MetadataValue {
    fn default() -> Self {
        MetadataValue::DownloadFailed
    }
}

impl Default for StyleType {
    fn default() -> Self {
        StyleType::Image
    }
}

impl Default for ModelState {
    fn default() -> Self {
        ModelState::Known
    }
}

impl fmt::Display for RequestErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestErrorCode::MissingPrompt => write!(f, "MissingPrompt"),
            RequestErrorCode::CorruptPrompt => write!(f, "CorruptPrompt"),
            RequestErrorCode::KudosValidationError => write!(f, "KudosValidationError"),
            RequestErrorCode::NoValidActions => write!(f, "NoValidActions"),
            RequestErrorCode::InvalidSize => write!(f, "InvalidSize"),
            RequestErrorCode::InvalidPromptSize => write!(f, "InvalidPromptSize"),
            RequestErrorCode::TooManySteps => write!(f, "TooManySteps"),
            RequestErrorCode::Profanity => write!(f, "Profanity"),
            RequestErrorCode::ProfaneWorkerName => write!(f, "ProfaneWorkerName"),
            RequestErrorCode::ProfaneBridgeAgent => write!(f, "ProfaneBridgeAgent"),
            RequestErrorCode::ProfaneWorkerInfo => write!(f, "ProfaneWorkerInfo"),
            RequestErrorCode::ProfaneUserName => write!(f, "ProfaneUserName"),
            RequestErrorCode::ProfaneUserContact => write!(f, "ProfaneUserContact"),
            RequestErrorCode::ProfaneAdminComment => write!(f, "ProfaneAdminComment"),
            RequestErrorCode::ProfaneTeamName => write!(f, "ProfaneTeamName"),
            RequestErrorCode::ProfaneTeamInfo => write!(f, "ProfaneTeamInfo"),
            RequestErrorCode::TooLong => write!(f, "TooLong"),
            RequestErrorCode::TooLongWorkerName => write!(f, "TooLongWorkerName"),
            RequestErrorCode::TooLongUserName => write!(f, "TooLongUserName"),
            RequestErrorCode::NameAlreadyExists => write!(f, "NameAlreadyExists"),
            RequestErrorCode::WorkerNameAlreadyExists => write!(f, "WorkerNameAlreadyExists"),
            RequestErrorCode::TeamNameAlreadyExists => write!(f, "TeamNameAlreadyExists"),
            RequestErrorCode::PolymorphicNameConflict => write!(f, "PolymorphicNameConflict"),
            RequestErrorCode::ImageValidationFailed => write!(f, "ImageValidationFailed"),
            RequestErrorCode::SourceImageResolutionExceeded => {
                write!(f, "SourceImageResolutionExceeded")
            }
            RequestErrorCode::SourceImageSizeExceeded => write!(f, "SourceImageSizeExceeded"),
            RequestErrorCode::SourceImageUrlInvalid => write!(f, "SourceImageUrlInvalid"),
            RequestErrorCode::SourceImageUnreadable => write!(f, "SourceImageUnreadable"),
            RequestErrorCode::InpaintingMissingMask => write!(f, "InpaintingMissingMask"),
            RequestErrorCode::SourceMaskUnnecessary => write!(f, "SourceMaskUnnecessary"),
            RequestErrorCode::UnsupportedSampler => write!(f, "UnsupportedSampler"),
            RequestErrorCode::UnsupportedModel => write!(f, "UnsupportedModel"),
            RequestErrorCode::ControlNetUnsupported => write!(f, "ControlNetUnsupported"),
            RequestErrorCode::ControlNetSourceMissing => write!(f, "ControlNetSourceMissing"),
            RequestErrorCode::ControlNetInvalidPayload => write!(f, "ControlNetInvalidPayload"),
            RequestErrorCode::SourceImageRequiredForModel => {
                write!(f, "SourceImageRequiredForModel")
            }
            RequestErrorCode::UnexpectedModelName => write!(f, "UnexpectedModelName"),
            RequestErrorCode::TooManyUpscalers => write!(f, "TooManyUpscalers"),
            RequestErrorCode::ProcGenNotFound => write!(f, "ProcGenNotFound"),
            RequestErrorCode::InvalidAestheticAttempt => write!(f, "InvalidAestheticAttempt"),
            RequestErrorCode::AestheticsNotCompleted => write!(f, "AestheticsNotCompleted"),
            RequestErrorCode::AestheticsNotPublic => write!(f, "AestheticsNotPublic"),
            RequestErrorCode::AestheticsDuplicate => write!(f, "AestheticsDuplicate"),
            RequestErrorCode::AestheticsMissing => write!(f, "AestheticsMissing"),
            RequestErrorCode::AestheticsSolo => write!(f, "AestheticsSolo"),
            RequestErrorCode::AestheticsConfused => write!(f, "AestheticsConfused"),
            RequestErrorCode::AestheticsAlreadyExist => write!(f, "AestheticsAlreadyExist"),
            RequestErrorCode::AestheticsServerRejected => write!(f, "AestheticsServerRejected"),
            RequestErrorCode::AestheticsServerError => write!(f, "AestheticsServerError"),
            RequestErrorCode::AestheticsServerDown => write!(f, "AestheticsServerDown"),
            RequestErrorCode::AestheticsServerTimeout => write!(f, "AestheticsServerTimeout"),
            RequestErrorCode::InvalidAPIKey => write!(f, "InvalidAPIKey"),
            RequestErrorCode::WrongCredentials => write!(f, "WrongCredentials"),
            RequestErrorCode::NotAdmin => write!(f, "NotAdmin"),
            RequestErrorCode::NotModerator => write!(f, "NotModerator"),
            RequestErrorCode::NotOwner => write!(f, "NotOwner"),
            RequestErrorCode::NotPrivileged => write!(f, "NotPrivileged"),
            RequestErrorCode::AnonForbidden => write!(f, "AnonForbidden"),
            RequestErrorCode::AnonForbiddenWorker => write!(f, "AnonForbiddenWorker"),
            RequestErrorCode::AnonForbiddenUserMod => write!(f, "AnonForbiddenUserMod"),
            RequestErrorCode::NotTrusted => write!(f, "NotTrusted"),
            RequestErrorCode::UntrustedTeamCreation => write!(f, "UntrustedTeamCreation"),
            RequestErrorCode::UntrustedUnsafeIP => write!(f, "UntrustedUnsafeIP"),
            RequestErrorCode::WorkerMaintenance => write!(f, "WorkerMaintenance"),
            RequestErrorCode::WorkerFlaggedMaintenance => write!(f, "WorkerFlaggedMaintenance"),
            RequestErrorCode::TooManySameIPs => write!(f, "TooManySameIPs"),
            RequestErrorCode::WorkerInviteOnly => write!(f, "WorkerInviteOnly"),
            RequestErrorCode::UnsafeIP => write!(f, "UnsafeIP"),
            RequestErrorCode::TimeoutIP => write!(f, "TimeoutIP"),
            RequestErrorCode::TooManyNewIPs => write!(f, "TooManyNewIPs"),
            RequestErrorCode::KudosUpfront => write!(f, "KudosUpfront"),
            RequestErrorCode::SharedKeyEmpty => write!(f, "SharedKeyEmpty"),
            RequestErrorCode::InvalidJobID => write!(f, "InvalidJobID"),
            RequestErrorCode::RequestNotFound => write!(f, "RequestNotFound"),
            RequestErrorCode::WorkerNotFound => write!(f, "WorkerNotFound"),
            RequestErrorCode::TeamNotFound => write!(f, "TeamNotFound"),
            RequestErrorCode::FilterNotFound => write!(f, "FilterNotFound"),
            RequestErrorCode::UserNotFound => write!(f, "UserNotFound"),
            RequestErrorCode::DuplicateGen => write!(f, "DuplicateGen"),
            RequestErrorCode::AbortedGen => write!(f, "AbortedGen"),
            RequestErrorCode::RequestExpired => write!(f, "RequestExpired"),
            RequestErrorCode::TooManyPrompts => write!(f, "TooManyPrompts"),
            RequestErrorCode::NoValidWorkers => write!(f, "NoValidWorkers"),
            RequestErrorCode::MaintenanceMode => write!(f, "MaintenanceMode"),
            RequestErrorCode::TargetAccountFlagged => write!(f, "TargetAccountFlagged"),
            RequestErrorCode::SourceAccountFlagged => write!(f, "SourceAccountFlagged"),
            RequestErrorCode::FaultWhenKudosReceiving => write!(f, "FaultWhenKudosReceiving"),
            RequestErrorCode::FaultWhenKudosSending => write!(f, "FaultWhenKudosSending"),
            RequestErrorCode::TooFastKudosTransfers => write!(f, "TooFastKudosTransfers"),
            RequestErrorCode::KudosTransferToAnon => write!(f, "KudosTransferToAnon"),
            RequestErrorCode::KudosTransferToSelf => write!(f, "KudosTransferToSelf"),
            RequestErrorCode::KudosTransferNotEnough => write!(f, "KudosTransferNotEnough"),
            RequestErrorCode::NegativeKudosTransfer => write!(f, "NegativeKudosTransfer"),
            RequestErrorCode::KudosTransferFromAnon => write!(f, "KudosTransferFromAnon"),
            RequestErrorCode::InvalidAwardUsername => write!(f, "InvalidAwardUsername"),
            RequestErrorCode::KudosAwardToAnon => write!(f, "KudosAwardToAnon"),
            RequestErrorCode::NotAllowedAwards => write!(f, "NotAllowedAwards"),
            RequestErrorCode::NoWorkerModSelected => write!(f, "NoWorkerModSelected"),
            RequestErrorCode::NoUserModSelected => write!(f, "NoUserModSelected"),
            RequestErrorCode::NoHordeModSelected => write!(f, "NoHordeModSelected"),
            RequestErrorCode::NoTeamModSelected => write!(f, "NoTeamModSelected"),
            RequestErrorCode::NoFilterModSelected => write!(f, "NoFilterModSelected"),
            RequestErrorCode::NoSharedKeyModSelected => write!(f, "NoSharedKeyModSelected"),
            RequestErrorCode::BadRequest => write!(f, "BadRequest"),
            RequestErrorCode::Forbidden => write!(f, "Forbidden"),
            RequestErrorCode::Locked => write!(f, "Locked"),
            RequestErrorCode::ControlNetInpaintingMismatch => {
                write!(f, "ControlNetInpaintingMismatch")
            }
            RequestErrorCode::HiResFixMismatch => write!(f, "HiResFixMismatch"),
            RequestErrorCode::TooManyLoras => write!(f, "TooManyLoras"),
            RequestErrorCode::BadLoraVersion => write!(f, "BadLoraVersion"),
            RequestErrorCode::TooManyTIs => write!(f, "TooManyTIs"),
            RequestErrorCode::BetaAnonForbidden => write!(f, "BetaAnonForbidden"),
            RequestErrorCode::BetaComparisonFault => write!(f, "BetaComparisonFault"),
            RequestErrorCode::BadCFGDecimals => write!(f, "BadCFGDecimals"),
            RequestErrorCode::BadCFGNumber => write!(f, "BadCFGNumber"),
            RequestErrorCode::BannedClientAgent => write!(f, "BannedClientAgent"),
            RequestErrorCode::SpecialMissingPayload => write!(f, "SpecialMissingPayload"),
            RequestErrorCode::SpecialForbidden => write!(f, "SpecialForbidden"),
            RequestErrorCode::SpecialMissingUsername => write!(f, "SpecialMissingUsername"),
            RequestErrorCode::SpecialModelNeedsSpecialUser => {
                write!(f, "SpecialModelNeedsSpecialUser")
            }
            RequestErrorCode::SpecialFieldNeedsSpecialUser => {
                write!(f, "SpecialFieldNeedsSpecialUser")
            }
            RequestErrorCode::Img2ImgMismatch => write!(f, "Img2ImgMismatch"),
            RequestErrorCode::TilingMismatch => write!(f, "TilingMismatch"),
            RequestErrorCode::ControlNetMismatch => write!(f, "ControlNetMismatch"),
            RequestErrorCode::HiResMismatch => write!(f, "HiResMismatch"),
            RequestErrorCode::EducationCannotSendKudos => write!(f, "EducationCannotSendKudos"),
            RequestErrorCode::InvalidPriorityUsername => write!(f, "InvalidPriorityUsername"),
            RequestErrorCode::SharedKeyExpired => write!(f, "SharedKeyExpired"),
            RequestErrorCode::SharedKeyInsufficientKudos => write!(f, "SharedKeyInsufficientKudos"),
            RequestErrorCode::OnlyServiceAccountProxy => write!(f, "OnlyServiceAccountProxy"),
            RequestErrorCode::RequiresTrust => write!(f, "RequiresTrust"),
            RequestErrorCode::Unknown => write!(f, "Unknown"),
        }
    }
}
