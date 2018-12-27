# incredi panel

Floating window "panel". Instead of showing as a bar constantly on screen,
incredi will show as a floating window whenever an event happens.

incredi can be configured with a `.yaml` file:
```yaml
items:
# Call a script every 5 seconds, and place the output in the panel
- name: pulled-command
  script-path: path/to/script.sh
  interval-sec: 5.0

# Run a script in the background, and everytime it prints a line to stdout, put
# the output in the panel
- name: pushed-command
  script-path: path/to/script.sh
  # When the script prints a line, display the panel
  trigger-show: true

# Can specifiy commands in the configuration...
- name: pushed-command
  command: [tail, -f, /tmp/server.log]
# ...or write scripts in the configuration
- name: pulled-command
  interpreter: python
  script: |
    import time
    print(time.time())
  interval-sec: 1.0

# Display the panel in the top-left corner
anchor: top-left
```

The output of any scripts called should be XML:
```sh
#!/usr/bin/env sh

# Output can be plain text
echo "Hello, world!"

# If you want to colour the output, use the `color` tag
# Note: WIP
echo "Hello, <color hex='00ff00'>world!</color>"

# If you want to include an image, use the `image` tag
# Note: WIP
echo "Battery: <image src='battery-full.png'/>"
```

## Prerequisites
- `rustc` >= 1.31.0
- `xprop` >= 1.2.3
- `xdotool` >= 3.20160805.1
- `csfml` >= 2.5-2

## To do
- Add transparency to unused grid cells
- Add XML parsing of command output
- Add FontAwesome for icons
- Add images
