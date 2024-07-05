use slint::{Image, Rgb8Pixel, Rgba8Pixel, SharedPixelBuffer};

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

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    const WIDTH: u32 = 100;
    const HEIGHT: u32 = 100;

    ui.set_img(one_dimensional_ca_capixel(WIDTH, HEIGHT, 1));

    ui.on_submit_clicked({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let rule = ui.get_rule().parse::<u8>().unwrap();
            ui.set_img(one_dimensional_ca_capixel(WIDTH, HEIGHT, rule));
        }
    });
    ui.on_text_accepted({
        let ui_handle = ui.as_weak();
        move |new_text| {
            let ui = ui_handle.unwrap();
            let rule = match (ui.get_rule().parse::<u8>()) {
                Ok(r) => r,
                Err(e) => {println!("Enter a number: {}", e); 0},
            };
            ui.set_img(one_dimensional_ca_capixel(WIDTH, HEIGHT, rule));
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
}
