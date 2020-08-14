mod status_code;
mod response;

use status_code::Ctap2StatusCode;
use response::ResponseData;

fn main() {
    println!("Hello, world!");

    let cbor_bytes = [Command::AUTHENTICATOR_GET_INFO];
    println!("cobr_bytes[0] = {}",cbor_bytes[0]);

    let cmd = Command::deserialize(&cbor_bytes);
    match cmd {
        Ok(command) => {

            let response = match command {
                Command::AuthenticatorGetInfo => process_get_info(),
                _ => process_unknown_command(),
            };

            /*
            #[cfg(feature = "debug_ctap")]
            writeln!(&mut Console::new(), "Sending response: {:#?}", response).unwrap();
            match response {
                Ok(response_data) => {
                    let mut response_vec = vec![0x00];
                    if let Some(value) = response_data.into() {
                        if !cbor::write(value, &mut response_vec) {
                            response_vec = vec![
                                Ctap2StatusCode::CTAP2_ERR_VENDOR_RESPONSE_CANNOT_WRITE_CBOR
                                    as u8,
                            ];
                        }
                    }
                    response_vec
                }
                Err(error_code) => vec![error_code as u8],
            }
            */
        }
        Err(error_code) => ()
    }
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


pub enum Command {
    AuthenticatorGetInfo,
}

impl Command {
    
    const AUTHENTICATOR_GET_INFO: u8 = 0x04;

    pub fn deserialize(bytes: &[u8]) -> Result<Command, Ctap2StatusCode> {
        if bytes.is_empty() {
            // The error to return is not specified, missing parameter seems to fit best.
            return Err(Ctap2StatusCode::CTAP2_ERR_MISSING_PARAMETER);
        }

        let command_value = bytes[0];
        match command_value {
            Command::AUTHENTICATOR_GET_INFO => {
                // Parameters are ignored.
                Ok(Command::AuthenticatorGetInfo)
            }
            _ => Err(Ctap2StatusCode::CTAP1_ERR_INVALID_COMMAND),
        }
    }
    
}

/*
mod test {
    use super::*;

    fn test_deserialize_get_info() {
        let cbor_bytes = [Command::AUTHENTICATOR_GET_INFO];
        //let command = Command::deserialize(&cbor_bytes);
        //assert_eq!(command, Ok(Command::AuthenticatorGetInfo));
    }
}
*/
