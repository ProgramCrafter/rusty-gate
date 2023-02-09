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
  if verb == "CONNECT" {
    let dest = request_params[1];
    
    let (dest_server, dest_port) = match dest.split_once(':') {
      Some(v) => {v}
      None    => {return respond_failure(wr, &format!("failed to parse destination")).await}
    };
    
    if !dest_server.ends_with(".ton") {
      return respond_failure(wr, &format!("not a .ton server: {dest_server}")).await;
    }
    if dest_port != "80" && dest_port != "8080" && dest_port != "443" {
      return respond_failure(wr, "not a safe port (80/8080/443)").await;
    }
    if dest_port == "443" {
      // terminating connection without response
      return Err(Error::new(ErrorKind::Other, "TLS redirection breaks certificates chain"));
    }
    
    let server = format!("{dest_server}.run:{dest_port}");  // foundation.ton.run:443
    println!("Connecting to {server}");
    
    loop {
      proxy_to.clear();
      rd.read_line(&mut proxy_to).await?;
      println!("Skipping headers line {proxy_to:?}");
      if proxy_to == "\r\n" {break;}
    }
    
    wr.write_all(b"HTTP/1.0 200 Connection established\r\n\r\n").await?;
    
    let mut server_socket = TcpStream::connect(server).await?;
    
    // copying buffer from `rd`
    if rd.buffer().len() > 0 {
      server_socket.write_all(rd.buffer()).await?;
    }
    drop(rd); drop(wr);
    
    tokio::io::copy_bidirectional(&mut socket, &mut server_socket).await?;
  } else if verb == "GET" {
    let proto_dest_url = request_params[1];
    
    let dest_url = match proto_dest_url.strip_prefix("http://") {
      Some(v) => {v}
      None    => {return respond_failure(wr, "not http:// protocol").await}
    };
    
    let (server, url) = match dest_url.split_once('/') {
      Some(v) => {v}
      None    => {(dest_url, "")}
    };
    if server.contains(':') {
      return respond_failure(wr, "supplying custom ports is not supported").await;
    }
    
    let mut server = server.to_owned();
    if server.ends_with(".ton") {
      server += ".run";
    }
    
    println!("Connecting to {server:?} /{url}");
    
    let mut server_socket = TcpStream::connect(format!("{server}:80")).await?;
    server_socket.write_all(format!("GET /{url} HTTP/1.0\r\n").as_bytes()).await?;
    
    loop {
      proxy_to.clear();
      rd.read_line(&mut proxy_to).await?;
      
      if proxy_to.to_lowercase().starts_with("host:") {
        server_socket.write_all(proxy_to.trim_end().as_bytes()).await?;
        server_socket.write_all(b".run\r\n").await?;
      } else {
        server_socket.write_all(proxy_to.as_bytes()).await?;
      }
      
      if proxy_to == "\r\n" {break;}
    }
    
    // copying buffer from `rd`
    if rd.buffer().len() > 0 {
      server_socket.write_all(rd.buffer()).await?;
    }
    drop(rd); drop(wr);
    
    tokio::io::copy_bidirectional(&mut socket, &mut server_socket).await?;
  } else {
    respond_failure(wr, &format!("unexpected HTTP verb: {verb}")).await?;
  }
  
  Ok(())
}
