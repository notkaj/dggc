use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

mod data;
mod display;

#[tokio::main]
async fn main() {
    println!("Connecting to dgg chat");
    dggc().await;
}

async fn dggc() {
    // let connect_addr =
    //     env::args().nth(1).unwrap_or_else(|| panic!("this program requires at least one argument"));

    let connect_addr = String::from("wss://chat.destiny.gg/ws");

    let url = url::Url::parse(&connect_addr).unwrap();

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = match message {
                Ok(message) => message.into_data(),
                Err(error) => format!("ERROR CLIENT: {:?}", error).as_bytes().to_vec()
            };
            data::process(data.as_slice()).await;
//            let mut stdout = tokio::io::stdout();
//            stdout.write_all(&data).await.unwrap();
//            stdout.write_u8(b'\n').await.unwrap();
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).unwrap();
    }
}
