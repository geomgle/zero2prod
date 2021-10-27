#!/bin/sh

sigint_handler() 
{
    kill $PID
    exit
}

trap sigint_handler SIGINT

while true; do
    cargo run &
    PID=$!
    inotifywait src -r -e modify
    kill $PID
done
