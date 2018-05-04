<p align="center">
   <img src="assets/logo.png" alt="haproxy-alert" title="haproxy-alert" />
</p>

A tool to convert TOML into issues. Supports only trello for now.

## Installation

### Requirements

* Linux
* Rust (tested on rustc 1.25.0-nightly (0c6091fbd 2018-02-04))
* Node (tested on v8.9.1)
* [trello-cli](https://github.com/mheap/trello-cli)

```bash
$ npm install -g trello-cli
$ trello set-auth [your trello key]
$ cargo install --git https://github.com/blockexplorer/issueboss
```

## Usage

```bash
$ issueboss trello -b board -l list ./issues.toml
```

## License

Distributed under the MIT license. See `LICENSE` for more information.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.
