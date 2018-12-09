# incredi panel

Floating window "panel". Instead of showing as a bar constantly on screen,
incredi will show as a floating window whenever an event happens.

incredi can be configured with a `.yaml` file:
```yaml
items:

# Various built in items
- date-time
- battery
- cpu-usage

# Call a script every 5 seconds, and place the output in the panel
- name: script
  path: path/to/script.sh
  interval-sec: 5

# Run a script in the background, and everytime it prints a line to stdout, put
# the output in the panel
- name: background-script
  path: path/to/script.sh
  # When the script prints a line, display the panel
  triggers-show: true

# Display the panel in the top-left corner
anchored: "top-left"
```

The output of any scripts called should be XML:
```sh
#!/usr/bin/env sh

# Output can be plain text
echo "Hello, world!"

# If you want to colour the output, use the `color` tag
echo "Hello, <color hex='00ff00'>world!</color>"

# If you want to include an image, use the `image` tag
echo "Battery: <image src='battery-full.png'/>"
```
