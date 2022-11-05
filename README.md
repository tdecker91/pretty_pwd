# Pretty pwd
Simple tool to enhance the output of the `pwd` command.

This tool was mostly an opportunity for me to learn the basics of Rust. Use if you'd like. Open to any contributions you'd see fit.

## Boring pwd
<img src="https://raw.githubusercontent.com/tdecker91/pretty_pwd/master/assets/pwd.png" />

## Awesome pwd!
<img src="https://raw.githubusercontent.com/tdecker91/pretty_pwd/master/assets/ppwd.png" />

you can optionally truncate the home directory.
<img src="https://raw.githubusercontent.com/tdecker91/pretty_pwd/master/assets/ppwd_home.png" />

## Installation
Install on OSX using homebrew
```bash
> brew tap tdecker91/ppwd
> brew install ppwd
```

## Usage
run the ppwd command
```
> ppwd
```

OR, even better. Create an alias for pwd. Add this alias to your bashrc file.
```
> alias pwd="ppwd"
> pwd
```

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
