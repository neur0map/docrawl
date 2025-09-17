---
title: "Runtime Interaction | Nmap Network Scanning"
source_url: https://nmap.org/book/man-runtime-interaction.html
fetched_at: 2025-09-17T16:36:19.056164+00:00
---

[Download](https://nmap.org/download.html)[Reference Guide](https://nmap.org/book/man.html)[Book](https://nmap.org/book/)[Docs](https://nmap.org/docs.html)[Zenmap GUI](https://nmap.org/zenmap/)[In the Movies](https://nmap.org/movies/)

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 15. Nmap Reference Guide](https://nmap.org/book/man.html)
* Runtime Interaction

[Prev](https://nmap.org/book/man-misc-options.html)

[Next](https://nmap.org/book/man-examples.html)

Runtime Interaction[]()
----------

During the execution of Nmap, all key presses are captured. This allows you to interact with the program without aborting and restarting it. Certain special keys will change options, while any other keys will print out a status message telling you about the scan. The convention is that *lowercase letters increase* the amount of printing, and *uppercase letters decrease* the printing. You may also press ‘*?*’ for help.

`v` / `V`

Increase / decrease the verbosity level

`d` / `D`

Increase / decrease the debugging Level

`p` / `P`

Turn on / off packet tracing

`?`

Print a runtime interaction help screen

 Anything else

Print out a status message like this:

```
Stats: 0:00:07 elapsed; 20 hosts completed (1 up), 1 undergoing Service Scan
Service scan Timing: About 33.33% done; ETC: 20:57 (0:00:12 remaining)

```

---

[Prev](https://nmap.org/book/man-misc-options.html)Miscellaneous Options

[Up](https://nmap.org/book/man.html)Chapter 15. Nmap Reference Guide

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/man-examples.html)Examples