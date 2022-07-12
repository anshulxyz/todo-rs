<p align="center">
  <img height="200" src="https://user-images.githubusercontent.com/6121530/178105221-a2128126-c8d2-43f1-9e2d-8a20606d4f24.png">
</p>

## Why

I was using these note taking apps to write my tasks on the daily-pages. The
problem was I wanted to see all the undone tasks until current-day. Which was
possible in a pure text-file based solutions. The tasks were getting lost in
many markdown files.

## What

A command-line TUI ToDo app, that uses SQLite for storing tasks.

## How

App is made using Rust. I am using [cursive](https://github.com/gyscos/cursive/)
library for the TUI view, and using [SeaORM](https://github.com/SeaQL/sea-orm)
as an ORM for SQLite.

## Run

```sh
git clone https://github.com/anshulxyz/todo-rs/
cd todo-rs/
echo "DATABASE_URL='sqlite://tasks.sqlite?mode=rwc'" > .env
make run
```

### Usage

- Press `q` to quit the app.
- Upon running the app, you will see all the undone tasks. And all the tasks you
finished _today_.
- Use arrow-keys to move up-and-down, press Enter or Space to mark a task done/undone
- Press `a` to add a task.

## Development

Install dev dependencies and run tests

```
make install-deps
make build
make test
```

## Demo

https://user-images.githubusercontent.com/6121530/178357996-c67a1c3d-72d7-44b8-81c7-2f822925acbf.mp4

