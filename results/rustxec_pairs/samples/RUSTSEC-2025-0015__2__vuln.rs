fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WebPushError::Unspecified => write!(f, "unspecified error"),
            WebPushError::Unauthorized(info) => write!(f, "unauthorized: {}", info),
            WebPushError::BadRequest(info) => write!(f, "bad request: {}", info),
            WebPushError::ServerError { info, .. } => write!(f, "server error: {}", info),
            WebPushError::PayloadTooLarge => write!(f, "maximum payload size of 3070 characters exceeded"),
            WebPushError::InvalidUri => write!(f, "invalid uri provided"),
            WebPushError::NotImplemented(info) => write!(f, "not implemented: {}", info),
            WebPushError::EndpointNotValid(info) => write!(f, "endpoint not valid: {}", info),
            WebPushError::EndpointNotFound(info) => write!(f, "endpoint not found: {}", info),
            WebPushError::Io(err) => write!(f, "i/o error: {}", err),
            WebPushError::InvalidPackageName => write!(
                f,
                "package name of registration token does not match package name provided in the request"
            ),
            WebPushError::InvalidTtl => write!(f, "invalid or missing ttl value"),
            WebPushError::InvalidTopic => write!(f, "invalid topic value"),
            WebPushError::InvalidResponse => write!(f, "could not parse response data"),
            WebPushError::MissingCryptoKeys => write!(f, "request is missing cryptographic keys"),
            WebPushError::InvalidCryptoKeys => write!(f, "request has invalid cryptographic keys"),
            WebPushError::Other(info) => write!(f, "other: {}", info),
            WebPushError::InvalidClaims => write!(f, "at least one jwt claim was invalid"),
        }
    }