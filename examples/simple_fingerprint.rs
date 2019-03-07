use chromaprint_rs as chroma;
use claxon;
use failure::{format_err, Error};
use pretty_env_logger;
use log::{info, error, debug};

const AUDIO_FILE: &str = "./data/stereo-sweep-1Hz-96KHz.flac";

fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    info!("Opening file {}", AUDIO_FILE);
    let mut reader = claxon::FlacReader::open(AUDIO_FILE)?;
    let num_channels = reader.streaminfo().channels as usize;
    let sample_rate = reader.streaminfo().sample_rate as usize;
    let bits_per_sample = reader.streaminfo().bits_per_sample;
    debug!("Channels: {}", num_channels);
    debug!("Sample: {}", sample_rate);
    debug!("Bits per sample: {}", bits_per_sample);
    if bits_per_sample != 16 {
        error!("Bit depth is not 16!");
        return Err(format_err!(
            "Invalid bit depth of {}. Expected 16 bits per sample.",
            bits_per_sample
        ));
    }

    let mut ctx = chroma::Context::default();
    ctx.start(sample_rate, num_channels)?;

    let mut block_buffer: Vec<i32> = Vec::with_capacity(0x1_0000);
    let mut transform_buffer: Vec<i16> = Vec::with_capacity(0x10000);

    let mut block_reader = reader.blocks();
    // let mut counter = 0;
    while let Some(block) = block_reader.read_next_or_eof(block_buffer)? {
        //debug!("Feeding block {}", counter);
        block_buffer = block.into_buffer();
        transform_buffer.extend(block_buffer.iter().map(|s| *s as i16));
        let foo = [0;100];
        ctx.feed(&foo)?;
        transform_buffer.clear();
        // counter += 1;
    }

    ctx.finish()?;
    info!("Finished data input");
    let fp = ctx.get_fingerprint()?;
    info!("Got fingerprint!: {}", fp);
    println!("{}", fp);

    Ok(())
}
