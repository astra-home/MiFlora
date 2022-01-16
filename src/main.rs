use btleplug::api::{
    BDAddr, Central, CentralEvent, CharPropFlags, Characteristic, Manager as _, Peripheral as _,
    ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::stream::StreamExt;
use std::error::Error;
use std::str;
use std::time::Duration;
use tokio::time;

fn is_flower_care_device(address: BDAddr) -> bool {
    address.to_string().starts_with("C4:7C:8D:")
}

enum FlowerCharacteristic {
    Version,
    Battery,
    RealTime,
    Mode,
}

struct Flower {
    peripheral: Peripheral,
}

impl Flower {
    pub fn new(peripheral: Peripheral) -> Flower {
        Flower { peripheral }
    }

    pub async fn connect(&self) -> Result<(), btleplug::Error> {
        match self.peripheral.connect().await {
            Ok(()) => {
                match self.peripheral.discover_services().await {
                    Ok(()) => Ok(()),
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }

    pub async fn connect_with_retry(&self, attempts: u8) -> Result<(), &'static str> {
        for attempt in 1..(attempts + 1) {
            println!("Connecting to flower care device attempt {}", attempt);
            if self.connect().await.is_ok() {
                return Ok(());
            }
            time::sleep(Duration::from_secs(5)).await;
        }

        Err("Failed to connect to flower care device")
    }

    fn characteristic(&self, flower_char: FlowerCharacteristic) -> Characteristic {
        let characteristics = self.peripheral.characteristics();
        match flower_char {
            FlowerCharacteristic::Version => characteristics
                .iter()
                .find(|c| c.uuid.to_string() == "00001a02-0000-1000-8000-00805f9b34fb")
                .unwrap()
                .to_owned(),
            FlowerCharacteristic::Battery => characteristics
                .iter()
                .find(|c| c.uuid.to_string() == "00001a02-0000-1000-8000-00805f9b34fb")
                .unwrap()
                .to_owned(),
            FlowerCharacteristic::RealTime => characteristics
                .iter()
                .find(|c| c.uuid.to_string() == "00001a01-0000-1000-8000-00805f9b34fb")
                .unwrap()
                .to_owned(),
            FlowerCharacteristic::Mode => characteristics
                .iter()
                .find(|c| c.uuid.to_string() == "00001a00-0000-1000-8000-00805f9b34fb")
                .unwrap()
                .to_owned(),
        }
    }

    pub async fn battery(&self) -> Result<u8, &'static str> {
        let char = self.characteristic(FlowerCharacteristic::Battery);
        let battery = self.peripheral.read(&char).await.unwrap();
        Ok(battery[0])
    }

    pub async fn version(&self) -> Result<String, &'static str> {
        let char = self.characteristic(FlowerCharacteristic::Version);
        let version = self.peripheral.read(&char).await.unwrap();
        Ok(str::from_utf8(&version[2..]).unwrap().to_string())
    }

    pub async fn real_time_read(&self) -> Result<(), &'static str> {
        let mode_char = self.characteristic(FlowerCharacteristic::Mode);
        self.peripheral
            .write(&mode_char, &[0xA0, 0x1F], WriteType::WithResponse)
            .await
            .unwrap();
        let char = self.characteristic(FlowerCharacteristic::RealTime);
        self.peripheral.subscribe(&char).await.unwrap();
        let mut notification_stream = self.peripheral.notifications().await.unwrap();
        while let Some(data) = notification_stream.next().await {
            if data
                .uuid
                .to_string()
                .eq("00001a01-0000-1000-8000-00805f9b34fb")
            {
                println!("Temperature: {:?}°C", (data.value[0] as f32) / 10.0);
                println!(
                    "Sunlight: {:?} LUX",
                    i32::from_le_bytes(data.value[3..=6].try_into().unwrap())
                );
                println!("Moisture: {:?}%", data.value[7]);
                println!(
                    "Fertilization: {:?}µS/cm",
                    u16::from_le_bytes(data.value[8..=9].try_into().unwrap())
                );
                println!("---");
            } else {
                println!("Unknown notification: {:?}", data);
            }
        }
        Ok(())
    }

    pub async fn disconnect(&self) -> Result<(), &'static str> {
        if self.peripheral.disconnect().await.is_err() {
            return Err("Failed to disconnect from flower care device");
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn read_all(&self) -> Result<(), Box<dyn Error>> {
        let chars = self.peripheral.characteristics();
        for char in chars.iter() {
            println!("{}", char);
            if char.properties.contains(CharPropFlags::READ) {
                let data = self.peripheral.read(char).await.unwrap();
                println!("{:?}", data);
            }
        }
        Ok(())
    }
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
                            Ok(()) => {},
                            Err(e) => {
                                println!("Failed to connect to device: {:?}", e);
                                return;
                            },
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
