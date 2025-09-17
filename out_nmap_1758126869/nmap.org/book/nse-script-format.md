---
title: "Script Format | Nmap Network Scanning"
source_url: https://nmap.org/book/nse-script-format.html
fetched_at: 2025-09-17T16:43:39.807054+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 9. Nmap Scripting Engine](https://nmap.org/book/nse.html)
* Script Format

[Prev](https://nmap.org/book/nse-usage.html)

[Next](https://nmap.org/book/nse-language.html)

Script Format
----------

NSE scripts consist of a handful of descriptive fields, a rule defining when the script should be executed, and an `action` function containing the actual script instructions. Values can be assigned to the descriptive fields just as you would assign any other Lua variables. Their names must be lowercase as shown in this section.

### `description` Field ###

[]()

The `description` field describes what a script is testing for and any important notes the user should be aware of. Depending on script complexity, descriptions may vary in length from a few sentences to a few paragraphs. The first paragraph should be a brief synopsis of the script function suitable for stand-alone presentation to the user. Further paragraphs may provide much more script detail.

### `categories` Field ###

[]()

The `categories` field defines one or more categories to which a script belongs (see [the section called “Script Categories”](https://nmap.org/book/nse-usage.html#nse-categories)). The categories are case-insensitive and may be specified in any order. They are listed in an array-style Lua table as in this example:

```
categories = {"default", "discovery", "safe"}

```

### `author` Field  ###

[]()

 The `author` field contains the script authors' names and can also contain contact information (such as home page URLs). We no longer recommend including email addresses because spammers might scrape them from the NSEDoc web site. This optional field is not used by NSE, but gives script authors their due credit or blame.

### `license` Field  ###

[]()[]()

Nmap is a community project and we welcome all sorts of code contributions, including NSE scripts. So if you write a valuable script, don't keep it to yourself! The optional `license` field helps ensure that we have legal permission to distribute all the scripts which come with Nmap. All of those scripts currently use the standard Nmap license (described in [the section called “Nmap Copyright and Licensing”](https://nmap.org/book/man-legal.html#nmap-copyright)). They include the following line:

```
license = "Same as Nmap--See https://nmap.org/book/man-legal.html"

```

The Nmap license is similar to the GNU GPL. Script authors may
use a BSD-style license (no advertising clause) instead if they prefer
that. For a BSD-style license, please include this line:

```
license = "Simplified (2-clause) BSD license--See https://nmap.org/svn/docs/licenses/BSD-simplified"

```

### `dependencies` Field ###

[]()[]()

 The `dependencies` field is an array containing the names of scripts that should run before this script, if they are also selected. This is used when one script can make use of the results of another. For example, most of the `smb-*` scripts depend on `smb-brute`,[]() because the accounts found by `smb-brute` may allow the other scripts to get more information. Listing a script in `dependencies` doesn't cause that script to be run; it still has to be selected through the `--script` option or otherwise. `dependencies` merely forces an ordering among the scripts that *are* selected. This is an example of a `dependencies` table, from `smb-os-discovery`:[]()

```
dependencies = {"smb-brute"}

```

 The dependencies table is optional. NSE will assume the script has no dependencies if the field is omitted.

 Dependencies establish an internal ordering of scripts, assigning each one a number called a “runlevel”[<sup id="idm45818751787136" class="footnote">[12]</sup>](https://nmap.org/book/nse-script-format.html#ftn.idm45818751787136).[]() When running your scripts you will see the runlevel (along with the total number of runlevels) of each grouping of scripts run in NSE's output:

```
NSE: Script scanning 127.0.0.1.
NSE: Starting runlevel 1 (of 3) scan.
Initiating NSE at 17:38
Completed NSE at 17:38, 0.00s elapsed
NSE: Starting runlevel 2 (of 3) scan.
Initiating NSE at 17:38
Completed NSE at 17:38, 0.00s elapsed
NSE: Starting runlevel 3 (of 3) scan.
Initiating NSE at 17:38
Completed NSE at 17:38, 0.00s elapsed
NSE: Script Scanning completed.

```

### Rules ###

[]()[]()[]()[]()[]()

 Nmap uses the script rules to determine whether a script should be run against a target. A rule is a Lua function that returns either `true` or `false`. The script `action` function is only performed if the rule evaluates to `true`.

 A script must contain one or more of the following functions that determine when the script will be run:

|     `prerule()`      |
|----------------------|
|   `hostrule(host)`   |
|`portrule(host, port)`|
|     `postrule()`     |

`prerule` scripts run once, before any hosts are scanned, during the script pre-scanning phase.[]() `hostrule` and `portrule` scripts run after each batch of hosts is scanned. `postrule` scripts run once after all hosts have been scanned, in the script post-scanning phase.[]() A script may run in more than one phase if it has several rules.

`prerule` and `postrule` do not accept arguments. `hostrule` accepts a host table and may test, for example, the IP address or hostname of the target. `portrule` accepts both a host table and a port table for any port in the `open`[](), `open|filtered`[](), or `unfiltered`[]() port states. Port rules generally test factors such as the port number, port state, or listening service name in deciding whether to run against a port. Example rules are shown in [the section called “The Rule”](https://nmap.org/book/nse-tutorial.html#nse-tutorial-rule).

Advanced users may force a script to run regardless of the results of these rule functions by prefixing the script name (or category or other expression) with a `+` in the `--script` argument.

 The current standard to choose between a `prerule` or a `postrule` is this: if the script is doing host discovery or any other network operation then the `prerule` should be used. `postrule` is reserved for reporting of data and statistics that were gathered during the scan.

### Action ###

[]()

The action is the heart of an NSE script. It contains all of the
instructions to be executed when the script's prerule, portrule, hostrule or postrule
triggers. It is a Lua function which accepts the same arguments as the
rule. The return value of the action value may be a table of
name–value pairs, a string, or `nil`. For an example of
an NSE action refer to [the section called “The Action”](https://nmap.org/book/nse-tutorial.html#nse-tutorial-action).

If the output of the action is a table, it is automatically formatted in
a structured fashion for inclusion in the normal (`-oN`)
and XML (`-oX`) output formats. If a string, the text is
displayed directly in normal output, and written as an XML attribute in
XML output, No output is produced if the script returns`nil`. See [the section called “Structured and Unstructured Output”](https://nmap.org/book/nse-api.html#nse-structured-output) for
details of how different return values are handled.

### Environment Variables ###

[]()

Each script has its own set of environment variables:

`SCRIPT_PATH`

 The script path.

`SCRIPT_NAME`

 The script name. This variable can be used in debug output.

`SCRIPT_TYPE`

 Since a script can have multiple rule functions, this environment variable will show which rule has activated the script, this would be useful if the script wants to share some code between different Script Scan phases. It will take one of these four string values: `"prerule"`, `"hostrule"`, `"portrule"` or `"postrule"`. This variable is only available during and after the evaluation of the rule functions.

 This is an example of a debug code that uses the previous environment variables, followed by the output message, from dns-zone-transfer:

```
          stdnse.print_debug(3, "Skipping '%s' %s, 'dnszonetransfer.server' argument is missing.", SCRIPT_NAME, SCRIPT_TYPE)

```

```
          Initiating NSE at 15:31
          NSE: Skipping 'dns-zone-transfer' prerule, 'dnszonetransfer.server' argument is missing.

```

---

[<sup class="para">[12] </sup>](https://nmap.org/book/nse-script-format.html#idm45818751787136)Up through Nmap version 5.10BETA2, dependencies didn't exist and script authors had to set a `runlevel` field manually.

---

[Prev](https://nmap.org/book/nse-usage.html)Usage and Examples

[Up](https://nmap.org/book/nse.html)Chapter 9. Nmap Scripting Engine

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/nse-language.html)Script Language