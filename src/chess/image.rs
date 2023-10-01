use image::{
    codecs::png::PngEncoder,
    imageops::{overlay, resize, FilterType::Nearest},
    load_from_memory, DynamicImage, EncodableLayout, ImageBuffer, ImageEncoder, Rgba, RgbaImage,
};

use super::structs::*;

struct ImageGen {
    tile_size: u32,
    base_board: ImageBuffer<Rgba<u8>, Vec<u8>>,
    sprite_sheet: DynamicImage,
}

impl ImageGen {
    pub fn new(light: Rgba<u8>, dark: Rgba<u8>, tile_size: u32) -> Self {
        // Create 8x8 image
        let mut board = RgbaImage::new(8, 8);
        for (x, y, pixel) in board.enumerate_pixels_mut() {
            // Every other square light / dark
            if (x + y) & 1 == 0 {
                *pixel = light;
            } else {
                *pixel = dark;
            }
        }
        let sheet_bytes = include_bytes!("resources/sheet.png");
        let sprite_sheet = load_from_memory(sheet_bytes).expect("failed to load sprite sheet");
        let sprite_sheet: DynamicImage = image::DynamicImage::ImageRgba8(resize(
            &sprite_sheet,
            tile_size * 6,
            tile_size * 2,
            image::imageops::FilterType::Lanczos3,
        ));
        // Resize image to proper scale
        let base_board = resize(&board, tile_size * 8, tile_size * 8, Nearest);
        Self {
            tile_size,
            base_board,
            sprite_sheet,
        }
    }

    pub fn get_piece_sprite(&self, piece: u32) -> RgbaImage {
        let sprite_sheet = &self.sprite_sheet;
        let w = self.tile_size;
        let x = 5 - ((piece - 1) % 6);
        let y = (piece - 1) / 6;
        sprite_sheet.crop_imm(x * w, y * w, w, w).into_rgba8()
    }

    pub fn create_image(&self, game: &Game, rev: bool) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut board = self.base_board.clone();
        for (i, p) in game.board.pieces.iter().enumerate() {
            if p.0 == 0 {
                continue;
            }
            let x = (!rev as i32 * 7 - (i as i32 % 8)).abs() as u32;
            let y = (rev as i32 * 7 - (i as i32 / 8)).abs() as u32;
            let img = self.get_piece_sprite(p.0 as u32);
            overlay(
                &mut board,
                &img,
                (x * self.tile_size) as i64,
                (y * self.tile_size) as i64,
            );
        }
        return board;
    }

    pub fn encode_png(&self, img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Vec<u8> {
        let mut buffer = Vec::new();
        let bytes = img.as_bytes();
        PngEncoder::new(&mut buffer)
            .write_image(
                bytes,
                self.tile_size * 8,
                self.tile_size * 8,
                image::ColorType::Rgba8,
            )
            .unwrap();
        return buffer;
    }
}

mod test {
    use std::fs;

    use image::Rgba;

    use crate::chess::structs::{Game, Board};

    use super::ImageGen;
    const LIGHT: Rgba<u8> = Rgba([255u8, 255u8, 255u8, 255u8]);
    const DARK: Rgba<u8> = Rgba([255, 0, 0, 255]);

    #[test]
    pub fn tst_board() {
        let img_gen = ImageGen::new(LIGHT, DARK, 50);
        let board = Board::new_normal();
        let game = Game::new(board);
        let img = img_gen.create_image(&game, false);
        let bytes = img_gen.encode_png(img);
        fs::write("img.png", bytes).unwrap();
    }
}
