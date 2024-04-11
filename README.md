# ele-backup
My first attempt at something useful with Rust. Needless to say still a Work in Progress (WIP).<br> <br>
To compile from source ensure cargo is installed on the system . Clone the repo and run `cargo build` binary will be available `target/debug/ele` .

## USAGE
This package is to be used on Fedora Linux. Timeshift must be installed and configured. 

### Update
Update the system with `dnf` after a successful timeshift snapshot is created
```
sudo ele update
```
## Install 
Install a package with `dnf` after a successful timeshift snapshot is created. 
```
sudo ele install <package>
```

