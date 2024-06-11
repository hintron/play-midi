// Standard library imports
use std::env;
use std::fs;
// use std::fs::OpenOptions;
// use std::io::Write;

// External imports
use anyhow::Result;
use hidapi_rusb::HidApi;
use midly::{num, MetaMessage, Smf, TrackEventKind};

// Internal imports

fn main() -> Result<()> {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("ERROR: MIDI file input arg is required");
    }

    let _executable = &args[0];
    let file = &args[1];

    print_meta(file)?;
    send_midi_file(file)?;
    Ok(())
}

/// Send the contents of a midi file to a USB midi interface
fn send_midi_file(file: &str) -> Result<()> {
    // Get the USB Midi interface
    let api = HidApi::new().unwrap();
    // Print out information about all connected devices
    for device in api.device_list() {
        println!("{:#X?}", device);
    }

    println!("Got here");

    // Connect to device using its VID and PID
    // For now, hard code to IDs for my "USB MIDI Cable"
    // let (vid, pid) = (0x46D, 0xC02C);
    let (vid, pid) = (0xFC02, 0x0101);
    let device = api.open(vid, pid)?;

    // // Read data from device
    // let mut buf = [0u8; 8];
    // let res = device.read(&mut buf[..])?;
    // println!("Read: {:?}", &buf[..res]);

    // Write data to device
    let bytes = fs::read(file)?;
    let res = device.write(&bytes[..])?;
    println!("Wrote: {:?} bytes to {}", res, file);
    Ok(())
}

/// Print out the meta data of a midi file and their tick timestamp while
/// ignoring most of the data.
fn print_meta(file: &str) -> Result<()> {
    let bytes = fs::read(file)?;
    let smf = Smf::parse(&bytes).unwrap();

    let mut ticks: num::u28 = num::u28::new(0);
    println!("header: {:?}", smf.header);
    for (i, track) in smf.tracks.iter().enumerate() {
        println!("track {} has {} events", i, track.len());
        for event in track {
            ticks += event.delta;
            match event.kind {
                TrackEventKind::Midi { .. } => {}
                TrackEventKind::SysEx(bytes) => println!("{ticks}: SysEx: {bytes:X?}"),
                TrackEventKind::Escape(bytes) => println!("{ticks}: Escape: {bytes:X?}"),
                TrackEventKind::Meta(MetaMessage::Text(string)) => {
                    println!("{ticks}: Meta:Text: {}", std::str::from_utf8(string)?)
                }
                TrackEventKind::Meta(MetaMessage::Lyric(..)) => {
                    // println!("{ticks}: Meta:Lyric: {}", std::str::from_utf8(string)?)
                }
                TrackEventKind::Meta(MetaMessage::TrackName(string)) => {
                    println!("{ticks}: Meta:TrackName: {}", std::str::from_utf8(string)?)
                }
                TrackEventKind::Meta(MetaMessage::Copyright(string)) => {
                    println!("{ticks}: Meta:Copyright: {}", std::str::from_utf8(string)?)
                }
                TrackEventKind::Meta(MetaMessage::Marker(string)) => {
                    println!("{ticks}: Meta:Marker: {}", std::str::from_utf8(string)?)
                }
                TrackEventKind::Meta(MetaMessage::InstrumentName(string)) => {
                    println!(
                        "{ticks}: Meta:InstrumentName: {}",
                        std::str::from_utf8(string)?
                    )
                }
                TrackEventKind::Meta(meta) => {
                    println!("{ticks}: Meta: {meta:?}")
                }
            }
        }
    }

    Ok(())
}
