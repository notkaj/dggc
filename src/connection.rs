use futures_util::StreamExt;
use tokio::{
    // io::AsyncReadExt,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tokio_tungstenite::connect_async;

pub struct Connection {
    handle: JoinHandle<()>,
    receiver: UnboundedReceiver<Vec<u8>>,
}

impl Connection {
    pub async fn new() -> Connection {
        let (tx, rx) = mpsc::unbounded_channel();
        let handle = Self::connect(tx).await;
        Connection {
            handle,
            receiver: rx,
        }
    }

    pub async fn capture(&mut self) -> Vec<Vec<u8>> {
        let mut capture = Vec::new();
        while let Ok(c) = self.receiver.try_recv() {
            capture.push(c);
        }
        capture
    }

    // pub fn capture_sync(&mut self) -> Vec<Vec<u8>> {
    //     tokio::runtime::Builder::new_multi_thread()
    //         .enable_all()
    //         .build()
    //         .unwrap()
    //         .block_on(async { self.capture().await })
    // }

    async fn connect(tx: UnboundedSender<Vec<u8>>) -> JoinHandle<()> {
        let connect_addr = String::from("wss://chat.destiny.gg/ws");

        let url = url::Url::parse(&connect_addr).unwrap();

        // let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
        // tokio::spawn(read_stdin(stdin_tx));

        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        // println!("WebSocket handshake has been successfully completed");

        let (_write, read) = ws_stream.split();
        // let stdin_to_ws = stdin_rx.map(Ok).forward(write);

        tokio::spawn(async move {
            // let atx = Arc::new(tx);
            let tx_owned = tx;
            read.for_each(|message| async {
                let data = match message {
                    Ok(m) => m.into_data(),
                    Err(error) => format!("ERROR CLIENT: {:?}", error).as_bytes().to_vec(),
                };
                // let result = Arc::clone(&atx).send(data);
                let result = tx_owned.send(data);
                match result {
                    Ok(_) => (),
                    Err(e) => println!("{}", e),
                }
            })
            .await
        })

        // self.handle = Some(handle);
        // pin_mut!(stdin_to_ws, ws_to_stdout);
        // future::select(stdin_to_ws, ws_to_stdout).await;
    }
}

// async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
//     let mut stdin = tokio::io::stdin();
//     loop {
//         let mut buf = vec![0; 1024];
//         let n = match stdin.read(&mut buf).await {
//             Ok(n) => n,
//             Err(_) | Ok(0) => break,
//         };
//         buf.truncate(n);
//         tx.unbounded_send(Message::binary(buf)).unwrap();
//     }
// }
