#!/usr/bin/env bash

ls src/* | entr sh -c "clear && ./bin/build.sh"
