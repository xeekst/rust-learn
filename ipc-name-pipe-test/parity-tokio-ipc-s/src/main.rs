use futures::StreamExt;
use tokio::{io::{split, AsyncReadExt, AsyncWriteExt}};

use parity_tokio_ipc::{Endpoint, SecurityAttributes};

async fn run_server(path: String) {
	let mut endpoint = Endpoint::new(path);
	endpoint.set_security_attributes(SecurityAttributes::allow_everyone_create().unwrap());

	let incoming = endpoint.incoming().expect("failed to open new socket");
	futures::pin_mut!(incoming);

	while let Some(result) = incoming.next().await
	{
		match result {
			Ok(stream) => {
				let (mut reader, mut writer) = split(stream);

				tokio::spawn(async move {
					loop {
						let mut buf = [0u8; 4];
						let pong_buf = b"pong";
						if let Err(_) = reader.read_exact(&mut buf).await {
							println!("Closing socket");
							break;
						}
						if let Ok("ping") = std::str::from_utf8(&buf[..]) {
							println!("RECIEVED: PING");
							writer.write_all(pong_buf).await.expect("unable to write to socket");
							println!("SEND: PONG");
						}
					}
				});
			}
			_ => unreachable!("ideally")
		}
	};
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
	let path = "\\\\.\\pipe\\named.ipc".to_owned();
	run_server(path).await
}