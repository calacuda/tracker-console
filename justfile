default:
  just -l

install-lib:
  pip uninstall -y tracker-backend && maturin develop

only-run:
  python ./gui/MIDI-Tracker.pygame

run-new: install-lib only-run

flash-scp:
  scp -r ./{gui/{MIDI-Tracker.pygame,midi_tracker},dist/tracker_backend-0.1.0-cp312-cp312-manylinux_2_*_aarch64.whl}  root@192.168.1.112:/userdata/roms/ports/MIDI-Tracker/
  ssh root@192.168.1.112 "cd /userdata/roms/ports/MIDI-Tracker/; .venv/bin/python -m pip install --force-reinstall --no-index ./tracker_backend-*aarch64.whl"

flash-adb:
  adb shell "mkdir /userdata/roms/ports/MIDI-Tracker/"
  adb push ./{gui/{MIDI-Tracker.pygame,midi_tracker},dist/tracker_backend-0.1.0-cp312-cp312-manylinux_2_*_aarch64.whl} /userdata/roms/ports/MIDI-Tracker/
  adb shell "cd /userdata/roms/ports/MIDI-Tracker/; .venv/bin/python -m pip install --force-reinstall --no-index ./tracker_backend-*aarch64.whl"

build:
  # PKG_CONFIG_SYSROOT_DIR=/opt/ArchARM maturin build --out dist --find-interpreter --target aarch64-unknown-linux-gnu
  PKG_CONFIG_SYSROOT_DIR=./cross-build-deps/aarch64 maturin build --out dist --find-interpreter --target aarch64-unknown-linux-gnu

build-release:
  # PKG_CONFIG_SYSROOT_DIR=/opt/ArchARM maturin build --out dist --find-interpreter --target aarch64-unknown-linux-gnu
  PKG_CONFIG_SYSROOT_DIR=./cross-build-deps/aarch64 maturin build --out dist --find-interpreter --release --target aarch64-unknown-linux-gnu --zig

flash: build-release flash-adb

kill:
  adb shell "PID=\$(ps aux | grep MIDI-Tracker.pygame | grep -v \"grep\" | awk -F ' ' '{ print \$2}'); echo \"killing PID: \$PID\"; kill \$PID"

# build-nmap:
#   PKG_CONFIG_SYSROOT_DIR="./cross-build-deps/" PKG_CONFIG_PATH="./cross-build-deps/usr/lib/pkgconfig" aarch64-linux-gnu-gcc -Os -I ./cross-build-deps/usr/include/python3.12 -o nmap nmap.c -L./cross-build-deps/usr/lib/ -lpython3.12 -L/usr/aarch64-linux-gnu/lib/ -lpthread -L/usr/aarch64-linux-gnu/lib/ -lm -L/usr/aarch64-linux-gnu/lib/ -lutil -L/usr/aarch64-linux-gnu/lib/ -ldl --sysroot=./cross-build-deps/

ssh:
  ssh root@192.168.1.112

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

cross-build-deps:
  # wget -P ./cross-build-deps/aarch64/ https://github.com/trimui/toolchain_sdk_smartpro/releases/download/20231018/SDL2-2.26.1.GE8300.tgz
  # wget -P ./cross-build-deps/aarch64/ http://mirror.archlinuxarm.org/aarch64/extra/jack2-1.9.22-1-aarch64.pkg.tar.xz
  # wget -P ./cross-build-deps/aarch64/ http://mirror.archlinuxarm.org/aarch64/extra/libsamplerate-0.2.2-3-aarch64.pkg.tar.xz
  # wget -P ./cross-build-deps/aarch64/ http://mirror.archlinuxarm.org/aarch64/extra/alsa-lib-1.2.14-1-aarch64.pkg.tar.xz
  # wget -P ./cross-build-deps/aarch64/ http://mirror.archlinuxarm.org/aarch64/core/gcc-libs-14.2.1+r753+g1cd744a6828f-1-aarch64.pkg.tar.xz
  # wget -P ./cross-build-deps/aarch64/ http://mirror.archlinuxarm.org/aarch64/core/glibc-2.41+r6+gcf88351b685d-1-aarch64.pkg.tar.xz
  # wget -P ./cross-build-deps/aarch64/ http://mirror.archlinuxarm.org/aarch64/core/gcc-14.2.1+r753+g1cd744a6828f-1-aarch64.pkg.tar.xz
  # wget -P ./cross-build-deps/aarch64/ http://mirror.archlinuxarm.org/aarch64/core/db5.3-5.3.28-5-aarch64.pkg.tar.xz
  cd ./cross-build-deps/aarch64/; echo "all archives"; ls -l *.tar.xz; echo "starting to unarchive..."; for f in $(ls *.tar.xz); do echo "extracting archiver: $f"; tar xf $f; rm $f; done
  cd ./cross-build-deps/aarch64/; ln -s usr/lib

