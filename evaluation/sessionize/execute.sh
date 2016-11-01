#!/bin/bash
set -euo pipefail

PREFIX=${1?"Usage: $0 [PREFIX] [THREADS] [TOPO]"}
THREADS=${2?"Usage: $0 [PREFIX] [THREADS] [TOPO]"}
TOPO=${3?"Usage: $0 [PREFIX] [THREADS] [TOPO]"}
PROCESSES=1

LOGS="logs/$(hostname)_$(date +%Y%m%d-%H%M%S)_p${PROCESSES}_t${THREADS}/"
PATH="$PATH:~/release"
export RUST_LOG=debug

mkdir -p "$LOGS"

coordinator &>"$LOGS/coordinator.log" &
coord=$!
sleep 3
executor &>"$LOGS/executor.log" &
executor=$!
sleep 3

queries='txdepth servicetop10 txsigtop10 txns msgspan msgcount sessionize'

for query in $queries; do 
    submit "$PROCESSES" "$THREADS"  ~/release/$query "$PREFIX" "$LOGS"
    taskset -pca "$TOPO" "$(pgrep $query)"
done

python2 ~/monitor.py $queries | tee "$LOGS/monitor.log"
kill $executor
kill $coord
