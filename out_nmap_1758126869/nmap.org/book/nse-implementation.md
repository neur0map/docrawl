---
title: "Implementation Details | Nmap Network Scanning"
source_url: https://nmap.org/book/nse-implementation.html
fetched_at: 2025-09-17T16:44:21.306320+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 9. Nmap Scripting Engine](https://nmap.org/book/nse.html)
* Implementation Details

[Prev](https://nmap.org/book/nse-example-scripts.html)

[Next](https://nmap.org/book/firewalls.html)

Implementation Details
----------

[]()

 Now it is time to explore the NSE implementation details in depth. Understanding how NSE works is useful for designing efficient scripts and libraries. The canonical reference to the NSE implementation is the source code, but this section provides an overview of key details. It should be valuable to folks trying to understand and extend the NSE source code, as well as to script authors who want to better understand how their scripts are executed.

### Initialization Phase ###

 NSE is initialized before any scanning when Nmap first starts, by the `open_nse` function. `open_nse` creates a fresh Lua state that will persist across host groups,[]() until the program exits. It then loads the standard Lua libraries and compiled NSE libraries. The standard Lua libraries are documented in the [Lua Reference Manual](https://lua.org/manual/5.4/manual.html). The standard Lua libraries available to NSE are `debug`, `io`, `math`, `os`, `package`, `string`, and `table`. Compiled NSE libraries are those that are defined in a C++ file instead of a Lua file. They include `nmap`, `pcre`, `db`, `lpeg`, `debug`, `zlib`, `libssh2`, and `openssl` (if available).

 After loading the basic libraries, `open_nse` loads the file `nse_main.lua`. The NSE core is in this file—Lua code manages scripts and sets up the appropriate environment. In this situation Lua really shines as a glue language. C++ is used to provide the network framework and low-level libraries. Lua is used to structure data, determine which scripts to load, and schedule and execute scripts.

`nse_main.lua` sets up the Lua environment to be ready for script scanning later on. It loads all the scripts the user has chosen and returns a function that does the actual script scanning to `open_nse`.

 The `nselib` directory is added to the Lua path to give scripts access to the standard NSE library. NSE loads replacements for the standard coroutine functions so that yields initiated by NSE are caught and propagated back to the NSE scheduler.

`nse_main.lua` next defines classes and functions to be used during setup. The script arguments (`--script-args`) are loaded into `nmap.registry.args`. A script database is created if one doesn't already exist or if this was requested with `--script-updatedb`.

 Finally, the scripts listed on the command line are loaded. The `get_chosen_scripts` function works to find chosen scripts by comparing categories, filenames, and directory names. The scripts are loaded into memory for later use. `get_chosen_scripts` works by transforming the argument to `--script` into a block of Lua code and then executing it. (This is how the `and`, `or`, and `not` operators are supported.) Any specifications that don't directly match a category or a filename from `script.db`[]() are checked against file and directory names. If the specification is a regular file, it's loaded. If a directory, all the `*.nse` files within it are loaded. Otherwise, the engine raises an error.

`get_chosen_scripts` finishes by arranging the selected scripts according to their dependencies (see [the section called “`dependencies` Field”](https://nmap.org/book/nse-script-format.html#nse-format-dependencies)). Scripts that have no dependencies are in runlevel 1. Scripts that directly depend on these are in runlevel 2, and so on. When a script scan is run, each runlevel is run separately and in order.

`nse_main.lua` defines two classes: `Script` and `Thread`. These classes are the objects that represent NSE scripts and their script threads. When a script is loaded, `Script.new` creates a new Script object. The script file is loaded into Lua and saved for later use. These classes and their methods are intended to encapsulate the data needed for each script and its threads. `Script.new` also contains sanity checks to ensure that the script has required fields such as the `action` function.

### Script Scanning ###

 When NSE runs a script scan, `script_scan` is called in `nse_main.cc`. Since there are three script scan phases, `script_scan` accepts two arguments, a script scan type which can be one of these values: `SCRIPT_PRE_SCAN` (Script Pre-scanning phase) or `SCRIPT_SCAN` (Script scanning phase) or `SCRIPT_POST_SCAN` (Script Post-scanning phase), and a second argument which is a list of targets to scan if the script scan phase is `SCRIPT_SCAN`. These targets will be passed to the `nse_main.lua` main function for scanning.

 The main function for a script scan generates a number of script threads based on whether the `rule` function returns true. The generated threads are stored in a list of runlevel lists. Each runlevel list of threads is passed separately to the `run` function. The `run` function is the main worker function for NSE where all the magic happens.

 The `run` function's purpose is run all the threads in a runlevel until they all finish. Before doing this however, `run` redefines some Lua registry values that help C code function. One such function, `_R[WAITING_TO_RUNNING]`, allows the network library binding written in C to move a thread from the waiting queue to the running queue. Scripts are run until the running and waiting queues are both empty. Threads that yield are moved to the waiting queue; threads that are ready to continue are moved back to the running queue. The cycle continues until the thread quits or ends in error. Along with the waiting and running queues, there is a pending queue. It serves as a temporary location for threads moving from the waiting queue to the running queue before a new iteration of the running queue begins.

---

[Prev](https://nmap.org/book/nse-example-scripts.html)Example Script: finger

[Up](https://nmap.org/book/nse.html)Chapter 9. Nmap Scripting Engine

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/firewalls.html)Chapter 10. Detecting and Subverting Firewalls and Intrusion Detection Systems