use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::io::Result;

use std::io::ErrorKind;
use std::io::Error;

#[tokio::main]
async fn main() {
  // 7651 = zlib.crc32(b'adnl-proxy') % 10000
  let listener = TcpListener::bind("0.0.0.0:7651").await.unwrap();

  loop {
    let (socket, peer_addr) = listener.accept().await.unwrap();
    tokio::spawn(async move {
      println!("Started interaction with {}", peer_addr);
      match serve_proxy(socket).await {
        Ok(()) => println!("Finished interaction with {}", peer_addr),
        Err(e) => println!("Got a error {e:?} while handling request from {}", peer_addr)
      };
    });
  }
}

async fn respond_failure(mut wr: impl AsyncWriteExt+Unpin, reason: &str) -> Result<()> {
  wr.write_all(b"HTTP/1.0 500 Internal Server Error\r\n").await?;
  wr.write_all(b"Content-Type: text/plain\r\n").await?;
  wr.write_all(b"Connection: close\r\n\r\n").await?;
  wr.write_all(reason.as_bytes()).await?;
  
  Err(Error::new(ErrorKind::Other, reason))
}

async fn serve_proxy(mut socket: TcpStream) -> Result<()> {
  let (rd, mut wr) = socket.split();
  let mut rd = BufReader::new(rd);
  
  // Parsing the incoming request
  
  let mut proxy_to = String::new();
  rd.read_line(&mut proxy_to).await?;
  
  let request_params: Vec<&str> = proxy_to.split(' ').collect();
  let verb = request_params[0];
  if verb == "PROXY" {
    if request_params[1] != "TCP4" {
      return respond_failure(wr, "unsupported protocol to forward").await;
    }
    
    let dest_ip   = request_params[3];
    let dest_port = request_params[5];
    
    let dest_pt = match dest_port.parse() {
      Ok(port) => {port}
      Err(_)   => {return respond_failure(wr, "invalid destination port").await;}
    };
    
    let mut server_socket = TcpStream::connect((dest_ip, dest_pt)).await?;
    
    // copying buffer from `rd`
    server_socket.write_all(rd.buffer()).await?;
    drop(rd);
    
    tokio::io::copy_bidirectional(&mut socket, &mut server_socket).await?;
  } else if verb == "GET" {
    wr.write_all(b"HTTP/1.0 200 OK\r\n").await?;
    wr.write_all(b"Content-Type: text/plain\r\n").await?;
    wr.write_all(b"Connection: close\r\n\r\n").await?;
    wr.write_all( format!("proxy_to: {proxy_to:?}\r\n").as_bytes() ).await?;
    wr.write_all( format!("request:  {request_params:?}\r\n").as_bytes() ).await?;
  } else {
    respond_failure(wr, &format!("unexpected HTTP verb: {verb}")).await?;
  }
  
  Ok(())
}
