#!/bin/bash

# cargo sqlx prepare -D postgres://geomma:samplepassword@db:5432/nesletter -- --bin zero2prod
# cargo watch --watch src --exec run
cargo watch --watch src -x 'sqlx prepare -D postgres://geomma:samplepassword@db:5432/nesletter -- --bin zero2prod' -x 'run' 
