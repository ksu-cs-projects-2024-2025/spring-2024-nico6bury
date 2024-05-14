use image::RgbImage;

use crate::squares::SquareGrid;

pub fn generate_img_from_map(map: &SquareGrid) -> RgbImage {
	let mut img = RgbImage::new(*map.img_width() as u32, *map.img_height() as u32);
	for square in map.iter() {
		for x in *square.x()..(square.x()+square.width()) {
			for y in *square.y()..(square.y()+square.height()) {
				match img.get_pixel_mut_checked(x as u32, y as u32) {
					Some(pixel) => {
						let color = [square.color().0, square.color().1, square.color().2];
						*pixel = image::Rgb(color);
					},
					None => println!("Couldn't get pixel at x:{}, y:{} ?", x, y),
				}//end matching img
			}//end looping through y values in square
		}//end looping through x values in square
	}//end copying pixel information from each square into img
	img
}//end generate_img_from_map(map)
