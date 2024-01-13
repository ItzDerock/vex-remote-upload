use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::Mutex;
use vexv5_serial::devices::{genericv5::find_generic_devices, VexDevice, VexDeviceType};

pub struct DeviceManager {
    pub devices: Vec<VexDevice>,
}

impl DeviceManager {
    /// Scans for new devices and adds them to the list of devices
    pub fn scan(&mut self) {
        println!("Scanning for devices...");
        match find_generic_devices() {
            Ok(ports) => {
                println!("🔍 Found {} devices", ports.len());

                for port in ports {
                    if self
                        .devices
                        .iter()
                        .find(|device| device.system_port == port.system_port)
                        .is_none()
                    {
                        let clone = port.clone();

                        println!(
                            "👀 Found new device on sys port: {}, user port: {}: {}",
                            port.system_port,
                            port.user_port.unwrap_or("None".to_string()),
                            match port.device_type {
                                VexDeviceType::Brain => "Brain",
                                VexDeviceType::Controller => "Controller",
                                VexDeviceType::Unknown => "Unknown",
                            }
                        );

                        self.devices.push(clone);
                    }
                }
            }

            Err(err) => {
                println!("Error scanning for devices: {}", err);
            }
        }
    }

    pub fn new() -> Arc<Mutex<DeviceManager>> {
        let manager = Arc::new(Mutex::new(Self {
            devices: Vec::new(),
        }));

        // in the background, scan for devices every 5 seconds
        let manager_clone = Arc::clone(&manager);
        tokio::spawn(async move {
            loop {
                let mut manager = manager_clone.lock().await;
                manager.scan();
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        });

        manager
    }
}

lazy_static! {
    pub static ref DEVICE_MANAGER: Arc<Mutex<DeviceManager>> = DeviceManager::new();
}
