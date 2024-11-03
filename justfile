default:
  just -l

install-lib:
  pip uninstall -y tracker-backend && maturin develop

only-run:
  python ./gui/MIDI-Tracker.pygame

run-new: install-lib only-run

flash-scp:
  scp -r ./{gui/MIDI-Tracker.pygame,dist/tracker_backend-0.1.0-cp311-cp311-manylinux_2_34_aarch64.whl}  root@192.168.1.55:/userdata/roms/ports/MIDI-Tracker/

build:
  # PKG_CONFIG_SYSROOT_DIR=/opt/ArchARM maturin build --out dist --find-interpreter --target aarch64-unknown-linux-gnu
  PKG_CONFIG_SYSROOT_DIR=./cross-build-deps/ maturin build --out dist --find-interpreter --target aarch64-unknown-linux-gnu

ssh:
  ssh root@192.168.1.55

new-window NAME CMD:
  tmux new-w -t midi-tracker -n "{{NAME}}"
  tmux send-keys -t midi-tracker:"{{NAME}}" "{{CMD}}" ENTER

tmux:
  tmux new -ds midi-tracker -n "README"
  tmux send-keys -t midi-tracker:README 'nv ./README.md "+set wrap"' ENTER
  @just new-window "Edit" ""
  @just new-window "Run" ""
  @just new-window "git" "git status"
  tmux a -t midi-tracker

