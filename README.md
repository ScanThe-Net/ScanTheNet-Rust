# ScanTheNet-Rust
Rust API Data Fetcher.

This project is a Rust application that retrieves and displays data from our API endpoint. 

## Features

- **API Interaction**: Makes GET requests to our API endpoint and retrieves JSON data.
- **Dynamic Entry Limit**: Users can specify the maximum number of data entries to display (up to 100).

## Requirements

- **curl**
- **rust**
- **cargo**

## Installation

### Install Dependencies

For Debian-based systems, you can install the required dependencies:

```
apt install curl
curl https://sh.rustup.rs -sSf | sh
```

Clone the Repository

```
git clone https://github.com/ScanThe-Net/ScanTheNet-Rust.git
cd ScanTheNet-Rust/ScanTheNet
```

Usage without build

Run it from the command line. Optionally, you can specify the maximum number of entries:

```
cargo run [maxEntries]
```

Build

```
cargo build
```

Usage with build

Run it from the command line. Optionally, you can specify the maximum number of entries:

```
cd ScanTheNet-Rust/ScanTheNet/target/debug
./ScanTheNet [maxEntries]
```

Replace [maxEntries] with a number between 1 and 100. If not specified, the default value is 100.

Example Output

When executed successfully, it will display results in the following format:

```
      _______                    _______ __           ____ __         __
     |     __|.----.---.-.----- |_     _|  |--.-----.|    |  |.-----.|  |_
     |__     ||  __|  _  |     |  |   | |     |  -__||       ||  -__||   _|
     |_______||____|___._|__|__|  |___| |__|__|_____||__|____||_____||____|

ID: "5702635"
Timestamp: "2024-11-17 09:26:45"
Source IP: "51.91.110.49"
Source Port: "41756"
Destination Port: "22"
Data: "Flags [S], seq 2981374044, win 29200, options [mss 1400,sackOK,TS val 1400581321 ecr 0,nop,wscale 7], length 0"
----------
```
