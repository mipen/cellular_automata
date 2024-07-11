use once_cell::sync::Lazy;
use slint::{Image, Rgb8Pixel, SharedPixelBuffer, Timer};
use std::{sync::Mutex, time::Duration};

slint::include_modules!();

const COLOURS: [u8; 2] = [255u8, 0u8];

#[derive(Clone, Copy)]
struct CAPixel {
    state: u8,
}

impl CAPixel {
    fn get_pixel(&self) -> Rgb8Pixel {
        let colour = COLOURS[self.state as usize];
        Rgb8Pixel::new(colour, colour, colour)
    }
}

static TWOD_CA: Lazy<Mutex<Vec<CAPixel>>> =
    Lazy::new(|| Mutex::new(vec![CAPixel { state: 0 }; 100 * 100]));

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    const WIDTH: u32 = 100;
    const HEIGHT: u32 = 100;

    ui.set_oned_img(one_dimensional_ca_capixel(WIDTH, HEIGHT, 1));
    let mut initial_state = vec![CAPixel { state: 0 }; 100 * 100];
    {
        initial_state[5250].state = 1;
        initial_state[5251].state = 1;
        initial_state[5351].state = 1;
        initial_state[5151].state = 1;
        initial_state[5152].state = 1;

        initial_state[550].state = 1;
        initial_state[551].state = 1;
        initial_state[552].state = 1;
        initial_state[553].state = 1;
        initial_state[554].state = 1;
        initial_state[555].state = 1;

        initial_state[650].state = 1;
        initial_state[651].state = 1;
        initial_state[652].state = 1;
        initial_state[653].state = 1;
        initial_state[654].state = 1;
        initial_state[655].state = 1;

        initial_state[750].state = 1;
        initial_state[751].state = 1;
        initial_state[752].state = 1;
        initial_state[753].state = 1;
        initial_state[754].state = 1;
        initial_state[755].state = 1;

        let mut current_state = TWOD_CA.lock().unwrap();
        current_state.copy_from_slice(&initial_state);
    }
    ui.set_twod_img(create_twod_img(&initial_state, 100 as isize, 100 as isize));
    // ui.set_twod_img(two_dim_ca(100, 100));
    let timer = Timer::default();
    let ui_handle = ui.as_weak();
    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(32),
        move || {
            let ui = ui_handle.unwrap();
            ui.set_twod_img(two_dim_ca(100, 100));
        },
    );

    ui.on_submit_clicked({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let rule = match ui.get_rule().parse::<u8>() {
                Ok(r) => r,
                Err(e) => {
                    println!("Enter a number: {}", e);
                    0
                }
            };
            ui.set_oned_img(one_dimensional_ca_capixel(WIDTH, HEIGHT, rule));
        }
    });
    ui.on_text_accepted({
        let ui_handle = ui.as_weak();
        move |_new_text| {
            let ui = ui_handle.unwrap();
            let rule = match ui.get_rule().parse::<u8>() {
                Ok(r) => r,
                Err(e) => {
                    println!("Enter a number: {}", e);
                    0
                }
            };
            ui.set_oned_img(one_dimensional_ca_capixel(WIDTH, HEIGHT, rule));
        }
    });

    ui.on_text_edited({
        let ui_handle = ui.as_weak();
        move |new_text| {
            let ui = ui_handle.unwrap();
            ui.set_rule(new_text);
        }
    });

    ui.run()
}

fn two_dim_ca(width: isize, height: isize) -> Image {
    let array_len = width * height;

    let mut current_state = TWOD_CA.lock().unwrap();
    let mut next_state = vec![CAPixel { state: 0 }; array_len as usize];

    for ind in 0..array_len {
        let neighbour_sum = current_state[wrapped_index(ind - width - 1, array_len)].state
            + current_state[wrapped_index(ind - width, array_len)].state
            + current_state[wrapped_index(ind - width + 1, array_len)].state
            + current_state[wrapped_index(ind - 1, array_len)].state
            + current_state[wrapped_index(ind + 1, array_len)].state
            + current_state[wrapped_index(ind + width - 1, array_len)].state
            + current_state[wrapped_index(ind + width, array_len)].state
            + current_state[wrapped_index(ind + width + 1, array_len)].state;
        let cur_state = current_state[ind as usize].state;
        let lookup_table = [0, 0, cur_state, 1, 0, 0, 0, 0, 0];

        next_state[ind as usize].state = lookup_table[neighbour_sum as usize];
    }
    current_state.copy_from_slice(&next_state);
    return create_twod_img(&next_state, width, height);
}

fn create_twod_img(pixels: &Vec<CAPixel>, width: isize, height: isize) -> Image {
    let mut pixel_buffer = SharedPixelBuffer::<Rgb8Pixel>::new(width as u32, height as u32);
    let pixel_buffer_slice = pixel_buffer.make_mut_slice();
    pixel_buffer_slice.copy_from_slice(
        &pixels
            .iter()
            .map(|x| x.get_pixel())
            .collect::<Vec<Rgb8Pixel>>(),
    );
    return Image::from_rgb8(pixel_buffer);
}

fn one_dimensional_ca_capixel(w: u32, h: u32, rule: u8) -> Image {
    let ruleset = vec![
        (rule & 128u8) >> 7,
        (rule & 64u8) >> 6,
        (rule & 32u8) >> 5,
        (rule & 16u8) >> 4,
        (rule & 8u8) >> 3,
        (rule & 4u8) >> 2,
        (rule & 2u8) >> 1,
        rule & 1u8,
    ];
    let mut pixel_buffer = SharedPixelBuffer::<Rgb8Pixel>::new(w, h);

    let mut pixels = vec![CAPixel { state: 0 }; (w * h) as usize];
    pixels[(w / 2) as usize].state = 1u8;

    for row_index in 0..h - 1 {
        let row_start = (row_index * h) as usize;
        let row = &mut pixels[row_start..row_start + w as usize];
        let calculated_row = calc_1d_row_capixel(row, &ruleset);
        let row_start = row_start + w as usize;
        (&mut pixels[row_start..row_start + w as usize]).copy_from_slice(&calculated_row);
    }
    let pixel_buffer_slice = pixel_buffer.make_mut_slice();
    pixel_buffer_slice.copy_from_slice(
        &pixels
            .iter()
            .map(|x| x.get_pixel())
            .collect::<Vec<Rgb8Pixel>>(),
    );
    return Image::from_rgb8(pixel_buffer);
}

fn calc_1d_row_capixel(row: &mut [CAPixel], ruleset: &Vec<u8>) -> Vec<CAPixel> {
    let mut res = vec![CAPixel { state: 0 }; row.len() as usize];
    for index in 0..row.len() as isize {
        res[index as usize].state =
            ruleset[((row[wrapped_index(index - 1, row.len() as isize)].state << 2)
                + (row[index as usize].state << 1)
                + row[wrapped_index(index + 1, row.len() as isize)].state)
                as usize];
    }
    return res;
}

fn wrapped_index(index: isize, array_len: isize) -> usize {
    (((index % array_len) + array_len) % array_len) as usize
    // let step1 = index % array_len;
    // let step2 = step1 + array_len;
    // let step3 = step2 % array_len;
    // let result = step3 as usize;

    // println!("Index:{:#}; Step1: {:#}; Step2:{:#}; Step3:{:#}; Result:{:#}", index, step1, step2, step3, result);

    // return result;
}
