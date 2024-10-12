# RUSCAN
Port scanner!

## Requirements for Compilation

- `rustc`, `cargo`

## Instalation
Copy and paste this into your terminal
```shell
git clone https://github.com/sharkvdwho/ruscan && cd ruscan && make install
```
<span style="color:red">**Warning:**</span> Make sure to add /home/<you_user>/.cargo/bin to your PATH variable or just move .cargo/bin/ruscan to ~/.local/bin/      

## Usage

```shell
Usage: ruscan.exe ps [OPTIONS] <ADDR>

Arguments:
  <ADDR>  ip addres of the target network. Example: ruscan ps <192.168.0.1>

Options:
  -i, --ip <IP>          ip addres of the target network. Example: ruscan ps <127.0.0.1>
  -d, --domain <DOMAIN>  domain name of the target network. Example: ruscan ps <localhost>
  -p, --port <PORT>      single port to be scaned on the target network. Example: ruscan ps target_address -p <443>
  -r, --range <RANGE>    range of ports to be scaned on the target network. Example: ruscan ps target_address -r <1-65535>
  -l, --list <LIST>      list of ports to be scaned on the target network. Example: ruscan ps target_address -l <21,80,443>
  -h, --help             Print help
