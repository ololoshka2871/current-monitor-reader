# Current monitor reader
Read data from [current-monitor-f0](https://github.com/ololoshka2871/current-monitor-f0) and print it to stdout.

# Display data
- Write to file
```shell
$ current-monitor-reader > file.txt
``` 

- Realtime graph
Using python progect [livechart](https://github.com/ololoshka2871/livechart)
```shell
$ current-monitor-reader | livechart -i 0.1 -L 1000
```
Here:
- `-i 0.1` - update interval in seconds
- `-L 1000` - total points limit
