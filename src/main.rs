// Standard library imports
use std::env;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

// External imports
use anyhow::{bail, Result};
use hidapi_rusb::HidApi;
use midir::{Ignore, MidiInput, MidiOutput, MidiOutputConnection, MidiOutputPort};
use midly::SmfBytemap;
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
    list_midi_ports()?;
    let mut usb_midi_out = open_midi_usb()?;
    test_play(&mut usb_midi_out)?;
    play_midi_file(&mut usb_midi_out, file)?;
    Ok(())
}

fn _send_midi_to_usb(file: &str) -> Result<()> {
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

fn list_midi_ports() -> Result<()> {
    let mut midi_in = MidiInput::new("midir test input")?;
    midi_in.ignore(Ignore::None);
    let midi_out = MidiOutput::new("midir test output")?;

    println!("Available input ports:");
    for (i, p) in midi_in.ports().iter().enumerate() {
        println!("{}: {}", i, midi_in.port_name(p)?);
    }

    println!("Available output ports:");
    for (i, p) in midi_out.ports().iter().enumerate() {
        println!("{}: {}", i, midi_out.port_name(p)?);
    }

    Ok(())
}

// Adapted from https://github.com/Boddlnagg/midir/blob/master/examples/test_play.rs
fn open_midi_usb() -> Result<MidiOutputConnection> {
    let midi_out = MidiOutput::new("My Test Output")?;

    // Get an output port (read from console if multiple are available)
    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => bail!("no output port found"),
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(&out_ports[0]).unwrap()
            );
            &out_ports[0]
        }
        _ => {
            println!("Selecting USB MIDI output ports:");
            let mut port_index = 0;
            for (i, p) in out_ports.iter().enumerate() {
                let port_name = midi_out.port_name(p).unwrap();
                if port_name.contains("USB MIDI Interface") {
                    println!("Found USB MIDI Interface!");
                    port_index = i;
                }
            }
            &out_ports[port_index]
        }
    };
    println!("Opening connection");
    let conn_out = midi_out.connect(out_port, "midir-test")?;
    Ok(conn_out)
}

fn test_play(conn_out: &mut MidiOutputConnection) -> Result<()> {
    {
        // Define a new scope in which the closure `play_note` borrows conn_out, so it can be called easily
        let mut play_note = |note: u8, duration: u64| {
            const NOTE_ON_MSG: u8 = 0x90;
            const NOTE_OFF_MSG: u8 = 0x80;
            const VELOCITY: u8 = 0x64;
            // We're ignoring errors in here
            println!("Playing note {note}");
            let _ = conn_out.send(&[NOTE_ON_MSG, note, VELOCITY]);
            sleep(Duration::from_millis(duration * 150));
            let _ = conn_out.send(&[NOTE_OFF_MSG, note, VELOCITY]);
        };

        sleep(Duration::from_millis(4 * 150));
        println!("Playing test song...");

        play_note(66, 4);
        play_note(65, 3);
        play_note(63, 1);
        play_note(61, 6);
        play_note(59, 2);
        play_note(58, 4);
        play_note(56, 4);
        play_note(54, 4);
    }
    sleep(Duration::from_millis(150));
    Ok(())
}

fn play_midi_file(conn_out: &mut MidiOutputConnection, file: &str) -> Result<()> {
    // Write data to device
    let bytes = fs::read(file)?;
    let smf = SmfBytemap::parse(&bytes)?;

    let mut ticks: u32 = 0;
    println!("header: {:?}", smf.header);

    let mut us_per_tick: u64 = 0;
    let ticks_per_beat = match smf.header.timing {
        midly::Timing::Metrical(ticks_per_beat) => ticks_per_beat.as_int() as u64,
        midly::Timing::Timecode(_, _) => {
            unimplemented!();
        }
    };

    println!("ticks_per_beat: {ticks_per_beat}");

    for (i, track) in smf.tracks.iter().enumerate() {
        println!("track {} has {} events", i, track.len());
        for (bytes, event) in track {
            let delta_ticks = event.delta.as_int();
            if delta_ticks > 0 {
                assert!(us_per_tick > 0);
                let us = (delta_ticks as u64) * us_per_tick;
                sleep(Duration::from_micros(us));
                println!("{ticks}: Sleeping for {us} us");
            }
            ticks += delta_ticks;
            match event.kind {
                TrackEventKind::Meta(MetaMessage::Tempo(us_per_beat)) => {
                    // Change the tempo
                    // us_per_tick = (us/beat) / (tick/beat)
                    us_per_tick = us_per_beat.as_int() as u64 / ticks_per_beat;
                    println!("us_per_tick = {us_per_tick}");
                }
                _ => {}
            }
            // println!("{ticks} ({delta_ticks}): Sending {} bytes", bytes.len());
            // Transmit the MIDI bytes to the USB MIDI Interface
            let _ = conn_out.send(bytes)?;
        }
    }

    Ok(())
}
