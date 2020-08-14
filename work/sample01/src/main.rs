mod status_code;
mod response;
mod command;

use status_code::Ctap2StatusCode;
use response::ResponseData;
use command::Command;

fn main() {
    println!("Hello, world!");

    Command::test_deserialize_get_info();

}

fn process_unknown_command() -> Result<ResponseData, Ctap2StatusCode> {
    Err(Ctap2StatusCode::CTAP1_ERR_INVALID_COMMAND)
}

fn process_get_info() -> Result<ResponseData, Ctap2StatusCode> {
    Err(Ctap2StatusCode::CTAP2_ERR_MISSING_PARAMETER)

    /*
    let mut options_map = BTreeMap::new();
    // TODO(kaczmarczyck) add authenticatorConfig and credProtect options
    options_map.insert(String::from("rk"), true);
    options_map.insert(String::from("up"), true);
    options_map.insert(
        String::from("clientPin"),
        self.persistent_store.pin_hash().is_some(),
    );
    Ok(ResponseData::AuthenticatorGetInfo(
        AuthenticatorGetInfoResponse {
            versions: vec![
                #[cfg(feature = "with_ctap1")]
                String::from(U2F_VERSION_STRING),
                String::from(FIDO2_VERSION_STRING),
            ],
            extensions: Some(vec![String::from("hmac-secret")]),
            aaguid: *self.persistent_store.aaguid()?,
            options: Some(options_map),
            max_msg_size: Some(1024),
            pin_protocols: Some(vec![
                CtapState::<R, CheckUserPresence>::PIN_PROTOCOL_VERSION,
            ]),
            #[cfg(feature = "with_ctap2_1")]
            max_credential_count_in_list: MAX_CREDENTIAL_COUNT_IN_LIST.map(|c| c as u64),
            // You can use ENCRYPTED_CREDENTIAL_ID_SIZE here, but if your
            // browser passes that value, it might be used to fingerprint.
            #[cfg(feature = "with_ctap2_1")]
            max_credential_id_length: None,
            #[cfg(feature = "with_ctap2_1")]
            transports: Some(vec![AuthenticatorTransport::Usb]),
            #[cfg(feature = "with_ctap2_1")]
            algorithms: Some(vec![ES256_CRED_PARAM]),
            default_cred_protect: DEFAULT_CRED_PROTECT,
            #[cfg(feature = "with_ctap2_1")]
            firmware_version: None,
        },
    ))
    */
}

