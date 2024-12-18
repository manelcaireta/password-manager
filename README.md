# Password Manager <!-- omit in toc -->

A simple CLI application to manage passwords written in [Rust](https://www.rust-lang.org/es).

## Contents <!-- omit in toc -->

- [Setup](#setup)
  - [Installation](#installation)
  - [Initialize application](#initialize-application)
- [Commands](#commands)

## Setup

### Installation

To install this project download this repository and install the package using [Cargo](https://doc.rust-lang.org/cargo/).

```bash
git clone https://github.com/manelcaireta/password-manager.git
cargo install --path ./password-manager
```

### Initialize application

To create all the necessary folders for the password manager run

```bash
pwm init
```

This will create a folder on `~/.passwords` where all passwords will be stored. This behaviour can be overridden using the `PASSWORD_HOME` environment variable.

## Commands

|    Command    | Description                                          |
| :-----------: | :--------------------------------------------------- |
|    `init`     | Initialize password manager                          |
|     `gen`     | Generate a random password without storing its value |
|     `new`     | Create and stores a new password                     |
|    `list`     | List all passwords                                   |
|     `get`     | Recover the value of a password                      |
|   `update`    | Update a password creating a new version             |
| `rm`/`remove` | Remove a password                                    |
|    `help`     | Show documentation                                   |

> [!WARNING]
> This application is for educational purposes and not suited for actual password management. Use it at your own risk.
