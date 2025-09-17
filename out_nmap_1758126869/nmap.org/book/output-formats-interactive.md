---
title: "Interactive Output | Nmap Network Scanning"
source_url: https://nmap.org/book/output-formats-interactive.html
fetched_at: 2025-09-17T16:47:28.305537+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 13. Nmap Output Formats](https://nmap.org/book/output.html)
* Interactive Output

[Prev](https://nmap.org/book/output-formats-commandline-flags.html)

[Next](https://nmap.org/book/output-formats-normal-output.html)

Interactive Output
----------

[]()

Interactive output is what Nmap prints to the
stdout stream,[]()which usually appears on the terminal window you executed Nmap from.
In other circumstances, you might have redirected stdout to a file or
another application such as Nessus or an Nmap GUI may be reading the
results. If a larger application is interpreting the results rather
than printing Nmap output directly to the user, then using the XML
output discussed in [the section called “XML Output (`-oX`)”](https://nmap.org/book/output-formats-xml-output.html) would
be more appropriate.

This format has but one goal: to present results that will be
valuable to a human reading over them. No effort is made to make
these easily machine parsable or to maintain a stable format between
Nmap versions. Better formats exist for these things. The toughest
challenge is deciding which information is valuable enough to print.
Omitting data that a user wants is a shame, though flooding the user
with pages of mostly irrelevant output can be even worse. The
verbosity, debugging, and packet tracing flags are available to shift
this balance based on individual users' preferences.

This output format needs no extensive description here, as most
Nmap examples in this book already show it. To understand Nmap's
interactive output for a certain feature, see the section of this
book dedicated to that feature. Typical examples of interactive
output are given in [Example 13.3, “Interactive output without verbosity enabled”](https://nmap.org/book/output-formats-commandline-flags.html#output-formats-ex-nonverbose)and [Example 13.4, “Interactive output with verbosity enabled”](https://nmap.org/book/output-formats-commandline-flags.html#output-formats-ex-verbose).

---

[Prev](https://nmap.org/book/output-formats-commandline-flags.html)Command-line Flags

[Up](https://nmap.org/book/output.html)Chapter 13. Nmap Output Formats

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/output-formats-normal-output.html)Normal Output (-oN)