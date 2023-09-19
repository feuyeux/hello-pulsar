#![feature(future_join)]

use std::env;
use pulsar::authentication::oauth2::OAuth2Authentication;
use pulsar::message::proto;
use pulsar::{Error, producer, Producer};
use pulsar::Authentication;
use pulsar::Pulsar;
use pulsar::TokioExecutor;
use pulsar::Consumer;
use pulsar::SubType;
use futures::TryStreamExt;


#[tokio::main]
async fn main() -> Result<(), pulsar::Error> {
    env_logger::init();

    let addr = env::var("PULSAR_ADDRESS")
        .ok()
        .unwrap_or_else(|| "pulsar://127.0.0.1:6650".to_string());

    let mut builder = Pulsar::builder(addr, TokioExecutor);

    if let Ok(token) = env::var("PULSAR_TOKEN") {
        let authentication = Authentication {
            name: "token".to_string(),
            data: token.into_bytes(),
        };
        builder = builder.with_auth(authentication);
    } else if let Ok(oauth2_cfg) = env::var("PULSAR_OAUTH2") {
        builder = builder.with_auth_provider(OAuth2Authentication::client_credentials(
            serde_json::from_str(oauth2_cfg.as_str())
                .unwrap_or_else(|_| panic!("invalid oauth2 config [{}]", oauth2_cfg.as_str())),
        ));
    }

    let pulsar: Pulsar<_> = builder.build().await?;

    let mut producer = pulsar
        .producer()
        .with_topic("hello_topic")
        .with_name("rust_producer")
        .with_options(producer::ProducerOptions {
            schema: Some(proto::Schema {
                r#type: proto::schema::Type::String as i32,
                ..Default::default()
            }),
            ..Default::default()
        })
        .build()
        .await?;

    let mut consumer: Consumer<String, _> = pulsar
        .consumer()
        .with_topic("hello_topic")
        .with_consumer_name("rust_consumer")
        .with_subscription_type(SubType::Exclusive)
        .with_subscription("sub_rust")
        .build()
        .await?;

    send(&mut producer).await?;
    receive(&mut consumer).await?;
    log::info!("Bye");
    Ok(())
}

async fn receive(consumer: &mut Consumer<String, TokioExecutor>) -> Result<(), Error> {
    let mut counter = 0usize;
    while let Some(msg) = consumer.try_next().await? {
        consumer.ack(&msg).await?;
        log::debug!("metadata: {:?}", msg.metadata());
        let data = match msg.deserialize() {
            Ok(data) => data,
            Err(e) => {
                log::error!("could not deserialize message: {:?}", e);
                break;
            }
        };
        counter += 1;
        let msg_id = msg.message_id();
        log::info!("Message received: [{:?}:{:?},{}]", msg_id.ledger_id,msg_id.entry_id,data);
        if counter > 10 {
            consumer.close().await.expect("Unable to close consumer");
            break;
        }
    }
    Ok(())
}

async fn send(producer: &mut Producer<TokioExecutor>) -> Result<(), Error> {
    let mut counter = 0usize;
    loop {
        producer
            .send(format!("Hello-Rust-{}", counter))
            .await?
            .await
            .unwrap();
        counter += 1;
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        if counter > 10 {
            producer.close().await.expect("Unable to close connection");
            break;
        }
    }
    Ok(())
}