// Standard library imports
use std::env;
use std::fs;
use std::time::Duration;
// use std::fs::OpenOptions;
// use std::io::Write;

// External imports
use anyhow::{bail, Result};
use hidapi_rusb::HidApi;
use midly::{num, MetaMessage, Smf, TrackEventKind};
use rusb;

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
    show_usb_devices()?;
    send_midi_to_usb(file)?;
    Ok(())
}

fn send_midi_to_usb(file: &str) -> Result<()> {
    let (vid, pid) = (0xFC02, 0x0101);
    let handle = match rusb::open_device_with_vid_pid(vid, pid) {
        Some(handle) => handle,
        None => bail!(
            "Could not get device handle for USB MIDI interface with vid={vid:04X} pid={pid:04X}"
        ),
    };

    // Write data to device
    let bytes = fs::read(file)?;
    println!("Writing to USB MIDI...");
    let written = handle.write_bulk(0x0, &bytes[..], Duration::from_secs(1))?;
    println!("Wrote: {:?} bytes to {}", written, file);
    Ok(())
}

fn show_usb_devices() -> Result<()> {
    let devices = rusb::devices()?;
    for device in devices.iter() {
        let device_desc = device.device_descriptor()?;
        let vid = device_desc.vendor_id();
        let pid = device_desc.product_id();
        println!(
            "Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            vid,
            pid
        );
    }
    Ok(())
}

/// Send the contents of a midi file to a USB midi interface
fn _send_midi_file(file: &str) -> Result<()> {
    // Get the USB Midi interface
    let api = HidApi::new().unwrap();
    // Print out information about all connected devices
    for device in api.device_list() {
        println!("{:#X?}", device);
    }

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
