#!/usr/bin/env bash

vsystemfd --no-pid -s http::3000 -- cargo watch -x "run --bin backend"
