use std::fs;

mod codenames;

struct Parser {
  bytes_per_record: usize,
  image_bytes_start: usize,
  bits_per_pixel: u8,
  width: u8,
  height: u8,
  get_character: fn(&Vec<u8>) -> Option<char>,
}

impl Parser {
  fn image_bytes_end(&self) -> usize {
    (self.image_bytes_start as u32 + self.bits_per_pixel as u32 * self.width as u32 * self.height as u32 / 8 - 1) as usize
  }
}

fn main() {
  for dataset_name in ["ETL1", "ETL6", "ETL7"] {
    parse(
      dataset_name,
      Parser {
        bytes_per_record: 2052,
        image_bytes_start: 32,
        bits_per_pixel: 4,
        width: 64,
        height: 63,
        get_character: (|record| codenames::x_0201::to_utf_8(record[6])),
      }
    )
  }

  for dataset_name in ["ETL2"] {
    parse(
      dataset_name,
      Parser {
        bytes_per_record: 2745,
        image_bytes_start: 45,
        bits_per_pixel: 6,
        width: 60,
        height: 60,
        get_character: (|record| {
          let left_byte = ((record[21] & 0b11111100) as u16) << 6;
          let right_byte = (((record[21] & 0b00000011) << 4) | ((record[22] & 0b11110000) >> 4)) as u16;
          codenames::co59::to_utf_8(left_byte | right_byte)
        }),
      }
    )
  }

  for dataset_name in ["ETL3", "ETL4", "ETL5"] {
    parse(
      dataset_name,
      Parser {
        bytes_per_record: 2952,
        image_bytes_start: 216,
        bits_per_pixel: 4,
        width: 72,
        height: 76,
        get_character: (|record| codenames::x_0201::to_utf_8(record[9])),
      }
    )
  }

  for dataset_name in ["ETL8B"] {
    parse(
      dataset_name,
      Parser {
        bytes_per_record: 512,
        image_bytes_start: 8,
        bits_per_pixel: 1,
        width: 64,
        height: 63,
        get_character: (|record| codenames::x_0208::to_utf_8(u16::from_be_bytes([record[2], record[3]]))),
      }
    )
  }

  for dataset_name in ["ETL8G"] {
    parse(
      dataset_name,
      Parser {
        bytes_per_record: 8199,
        image_bytes_start: 60,
        bits_per_pixel: 4,
        width: 128,
        height: 127,
        get_character: (|record| codenames::x_0208::to_utf_8(u16::from_be_bytes([record[2], record[3]]))),
      }
    )
  }

  for dataset_name in ["ETL9B"] {
    parse(
      dataset_name,
      Parser {
        bytes_per_record: 576,
        image_bytes_start: 8,
        bits_per_pixel: 1,
        width: 64,
        height: 63,
        get_character: (|record| codenames::x_0208::to_utf_8(u16::from_be_bytes([record[2], record[3]]))),
      }
    )
  }

  for dataset_name in ["ETL9G"] {
    parse(
      dataset_name,
      Parser {
        bytes_per_record: 8199,
        image_bytes_start: 64,
        bits_per_pixel: 4,
        width: 128,
        height: 127,
        get_character: (|record| codenames::x_0208::to_utf_8(u16::from_be_bytes([record[2], record[3]]))),
      }
    )
  }
}

fn parse(dataset_name: &str, parser: Parser) {
  let paths = fs::read_dir(format!("data/{}", dataset_name)).unwrap().map(|path| path.unwrap().path().into_os_string().into_string().unwrap());
  for path in paths {
    let content = std::fs::read(&path).unwrap();
    for i in 0..(content.len() / parser.bytes_per_record) {
      let offset = i * parser.bytes_per_record;
      let record = content[offset..(offset + parser.bytes_per_record)].to_vec();
      let image_bytes: Vec<u8> = record[parser.image_bytes_start..=parser.image_bytes_end()].to_vec();

      let image_bytes_transformed: Vec<u8> = if parser.bits_per_pixel == 6 {
        vertically_flip_image(shift_6bpp_image(image_bytes), parser.width)
      } else {
        vertically_flip_image(image_bytes, parser.width / (8 / parser.bits_per_pixel))
      };

      let bmp_bytes = to_bmp(
        image_bytes_transformed,
        parser.bits_per_pixel,
        parser.width,
        parser.height
      );

      let character: char = match (parser.get_character)(&record) {
        Some(x) => x,
        None => continue,
      };

      let path = format!(
        "data/images/{}/{}-{}-{}.bmp",
        dataset_name,
        character,
        path.split("/").last().unwrap(),
        i,
      );

      match fs::write(&path, bmp_bytes) {
        Err(e) => println!("{:?}", e),
        _ => ()
      }
    }
  }
}

