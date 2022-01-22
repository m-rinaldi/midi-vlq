# `midi-vlq`

The *Variable-Length Quantities* (VLQs) as defined by the Standard MIDI file format.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://app.travis-ci.com/m-rinaldi/midi-vlq.svg?branch=main)](https://app.travis-ci.com/m-rinaldi/midi-vlq)
[![Unsafety](https://img.shields.io/badge/Unsafety-0%25-brightgreen.svg)](https://doc.rust-lang.org/std/keyword.unsafe.html)


---

From the Standard MIDI-File Format Spec. 1.1:

>Some numbers in MIDI Files are represented in a form called a variable-length quantity. These
numbers are represented 7 bits per byte, most significant bits first. All bytes except the last
have bit 7 set, and the last byte has bit 7 clear. If the number is between 0 and 127, it is thus
represented exactly as one byte. 

### Example

    use midi_vlq::MidiVlq;
    
    // encode 127 as VLQ
    let vlq = MidiVlq::from(127u8);
    // 127 is encoded with a single byte
    assert_eq!(vlq, [127]);

    // encode 128 as VLQ
    let vlq = MidiVlq::from(128u8);
    // 128 needs two bytes
    assert_eq!(vlq, [0x81, 0x00]);