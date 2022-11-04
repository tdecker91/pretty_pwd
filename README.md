# Pretty pwd

Simple tool to enhance the output of the `pwd` command.

## Boring pwd
<img src="https://raw.githubusercontent.com/tdecker91/pretty_pwd/master/assets/pwd.png" />

## Awesome pwd!
<img src="https://raw.githubusercontent.com/tdecker91/pretty_pwd/master/assets/ppwd.png" />

you can optionally truncate the home directory.
<img src="https://raw.githubusercontent.com/tdecker91/pretty_pwd/master/assets/ppwd_home.png" />

## Installation
...coming soon

## Usage
...coming soon

### Customize
customize the foreground and background colors of different path segments using a file in your home directory. `~/.ppwd`

This is a config with the default values
```toml
truncate_home = false
tilde = "blue"
path_separator = "red"
dir_name = "cyan"
```

Possible keys
- truncate_home
- tilde
- tilde_bg
- path_separator
- path_separator_bg
- dir_name
- dir_name_bg

Possible colors
- red
- yellow
- green
- blue
- purple
- white
- black
- cyan