
# cfm

## Why?

There are [many][0] [non][1]-[GUI][2] [file][3] [managers][4]. Unfortunately, they
are all [TUIs][5] which means that they miss out on many of the benefits of
CLIs: efficiency, scriptability, and speed. Although I've never seen it online,
this really comes down to the CLI vs TUI debate.

`cfm` aims to make file management using a CLI more efficient than it was
previously possible with the coreutils and other CLI tools. It works by being a
wrapper, around any already existing command, that expands file paths using
some predefined rules.

## License and Copyright

`cfm` is licensed under the GNU GENERAL PUBLIC LICENSE Version 3.

Copyright © 2021 Nils André

[0]: https://github.com/ranger/ranger
[1]: https://github.com/jarun/nnn
[2]: https://vifm.info/
[3]: https://github.com/dylanaraps/fff
[4]: https://midnight-commander.org/
[5]: https://en.wikipedia.org/wiki/Text-based_user_interface
