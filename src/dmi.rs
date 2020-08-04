/// Represents an individual icon state from a [`Dmi`] file
struct IconState<'a> {
    /// The state's name
    name: &'a str,
    /// The number of directions the state supports
    num_dirs: u8,
    /// The number of frames the state has
    num_frames: u8,
    /// Whether the state rewinds after animating
    rewinds: bool,
    /// The number of times the state loops if not infinite
    loop_count: u8,
    /// The delays between each frames in the state
    frame_delays: Vec<f32>
}

/// Represents a .dmi file
struct Dmi<'a> {
    /// The version of the file
    /// TODO: semver conformity
    version: &'a str,
    /// The width of one icon in the file
    width: u16,
    /// The height of one icon in the file
    height: u16,
    /// The [`IconState`]s in the file
    icon_states: Vec<&'a IconState<'a>>
} 

type DmiMetadata<'a> = (&'a str, u16, u16, Vec<&'a IconState<'a>>);

impl Dmi<'_> {
    pub fn new<'a>(data: Vec<u8>) -> Dmi<'a> {
        println!("Extracting meta data...");
        let metadata: DmiMetadata<'a> = Dmi::extract_metadata(&data).expect("Dmi::new metadata failed");

        // TODO: use metadata
        Dmi {
            version: "1.0",
            width: 0,
            height: 0,
            icon_states: Vec::new()
        }
    }

    fn extract_metadata<'a>(data: &[u8]) -> Result<DmiMetadata<'a>, &'a str> {
        let data_len = data.len() as u64;

        // Read the PNG data
        let metadata_result: std::result::Result<DmiMetadata<'a>, &'a str> = {
            use std::borrow::Cow;
            use std::io::{Cursor, Read};
            use byteorder::{BigEndian, ReadBytesExt};
            use miniz_oxide::inflate::decompress_to_vec_zlib;

            let mut cursor = Cursor::new(data);
            // Check PNG signature
            let mut signature = vec![0u8; 8];
            cursor.read_exact(&mut signature).expect("Dmi::extract_metadata failed");
            if &signature[..] != b"\x89PNG\r\n\x1a\n" {
                return Err("Invalid PNG signature");
            }

            // Browse the chunks till we find a tEXt or zTXt with Description
            let mut maybe_metadata_readable: Option<Cow<'a, [u8]>> = None;
            while cursor.position() < data_len {
                let chunk_length = cursor.read_u32::<BigEndian>().expect("Dmi::extract_metadata chunk_length failed");
                let mut chunk_type = vec![0u8; 4];
                cursor.read_exact(&mut chunk_type).expect("Dmi::extract_metadata chunk_type failed");
                let mut chunk_data = vec![0u8; chunk_length as usize];
                cursor.read_exact(&mut chunk_data).expect("Dmi::extract_metadata chunk_data failed");
                let mut chunk_crc = vec![0u8; 4];
                cursor.read_exact(&mut chunk_crc).expect("Dmi::extract_metadata chunk_crc failed");
                
                let mut maybe_desc_raw: Option<Cow<'a, [u8]>> = None;
                if &chunk_type == b"tEXt" {
                    // TODO
                } else if &chunk_type == b"zTXt" {
                    let mut keyword: &[u8] = &[0u8, 1];
                    for x in 0..78 {
                        if chunk_data[x] == 0 {
                            keyword = &chunk_data[0..x];
                            break;
                        }
                    }
                    
                    if &keyword == b"Description" {
                        let _compression_method = chunk_data[keyword.len() + 1];
                        let text_vec = decompress_to_vec_zlib(&chunk_data[(keyword.len() + 2)..]).expect("Dmi::extract_metadata text_vec failed");
                        maybe_desc_raw = Some(Cow::from(text_vec));
                    }
                }

                if let Some(x) = maybe_desc_raw {
                    maybe_metadata_readable = Some(x);
                    break;
                }
            }

            match maybe_metadata_readable {
                Some(x) => Ok(Dmi::parse_metadata(std::str::from_utf8(&x).unwrap())),
                None => Err("Dmi::extract_metadata metadata_raw not found")
            }
        };
    
        match metadata_result {
            Ok(x) => Ok(x),
            Err(e) => Err(e)
        }
    }

    fn parse_metadata<'a>(metadata_raw: &str) -> DmiMetadata<'a> {
        let mut version: &'a str = "";
        let mut width: u16 = 0;
        let mut height: u16 = 0;
        let mut icon_states: Vec<&'a IconState<'a>> = Vec::new();

        // TODO: parse it
        println!("{}", metadata_raw);

        (version, width, height, icon_states)
    }
}

/// Represents an image file
pub struct Image<'a> {
    dmi: Dmi<'a>
}

impl Image<'_> {
    pub fn from_file(path: &std::path::Path) -> Image {
        let read_data = std::fs::read(path).expect("Image::from_file failed");

        Image {
            dmi: Dmi::new(read_data)
        }
    }
}