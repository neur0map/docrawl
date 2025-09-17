---
title: "Script Language | Nmap Network Scanning"
source_url: https://nmap.org/book/nse-language.html
fetched_at: 2025-09-17T16:43:35.726500+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 9. Nmap Scripting Engine](https://nmap.org/book/nse.html)
* Script Language

[Prev](https://nmap.org/book/nse-script-format.html)

[Next](https://nmap.org/book/nse-scripts.html)

Script Language
----------

[]()

 The core of the Nmap Scripting Engine is an embeddable Lua interpreter. Lua is a lightweight language designed for extensibility. It offers a powerful and well-documented API for interfacing with other software such as Nmap.

[]()

 The second part of the Nmap Scripting Engine is the NSE Library, which connects Lua and Nmap. This layer handles issues such as initialization of the Lua interpreter, scheduling of parallel script execution, script retrieval and more. It is also the heart of the NSE network I/O framework and the exception handling mechanism. It also includes utility libraries to make scripts more powerful and convenient. The utility library modules and extensions are described in [the section called “NSE Libraries”](https://nmap.org/book/nse-library.html).

### Lua Base Language ###

[]()

 The Nmap scripting language is an embedded [Lua](https://lua.org/) interpreter which is extended with libraries for interfacing with Nmap. The Nmap API is in the Lua namespace `nmap`. This means that all calls to resources provided by Nmap have an `nmap` prefix.[]() `nmap.new_socket()`, for example, returns a new socket wrapper object. The Nmap library layer also takes care of initializing the Lua context, scheduling parallel scripts and collecting the output produced by completed scripts.

 During the planning stages, we considered several programming languages as the base for Nmap scripting. Another option was to implement a completely new programming language. Our criteria were strict: NSE had to be easy to use, small in size, compatible with the Nmap license, scalable, fast and parallelizable. Several previous efforts (by other projects) to design their own security auditing language from scratch resulted in awkward solutions, so we decided early not to follow that route. First the Guile Scheme interpreter was considered, but the preference drifted towards the Elk interpreter due to its more favorable license. But parallelizing Elk scripts would have been difficult. In addition, we expect that most Nmap users prefer procedural programming over functional languages such as Scheme. Larger interpreters such as Perl, Python, and Ruby are well-known and loved, but are difficult to embed efficiently. In the end, Lua excelled in all of our criteria. It is small, distributed under the liberal MIT open source license, has coroutines for efficient parallel script execution, was designed with embeddability in mind, has excellent documentation, and is actively developed by a large and committed community. Lua is now even embedded in other popular open source security tools including the Wireshark sniffer and Snort IDS.

---

[Prev](https://nmap.org/book/nse-script-format.html)Script Format

[Up](https://nmap.org/book/nse.html)Chapter 9. Nmap Scripting Engine

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/nse-scripts.html)NSE Scripts