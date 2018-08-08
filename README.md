# Bing-rs
Fetch wallpapers from Bing, made with rust.

## Why another Bing wallpaper fetcher?

why not!

## Usage:

     bing-rs [FLAGS] [OPTIONS]

    FLAGS:
        -d, --delete     delete current wallpaper (local)
        -h, --help       Prints help information
        -l, --local      fetch image from saved wallpapers
                           you must have at least one saved local Bing image 
        -r, --random     fetch a random image from Bing images or local wallpapers
                          example: bing -l -r
        -R, --recall     recall last wallpaper set (local)
        -t, --today      fetch today's image from bing (needs network)
        -V, --version    Prints version information

    OPTIONS:
        -n, --next <next>            get the nth next image (works on local mode only)
                                      example: bing-rs -n 3
        -p, --previous <previous>    get the nth previous image
                                     works a bit diffrently depending on local flag:
                                     if local set:
                                     previous command will be relative to the current set wallpaper.
                                     if local is not set:
                                     previous command will be relative to today's Bing wallpaper.

## Important

Bing-rs uses `gsettings` command to change wallpaper so its limited to gnome and varients (mate,etc..) for now.

## Credits

Got the idea from https://github.com/rjstyles/BingWallpaper .
