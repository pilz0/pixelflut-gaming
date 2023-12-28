use std::net::{SocketAddr, TcpStream};
// use tokio::net::{TcpSocket, TcpStream};
use fontdue::Font;
use std::io::prelude::*;
use threadpool::ThreadPool;

const THREADS: i32 = 1;
const ADDR: &str = "151.217.15.90:1337";

// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     let font = Font::from_bytes(include_bytes!("../resources/font.ttf") as &[u8], fontdue::FontSettings::default()).unwrap();
//
//     let dims = (1280, 720);
//     // let dims = (100, 100);
//     let (width, height) = dims;
//     let buf = &mut vec![];
//
//     for (x,y) in iterate_pixels(dims) {
//         buf.extend([
//             // x pos
//             (0xff) as u8,
//             // y pos
//             (0xff) as u8,
//             0x00u8,
//             0xff,
//         ]);
//     }
//
//     write_to_threads(buf, dims).await?;
//
//     Ok(())
// }

async fn gen_threads() -> std::io::Result<Vec<TcpStream>> {
    let mut streams: Vec<TcpStream> = Vec::new();
    // for _ in 0..THREADS {
    //     let addr = ADDR.parse().unwrap();
    //
    //     let socket = TcpSocket::new_v4()?;
    //     let stream = socket.connect(addr).await?;
    //
    //     streams.push(stream);
    // }
    Ok(streams)
}

fn get_buf_dim(idx: usize, dims: (usize, usize)) -> (usize, usize) {
    let (width, height) = dims;
    let x = (idx / 4) % width;
    let y = (idx / 4) / width;
    (x, y)
}

fn get_buf_idx(x: usize, y: usize, dims: (usize, usize)) -> usize {
    let (width, _) = dims;
    4 * (x + y * width)
}

fn connect() -> TcpStream {
    let addr: SocketAddr = ADDR.parse().unwrap();
    loop {
        if let Ok(stream) = TcpStream::connect(addr) {
            return stream;
        }
    }
}

async fn write_to_threads(buf: &[u8], dims: (usize, usize)) -> std::io::Result<()> {
    // let mut handles = Vec::new();
    // for idx in 0..THREADS {
    //     let buf = buf.to_vec();
    //     handles.push(tokio::spawn(async move {
    //         let mut stream = connect().await;
    //         for (x, y) in iterate_pixels(dims) {
    //             if ((x + y) % THREADS) != idx {
    //                 continue;
    //             }
    //             let pixel = &buf[get_buf_idx(x,y,dims)..];
    //             if pixel[3] > 0 {
    //                 loop {
    //                     if let Ok(_) = stream.write(format!("PX {x} {y} {}\n", pix2hex(pixel)).into_bytes().as_ref()).await {
    //                         break;
    //                     } else {
    //                         stream = connect().await;
    //                     }
    //                 }
    //             }
    //         }
    //     }));
    // }
    // for handle in handles {
    //     handle.await?;
    // }
    Ok(())
}

fn iterate_pixels(dims: (i32, i32)) -> impl Iterator<Item=(i32, i32)> {
    let (width, height) = dims;
    (0..width).flat_map(move |x| (0..height).map(move |y| (x, y)))
}

