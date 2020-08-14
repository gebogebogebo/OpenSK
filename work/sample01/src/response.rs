#[cfg(feature = "with_ctap2_1")]
use super::data_formats::{AuthenticatorTransport, PublicKeyCredentialParameter};

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(any(test, feature = "debug_ctap"), derive(Debug))]
pub enum ResponseData {
    AuthenticatorGetInfo(AuthenticatorGetInfoResponse),
}

#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(any(test, feature = "debug_ctap"), derive(Debug))]
pub struct AuthenticatorGetInfoResponse {
    // TODO(kaczmarczyck) add maxAuthenticatorConfigLength and defaultCredProtect
    pub versions: Vec<String>,
    pub extensions: Option<Vec<String>>,
    pub aaguid: [u8; 16],
    //pub options: Option<BTreeMap<String, bool>>,
    pub max_msg_size: Option<u64>,
    pub pin_protocols: Option<Vec<u64>>,
}
