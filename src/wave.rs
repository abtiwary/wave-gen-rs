/*!
 * This module defines functions to generate a wave header.
 * 
 * The following resource was used in order to understand the WAVE spec:
 * http://www-mmsp.ece.mcgill.ca/Documents/AudioFormats/WAVE/WAVE.html
!*/

pub fn le_u32_to_bytes(value: u32) -> [u8;4] {
    let mut byte_slice = [0u8; 4];
    for i in 0..4 {
        byte_slice[i] = ((value >> (8 * i)) & 0xFF) as u8;
    }

    byte_slice
}

pub fn le_u16_to_bytes(value: u16) -> [u8;2] {
    let mut byte_slice = [0u8; 2];
    for i in 0..2 {
        byte_slice[i] = ((value >> (8 * i)) & 0xFF) as u8;
    }

    byte_slice
}

pub fn generate_wave_header(nb_samples: usize, sample_rate: u32) -> Vec<u8> {
    let mut header_buf = Vec::new();
    header_buf.extend_from_slice("RIFF".as_bytes());
    let rsize = le_u32_to_bytes(20 + nb_samples as u32);
    header_buf.extend_from_slice(&rsize);

    // WAVE chunk
    header_buf.extend_from_slice("WAVE".as_bytes());

    // start fmt chunk
    header_buf.extend_from_slice("fmt ".as_bytes());
    // fmt chunk size
    header_buf.extend_from_slice(&le_u32_to_bytes(16));
    // format code: PCM
    header_buf.extend_from_slice(&le_u16_to_bytes(1));
    // number of channels
    header_buf.extend_from_slice(&le_u16_to_bytes(1));
    header_buf.extend_from_slice(&le_u32_to_bytes(sample_rate));
    header_buf.extend_from_slice(&le_u32_to_bytes(sample_rate));
    // block size
    header_buf.extend_from_slice(&le_u16_to_bytes(1));
    // bits per sample
    header_buf.extend_from_slice(&le_u16_to_bytes(8));
    // end fmt chunk 

    // data chunk
    header_buf.extend_from_slice("data".as_bytes());
    header_buf.extend_from_slice(&le_u32_to_bytes(nb_samples as u32));

    header_buf
}
