# Fruity

Fruity is a GUI for [rCAS](https://github.com/nw-rs/rcas).

## Setup

Fruity compiles to wasm in order to be served as a webpage. To compile to wasm, you must install the wasm32-unknown-unknown target:

```
rustup target add wasm32-unknown-unknown
```


You also need to install trunk in order to serve the webpage:

```
cargo install --locked trunk
```


## Running

When you're ready to compile and run Fruity, trunk can handle everything for you:

```
trunk serve
```

## License
Copyright (C) 2022 Alessandra Simmons & Fruity Contributors

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program. If not, see https://www.gnu.org/licenses/.