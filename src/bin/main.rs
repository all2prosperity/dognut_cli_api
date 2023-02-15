extern crate core;

use std::{env, ptr, slice};
use std::io::ErrorKind;
use crossbeam_channel::select;
use ffmpeg_next::codec::Parameters;
use ffmpeg_next::ffi::{av_packet_unref, avformat_open_input, AVPacket};
use ffmpeg_next::format::input;
use pixels::{wgpu::Surface, Pixels};
use winit::{event::*, window::WindowBuilder};
use winit::event_loop::{ControlFlow, EventLoop};
use dognut_cli_lib::decode::rgbaDecoder;
use dognut_cli_lib::network;
use dognut_cli_lib::pb::avpacket::VideoPacket;

use ffmpeg_next::media::Type;
use ffmpeg_next::Packet;
use log::error;
use log::Level::Error;
use pixels::wgpu::{Color};
use pixels::SurfaceTexture;
use protobuf::{EnumOrUnknown, Message};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

use bytes::{BufMut, Buf};
use dognut_cli_lib::pb::netpacket::{NetPacket, PacketKind};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 360;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Pixels example")
        .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();

    let size = window.inner_size();
    let surface = SurfaceTexture::new(size.width,  size.height, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface).expect("Failed to create pixels");
    pixels.set_clear_color(Color::WHITE);
    //let pair = read_a_av_net_packet().unwrap();
    let (packet_tx, packet_rx) = crossbeam_channel::unbounded::<Vec<u8>>();
    let (net_tx, net_rx) = crossbeam_channel::unbounded::<Vec<u8>>();
    //let handle = rgbaDecoder::run_from_parameter(net_rx, packet_tx, (WIDTH, HEIGHT), pair.1);
    let handle = rgbaDecoder::run(net_rx, packet_tx, (WIDTH, HEIGHT));

    //let packet = pair.0.write_to_bytes().unwrap();

    //net_tx.send(packet).expect("should send ok");

    let handle = std::thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        let addr = env::args().nth(1).unwrap();
        rt.block_on(keep_reading_packet_from_net(net_tx, addr));
    });


    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }

        select! {
            recv(packet_rx) -> data => {
                match data {
                    Ok(data) => {
                        let mut buffer = pixels.get_frame_mut();
                        println!("data size is {}, buffer size is {}", data.len(), buffer.len());

                        buffer.copy_from_slice(data.as_slice());
                        //pixels.get_frame_mut().copy_from_slice(data.as_slice());
                        pixels.render().unwrap();
                    }
                    Err(err) => {
                        error!("Fuck error {:?}", err.to_string());
                    }
                }
            }
            default(tokio::time::Duration::from_millis(100)) => (),
        }

        //pixels.render().expect("Failed to render");
    });
}

fn read_a_av_packet() -> Option<(Packet, Parameters)> {
    if let Ok(mut ictx) = input(&env::args().nth(1).expect("Cannot open file.")) {
        let input = ictx.streams()
            .best(Type::Video)
            .ok_or(ffmpeg_next::Error::StreamNotFound).unwrap();

        let par = input.parameters();

        let video_stream_index = input.index();

        for (stream, packet) in ictx.packets() {
            if stream.index() == video_stream_index {
                let p = Packet::copy(packet.data().unwrap());
                return Some((p, par));
            }
        }
    }

    return None;
}


fn read_a_av_net_packet() -> Option<(VideoPacket, Parameters)> {
    if let Ok(mut ictx) = input(&env::args().nth(1).expect("Cannot open file.")) {
        let input = ictx.streams()
            .best(Type::Video)
            .ok_or(ffmpeg_next::Error::StreamNotFound).unwrap();

        let par = input.parameters();

        let video_stream_index = input.index();

        for (stream, packet) in ictx.packets() {
            if stream.index() == video_stream_index {
                let p = Packet::copy(packet.data().unwrap());
                let mut v = VideoPacket::new();
                v.data = p.data().unwrap().to_vec();
                v.data_len = v.data.len() as u32;
                v.dts = p.dts().unwrap_or(0);
                v.pts = p.pts().unwrap_or(0);
                v.duration = p.duration();
                v.flags = 0;
                return Some((v, par));
            }
        }
    }
    return None;
}

pub async fn keep_reading_packet_from_net(sender: crossbeam_channel::Sender<Vec<u8>>, addr: String) {
    let mut stream = TcpStream::connect(addr).await.unwrap();

    let mut length = vec![0u8;4];
    //let mut length = bytes::Bytes::from(length);

    loop {
        if let Err(es) = stream.read_exact(length.as_mut_slice()).await {
            if es.kind() == ErrorKind::UnexpectedEof {
                break;
            } else {
                continue;
            }
        };
        let mut len = bytes::Bytes::from(length.clone().to_vec());
        let size = len.get_u32();


        let mut buffer = vec![0u8;size as usize];

        if let Err(es) = stream.read_exact(&mut buffer).await {
            println!("error: !!! {}", es);
            break;
        }

        let packet = NetPacket::parse_from_bytes(buffer.as_slice()).unwrap();

        match packet.kind.unwrap() {
            PacketKind::VideoPacket => {
                sender.send(packet.data).unwrap();
            }
        }
    }
}