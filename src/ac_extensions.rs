use std::fmt::Debug;

use ciborium_io::{Read, Write};
use adnl::AdnlClient;


/// Extension for AdnlClient allowing to skip nonce generation
pub trait AdnlClientExt {
  fn send_rand_nonce(&mut self, data: &mut [u8]) -> Result<(), String>;
  fn receive_se<C: Write+Debug, const BUFFER: usize>(&mut self, consumer: &mut C) -> Result<(), String>
      where <C as Write>::Error: Debug;
}

impl<T: Read+Write+Debug> AdnlClientExt for AdnlClient<T>
    where <T as Write>::Error: Debug, <T as Read>::Error: Debug {
  fn send_rand_nonce(&mut self, data: &mut [u8]) -> Result<(), String> {
    self.send(data, &mut rand::random()).map_err(|e| format!("{:?}", e))
  }
  fn receive_se<C: Write+Debug, const BUFFER: usize>(&mut self, consumer: &mut C) -> Result<(), String>
      where <C as Write>::Error: Debug {
    self.receive::<C, BUFFER>(consumer).map_err(|e| format!("{:?}", e))
  }
}
