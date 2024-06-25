use axum::{extract::Path, routing::{get, put}, Json};
use axum_server::tls_rustls::RustlsConfig;
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use serde_json::{json, Map, Value};
use std::time::Duration;

const C8Y_JWT_RESPONSE: &str = "71,eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tokio::spawn(run_http_server());

    let mqtt_options = MqttOptions::new("fake-c8y", "localhost", 1883);
    let (client, mut connection) = AsyncClient::new(mqtt_options, 10);
    client.subscribe("s/uat", QoS::AtLeastOnce).await.unwrap();
    client.subscribe("s/us", QoS::AtLeastOnce).await.unwrap();
    let mut ever_connected = false;
    loop {
        let notification = connection.poll().await;
        ever_connected |= notification.is_ok();
        match notification {
            Ok(Event::Incoming(Incoming::Publish(publish))) if publish.topic == "s/uat" => {
                client
                    .publish("s/dat", QoS::AtLeastOnce, false, C8Y_JWT_RESPONSE)
                    .await
                    .unwrap();
            }
            Ok(Event::Incoming(Incoming::Publish(publish))) if publish.topic == "s/us" => {
                let Ok(payload) = std::str::from_utf8(&publish.payload) else {
                    continue;
                };
                if payload.starts_with("100,") {
                    client
                        .publish(
                            "s/e",
                            QoS::AtLeastOnce,
                            false,
                            "41,100,Device already exists",
                        )
                        .await
                        .unwrap();
                }
            }
            Err(err) => {
                eprintln!("MQTT client error: {err}");
                if err.to_string().contains("Connection refused") && ever_connected {
                    std::process::exit(1);
                }
                std::thread::sleep(Duration::from_millis(500))
            }
            _ => (),
        }
    }
}

async fn run_http_server() {
    let app = axum::Router::<()>::new().route(
        "/identity/externalIds/c8y_Serial/*identity",
        get(internal_id),
    ).route(
        "/inventory/managedObjects/*id",
        put(specific_managed_object),
    );
    let rustls_config = RustlsConfig::from_pem_file("/certs/c8y.crt", "/certs/c8y.key")
        .await
        .unwrap();

    axum_server::bind_rustls("0.0.0.0:443".parse().unwrap(), rustls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn internal_id(Path(id): Path<String>) -> Json<serde_json::Value> {
    Json(json!({
        "managedObject": {
            "id": "12345",
        },
        "externalId": id
    }))
}

async fn specific_managed_object(Path(id): Path<String>, Json(mut body): Json<serde_json::Map<String, serde_json::Value>>) -> Json<serde_json::Value> {
    let mut map = Map::from_iter([("id".into(), Value::String(id)), ("c8y_IsDevice".into(), Value::Object(Map::new()))]);
    map.append(&mut body);
    Json(Value::Object(map))
}