fn shift_6bpp_image(image_bytes: Vec<u8>) -> Vec<u8> {
  let mut shifted_bytes: Vec<u8> = vec![0; image_bytes.len() / 6 as usize * 8 as usize];

  for i in 0..(shifted_bytes.len() / 4) {
    let original_offset = i * 3;
    let shifted_offset = i * 4;

    shifted_bytes[shifted_offset] = (0b11111100 & image_bytes[original_offset]) >> 2;
    shifted_bytes[shifted_offset + 1] = ((0b00000011 & image_bytes[original_offset]) << 4) | ((0b11110000 & image_bytes[original_offset + 1]) >> 4);
    shifted_bytes[shifted_offset + 2] = ((0b00001111 & image_bytes[original_offset + 1]) << 2) | ((0b11000000 & image_bytes[original_offset + 2]) >> 6);
    shifted_bytes[shifted_offset + 3] = 0b00111111 & image_bytes[original_offset + 2];
  }

  shifted_bytes
}

fn to_bmp(image_bytes: Vec<u8>, bits_per_pixel: u8, width: u8, height: u8) -> Vec<u8> {
  let bmp_valid_bpp = if bits_per_pixel == 6 { 8 } else { bits_per_pixel };
  let n_colors: u32 = 1 << bmp_valid_bpp;
  let header_size = (14 + 40 + 4 * n_colors) as u32;

  vec![
    vec![b'B', b'M'], // header__signature
    u32_as_bytes(header_size + (image_bytes.len() as u32)), // header__file_size
    u32_as_bytes(0), // header__reserved
    u32_as_bytes(header_size), // header__data_offset
    u32_as_bytes(40), // info_header__size
    u32_as_bytes(width as u32), // info_header__width
    u32_as_bytes(height as u32), // info_header__height
    u16_as_bytes(1), // info_header__planes
    u16_as_bytes(bmp_valid_bpp as u16), // info_header__bpp
    u32_as_bytes(0), // info_header__compression
    u32_as_bytes(0), // info_header__image_size
    u32_as_bytes(0), // info_header__XpixelsPerM
    u32_as_bytes(0), // info_header__YpixelsPerM
    u32_as_bytes(n_colors), // info_header__colors_used
    u32_as_bytes(n_colors), // info_header__important_colors
    color_table(n_colors, bits_per_pixel), // flat array of [b, g, r, 0] * n_colors
    image_bytes, // pixel_data
  ].into_iter().flatten().collect()
}

fn color_table(n_colors: u32, bits_per_pixel: u8) -> Vec<u8> {
  let mut color_table: Vec<Vec<u8>> = vec![vec![0; 4 as usize]; n_colors as usize];
  let color_step = 255.0 / (((1 << bits_per_pixel) - 1) as f32);
  for i in 0..n_colors {
    let color = std::cmp::min(255, (color_step * (i as f32)) as u16) as u8;
    color_table[i as usize] = vec![color, color, color, 0];
  }

  color_table.into_iter().flatten().collect()
}

fn vertically_flip_image(image_bytes: Vec<u8>, width: u8) -> Vec<u8> {
  image_bytes.chunks(width as usize).map( |x| x.to_vec() ).rev().flatten().collect()
}

fn u32_as_bytes(number: u32) -> Vec<u8> {
  number.to_le_bytes().to_vec()
}

fn u16_as_bytes(number: u16) -> Vec<u8> {
  number.to_le_bytes().to_vec()
}
