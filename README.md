# Tracker-Console

A midi tracker for linux powered handheld retro emulation handhelds such as the Trimui Smart Pro running custom [Knulli](https://knulli.org/) firmware.

## TODO

- [ ] add exit menu (triggered by menu/mode/home button + start)
- [ ] make tempo changable via UI
- [x] change next screen button to the west face button
- [ ] send off messages for all playing notes when playback stops
- [ ] add a highlight to indicate playhead location when playing
- [ ] add config file and the ability to connect to multiple USB Midi Devices

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


