#!/bin/sh

sigint_handler() 
{
    kill $PID
    exit
}

trap sigint_handler SIGINT

while true; do
    cargo test -- --show-output 
    inotifywait tests -r -e modify
done
