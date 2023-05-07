File content:

```py
1 import datetime
2 
3 def utc_now() -> datetime.datetime:
4     return datetime.datetime.now(tz=datetime.timezone.utc)
5 
6 print(utc_now())
```

New file content:

```py
1 import datetime
2 
3 
4 def utc_now() -> datetime.datetime:
5     return datetime.datetime.now(tz=datetime.timezone.utc)
6 
7 
8 print(f"Current time: {utc_now()}")
9 
```

Patch contents:

```diff
author: ItsAleph [igorekkrupskij@gmail.com]
date: 666
message: Switch to Python

= update: main.py >
2++ 
5++
6 + print(f"Current time: {utc_now()}")
6 - print(utc_now())
= update: main.py >

```
