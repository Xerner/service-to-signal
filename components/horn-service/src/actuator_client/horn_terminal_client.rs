use async_trait::async_trait;
use log::info;

use crate::actuator_client::HornActuatorClient;

#[derive(Default)]
pub struct TerminalClient {}

#[async_trait]
impl HornActuatorClient for TerminalClient {
    async fn set_horn_active(&self, is_active: bool) {
        let is_active_str = if is_active { "active" } else { "inactive" };
        info!("{}", is_active_str);
    }
}
