use std::{collections::HashMap, fmt::Write, io::Read, sync::Arc};

use tokio::{net::unix::SocketAddr, sync::Mutex};
use tokio_stream::{self as stream, StreamExt};
use vexv5_serial::{
    devices::{asyncdevice::AsyncDevice, VexDevice},
    file::{FTInit, FTInitResponse, FTWrite, GetFileMetadataByName},
    v5::{
        FileTransferFunction, FileTransferOptions, FileTransferTarget, FileTransferType,
        FileTransferVID,
    },
    VEX_CRC32,
};

type Tx = tokio::sync::mpsc::UnboundedSender<String>;
type Rx = tokio::sync::mpsc::UnboundedReceiver<String>;

pub struct Device {
    device: AsyncDevice<tokio_serial::SerialStream, tokio_serial::SerialStream>,
    recievers: HashMap<SocketAddr, Tx>,
}

impl Device {
    pub fn new(vex_device: VexDevice) -> Arc<Mutex<Device>> {
        let device = Self {
            device: vex_device.open_async().unwrap(),
            recievers: HashMap::new(),
        };

        let device = Arc::new(Mutex::new(device));
        let device_clone = Arc::clone(&device);

        tokio::spawn(async move {});

        device
    }

    pub async fn upload(&mut self, file_name: String, data: Vec<u8>) {
        // convert filename to bytes
        let mut file_name_bytes: [u8; 24] = [0; 24];
        for (i, byte) in file_name.as_bytes().iter().enumerate() {
            if i + 1 > 24 {
                break;
            }

            file_name_bytes[i] = *byte;
        }

        // grab metadata
        let metadata = self
            .device
            .send_request(GetFileMetadataByName(
                &file_name_bytes,
                FileTransferVID::User,
                FileTransferOptions::NONE,
            ))
            .await;

        // grab address from metadata
        let addr = match metadata {
            Ok(metadata) => metadata.addr,
            Err(_) => 0,
        };

        // create the vex CRC32
        let v5crc = crc::Crc::<u32>::new(&VEX_CRC32);

        // start the file transfer
        let ftm = self.device.send_request(FTInit {
            function: FileTransferFunction::Upload,
            target: FileTransferTarget::Flash,
            vid: FileTransferVID::User,
            options: FileTransferOptions::OVERWRITE,
            length: data.len() as u32,
            addr: 0x3800000,
            crc: v5crc.checksum(&data),
            file_type: FileTransferType::Ini,
            timestamp: 0,
            version: 1,
            name: file_name_bytes,
        });

        tracing::debug!("File transfer handle received");

        let ctx = async move |ftm: FTInitResponse, device: &mut Device<S, U>| -> Result<Vec<u8>> {
            // Get 3/4 of the max packet size to prevent packet headers from overflowing
            let max_size = ftm.max_packet_size / 2 + (ftm.max_packet_size / 4);

            // Use the length of the file in the metadata
            // unless vector is smaller than file size
            let size = std::cmp::min(ftm.file_size as u32, data.len() as u32);

            // keep track of what we've sent
            let mut sent: usize = 0;

            // iterate over the data
            for i in (0..size).step_by(max_size.into()) {
                // Determine the packet size. We do not want to write max_size if we're at the end of the file
                let packet_size = if size < max_size as u32 {
                    size as u16
                } else if i as u32 + max_size as u32 > size {
                    (size - i as u32) as u16
                } else {
                    max_size
                };

                // cut out the packet_size bytes from the provided buffer
                let payload = &data[i as usize..i as usize + packet_size as usize];

                // Write the payload to the file
                self.device.send_request(FTWrite(i + addr, payload)).await;

                // TODO: update progress

                sent += packet_size as usize;
            }
        };

        // let mut device = self.device;
        // Ok(());
    }
}
