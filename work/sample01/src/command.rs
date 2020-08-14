use std::fmt;

use super::status_code::Ctap2StatusCode;

// CTAP specification (version 20190130) section 6.1
pub enum Command {
    AuthenticatorGetInfo,
}

#[allow(dead_code)]
impl Command {
    const AUTHENTICATOR_MAKE_CREDENTIAL: u8 = 0x01;
    const AUTHENTICATOR_GET_ASSERTION: u8 = 0x02;
    const AUTHENTICATOR_GET_INFO: u8 = 0x04;
    const AUTHENTICATOR_CLIENT_PIN: u8 = 0x06;
    const AUTHENTICATOR_RESET: u8 = 0x07;
    const AUTHENTICATOR_GET_NEXT_ASSERTION: u8 = 0x08;
    const AUTHENTICATOR_VENDOR_FIRST: u8 = 0x40;
    const AUTHENTICATOR_VENDOR_LAST: u8 = 0xBF;

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

    pub fn test_deserialize_get_info() {
        let cbor_bytes = [Command::AUTHENTICATOR_GET_INFO];

        let command = Command::deserialize(&cbor_bytes);

        match command {
            Ok(v) => println!("ok value = {}", v),
            Err(e) => println!("err value = {}", e),
        };

        //assert_eq!(command, Ok(Command::AuthenticatorGetInfo));
    }

}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Command::AuthenticatorGetInfo => write!(f,"AuthenticatorGetInfo"),
        }        
    }
}
