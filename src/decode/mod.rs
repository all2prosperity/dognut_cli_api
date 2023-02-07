
use std::ffi::c_int;
use std::thread::JoinHandle;
use crossbeam_channel::{select};
use ffmpeg_next as ffmpeg;

use ffmpeg::decoder::video;
use ffmpeg::codec;
use ffmpeg::ffi;
use ffmpeg::software::scaling;
use ffmpeg_next::{Codec, Frame, Packet};
use ffmpeg_next::codec::Context;

use ffmpeg_next::codec::Id::H264;

use ffmpeg_next::format::Pixel;
use ffmpeg_next::frame::Video;
use ffmpeg_next::software::scaling::Flags;
use log::{error, info};
use std::time::Duration;
use protobuf::Message;
use crate::pb;

pub struct rgbaDecoder {
    net_rx: crossbeam_channel::Receiver<Vec<u8>>,
    rgb_tx: crossbeam_channel::Sender<Vec<u8>>,
    decoder: video::Video,
    scale_ctx: scaling::Context,
    dimension: (u32, u32),
    codec: Codec,
}


impl rgbaDecoder {

    pub fn run(rgb_rx: crossbeam_channel::Receiver<Vec<u8>>, network_tx: crossbeam_channel::Sender<Vec<u8>>, dimension:(u32, u32)) -> JoinHandle<()>{
        let handle = std::thread::spawn(move ||  {
            let encoder = unsafe {Self::new(network_tx, rgb_rx, dimension).expect("ffmpeg encoder init failed") };
            unsafe {
                encoder.run_decoding_pipeline();
            }
            return ();
        });

        return handle;
    }


    pub unsafe fn new(tx: crossbeam_channel::Sender<Vec<u8>>, rx: crossbeam_channel::Receiver<Vec<u8>>,  dimension: (u32, u32)) -> Result<Self, ffmpeg::Error> {
        ffmpeg::init()?;
        let codec = codec::decoder::find(H264).expect("can't find h264 encoder");

        let context = Self::wrap_context(&codec, dimension);

        let video = context.decoder().open_as(codec).unwrap();
        //let decoder = video.open_as(codec)?;
        //let decoder = decoder.video()?;

        let scaler = scaling::Context::get(Pixel::YUV420P, dimension.0, dimension.1,
                                           Pixel::RGBA, dimension.0, dimension.1, Flags::BILINEAR)?;

        Ok(Self {
            rgb_tx: tx,
            net_rx: rx,
            decoder:video.video().unwrap(),
            dimension,
            scale_ctx: scaler,
            codec
        })
    }

    unsafe fn wrap_context(codec: &Codec, dimension:(u32, u32)) -> Context {
        let raw_codec = codec.as_ptr();
        let raw_context = ffi::avcodec_alloc_context3(raw_codec);
        (*raw_context).width = dimension.0 as c_int;
        (*raw_context).height = dimension.1 as c_int;
        (*raw_context).pix_fmt = ffi::AVPixelFormat::AV_PIX_FMT_YUV420P;
        (*raw_context).time_base = ffi::AVRational{num: 1, den: 60};
        (*raw_context).bit_rate = 4 * 1000 * 1000;
        (*raw_context).rc_buffer_size = 8 * 1000 * 1000;
        (*raw_context).rc_max_rate = 10 * 1000 * 1000;
        (*raw_context).rc_min_rate = 2 * 1000 * 1000;
        (*raw_context).framerate = ffi::AVRational{num:60, den: 1};
        // disable b frame for realtime streaming
        (*raw_context).max_b_frames = 0;
        (*raw_context).has_b_frames = 0;
        let mut k = std::ffi::CString::new("preset").unwrap();
        let mut v = std::ffi::CString::new("fast").unwrap();
        ffi::av_opt_set(raw_context as *mut _, k.as_ptr(), v.as_ptr(), 0);
        k = std::ffi::CString::new("x264-params").unwrap();
        v = std::ffi::CString::new("keyint=60:min-keyint=60:scenecut=0:force-cfr=1").unwrap();
        ffi::av_opt_set(raw_context as *mut _, k.as_ptr(), v.as_ptr(), 0);

        return Context::wrap(raw_context, None);
    }

    pub fn send_packets(&mut self, net_data: &Vec<u8>) -> Result<(), ffmpeg::Error> {

        let net_packet = pb::avpacket::VideoPacket::parse_from_bytes(net_data.as_slice()).unwrap();

        let mut packet= ffmpeg::Packet::copy(net_packet.data.as_slice());
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
        rgb_frame.data(0).to_vec()
    }

    pub unsafe fn run_decoding_pipeline(mut self) {
        let mut frame = Video::empty();
        loop {
            select! {
                recv(self.net_rx) -> data =>  {
                    match data {
                        Ok(data) => {
                            self.send_packets(&data).expect("must send ok");
                        }
                        Err(err) => {
                            error!("frame buffer data recv error {:?}", err.to_string());
                            break;
                        }
                    }
                },
                default(Duration::from_millis(500)) => (),
            }
            while self.decoder.receive_frame(&mut frame).is_ok() {
                let data = self.unwrap_avframe_to_rgba(&frame);
                if self.rgb_tx.send(data).is_err() {
                    break;
                }
            }
        }
        info!("encoder thread quit");
    }
}