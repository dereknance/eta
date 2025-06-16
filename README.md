# Eta - η - Email Takehome Assignment

This project is an echo of the beginnings of a TUI email client the likes of [Mutt][mutt], [Aerc][aerc], or [Sup][sup].

## Configuration

`eta`'s configuration file, `eta.toml` is required in order to read SMTP connection information and send messages. `eta` will attempt to open `eta.toml` in the working directory from which `eta` is executed.

Only three properties are required, `host`, `username`, and `password`. A port of `465` is assumed, and may not be changed. For example:

```toml
host = "smtp.example.com"
username = "bob@example.com"
password = "badpassword"
```

## Run the project

Ensure that Rust and `cargo` are installed. Run the program using `cargo run --release`. Should it be so desired, copy the compiled binary out of the `target/` directory.

## Storage Backend

This version of `eta` uses SQLite as a fake backend in lieu of a bona-fide connection to an IMAP or JMAP server (this project chooses to exclude POP as an consideration).
It will automatically create a database file called `messages.db` in the working directory from which `eta` is executed.
`eta` will automatically create a `messages` table if it does not exist, and seed with a few sample messages it if there are no records in the table.

## Controls / Keybinds

`eta` predominantly attempts to use the home row of the right hand to navigate and change modes.
At any time, `Ctrl+c` may be pressed to close the application.
On the main page, `j` and `k` is used to select a message, `Enter` to read the selected message, `c` to compose a message, and `q` to quit the application.

When viewing a message, scrolling through the text vertically and horizontally is managed with `j`, `k`, `h`, `l`, or the arrow keys. Return to the main page with `q`.

On the composition page, `Tab` advances through each field, `Enter` selects a field for editing, `Esc` stops editing, and `Shift+s` will attempt to send the composed message to the configured SMTP server.

> Note that robust validation is not yet available when sending, so the program may crash if, for example, the `to` address is not in the _shape_ of an email address, at least according to the [lettre] library. If the program crashes but no prompt returns, slap `Ctrl-c` to make sure the program has actually stopped. Executing `reset` in the shell may also be required to rectify any anomalies.

> Also note that there is a bug with horizontal scrolling for messages with long lines.

After attempting to send a message, the user is returned to the main page. The success (or failure) of the sent message will be displayed as a status message in the lower-right area of the TUI.

## License

Copyright (c) Derek Nance <derek@dereknance.com>

This project is licensed under the GPLv3 license ([LICENSE] or <http://opensource.org/licenses/gpl-3-0>)

[LICENSE]: ./LICENSE
[aerc]: https://aerc-mail.org/
[lettre]: https://lettre.rs/
[mutt]: http://www.mutt.org/
[sup]: https://sup-heliotrope.github.io/