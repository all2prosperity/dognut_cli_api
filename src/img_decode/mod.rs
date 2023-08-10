use std::thread::JoinHandle;
use std::time::Instant;
use log::{error, info};
use protobuf::Message;
use turbojpeg::PixelFormat;
use crate::pb;

extern crate turbojpeg;

pub struct ImgDecoder {

    frame_rx: crossbeam_channel::Receiver<Vec<u8>>,
    rgb_tx: crossbeam_channel::Sender<Vec<u8>>,
    dimension: (u32, u32),
}



impl ImgDecoder {
    pub fn new(frame_rx: crossbeam_channel::Receiver<Vec<u8>>, rgb_tx: crossbeam_channel::Sender<Vec<u8>>, dimension: (u32, u32)) -> Self {
        Self { frame_rx, rgb_tx, dimension }
    }


    pub fn run(frame_rx: crossbeam_channel::Receiver<Vec<u8>>, rgb_tx: crossbeam_channel::Sender<Vec<u8>>, dimension: (u32, u32)) -> JoinHandle<()> {
        let handle = std::thread::Builder::new().name("dognut_image_decoding".into()).spawn(move || {
            let decoder = Self::new(frame_rx, rgb_tx, dimension);
            decoder.run_decoding_pipeline();
            return ();
        }).unwrap();

        return handle;
    }

    pub fn run_decoding_pipeline(self) {
        loop {
            if let Ok(data) = self.frame_rx.recv() {
                let now = Instant::now();
                let vid_packet = pb::avpacket::VideoPacket::parse_from_bytes(data.as_slice()).unwrap();

                let rgba = turbojpeg::decompress(vid_packet.data.as_slice(), PixelFormat::RGBA).unwrap();
                //info!("decoded frame width {}, height {}, cost {}", rgba.width, rgba.height, now.elapsed().as_millis());
                if self.rgb_tx.send(rgba.pixels).is_err() {
                    error!("send rgba data error");
                    break;
                }
            }
        }
    }

}
