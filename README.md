# `midi-vlq`

The VLQs defined by the MIDI file format.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://app.travis-ci.com/m-rinaldi/midi-vlq.svg?branch=main)](https://app.travis-ci.com/m-rinaldi/midi-vlq)


---

Some numbers in the MIDI file formate – e.g., delta times – are represented in a form called *Variable Length Quantity*.

These numbers are represented 7 bits per byte, most significant bytes come first (i.e., MSB). All bytes, except the last, have bit 7 set, and the last byte has bit 7 clear.