## USAGE

```
$ cargo run -- --path=db.pmanager list
$ cargo run -- --path=db.pmanager insert --key github.com --value user:pass
```

![](/img/keyvaldb.png)


```
key_value_database 0.1.0
* usage: * pmanager get --key=<key> * pmanager insert --key=<key> --value=<value> * pmanager delete
--key=<key> * pmanager update --key=<key> --value=<value> *

USAGE:
    key_value_database --path <PATH> [SUBCOMMAND]

OPTIONS:
    -h, --help           Print help information
    -p, --path <PATH>    Db path
    -V, --version        Print version information

SUBCOMMANDS:
    delete    Delete a key value pair from database
    get       Get value by key from database
    help      Print this message or the help of the given subcommand(s)
    insert    Insert a key value pair to database
    list      List every entry from the databse
    update    Update a record from database
```