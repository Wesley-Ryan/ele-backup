# ele-backup
A first attempt at something useful with Rust. Needless to say (WIP).<br> <br>
To compile from source ensure cargo is installed on the system . Clone the repo and run `cargo build` binary will be available `target/debug/ele` .

## USAGE

### Update
Update the system after a successful timeshift snapshot is created
```
sudo ele update
```
## Install 
Install a package using dnf after a successful timeshift snapshot is created. 
```
sudo ele install <package>
```

