#!/bin/bash
set -eu

cd webapp
basic-http-server --addr 127.0.0.1:3000 .
