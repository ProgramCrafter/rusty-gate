use std::io::{Read, Write};
use std::net::TcpStream;
use std::fmt::Debug;

use adnl::AdnlPublicKey;


/// Struct holding bag-of-files information sufficient to download it
pub struct BagOfFilesMeta {
  
}

/// Struct with fields necessary to set up WebRTC connection
pub struct RtcExchangeInfo {
  /// Server where offer must be sent and from which answer will be got back
  exchange_server_url: String
}

/// Ways to load .TON site
pub enum DomainConnectionType {
  WebTwoDirectHttp(String),    // host name only
  WebTwoDirectHttps(String),   // host name only
  StorageBof(BagOfFilesMeta),
  RldpHttpBasic(Box<dyn AdnlPublicKey>),
  
  DevHttpRtcSameSock(RtcExchangeInfo),
  DevAdnlHttpRtcSameSock(RtcExchangeInfo, Box<dyn AdnlPublicKey>),
  DevAdnlHttpRtcUpdating(RtcExchangeInfo, Box<dyn AdnlPublicKey>),
}
use DomainConnectionType::*;

pub fn resolve_domain(ton_domain: String) -> Result<DomainConnectionType, String> {
  if ton_domain != "chain-arrow-viewer.ton" {
    return Err("domain resolution not integrated yet".to_owned());
  }
  
  Ok(WebTwoDirectHttp(
    "ratingers.pythonanywhere.com".to_owned()
  ))
  
  /*
  Ok(DevHttpRtcSameSock(
    RtcExchangeInfo {
      exchange_server_url: "https://ratingers.pythonanywhere.com/webrtc-start/".to_owned()
    }
  ))
  */
}

fn split_stream(s: TcpStream) -> (Box<dyn Read>, Box<dyn Write>) {
  let copy = s.try_clone().unwrap();
  (Box::new(s), Box::new(copy))
}
fn err2s(e: impl std::error::Error) -> String {
  format!("{:?}", e)
}

pub fn connect_to(ton_domain: String) -> Result<(Box<dyn Read>, Box<dyn Write>), String> {
  let conn_type = resolve_domain(ton_domain)?;
  
  match conn_type {
    WebTwoDirectHttp(host)     => TcpStream::connect((host, 80)).map(split_stream).map_err(err2s),
    WebTwoDirectHttps(host)    => TcpStream::connect((host, 443)).map(split_stream).map_err(err2s),
    StorageBof(..)             => unimplemented!(),
    RldpHttpBasic(peer_key)    => unimplemented!(),
    
    DevHttpRtcSameSock(rtc_info) => {
      unimplemented!();
    }
    DevAdnlHttpRtcSameSock(..) => unimplemented!(),
    DevAdnlHttpRtcUpdating(..) => unimplemented!()
  }
}
