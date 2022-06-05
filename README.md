# rustina
Follina PoC written in rust

Quick POC to replicate the 'Follina' Office RCE vulnerability for testing purposes. This has been inspiered by chvancooten/follina.py and was a first attempt to code in Rust.

> ⚠ DO NOT USE IN PRODUCTION

> ⚠⚠⚠ DO NOT USE IF NOT EXPLICITLY AUTHORISED TO DO SO!!!

## Usage:
 $ rustina -h                                                                                      
Usage: rustina [Options]

Options:
        --server=interface  # Bind server to IP address of provided interface
                            # Default value "lo"
        --manual=ipadr      # Manual mode : Only generate docx and html files without binding a server
                            # Default value "127.0.0.1"
        --port=portnumber   # Bind server to provided port
                            # Default value "8080"
        --binary=binarypath # Make a payload to execue binarypath on the victime computer
                            # Default value "\\\\windows\\\\system32\\\\calc"
                            # Binary path should not include the file extention e.g. .exe
                            # On linux binarypath should be double excaped:
                            # e.g. \\\\windows\\\\system32\\\\calc
                            # On windows binarypath should be excaped:
                            # e.g. \\windows\\system32\\calc
        -h or --help        # print this message§
```
```

