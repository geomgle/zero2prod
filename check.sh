#!/bin/sh

sigint_handler() 
{
    kill $PID
    exit
}

trap sigint_handler SIGINT

while true; do
    clear
    cargo check 
    inotifywait src -r -e modify
done
