/// This is the default configuration for Lightfetch.
///
/// We use this to set the default values for the configuration.
///
///
///
pub(crate) const DEFAULT: &str = r#"# Lightfetch config file.
# Automatically generated, you can change the path with the --config [PATH] launch argument.

[ GENERAL ]
# Should we center the art and fetchtext to each other?
# Default = true
auto center = true

# Disable variables? Not recommended though if you don't need them you will make it faster i guess...
# Default = true
enable variables = true

# Change the prefix and suffix here.
# The "var" is an example variable, which will get swapped out with the actual ones.
# Default = "{" , "}"
^ prefix = "{"
^ suffix = "}"

# Should only CAPSLOCK be allowed?
# Default = false
^ case sensitive = false
 
# Should we enable variables for case formatting?
# Default = true
enable case variables = true

# Change the prefix and suffix here.
# Usage: [U]i am not small![/U] -> I AM NOT SMALL!
# Default = "U", "L", "{" , "}"
^ uppercase letter = "U"
^ lowercase letter = "L"
^ case prefix = "[{letter}]"
^ case suffix = "[/{letter}]"

[ FETCH ]
# Customize your fetch formatting here.
# Make sure to check out the custom variables and colors!
# You can also use unicode symbols if supported by your terminal.

# Example " {LIGHT_GRAY}Hello, I'm a {B}{white}very{R} {LIGHT_GRAY}quick text!".
# The prefix and suffix ("{" & "}" can also be changed below.

# Important! The first line must be placed right after the equal / '=' symbol.
# text = user@host
# 	  Distro: Arch Linux
# 	  ...
text = {white}╭───────────────────────────────────────────╮{ignore}
	│ {blue}{USERNAME}{gray}@{white}{HOSTNAME} {fill} │
    │ {gray} {blue}github {gray}~ {white}https://www.github.com/bwte {fill} │
    │ {blue}distro {gray}~ {white}[L]{DISTRO_NAME}[/L] {fill} │
    │ {blue}kernel {gray}~ {white}{KERNEL} {fill} │
    │ {blue}shell {gray}~ {white}{SHELL} {fill} │
    │ {blue}term {gray}~ {white}{TERMINAL} {fill} │
	│ {blue}pkgs {gray}~ {white}{PACKAGES} {fill} │
	│ {blue}cpu {gray}~ {white}[L]{cpu_model}[/L] {fill} │
    │ {white}omg dynamic 😳 blazing fast 🔥 {fill} │
    │ {white}no dependencies 😶 open source 🥵 {fill} │
	╰───────────────────────────────────────────╯{ignore}

# Change the gap between image and text here.
# Default = "   "
gap = "   "
# Should we swap the position of the text and art?
# Default = false
reverse = false

# Do we want some space on the left or the right?
# Default = 0 on both. TODO: HABS KAPUTT GEMACHT XDD
left padding = 0
right padding = 0
	
[ ART ]

# What art mode do you want to use?
# More advanced settings are found in their respective section.
# Too speed image mode up, i recommend enabling caching.
# Valid options: ascii / image.
# Default = ascii
mode = image

# The art that should be displayed here.
# Syntax is the same as the fetch-text. [Formatting && Variables] supported!
# Formatting syntax needs to be correct, otherwise any file type allowed.
# For images it obviously needs to be an image. Pixel Art is recommended!
# Use "~" for the HOME directory.
# Default = "~/.config/lightfetch/ascii/default.ini"
path = "~/.config/lightfetch/images/default.png"


[ IMAGE ]

# Choose your image filter here.
# Options: Nearest, Gaussian, Triangle, Catmull, Lanczos.
# Otherwise will fallback to Nearest.
# Default = Nearest
filter = Gaussian

# This will adjust the size of the ascii image.
# Aspect ratio will be respected.
# Default = 15
size = 15

[ ADVANCED MODE ]

# TODO!

[ MEMORY ]
# Number rounding strength.
# Default = 2
rounding = 2

# Below you are able to change the formatting of the size values.
# Default = " KB", " MB", " GB"
kb = " KB"
mb = " MB"
gb = " GB"

[ UPTIME ]
# Should we use a custom suffix?
# Example: [0 {DAY}s]
# Default: true
suffix = true

# Add plurals if value is more than 1?
# Default = true
^ plurals = true
 
# What character should we add?
# Default = "s"
^ character = "s"

hide if zero = true

# Change the formatting of the units here.
# Also determine the gap with spaces.
# Default = " d", " h", " h", " s"
day = " d"
hour = " h"
minute = " m"
second = " s"

[ CACHE ]
# EXPERIMENTAL SETTINGS!
# If you don't understand what this stuff means, don't change it!
# Enable caching for images and certain system values.
# This will make image fetching a significantly faster and skip values if cached and matched conditions.
# Default = true
enable = true

# Enable caching for images to not process them each time. [RECOMMENDED]
# Default = true
^ images = true

# Enable caching for certain system variables. [RECOMMENDED]
# Default = true
^ variables = true

# Should we enable clearing? We don't want this to pile up and take too much disk space.
# Default = true
clear it? = true
 	
# Set to 0 to disable.
# Default = 20
^ delete oldest if more than = 20
 		
# Delete caches of variables after an x amount of time.
# After that it will get the values like usual and cache it again.
# Default = 24
recache variables if older than x hours = 24

# Change the cache file path here.
# Use "~" for the HOME directory.
# Default = "~/.config/lightfetch/cache"
^ path = "~/.config/lightfetch/cache"
"#;
