# Small tool for get ip information

## Install 
Needed install [rust](https://www.rust-lang.org/tools/install) 

```bash
git clone https://github.com/3JIou-home/rust-ip-info.git
cd ./rust-ip-info
cargo buld --release
./target/release/rust-ip-info --ip 1.1.1.1
```

## Example output

```bash
+--------------+-------------------------------------------+
| Name         | Response                                  |
+--------------+-------------------------------------------+
| Request ip   |                                   1.1.1.1 |
+--------------+-------------------------------------------+
| Country      |                                 Australia |
+--------------+-------------------------------------------+
| Region       |                                Queensland |
+--------------+-------------------------------------------+
| City         |                            South Brisbane |
+--------------+-------------------------------------------+
| Provider     |                           Cloudflare, Inc |
+--------------+-------------------------------------------+
| Organization | APNIC and Cloudflare DNS Resolver project |
+--------------+-------------------------------------------+
| AS           |                  AS13335 Cloudflare, Inc. |
+--------------+-------------------------------------------+
| Is mobile    |                                     false |
+--------------+-------------------------------------------+
| Is proxy     |                                     false |
+--------------+-------------------------------------------+
| Is hosting   |                                      true |
+--------------+-------------------------------------------+
```
