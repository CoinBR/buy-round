#!/bin/sh

local_env_file="env/local.env"
test_env_file="env/test.env"

local_db() {
  _db $local_env_file
}


test_db() {
  _db $test_env_file
}


_db() {
   env_file=$1; shift
   docker-compose --env-file $env_file down
   docker-compose --env-file $env_file up
}

local_server(){
  env `cat $local_env_file` cargo run
}

test() {
  :
  # aaa
}


"$@"
