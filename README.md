# bar-rs

A simple rust program that I use to output my status bar on sway.  
It is used in the sway config as so: 
```
status_command while ~/.config/sway/bar-rs; do sleep 1; done
```
Current output is:
```
  Sun Nov 10 - 10:47 AM | 30%   | 31% 󰁽
```

It is untested on other systems, assumes the presence of a battery, speakers.