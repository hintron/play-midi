# TODO

## Tasks
* Improve commandline arg infrastructure.
* Send NOTE OFF commands for all notes on all channels before starting song,
  in case things were in a bad state.
* Handle ctrl-C so that it sends NOTE OFF commands before exiting.
* Specify a custom string to look for when getting the USB MIDI interface.
* Figure out why harpsicord in Warcraft is being played as a piano on the CP-170
  when played through the program, but is a harpsicord when played via floppy.
  * Make sure that the exact same midi file is being played.

## Ideas
* Create a UI (egui?).
* Create a web-based UI, where users can dynamically add songs to a playlist
  while the piano is playing.
* Create a playlist mode.
* Create controls, with skips forward and backward.
* Figure out a way to reverse the iteration on demand, for backwards skipping.
* Create a separate thread that listens to commands from a web service.
* See if I can get this all working on a Raspberry Pi, so I don't need a full
  computer connected to the piano.
* Add a side channel of midi notes from various sensors attached to a raspberry
  pi with IO pins.
* Extract MIDI files from GarageBand loop files.
