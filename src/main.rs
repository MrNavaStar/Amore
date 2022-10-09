use std::borrow::Borrow;
use poem::{get, handler, listener::TcpListener, Route, Server, IntoResponse, EndpointExt};
use poem::web::{Path, Json, Data, websocket::WebSocket};
use poem::web::websocket::Message;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Tank<> {
    name: String
}

#[handler]
fn get_tanks() -> &'static str {
    "Here is a list of tanks"
}

#[handler]
fn get_tank(Path(name): Path<String>) -> String {
     format!("Here is tank {}", name)
}

#[handler]
fn get_tank_devices(Path(name): Path<String>) -> String {
    format!("Here is a list of devices attached to tank {}", name)
}

#[handler]
fn put_tank(tank: Json<Tank>) -> Json<Tank> {
    tank
}

#[handler]
fn ws(ws: WebSocket, sender: Data<&tokio::sync::broadcast::Sender<String>>) -> impl IntoResponse {

    let mut receiver = sender.subscribe();
    let sender = sender.clone();
    ws.on_upgrade(move |socket| async move {
        let mut stream = socket;

        tokio::spawn(async move {
            while let Some(Ok(msg)) = stream.next().await {
                if let Message::Text(text) = msg {
                    if sender.send(format!("{}", text)).is_err() {
                        break;
                    }
                }
            }
        });
    })
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/", get(ws.data(tokio::sync::broadcast::channel::<String>(32).0)))
        .at("/tanks", get(get_tanks).put(put_tank))
        .at("/tanks/:name", get(get_tank))
        .at("/tanks/:name/devices", get(get_tank_devices));

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}