use log::{debug, error};
use std::cell::RefCell;
use tokio::{
    select,
    sync::mpsc::{Receiver, Sender},
};

use horn_proto::{
    horn_service::ActivateHornRequest,
    horn_topics::{HornMode, HornSequence},
};

/**
 * Handles processing incoming [ActivateHornRequest] instances and relaying them to Kuksa.
 */
struct HornRequestKuksaRelay {
    rx_requests: RefCell<Receiver<Option<ActivateHornRequest>>>,
    tx_kuksa_horn_is_active: Sender<bool>,
}

impl HornRequestKuksaRelay {
    pub fn new(rx_requests: Receiver<Option<ActivateHornRequest>>, tx_kuksa: Sender<bool>) -> Self {
        HornRequestKuksaRelay {
            rx_requests: RefCell::new(rx_requests),
            tx_kuksa_horn_is_active: tx_kuksa,
        }
    }

    /**
     * Blocks until the request rx channel is closed.
     *
     * Listens to the activate horn request channel and sends the requests to Kuksa.
     * Stops when the request channel returns None.
     */
    pub(crate) async fn relay_requests_to_kuksa(&self) {
        while let Some(activate_horn_request) = self.rx_requests.borrow_mut().recv().await {
            let mut rx_requests = self.rx_requests.borrow_mut();
            // this select! macro will kill the request_apply functions execution if the
            // rx_request_channel receives a new request
            let _: Option<()> = select! {
                _ = rx_requests.recv() => None,
                _ = self.send_request(activate_horn_request) => None,
            };
        }
    }

    /**
     * Handles sending horn requests to Kuksa.
     *
     * - If the request is None, it deactivates the horn.
     * - If the request is Some, it checks the horn mode and sends the appropriate request to Kuksa
     */
    pub(crate) async fn send_request(&self, request: Option<ActivateHornRequest>) {
        if request.is_none() {
            // treat None as a signal to deactivate the horn
            self.send_deactivate_horn_request().await;
            return;
        }
        let request = request.unwrap();
        let horn_mode = request.mode.enum_value();
        if let Err(e) = horn_mode {
            error!("Error in Horn Mode value {:?}", e);
        };
        let mode = horn_mode.unwrap();
        match mode {
            HornMode::HM_SEQUENCED => {
                self.send_sequential_horn_request_to_kuksa(request).await;
            }
            HornMode::HM_CONTINUOUS => self.send_continuous_horn_request_to_kuksa().await,
            HornMode::HM_UNKNOWN => println!("Horn Mode: Unknown"),
            HornMode::HM_UNSPECIFIED => println!("Horn Mode: Unspecified"),
        };
    }

    /**
     * Sends a deactivate horn request to Kuksa.
     */
    pub async fn send_deactivate_horn_request(&self) {
        debug!("Sending deactivate horn request");
        let _ = self.tx_kuksa_horn_is_active.send(false).await;
    }

    /**
     * Sends a continuous horn request to Kuksa.
     */
    pub async fn send_continuous_horn_request_to_kuksa(&self) {
        debug!("Sending continuous horn request");
        let _ = self.tx_kuksa_horn_is_active.send(true).await;
    }

    /**
     * Sends sequential horn requests to Kuksa.
     *
     * For each sequence in the request, it iterates through the horn cycles,
     * activating and deactivating the horn based on the specified on and off times.
     */
    pub async fn send_sequential_horn_request_to_kuksa(&self, request: ActivateHornRequest) {
        debug!("Sending sequential horn request(s)");
        let sequences = request.command;
        for sequence in sequences {
            for cycle in sequence.horn_cycles {
                debug!("\nOn Time: {}, Off Time: {}", cycle.on_time, cycle.off_time);
                let _ = self.tx_kuksa_horn_is_active.send(true).await;
                let _ = tokio::time::sleep(std::time::Duration::from_millis(cycle.on_time as u64))
                    .await;
                let _ = self.tx_kuksa_horn_is_active.send(false).await;
                let _ = tokio::time::sleep(std::time::Duration::from_millis(cycle.off_time as u64))
                    .await;
            }
        }
    }
}
