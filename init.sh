#!/bin/sh

sigint_handler() 
{
    kill $PID
    exit
}

trap sigint_handler SIGINT

while true; do
    clear
    cargo sqlx prepare -D postgres://geomma:samplepassword@db:5432/newsletter -- --bin zero2prod --target-dir /home/target
    cargo run --target-dir /home/target & 
    PID=$!
    inotifywait src -r -e close_write
    kill $PID
done

