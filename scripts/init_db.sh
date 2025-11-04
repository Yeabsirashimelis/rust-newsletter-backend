#!usr/bin/env bash
set -x
set -eo pipefail

# check of a custom user has been set, otherwise default to "postgres"
DB_USER="${POSTGRES_USER:=postgres}"
# check if the custom password has been set, otherwise default to "password"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# check if the custom database name has been set, othewise default to "newsletter"
DB_NAME="${POSTGRES_DB:=newsletter}"
# check if the custom port has been set, otherwise default to "5432"
DB_PORT="${POSTGRES_PORT:=5432}"
# check if a custom host has been set, otherwise default to "localhost'
DB_HOST="${POSTGRES_HOST:=localhost}"
# launch postgres using docker
  docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres-N 1000
 # ^Increased maximumnumberof connectionsfor testingpurposes