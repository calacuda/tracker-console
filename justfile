default:
  just -l

install-lib:
  pip uninstall -y tracker-backend && maturin develop

only-run:
  python ./gui/MIDI-Tracker.pygame

run-new: install-lib only-run

flash-scp:
  scp -r ./gui/MIDI-Tracker.pygame  root@192.168.1.55:/userdata/roms/pygame/MIDI-Tracker/

