use slint::{Image, Rgb8Pixel, Rgba8Pixel, SharedPixelBuffer};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    const WIDTH:u32 = 50;
    const HEIGHT:u32 = 50;

    let mut pixel_buffer = SharedPixelBuffer::<Rgb8Pixel>::new(WIDTH, HEIGHT);
    let ruleset = vec![0,1,1,0,1,1,1,1];
    let pixel_buffer_bytes = pixel_buffer.make_mut_bytes();
    one_dimensional_ca(WIDTH, HEIGHT, &ruleset, 3, pixel_buffer_bytes);
    for c in pixel_buffer_bytes {
        //*c = (*c*128)<<1;
        *c*=128;
    }
    let cell_grid = Image::from_rgb8(pixel_buffer);
    ui.set_img(cell_grid);

    ui.run()
}

fn one_dimensional_ca(w:u32, h:u32, ruleset: &Vec<u8>, chunk_size:u32, pixels:&mut[u8]) {
    let initial_ind =(w/2u32) as usize*chunk_size as usize;
    pixels[initial_ind..initial_ind+chunk_size as usize].iter_mut().for_each(|elem| *elem = 1u8);

    for row_index in 0..h-1 {
        let row_start = (row_index*h* chunk_size) as usize;
        let row:&[u8] = & pixels[row_start..(row_start + (w as usize * chunk_size as usize))];
        let res = calc_1d_row_chunked(row, chunk_size as isize, &ruleset);
        let next_row_start = (row_start + (h as usize* chunk_size as usize));
        (&mut pixels[next_row_start..(next_row_start + (w as usize* chunk_size as usize))]).copy_from_slice(&res);
    }
}

fn calc_1d_row(row: &[u8], chunk_size:usize, ruleset: &Vec<u8>) -> Vec<u8> {
    let mut calculated_row:Vec<u8> = vec![0u8; row.len()];
    for index in 0..row.len() as isize {
        calculated_row[index as usize] = ruleset[((row[wrapped_index(index-1, row.len() as isize)]<<2) + (row[index as usize]<<1) + row[wrapped_index(index+1, row.len() as isize)]) as usize];
    }
        return calculated_row;
}

fn calc_1d_row_chunked(row: &[u8], chunk_size:isize, ruleset: &Vec<u8>) -> Vec<u8> {
    let mut calculated_row:Vec<u8> = vec![0u8; row.len()];
    for start_ind in (0..row.len()as isize).step_by(chunk_size as usize) {
        let end_ind = start_ind+ chunk_size;
        let right_neighbour_start_ind = wrapped_index((start_ind+ chunk_size) as isize, row.len() as isize);
        let left_neighbour_start_ind = wrapped_index((start_ind- chunk_size) as isize, row.len() as isize);

        calculated_row[start_ind as usize..end_ind as usize].iter_mut().for_each(|elem| *elem = ruleset[((row[left_neighbour_start_ind]<<2)+(row[start_ind as usize]<<1)+row[right_neighbour_start_ind]) as usize]);
    }
    return calculated_row;
}

fn wrapped_index(index:isize, array_len:isize) -> usize{
    (((index % array_len) + array_len) % array_len) as usize
}
