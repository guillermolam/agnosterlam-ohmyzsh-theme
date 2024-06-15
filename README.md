# agnosterlam for oh-my-zsh

agnosterlam is a [oh-my-zsh shell](https://github.com/robbyrussell/oh-my-zsh) theme based on the
[Powerline Vim plugin](https://github.com/Lokaltog/vim-powerline) &
[Agnoster Theme](https://gist.github.com/agnoster/3712874).

It currently shows:
- Weather
- Location based on your IP
- Battery Life (in case of the laptop is not charging)
- Timestamp
- Current directory
- Git status
- User & Host status

## Project overview
```bash
weather-prompt/
├── install.sh
├── rust_script/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── zsh/
    └── weather.zsh
```

Clone the repository:

sh
Copy code
```
git clone https://github.com/yourusername/weather-prompt.git
cd weather-prompt
```
Make the install script executable and run it:

Copy code
```
chmod +x install.sh
./install.sh
```
This setup script checks for dependencies, installs them if necessary, compiles the Rust script, moves the executable to /usr/local/bin, and updates the user's .zshrc to include the custom prompt.








## Other color schemes

It's better to use agnosterlam with a Solarized dark color scheme or others dark color schemes.


## Requirements

In order to use the theme, you will first need:
