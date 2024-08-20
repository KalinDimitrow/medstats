#!/usr/bin/env bash

diesel migration generate --diff-schema $1
