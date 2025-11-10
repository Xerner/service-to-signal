use async_trait::async_trait;

#[async_trait]
pub trait HornActuatorClient: Send + Sync {
    async fn set_horn_active(&self, is_active: bool);
}
