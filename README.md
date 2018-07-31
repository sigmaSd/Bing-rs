# Bing-rs
Fetch wallpapers from Bing, made with rust.

# Why another Bing wallpaper fetcher?

why not!

# Usage:

    bing-rs [FLAGS] [OPTIONS]

FLAGS:

    -h, --help       Prints help information
    -l, --local      fetch image from saved wallpapers
                       you must have at least one saved local Bing image 
    -t, --today      fetch today's image from bing (needs network)
    -V, --version    Prints version information

OPTIONS:

    -n, --next <next>            get the nth next image (works on local mode only)
                                  example: bing-rs -n 3
    -p, --previous <previous>    get the nth previous image
                                  example: bing-rs -p 
    -r, --random                 fetch a random image from Bing images or local wallpapers
                                  example: bing -l -r


#Credits
Got the idea from https://github.com/rjstyles/BingWallpaper .
