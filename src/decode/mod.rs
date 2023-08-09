pub mod encode;

use std::thread::JoinHandle;
use crossbeam_channel::{select};
use ffmpeg_next as ffmpeg;
//use ffmpeg_next as ffmpeg;

use ffmpeg::decoder::video;

use ffmpeg::software::scaling;
use ffmpeg::{codec, Packet};
use ffmpeg::codec:: Parameters;

use ffmpeg::codec::Id::H264;

use ffmpeg::format::{Pixel};
use ffmpeg::frame::Video;
use ffmpeg::software::scaling::Flags;
use log::{error, info};
use std::time::Duration;
use ffmpeg::log::Level;
//use ffmpeg_next::log::Level;
use ffmpeg::ffi::*;
use protobuf::Message;
use tokio::fs::File;
use crate::pb;


pub struct RgbaDecoder {
    frame_rx: crossbeam_channel::Receiver<Vec<u8>>,
    rgb_tx: crossbeam_channel::Sender<Vec<u8>>,
    decoder: ffmpeg::decoder::video::Video,
    scale_ctx: scaling::Context,
    dimension: (u32, u32),
    index: i32,
}


impl RgbaDecoder {

    pub fn run(frame_rx: crossbeam_channel::Receiver<Vec<u8>>, rgb_tx: crossbeam_channel::Sender<Vec<u8>>, dimension:(u32, u32)) -> JoinHandle<()>{
        let handle = std::thread::spawn(move ||  {
            let encoder = unsafe {Self::new(rgb_tx, frame_rx, dimension).expect("ffmpeg encoder init failed") };
            unsafe {
                encoder.run_decoding_pipeline();
            }
            return ();
        });

        return handle;
    }

    pub unsafe fn new(tx: crossbeam_channel::Sender<Vec<u8>>, rx: crossbeam_channel::Receiver<Vec<u8>>,  dimension: (u32, u32)) -> Result<Self, ffmpeg::Error> {
        let codec = codec::decoder::find(H264).expect("can't find h264 encoder");

        let context = codec::context::Context::new();
        let video = context.decoder().open_as(codec).unwrap();
        let scaler = scaling::Context::get(Pixel::YUVJ420P, dimension.0, dimension.1,
                                           Pixel::RGBA, dimension.0, dimension.1, Flags::BILINEAR)?;

        Ok(Self {
            rgb_tx: tx,
            frame_rx: rx,
            decoder:video.video().unwrap(),
            dimension,
            scale_ctx: scaler,
            index: 0,
        })
    }

    pub fn send_packets(&mut self, net_data: &Vec<u8>) -> Result<(), ffmpeg::Error> {
        let net_packet = pb::avpacket::VideoPacket::parse_from_bytes(net_data.as_slice()).unwrap();

        let mut packet= Packet::copy(net_packet.data.as_slice());
        packet.set_dts(Some(net_packet.dts));
        packet.set_pts(Some(net_packet.pts));
        packet.set_duration(net_packet.duration);

        self.decoder.send_packet(&packet)?;
        //todo: encoder wait on another thread to recv encoded data and send to network;
        Ok(())
    }


    unsafe fn unwrap_avframe_to_rgba(&mut self, frame: &Video) -> Vec<u8> {
        let mut rgb_frame =  Video::empty();
        self.scale_ctx.run(frame, &mut rgb_frame).unwrap();
        save_file(&rgb_frame, self.index as usize).unwrap();
        self.index +=1;
        //if self.index == 0 {

//            self.index += 1;
//        }
        rgb_frame.data(0).to_vec()
    }

    pub unsafe fn run_decoding_pipeline(mut self) {
        let mut frame = Video::empty();
        let mut index = 0;
        loop {
            select! {
                recv(self.frame_rx) -> data =>  {
                    match data {
                        Ok(data) => {
                            if data.is_empty() {
                                continue;
                            }
                            self.send_packets(&data).expect("must send ok");
                            info!("received network packet and send to decoder, index {}", index);
                        }
                        Err(err) => {
                            error!("frame buffer data recv error {:?}", err.to_string());
                            break;
                        }
                    }
                },
                default(Duration::from_millis(1)) => (),
            }

            while self.decoder.receive_frame(&mut frame).is_ok() {
                info!("received decoded frame index {}, is key? {}", index, frame.is_key());
                index += 1;

                let data = self.unwrap_avframe_to_rgba(&frame);
                if self.rgb_tx.send(data).is_err() {
                    break;
                }
            }
        }
        info!("decoder thread quit");
    }
}

fn save_file(frame: &Video, index: usize) -> Result<(), std::io::Error> {
    if index > 0 {
        return Ok(());
    }
    let name = format!("frame{}.png", index);
    image::save_buffer(name, frame.data(0), frame.width(), frame.height(), image::ColorType::Rgba8).unwrap();
//
   //let mut file = File::create(format!("frame{}.ppm", index))?;
   //file.write_all(format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes())?;
   //file.write_all(frame.data(0))?;
    Ok(())
}
