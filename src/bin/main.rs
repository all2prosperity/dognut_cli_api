extern crate core;

use std::{env};
use std::io::ErrorKind;
use crossbeam_channel::select;
use ffmpeg_next::codec::Parameters;
use ffmpeg_next::format::input;
use pixels::{Pixels};
use winit::{event::*, window::WindowBuilder};
use winit::event_loop::{ControlFlow, EventLoop};




use ffmpeg_next::media::Type;
use ffmpeg_next::Packet;
use log::error;
use pixels::wgpu::{Color};
use pixels::SurfaceTexture;
use protobuf::{EnumOrUnknown, Message};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

use bytes::Buf;
use ffmpeg_next as ffmpeg;

use dognut_cli_lib::pb::netpacket::{PacketKind,NetPacket};
use dognut_cli_lib::pb::avpacket::VideoPacket;


const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() {
    ffmpeg::init().unwrap();
    ffmpeg::log::set_level(ffmpeg::log::Level::Trace);

    let env = env_logger::Env::default();
    env_logger::Builder::from_env(env).target(env_logger::Target::Stdout).filter(Some("wgpu_core"), log::LevelFilter::Error).
        filter_level(log::LevelFilter::Info)
        .format_timestamp_millis()
        .init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Controlled Window")
        .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();

    let size = window.inner_size();
    let surface = SurfaceTexture::new(size.width, size.height, &window);

    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface).expect("Failed to create pixels");
    let format = pixels.surface_texture_format();
    println!("surface texture format is {:?}", format);

    pixels.set_clear_color(Color::WHITE);

    let (packet_tx, packet_rx) = crossbeam_channel::unbounded::<Vec<u8>>();
    let (net_tx, net_rx) = crossbeam_channel::unbounded::<Vec<u8>>();

    let (rgb_tx, rgb_rx) = crossbeam_channel::unbounded::<Vec<u8>>();

    //let pair = read_a_av_net_packet().unwrap(); // file
    //let handle = RgbaDecoder::run_from_parameter(net_rx, packet_tx, (WIDTH, HEIGHT), pair.1); // file
    //let packet = pair.0.write_to_bytes().unwrap();  // file
    //net_tx.send(packet).expect("should send ok");  // file

    let handle = dognut_cli_lib::decode::RgbaDecoder::run(net_rx, packet_tx, (WIDTH, HEIGHT)); // network

    //donut_cli_lib::decode::encode::RgbaEncoder::run(rgb_rx, net_tx, (WIDTH, HEIGHT));
    let handle = std::thread::spawn(move || { // network
        let rt = Runtime::new().unwrap(); // network
        let addr = env::args().nth(1).unwrap(); // network
        rt.block_on(keep_reading_packet_from_net(net_tx, addr)); // network
    }); // network


    let mut quit = false;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
                quit = true;
            }
            Event::WindowEvent {
                event,
                ..
            } => {
                match event {
                    WindowEvent::Resized(_) => {}
                    WindowEvent::Moved(_) => {}
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(k) = input.virtual_keycode {
                            if k == VirtualKeyCode::Q {
                                *control_flow = ControlFlow::Exit;
                                quit = true;
                            }
                        }
                    }
                    _ => {}
                }
                //window.request_redraw();
            }
            Event::RedrawRequested(_) => {}

            _ => (),
        }

        if quit {
            return ;
        }

        select! {
            recv(packet_rx) -> data => {
                match data {
                    Ok(data) => {
                        pixels.get_frame_mut().copy_from_slice(data.as_slice());
                        pixels.render().unwrap();
                    }
                    Err(err) => {
                        //error!("Fuck error {:?}", err.to_string());
                    }
                }
            }
            //default(tokio::time::Duration::from_millis(5)) => (),
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

    let mut length = vec![0u8; 4];
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


        let mut buffer = vec![0u8; size as usize];

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
