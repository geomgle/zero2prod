#!/bin/sh

sigint_handler() 
{
    kill $PID
    exit
}

trap sigint_handler SIGINT

while true; do
    cargo check 
    inotifywait src -r -e modify
done
