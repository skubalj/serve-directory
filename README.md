
`serve-directory`
===============

`serve-directory` is a simple command line utility to serve static files from the command line. In
the general case, we really mean *simple*. To start a web server serving the contents of the current
directory and its subdirectories, just run the executable without arguments.

```bash
$ serve-directory
```

Files are accessed based on their relative position to the target folder. Consider the following 
(abbreviated) rust project structure:
```text
Project Structure:          Route:
 .                          /
 ├─ src/                    /src
 │  ├─ main.rs              /src/main.rs
 │  └─ stuff.rs             /src/stuff.rs
 ├─ target/                 /target
 │  └─ debug/               /target/debug
 │     └─ app.exe           /target/debug/app.exe
 └─ README.md               /README.md
```

As you can see, the correlation between the file structure and the route is quite natural.

## Why Should I Care?
Sometimes, configuring network folders is way more work than it's worth to simply pass a couple 
files between machines, especially if you're moving files to or from a headless system. While you
could use a tool like SFTP, `serve-directory` provides a nice graphical way to browse the filesystem
in a web browser that should prove less daunting for new users. It also allows downloading files to 
multiple clients without having to log in through SSH multiple times.

It's also easy and doesn't require any additional setup (provided the user has a compiled 
executable). Most users already have a browser on their machine. And, in the event that the user 
does not, they can use `curl` or `wget` to download files over HTTP.

While it's evident that there's a use case for a simple and erogonomic HTTP file server, why 
would you use `serve-directory` instead of one of the more established options like the 
[serve](https://www.npmjs.com/package/serve) package for NodeJS? I will concede that in many cases, 
serve will probably suit your needs better. However, because `serve-directory` is written in Rust
and natively compiled, it can be used and distributed in a single executable without a runtime. 

## Installation and Usage
The recommended way to install the latest version of this application from 
[crates.io](https://crates.io/crates/serve-directory) is using cargo:
```bash
$ cargo install serve-directory
```

Alternatively, you can build it from source with cargo, then add the application to your `PATH`.
```bash
$ cargo build --release
$ cp ./target/release/serve-directory ~/bin/serve-directory
```

Different features of the application are controlled through command line arguments. You can see
a full list of arguments using the `-h` or `--help` flag.
```bash
$ serve-directory --help
```

In general, simply point the application at the desired directory. The program will attempt to 
determine your current IP address and bind to that interface. To change the port that the 
application binds to, use the `-p` flag. For example, this command will serve content in the 
`~/www` directory on port 80.

> Note that in practice, you may have to run the program as root in order to access lower-numbered
> ports. Simply add a `sudo` to the start of the command.

```bash
$ serve-directory -p 80 ~/www
```

| Option                | Description                                           |
| :-------------------- | :---------------------------------------------------- |
| `[FOLDER]`            | The directory to serve (default: `.`)                 |
| `-p`, `--port <PORT>` | The port to bind to (default: `8080`)                 |
| `-l`, `--localhost`   | Bind to `127.0.0.1` instead of the external interface |
| `-v`, `--verbose`     | Log every request                                     |
| `-h`, `--help`        | Print the help                                        |
| `-V`, `--version`     | Print the version                                     |

## Acknowledgment
A special thank you to the [serve](https://www.npmjs.com/package/serve) project for providing
inspiration for this tool! Thank you as well to Material-UI for the icons used on the directory 
webpage!

This project is made possible by the work of the following great libraries:
| Crate                                                       | Owners                      |
| :---------------------------------------------------------- | :-------------------------- |
| [build_html](https://crates.io/crates/build_html)           | Joseph Skubal               |
| [env_logger](https://crates.io/crates/env_logger)           | The Rust Project Developers |
| [local_ipaddress](https://crates.io/crates/local_ipaddress) | Egmkang Wang                |
| [log](https://crates.io/crates/log)                         | The Rust Project Developers |
| [pico-args](https://crates.io/crates/pico-args)             | Yevhenii Reizner            |
| [tokio](https://crates.io/crates/tokio)                     | Tokio Contributors          |
| [warp](https://crates.io/crates/warp)                       | Sean McArthur               |

And, of course, the [Rust language](https://rust-lang.org)!

## License
This project is licensed under the [MIT license](https://mit-license.org). Feel free to use and 
remix it as you wish.

Copyright (C) 2020-21 Joseph Skubal
