# Tracker-Console

A midi tracker for linux powered handheld retro emulation handhelds such as the Trimui Smart Pro running custom [Knulli](https://knulli.org/) firmware.

## Statefull Rewrite

Goal: handle: state, audio syntheis, and Midi, in rust. handle display and graphics in python. rust will send state to python and python will parse.

## Screens

1. Song screen: with four channels of 16 chains.
  - lead 1
  - lead 2
  - bass
  - percussion
2. chain screen: 1 channel, each chain should contain up to 16 phrases.
3. phrase screen: 3 channels, each phrase should contain up to 16 rows.
  - note
  - instrument
  - command
4. Synth screen: play the synth over the backing track, accepts usb midi input and has on screen controls for a synth.
5. Instrument Screen: edit differnet instruments.
6. Settings Screen: settings, ex, bass/lead 1/lead 2 octave offset & default instrument.

## Song Screen


