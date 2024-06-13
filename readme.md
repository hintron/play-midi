# PlayMIDI

The `PlayMIDI` program will attempt to 'play' a `.mid` MIDI file directly by
sending the commands in the MIDI file to a USB MIDI interface found on the
computer. The USB MIDI interface will then forward those commands to whatever
external device it is connected to.

`PlayMIDI` is cross-platform, thanks to Rust and the underlying [`midir`][midir]
and [`midly`][midly] crates. `midly` quickly parses the `.mid` file, while
`midir` does the heavy lifting by finding available MIDI interfaces on the
computer and sending the parsed MIDI data to them in a cross-platform way.


# Motivation

I created `PlayMIDI` to play MIDI files directly on my Kawai CP 170 (circa 2004)
without needing to open the file up in a cumbersome, proprietary Digital Audio
Workstation (DAW) program. Plus, once the DAW opens the MIDI file, it splits
everything up into separate tracks that all need to be set to play externally
(e.g. in [Logic Pro][logic-pro], you have to set each track to
[play to an external instrument][4], and even when that is done for each track,
some MIDI meta data in the original MIDI file is never sent, like program
changes that set the instrument for the channel).

Playing a MIDI file and sending MIDI commands directly to the Kawai CP 170 means
I don't have to deal with unreliable, slow, limited floppy disks. It also means
that I can add custom playlist functionality, since I control what gets played
instead of the piano. In theory, I could create all sorts of playlists, shuffle
them, insert and delete songs from the current playlist on the fly, hook the
playlist to a web page for others to add songs to, have custom sensors in the
room add extra midi notes on top of whatever is playing, etc.


# USB MIDI Interface - physical device

The USB MIDI Interface I am using is a
[TENINYU USB MIDI Interface][usb-midi-interface-amazon]. It works on both the
latest Windows 11 and the latest MacOS. I assume that other USB MIDI interfaces
would work just fine, as I believe the drivers for them are generic and not
specific to the device. [`midir`][midir] finds it on both Windows and MacOS.


# Kawai CP 170

The Kawai CP 170 manual can be found [here][cp170-manual]. All Kawai manuals can
be found [here][kawai-manuals].

In order to enable playing MIDI on the Kawai CP 170, you have to first move the
switch on the back of the piano to "MIDI" mode (instead of PC1, PC2, or MAC).
This tells the piano to stop listening for input on the piano's serial port
and listen for MIDI input on the physical MIDI input port.

Connect the OUT cable of the USB MIDI interface to the IN port of the Kawai.
Now, the piano should be able to recieve MIDI commands. (Connect the USB MIDI
interface's IN cable to the piano's OUT port in order to use the piano as input
into a DAW).

The CP 170 supports [General MIDI (GM) 1][gm], meaning that it is guaranteed to
have a certain collection of instruments at certain banks.


# Playing MIDI files on the computer

[GarageBand][garageband] will open MIDI files on your Mac and play them, as well
as show the notes being played, the tempo, and other metadata.

[VLC][vlc] can play MIDI files directly, but only after downloading something
extra. The steps are [documented here][1].


# Creating MIDI files

GarageBand does not allow you to export MIDI files. To get around this, use
[this method][2], where MIDI tracks are converted to loops and then the MIDI
data is extracted from the loop file via [aif2midi.com][5] and saved to a MIDI
file.

If aif2midi.com stops working, then the [GB2MIDI][gb2midi] repo also contains a
[gb2midi.pl][gb2midi.pl] script that shows how the loop file is converted to a
MIDI file. I plan on writing a similar tool in this repo in Rust, to be
independent of these sketchy dependencies.

Note also that you can get a free 3 month trial of [Logic Pro][logic-pro], which
is basically the full-featured version of GarageBand, and that should enable you
to export MIDI files.


# Inspecting MIDI files

[midly][midly] does a great job at parsing MIDI files, if you can use it to
write a Rust program to print out the header and various meta events. It's easy
to use and is also the fastest Rust MIDI parsing crate.


[cp170-manual]: https://kawaius.com/wp-content/uploads/2019/04/Kawai-CP180-CP170-CP150-Digital-Piano-Manual.pdf
[kawai-manuals]: https://kawaius.com/ensemble-pianos-owners-manuals/
[midly]: https://github.com/kovaxis/midly
[midir]: https://github.com/Boddlnagg/midir
[usb-midi-interface-amazon]: https://www.amazon.com/dp/B07L8KFYBK
[garageband]: https://www.apple.com/mac/garageband/
[logic-pro]: https://www.apple.com/logic-pro/
[mac-mini]: https://www.apple.com/shop/buy-mac/mac-mini
[vlc]: https://www.videolan.org/
[1]: https://wiki.videolan.org/Midi/
[2]: https://www.youtube.com/watch?v=qWNkItS8mxk
[gm]: https://en.wikipedia.org/wiki/General_MIDI
[3]: https://forum.cockos.com/showthread.php?t=240898
[gb2midi]: https://github.com/larkob/GB2MIDI
[gb2midi.pl]: https://raw.githubusercontent.com/larkob/GB2MIDI/master/GB2MIDI.app/Contents/Resources/Scripts/gb2midi.pl
[4]: https://support.apple.com/guide/logicpro/external-instrument-lgcp12e7acdc/mac
[5]: https://aif2midi.com/
