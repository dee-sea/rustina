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
Usage: rustina <ip addr> <port> <binary to execute>
        # Manual mode : Only genetrates docx and html files
Usage: rustina
        # Manual mode : Only genetrates docx and html files pointing to 127.0.0.1:8080 and launching calc.exe
Usage: rustina --server
        # Server mode : Genetrates docx and html files and bind a web server to localhost:8080, the exploit launches calc.exe
Usage: rustina --server <binary to execute>
        # Server mode : Genetrates docx and html files and bind a web server to localhost:8080
Usage: rustina --server <network interface> <binary to execute>
        # Server mode : Genetrates docx and html files and bind a web server to iface_ip_addr:8080
Usage: rustina --help
```

