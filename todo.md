# TODO

## Tasks
* Support parallel songs (24.mid).
* Improve commandline arg infrastructure.
  * Debug mode, speedup modifier, help, usage, etc.
* Specify a custom string to look for when getting the USB MIDI interface.
* Create controls, with skips forward and backward within the song.
* Show total time of song and current time.
* Display current volume.
* Add controls to change volume.
* Create a playlist mode.
  * Point to directories to search through to build the playlist.
  * Display the playlist and upcoming song.
  * Support shuffle, next, previous, repeat, random, playlist restart/change.
  * Support filtering of songs - by filename, and also content like artist.
* Figure out a way to reverse the iteration on demand, for backwards skipping.

## Ideas
* Make lyrics show up earlier, and make them show as full lines, with the
  current lyric highlighted in some way. Also show the next line to come.
* Check to make sure MIDI file playback speed is the same as if the CP 170
  played it from disk (check Abba - Fernando - the 1.0x playback is slow).
* Figure out why harpsicord in Warcraft is being played as a piano on the CP-170
  when played through the program, but is a harpsicord when played via floppy.
  * Make sure that the exact same midi file is being played.
* Create a UI (egui?).
* Create a web-based UI, where users can dynamically add songs to a playlist
  while the piano is playing.
* Create a separate thread that listens to commands from a web service.
  * Use mio?
* See if I can get this all working on a Raspberry Pi, so I don't need a full
  computer connected to the piano.
* Add a side channel of midi notes from various sensors attached to a raspberry
  pi with IO pins.
* Extract MIDI files from GarageBand loop files.
* Create a tool to automatically determine average velocity or volume rating of
  a song
* Create a tool to normalize MIDI files to be a similar volume.
  * E.g. 24.mid is super loud, while Sea of Thieves/wellerman MIDI files are too
    quiet.

## Completed
* Account for how long it takes to play a note or send a command and compensate
  for that when sleeping between commands.
* Handle ctrl-C so that it sends NOTE OFF commands before exiting.
* Send NOTE OFF commands for all notes on all channels before starting song,
  in case things were in a bad state.
