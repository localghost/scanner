use raster;
use rayon;

fn threshhold(image: &mut raster::Image, threshold: u8) {
    image.bytes.iter_mut().for_each(|b| {
        if *b > threshold {
            *b = 255 as u8;
        } else {
            *b = 0 as u8;
        }
    });
}

fn check_block(image: &mut raster::Image, x: i32, y: i32, width: i32, height: i32, fill_percent: u8) -> bool {
    //    println!("Checking block ({}, {}) of size {}x{}", x, y, width, height);
    let mut blacks = 0;
    for i in x..x + width {
        for j in y..y + height {
            if image.get_pixel(i, j).unwrap().a == 0 {
                blacks += 1;
            };
        }
    }
    blacks as f32 > (width * height) as f32 * (fill_percent as f32 / 100 as f32)
}

fn discard_block(image: &mut raster::Image, x: i32, y: i32, width: i32, height: i32) {
    for i in x..x + width {
        for j in y..y + height {
            image.set_pixel(i, j, raster::Color::white()).unwrap();
        }
    }
}

struct Chunk {
    start: *mut usize,
    width: usize,
    height: usize,
}

impl Chunk {
    fn check(&self) {
        let mut blacks = 0;
        for i in x..x + width {
            for j in y..y + height {
                if image.get_pixel(i, j).unwrap().a == 0 {
                    blacks += 1;
                };
            }
        }
        blacks as f32 > (width * height) as f32 * (fill_percent as f32 / 100 as f32)
    }

    fn get_pixel_color(x: i32, y: i32) -> u8 {
        let rgba = 4;
        let start = (y * self.width) + x;
        let start = start * rgba;

            let slice = &self.bytes[start as usize..end as usize];
            Ok(Color {
                r: slice[0],
                g: slice[1],
                b: slice[2],
                a: slice[3],
            })
        }
    }
}

fn discard_blocks(image: &mut raster::Image, block_size: i32, fill_percent: u8) {
    let width = image.width;
    let height = image.height;

    let mut i = 0;
    let mut j;
    let splits = height / block_size;
    loop {
        j = 0;
        rayon::scope(|s| {
            for j in 0..splits + 1 {
                let y = j * block_size;
                s.spawn(|_| {
                    if check_block(
                        image,
                        i,
                        y,
                        std::cmp::min(block_size, width - i),
                        std::cmp::min(block_size, height - y),
                        fill_percent,
                    ) {
                        discard_block(
                            image,
                            i,
                            j,
                            std::cmp::min(block_size, width - i),
                            std::cmp::min(block_size, height - y),
                        );
                    }
                });
            }
        });
        //        loop {
        //            if check_block(
        //                image,
        //                i,
        //                j,
        //                std::cmp::min(block_size, width - i),
        //                std::cmp::min(block_size, height - j),
        //                fill_percent,
        //            ) {
        //                discard_block(
        //                    image,
        //                    i,
        //                    j,
        //                    std::cmp::min(block_size, width - i),
        //                    std::cmp::min(block_size, height - j),
        //                );
        //            }
        //            j += block_size;
        //            if j > height {
        //                break;
        //            }
        //        }
        i += block_size;
        if i > width {
            break;
        }
    }
}

fn main() {
    let mut image = raster::open("/tmp/page.jpg").unwrap();
    raster::filter::grayscale(&mut image).unwrap();
    threshhold(&mut image, 150);
    discard_blocks(&mut image, 50, 80);
    raster::save(&image, "/tmp/page_bw.jpg").unwrap();
}
