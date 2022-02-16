# away

# Description
The purpose of the away utility to keep your computer from falling asleep while you are away.
It does this by repeatedly pressing the scroll-lock key for you.

# How to install
Download Windows away installer v0.1.5 from [here](https://s3-us-west-2.amazonaws.com/blog.carmanbabin.net/tools/away-0.1.5-setup.exe)

# How to use examples
Run till time:
```sh
away -t 5:30:pm
```

Run for duration of 1 hour and 30 minutes:
```sh
away -d 1:30:0
```

Run indefinitely
```sh
away -i
```

Press "q" at any time to quit the program. The window does not need to be active.

### TODO:
 - Add mouse movement detection
 - Add "key to press" to config file
