# clinbox

[![Crates.io Version](https://img.shields.io/crates/v/clinbox?style=for-the-badge&color=%23FF0060)](https://crates.io/crates/clinbox)

Open source ommand line utility to generate and access disposable email addresses, written in Rust.

## Installation

Install using cargo:

```bash
cargo install clinbox
```

You can also download a compiled binary from the [releases](https://github.com/ooojustin/clinbox/releases) page.

## Usage

You can get started by using the `clinbox show` comand to see your random email address, and list the emails in your inbox.

By default you'll see your 5 most recent emails, but you can specify a count (ex: `clinbox show -c 10`) to change the count.

## Commands

| Command | Description                                                                                   |
| ------- | --------------------------------------------------------------------------------------------- |
| show    | Show inbox information and list emails.                                                       |
| open    | Open a specific email by providing the ID.                                                    |
| next    | Wait for a new email to be received and automatically open it.                                |
| copy    | Copy email address to clipboard.                                                              |
| delete  | Delete the current inbox and automatically generate new email address.                        |
| expires | Display the duration until the current inbox expires.                                         |
| website | Open the [website](https://www.disposablemail.com/) that this program uses behind the scenes. |
| github  | Open the GitHub repository for this application. _Welcome!_                                   |
| help    | Print this nifty list of commands in your console.                                            |

To see additional information about a specific command (and arguments), you can use the `-h` flag.

For example, `clinbox next -h` will provide the following information:

```
@justin ➜ clinbox git(master): clinbox next -h
Wait for a new email to be received and automatically open it

Usage: clinbox.exe next [OPTIONS]

Options:
  -t, --timeout <TIMEOUT>    The maximum amount of time to wait in seconds [default: 120]
  -i, --interval <INTERVAL>  The interval in between refreshing emails in seconds [default: 10]
  -h, --help                 Print help
```

## Questions/Suggestions

If you have any questions about this program or want to provide feedback, feel free to:

- ❓ [Create an issue](https://github.com/ooojustin/clinbox/issues) on GitHub.
- 📫 Reach me via email: [justin@garofolo.net](mailto:justin@garofolo.net)
