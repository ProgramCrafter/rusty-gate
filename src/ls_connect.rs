use std::net::{SocketAddrV4, TcpStream};
use std::error::Error;

use adnl::{AdnlBuilder, AdnlClient};
use x25519_dalek::StaticSecret;

#[non_exhaustive]
pub enum ServerKind {
  LiteServerMainnet,
  LiteServerTestnet
}

pub fn connect(remote_public: [u8; 32], ls_ip: &str, ls_port: u16) ->
    Result<AdnlClient<TcpStream>, Box<dyn Error>> {
  // generate private key
  let local_secret = StaticSecret::new(rand::rngs::OsRng);

  // use TCP over IPv4 as a transport for our ADNL connection
  let transport = TcpStream::connect(SocketAddrV4::new(ls_ip.parse()?, ls_port))?;

  // build handshake using random session keys
  // encrypt it with ECDH(local_secret, remote_public)
  // then perform handshake over our TcpStream
  let client = AdnlBuilder::with_random_aes_params(&mut rand::rngs::OsRng)
      .perform_ecdh(local_secret, remote_public)
      .perform_handshake(transport)
      .map_err(|e| format!("{:?}", e))?;
  Ok(client)
}

pub fn connect_to_any_ls(kind: ServerKind) -> Result<AdnlClient<TcpStream>, Box<dyn Error>> {
  match kind {
    ServerKind::LiteServerMainnet => {
      connect(
        *b"\xc2\xb4\x1ax\x81b\xb2\x93\xdf\x1ewD\x05\xcd,\xd1\xfc{\x19*|PQoy\xb9\xf2\xb5\x04\x1fs\xa0",
        "65.21.141.233", 30131)
    },
    ServerKind::LiteServerTestnet => {
      connect(
        *b"\xa7kR\x89\xa7\x92\xa9\x7f{\xf0\x1cD\xe72\xf1\xb94\x0c\xd3\xa5\x95\x0cJ\xdf\xe7\xfdyA:11\x80",
        "65.108.204.54", 29296)
    },
    _ => {
      Err(Box::<dyn Error>::from("no stored connection info for such server kind".to_owned()))
    }
  }
}
