use async_trait::async_trait;
use http::Uri;
use kuksa_rust_sdk::{
    kuksa::{common::ClientTraitV1, val::v1::KuksaClient},
    v1_proto,
};
use log::{error, info};
use std::{collections::HashMap, time::SystemTime};
use tokio::sync::Mutex;

use crate::actuator_client::HornActuatorClient;

pub struct HornKuksaClient {
    client: Mutex<KuksaClient>,
}

impl HornKuksaClient {
    pub fn new(kuksa_uri: Uri) -> Self {
        let client = HornKuksaClient::create_kuksa_client(&kuksa_uri);
        HornKuksaClient {
            client: Mutex::new(client),
        }
    }

    fn create_kuksa_client(kuksa_uri: &Uri) -> KuksaClient {
        info!("Connecting to Kuksa Databroker [{0}]", kuksa_uri);
        return KuksaClient::new(kuksa_uri.clone());
    }

    fn create_datapoint(is_active: bool) -> v1_proto::Datapoint {
        v1_proto::Datapoint {
            timestamp: Some(prost_types::Timestamp::from(SystemTime::now())),
            value: Some(v1_proto::datapoint::Value::Bool(is_active)),
        }
    }
}

#[async_trait]
impl HornActuatorClient for HornKuksaClient {
    async fn set_horn_active(&self, is_active: bool) {
        let datapoints = HashMap::from([(
            "Vehicle.Body.Horn.IsActive".to_string(),
            HornKuksaClient::create_datapoint(is_active),
        )]);
        let mut client = self.client.lock().await;
        if let Err(e) = client.set_target_values(datapoints).await {
            error!("Failed to send the Horn signal to Kuksa Databroker: {e}");
        }
        info!(
            "Horn signal sent to Kuksa Databroker: is_active = {}",
            is_active
        );
    }
}
