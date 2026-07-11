mod app 'crates/xpic-app/justfile'
mod cli 'crates/xpic/justfile'
mod spotlight 'crates/spotlight/justfile'

set shell := ["nu", "-c"]

default:
  @just --list

build *args: (cli::build args) (spotlight::build args) (app::build args)

release: (build "--release")
  mkdir bin
  mv target/release/xpic.exe bin/xpic-cli.exe
  mv target/release/xpic-app.exe bin/Xpic.exe
