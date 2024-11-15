default:
  just -l

install-lib:
  pip uninstall -y tracker-backend && maturin develop

only-run:
  python ./gui/MIDI-Tracker.pygame

run-new: install-lib only-run

hardware-test: build-debug flash-adb

flash-hardware: build-release flash-adb

hardware-errors:
  adb shell cat /userdata/roms/ports/MIDI-Tracker/out.txt

hardware-reboot:
  adb reboot

flash-adb:
  adb push ./{gui/{MIDI-Tracker.pygame,midi_tracker},dist/tracker_backend-0.1.0-cp311-cp311-manylinux_2_17_aarch64.manylinux2014_aarch64.whl} /userdata/roms/ports/MIDI-Tracker/
  adb shell "cd /userdata/roms/ports/MIDI-Tracker/; .venv/bin/python -m pip install --force-reinstall --no-index ./tracker_backend-*aarch64.whl"

build-debug:
  PKG_CONFIG_SYSROOT_DIR=./cross-build-deps/aarch64 maturin build --out dist --find-interpreter --target aarch64-unknown-linux-gnu --zig

build-release:
  PKG_CONFIG_SYSROOT_DIR=./cross-build-deps/aarch64 maturin build --out dist --find-interpreter --target aarch64-unknown-linux-gnu --zig --release

setup-aarch64:
  mkdir -p ./cross-build-deps/aarch64/
  wget -nv -P ./cross-build-deps/aarch64/ http://mirror.archlinuxarm.org/aarch64/core/systemd-libs-256.7-1-aarch64.pkg.tar.xz 
  wget -nv -P ./cross-build-deps/aarch64/ http://mirror.archlinuxarm.org/aarch64/core/gcc-libs-14.1.1+r1+g43b730b9134-1-aarch64.pkg.tar.xz
  wget -nv -P ./cross-build-deps/aarch64/ http://mirror.archlinuxarm.org/aarch64/core/glibc-2.39+r52+gf8e4623421-1-aarch64.pkg.tar.xz
  wget -nv -P ./cross-build-deps/aarch64/ http://mirror.archlinuxarm.org/aarch64/core/linux-api-headers-6.10-1-aarch64.pkg.tar.xz
  wget -nv -P ./cross-build-deps/aarch64/ http://mirror.archlinuxarm.org/aarch64/core/python-3.12.7-1-aarch64.pkg.tar.xz
  wget -nv -P ./cross-build-deps/aarch64/ http://mirror.archlinuxarm.org/aarch64/core/libcap-2.71-1-aarch64.pkg.tar.xz
  cd ./cross-build-deps/aarch64; for f in $(ls *.pkg.tar.xz); do echo "extracting archiver: $f"; tar xf $f && rm $f; done

new-window NAME CMD:
  tmux new-w -t midi-tracker -n "{{NAME}}"
  tmux send-keys -t midi-tracker:"{{NAME}}" ". ./.venv/bin/activate" ENTER
  tmux send-keys -t midi-tracker:"{{NAME}}" "{{CMD}}" ENTER

tmux:
  tmux new -ds midi-tracker -n "README"
  tmux send-keys -t midi-tracker:README 'nv ./README.md "+set wrap"' ENTER
  @just new-window "GUI" "nv ./gui/MIDI-Tracker.pygame +'setfiletype python'"
  @just new-window "Edit" ""
  @just new-window "Run" ""
  @just new-window "git" "git status"
  tmux a -t midi-tracker

