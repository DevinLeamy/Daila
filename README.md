# _Daila_

> _(Daily + Data)_

[![Crates.io](https://img.shields.io/crates/v/daila.svg)](https://crates.io/crates/daila)

<p>
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> 
</p>

**_Daila_** is a command line interface for quickly and easily recording and visualizing daily data.

## Installation

#### Cargo

```bash
$ cargo install daila
```

#### From source

```bash
$ git clone https://github.com/DevinLeamy/daila
$ cd daila
$ cargo build [--release]
```

Then add '**_$PWD/target/debug/_**' or '**_$PWD/target/release/_**' to your path.

## Usage

```bash
$ daila
```

#### Controls

-   `Arrow keys`: Change the selected activity
-   ` `: Toggle the selected activity
-   `e/x`: edit/delete the selected activity
-   `c`: Create a new activity type
-   `a/d/t`: Change day (prev/next/today)
-   `s`: Save and quit
-   `q`: Quit

#### Demo

https://user-images.githubusercontent.com/45083086/231334889-779508bd-bc84-4ed1-99a5-de81692bae40.mov
