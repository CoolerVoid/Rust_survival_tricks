# Rust simple UserManager CRUD 

I create this example to study Rust with ORM.

if u dont have Rust, install him following this steps:
```

To install Rust and modules
   $ source $HOME/.cargo/env
   $ rustup default stable
   $ cargo install diesel_cli --no-default-features --features sqlite


```

To run program look that following:

```
$ diesel migration run --database-url database/test.db
$ cargo build
$ cargo install --path . --force --root out

```

Test Binarys:

```
At the Dir of USerManagerCrud, execute each binarys to understand the program...

$ ./out/bin/add_user
$ ./out/bin/show_users
...
Read the code to know each step of CRUD

```

Thank you to read and test my code....

Author: CoolerVoid
E-mail: coolerlair@gmail.com
Date: 06/03/2019

