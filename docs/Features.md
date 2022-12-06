# Features

Cruster contains the following features (in terms of Rust):

- `rcgen-ca` - use `Rcgen` to build local CA;
- `crosstrem` - use `Crossterm` as Text User Interface backend (cross-platform);
- `default` - uncludes previous two features, enabled by default;
- `openssl-ca` - use `OpenSSL` to build local CA; requires `OpenSSL` (`libssl`) to be installed;
- `ncurses` - use `Ncurses` as Text User Interface backend; requires `Ncurses` (`libncurses`/`libncurses5`) to be installed;
- `termion` - use `Termion` as Text User Interface backend;