fn render_buf(i: i32, dims: (i32, i32), iimg: String) -> std::io::Result<Vec<u8>> {
    println!("init {i}");
    let font = include_bytes!("../resources/font.ttf") as &[u8];

    let (width, height) = dims;

    let font = Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();

    let mut buf = vec![];

    for _ in 0..width {
        for _ in 0..height {
            buf.extend([
                0x00u8,
                0x00u8,
                0x00u8,
                0x00,
            ]);
        }
    }

    macro_rules! buf_idx {
                ($x:expr, $y:expr) => {{
                    let x = $x;
                    let y = $y;
                    (4 * (x + y * width)) as usize
                }};
            }

    // let (mut top_x, top_y) = (0, 100);
    // for c in s.chars() {
    //     let (metric, bytes) = font.rasterize(c, font_size);
    //
    //     for (idx, lightness) in bytes.into_iter().enumerate() {
    //         if lightness < 0x80 {
    //             continue;
    //         }
    //         let (y, x) = ((idx / metric.width) as i32, (idx % metric.width) as i32);
    //         let pixel = &mut buf[buf_idx!(
    //             top_x + x + metric.xmin,
    //             top_y + y + (font_size - metric.bounds.height).round() as i32 - metric.ymin
    //         )..];
    //         pixel[0] = 0xff; // r
    //         pixel[1] = 0xff; // g
    //         pixel[2] = 0xff; // b
    //         pixel[3] = lightness; // a
    //     }
    //
    //     top_x += metric.advance_width as i32;
    // }
    // let (mut top_x, top_y) = (0, 250);
    // for c in "KITCTF was here".chars() {
    //     let (metric, bytes) = font.rasterize(c, font_size);
    //
    //     for (idx, lightness) in bytes.into_iter().enumerate() {
    //         if lightness < 0x80 {
    //             continue;
    //         }
    //         let (y, x) = ((idx / metric.width) as i32, (idx % metric.width) as i32);
    //         let pixel = &mut buf[buf_idx!(
    //             top_x + x + metric.xmin,
    //             top_y + y + (font_size - metric.bounds.height).round() as i32 - metric.ymin
    //         )..];
    //         pixel[0] = 0xff; // r
    //         pixel[1] = 0xff; // g
    //         pixel[2] = 0xff; // b
    //         pixel[3] = lightness; // a
    //     }
    //
    //     top_x += metric.advance_width as i32;
    // }
    //

    let img = image::open(iimg).unwrap().into_rgba8();
    // let img = image::open("resources/img.jpeg").unwrap().into_rgba8();
    let img_dims = img.dimensions();

    for (x,y) in iterate_pixels(dims) {
        if (buf_idx!(x, y) >= buf.len()) {
            continue;
        }
        let pixel = &mut buf[buf_idx!(x, y)..];
        if x >= img_dims.0 as i32 || y >= img_dims.1 as i32 {
            continue;
        }
        let rgba = img.get_pixel(x as u32, y as u32);
        if rgba.0[3] == 0 {
            continue;
        }
        pixel[0] = rgba.0[0]; // r
        pixel[1] = rgba.0[1]; // g
        pixel[2] = rgba.0[2]; // b
        pixel[3] = 0xff; // a
    }

    Ok(buf)
}

use rand::thread_rng;
use rand::seq::SliceRandom;


fn send_buf(i: i32, dims: (i32, i32), mut buf: Vec<u8>, ix: i32, iy: i32) -> std::io::Result<()> {
    let (width, height) = dims;
    let mut stream = connect();

    println!("start {i}");

    macro_rules! buf_idx {
                ($x:expr, $y:expr) => {{
                    let x = $x;
                    let y = $y;
                    (4 * (x + y * width)) as usize
                }};
            }
    let mut coords = iterate_pixels(dims).collect::<Vec<_>>();
    coords.shuffle(&mut thread_rng());
    for (x,y) in coords {
        // if ((x * 238584 % 23 + y * 347234 % 41) % THREADS) != i {
        //     continue;
        // }
        let pixel = &mut buf[buf_idx!(x, y)..];
        if pixel[3] > 0 {
            let y = y + iy;
            let x = x + ix;
            stream.write(format!("PX {x} {y} {}\n", pix2hex(pixel)).into_bytes().as_ref())?;
            // stream.write(format!("PX {x} {y} ff\n").into_bytes().as_ref())?;
        }
    }

    println!("end {i}");

    Ok::<(), std::io::Error>(())
}

fn main() -> std::io::Result<()> {
    let img = std::env::args().nth(1).unwrap();
    let x = std::env::args().nth(2).unwrap().parse::<i32>().unwrap();
    let y = std::env::args().nth(3).unwrap().parse::<i32>().unwrap();

    let pool = ThreadPool::new(THREADS as usize);

    loop {
        for i in 0..THREADS {
            let rand = rand::random::<i32>();
            let img = img.clone();
            pool.execute(move || {
                let buf = render_buf(i, (1280, 720), img).unwrap();
                loop { send_buf(rand, (1280, 720), buf.clone(), x, y).unwrap() };
            });
        }
        pool.join();
    }
    Ok(())
}

fn pix2hex(pixel: &[u8]) -> String {
    format!("{:02x}{:02x}{:02x}", pixel[0], pixel[1], pixel[2])
}
