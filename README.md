# rustina
Follina PoC written in rust

Quick POC to replicate the 'Follina' Office RCE vulnerability for testing purposes. This has been inspiered by chvancooten/follina.py and was a first attempt to code in Rust.

> ⚠ DO NOT USE IN PRODUCTION

> ⚠⚠⚠ DO NOT USE IF NOT EXPLICITLY AUTHORISED TO DO SO!!!

## Usage:

```
$ follina --help
****************************************************************
*                                                              *
*                           Follina                            *
*                                                              *
*                Good thing we disabeled macros                *
*                                                              *
****************************************************************
Usage: target/release/follina <ip addr> <port> <binary to execute>             # Manual mode : Only genetrates docx
and html files
Usage: target/release/follina --server <binary to execute>                     # Server mode : Genetrates docx and html files and bind a web server to localhost:8080
Usage: target/release/follina --server <network interface> <binary to execute> # Server mode : Genetrates docx and html files and bind a web server to iface_ip_addr:8080
Usage: target/release/follina --help                                           # Print this message.
```

