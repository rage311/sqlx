use bytes::{Buf, Bytes};

use crate::database::HasArguments;
use crate::error::Error;
use crate::io::Encode;
use crate::io::{BufExt, Decode};
use crate::mysql::protocol::auth::AuthPlugin;
use crate::mysql::protocol::Capabilities;

// https://dev.mysql.com/doc/dev/mysql-server/8.0.12/page_protocol_connection_phase_packets_protocol_auth_switch_request.html

#[derive(Debug)]
pub struct AuthSwitchRequest {
    pub plugin: AuthPlugin,
    pub data: Bytes,
}

impl Decode<'_> for AuthSwitchRequest {
    fn decode_with(mut buf: Bytes, _: ()) -> Result<Self, Error> {
        let header = buf.get_u8();
        if header != 0xfe {
            return Err(err_protocol!(
                "expected 0xfe (AUTH_SWITCH) but found 0x{:x}",
                header
            ));
        }

        let plugin = buf.get_str_nul()?.parse()?;
        let data = buf.get_bytes(buf.len());

        Ok(Self { plugin, data })
    }
}

#[derive(Debug)]
pub struct AuthSwitchResponse(pub Vec<u8>);

impl Encode<'_, Capabilities> for AuthSwitchResponse {
    fn encode_with(&self, buf: &mut Vec<u8>, _: Capabilities) {
        buf.extend_from_slice(&self.0);
    }
}
