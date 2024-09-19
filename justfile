default:
  just -l

install-lib:
  pip uninstall -y tracker-backend && maturin develop

only-run:
  python ./gui/MIDI-Tracker.pygame

run-new: install-lib only-run

