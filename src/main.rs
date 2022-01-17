mod flower;

use crate::flower::*;

use btleplug::api::{BDAddr, Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::Manager;
use futures::stream::StreamExt;
use std::error::Error;
use std::time::Duration;
use tokio::time;

fn is_flower_care_device(address: BDAddr) -> bool {
    address.to_string().starts_with("C4:7C:8D:")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    let adapters = match manager.adapters().await {
        Ok(adapters) => adapters,
        Err(e) => {
            panic!("Failed to get adapters: {}", e);
        }
    };

    let central = adapters.into_iter().next().unwrap();
    let mut events = central.events().await.unwrap();

    // start scanning for devices
    if central.start_scan(ScanFilter::default()).await.is_err() {
        panic!("Failed to start scan");
    }

    while let Some(event) = events.next().await {
        match event {
            CentralEvent::DeviceDiscovered(device) => {
                let peripheral = central.peripheral(&device).await.unwrap();
                if is_flower_care_device(peripheral.address()) {
                    tokio::spawn(async move {
                        println!("Discovered device: {:?}", device);
                        let flower = Flower::new(peripheral);
                        time::sleep(Duration::from_secs(5)).await;
                        match flower.connect_with_retry(5).await {
                            Ok(()) => {}
                            Err(e) => {
                                println!("Failed to connect to device: {:?}", e);
                                return;
                            }
                        };
                        let battery = flower.battery().await.unwrap();
                        println!("Battery: {}", battery);
                        let version = flower.version().await.unwrap();
                        println!("Version: {}", version);
                        flower.disconnect().await.unwrap();
                    });
                }
            }
            _ => {}
        }
    }
    central.stop_scan().await?;
    Ok(())
}
