use std::error::Error;

mod connections;    use connections::connect_to;
mod ls_connect;     use ls_connect::{ServerKind, connect_to_any_ls};
mod ac_extensions;  use ac_extensions::AdnlClientExt;


fn main() -> Result<(), Box<dyn Error>> {
  let mut client = connect_to("chain-arrow-viewer.ton".to_owned())?;
  
  Ok(())
  
  /*
  let mut client = connect_to_any_ls(ServerKind::LiteServerMainnet)?;

  // already serialized TL with gettime query
  let mut query = *b"z\xf9\x8b\xb45&>l\x95\xd6\xfe\xcbI}\xfd\n\xa5\xf01\xe7\xd4\x12\x98k\\\xe7 Im\xb5\x12\x05.\x8f-\x10\x0c\xdf\x06\x8cy\x044Z\xad\x16\x00\x00\x00\x00\x00\x00";

  client.send_rand_nonce(&mut query)?;
  
  let mut result: Vec<u8> = Vec::new();
  client.receive_se::<_, 8192>(&mut result)?;

  // get time from serialized TL answer
  println!(
    "received: {}",
    u32::from_le_bytes(result[result.len() - 7..result.len() - 3].try_into()?)
  );
  Ok(())
  */
}
