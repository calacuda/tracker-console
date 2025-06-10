_:
  just -l

install-lib:
  pip uninstall -y midi-band && maturin develop

only-run:
  python ./pygame-frontend/MIDI-Tracker.pygame

run-new: install-lib only-run

new-window NAME CMD:
  tmux new-w -t midi-tracker -n "{{NAME}}"
  tmux send-keys -t midi-tracker:"{{NAME}}" ". ./.venv/bin/activate" ENTER
  tmux send-keys -t midi-tracker:"{{NAME}}" "{{CMD}}" ENTER

tmux:
  tmux new -ds midi-tracker -n "README"
  tmux send-keys -t midi-tracker:README 'nv ./README.md "+set wrap"' ENTER
  # @just new-window "GUI" "nv ./gui/MIDI-Tracker.pygame +'setfiletype python'"
  @just new-window "Edit" ""
  @just new-window "Run" ""
  @just new-window "Git" "git status"
  @just new-window "Misc" ""
  tmux a -t midi-tracker

