---
title: "Command-line Options | Nmap Network Scanning"
source_url: https://nmap.org/book/zenmap-command-line.html
fetched_at: 2025-09-17T16:46:52.578050+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 12. Zenmap GUI Users' Guide](https://nmap.org/book/zenmap.html)
* Command-line Options

[Prev](https://nmap.org/book/zenmap-conf.html)

[Next](https://nmap.org/book/zenmap-history.html)

Command-line Options
----------

[]()[]()

 Being a graphical application, most of Zenmap's functionality is exposed through its graphical interface. Zenmap's command-line options are given here for completeness and because they are sometimes useful. In particular, it's good to know that the command **zenmap *`<results file>`*** starts Zenmap with the results in *`<results file>`* already open.

### Synopsis ###

`zenmap` [ *`<options>`* ] [ *`<results file>`* ]

### Options Summary ###

`-f`, `--file *`<results file>`*`[]()[

Open the given results file for viewing. The results file may be an Nmap XML output file (`.xml`, as produced by **nmap -oX**), or a file previously saved by Zenmap.

]()[`-h`, `--help`]()[]()[

Show a help message and exit.

]()[`--confdir *`<dir>`*`]()[]()[

Use *`<dir>`* as the per-user configuration directory.

]()[`-n`, `--nmap *`<Nmap command line>`*`]()[]()[

Run the given Nmap command within the Zenmap interface. After `-n` or `--nmap`, every remaining command line argument is read as the command line to execute. This means that `-n` or `--nmap` must be given last, after any other options. Note that the command line must include the **nmap** executable name: **zenmap -n nmap -sS target**.

]()[`-p`, `--profile *`<profile>`*`]()[]()[

Start with the given profile selected. The profile name is just a string: `"Regular scan"`. If combined with `-t`, begin a scan with the given profile against the specified target.

]()[`-t`, `--target *`<target>`*`]()[]()[

Start with the given target. If combined with `-p`, begin a scan with the given profile against the specified target.

]()[`-v`, `--verbose`]()[]()[

Increase verbosity (of Zenmap, not Nmap). This option may be given multiple times for even more verbosity printed to the console window used to start Zenmap.

]()

### [Error Output]() ###

[]()

 If Zenmap happens to crash, it normally helps you send a bug report with a stack trace. Set the environment variable `ZENMAP_DEVELOPMENT`[]() (the value doesn't matter) to disable automatic crash reporting and have errors printed to the console. Try the Bash shell command **ZENMAP\_DEVELOPMENT=1 zenmap -v -v -v** to get a useful debugging output.

 On Windows, standard error is redirected to the file `zenmap.exe.log` in the same directory as `zenmap.exe` rather than being printed to the console.

---

[Prev](https://nmap.org/book/zenmap-conf.html)Description of zenmap.conf

[Up](https://nmap.org/book/zenmap.html)Chapter 12. Zenmap GUI Users' Guide

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/zenmap-history.html)History