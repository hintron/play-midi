# MIDI Specification Notes

URL:
https://archive.org/details/Complete_MIDI_1.0_Detailed_Specification_96-1-3/page/n11/mode/2up

(pg. 8)
1 ms jitter is audible.
Musicians can control relative time intervals with a precision of 1.5 ms.
10 ms of latency is imperceptible, as long as jitter is low.
MIDI is serial, so two notes that occur at the exact same time must technically be sent one after the other.
However, humans almost never play two notes at exactly the same time.
Running status can be used to reduce repeated commands; the status byte is omitted for subsequent messages of the same type.

(pg. 9)
Note Off messages are often replaced with Note On + 0 velocity so that running status can be leveraged.
Real-time MIDI commands don't need timing. However, a MIDI file does.

(pg. 10)
General MIDI
Channel 10 is for percussion.
Channels 1-9 and 11-16 are for non-percussion instrucments.
Instruments are groups in sets. Program numbers 1-8 are pianos, 9-16 are chromatic percussion sounds, 17-24 are organs guitars, etc.

(pg. 24)
All Notes Off (mode message 123).
The receiver is not required to recognize it, so individual note off messages may have to be sent.

(pg. 25)
All Notes Off should not be sent periodically as part of normal operation.
All Sound Off (mode message 120).

