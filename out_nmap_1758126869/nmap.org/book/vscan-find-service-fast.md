---
title: "SOLUTION: Find All Servers Running an Insecure or Nonstandard Application Version | Nmap Network Scanning"
source_url: https://nmap.org/book/vscan-find-service-fast.html
fetched_at: 2025-09-17T16:42:11.560471+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 7. Service and Application Version Detection](https://nmap.org/book/vscan.html)
* SOLUTION: Find All Servers Running an Insecure or Nonstandard Application Version

[Prev](https://nmap.org/book/vscan-community.html)

[Next](https://nmap.org/book/vscan-hack-it.html)

SOLUTION: Find All Servers Running an Insecure or Nonstandard Application Version
----------

### Problem ###

A common task is scanning a range of IP addresses to find all servers
of a particular version or even satisfying a particular property. This is
something that Nmap's version detection
excels in.

One of the most popular database application is the open-sourceMySQL server. MySQLcan be configured to disallow all remote logins from untrusted IPs. This is
a good security practice when remote logins aren't required.
A case in point: in 2005 a MySQL remote code execution
vulnerability was [discovered and published](http://www.securityfocus.com/bid/12781).
Fortunately, an attacker must be able to log in first—no doubt saving the Internet from yet another devastating worm. In light of problems
like this and the fact that SQL logins and passwords are frequently
guessable or discoverable through SQL injection attacks, intuition,
and inside knowledge of the network, remote logins should be denied when
possible.

The problem for a network administrator is to discoverMySQL servers that needlessly allow logins
from untrusted IPs and take appropriate defensive measures.

|       ![[Note]](https://nmap.org/book/images/note.png)        |Note|
|---------------------------------------------------------------|----|
|This solution was contributed by Nmap developer Doug Hoyte.[]()|    |

### Solution ###

Nmap's version detection comes in handy in this situation
because it adds the word unauthorized to the service detection info line when the server forbids our host any access. If we want
to scan the network of 10.0.0.0/24 a simple
yet effective strategy is to run the following command from an untrusted source:

[]()[]()[]()**nmap -sV -p 3306 -oG 10.0.0-mysqls-032506.gnmap 10.0.0.0/24**

Next we can use the Unix **grep**utility to find IPs that accept connections from our IP and don't disallow
logins by default (**grep**'s `-v` switch specifies inverse results and only prints out
lines that *don't* match the given pattern):

**grep 'Ports: 3306/open/tcp//mysql' 10.0.0-mysqls-032506.gnmap | grep -v unauthorized**

The resulting output shows the MySQLservers that allow remote logins:

```
Host: 10.0.0.33 (foo.com) Ports: 3306/open/tcp//mysql//MySQL 4.1.11/
Host: 10.0.0.72 (bar.com) Ports: 3306/open/tcp//mysql//MySQL 4.0.24-standard/
Host: 10.0.0.99 () Ports: 3306/open/tcp//mysql//MySQL 4.1.11-Debian_4sarge2/
Host: 10.0.0.154 () Ports: 3306/open/tcp//mysql//MySQL 4.0.25-standard/
Host: 10.0.0.155 () Ports: 3306/open/tcp//mysql//MySQL 4.0.25-standard/

```

### Discussion ###

The trick to this is understanding some MySQL protocol basics and knowing how
to read the `nmap-service-probes` file. Grepping the file for `Probe` and `mysql` match lines leads to the following (line wrapped) output:

```
$ cat /usr/local/share/nmap/nmap-service-probes | egrep '^(Probe|match mysql)'
Probe TCP NULL q||
match mysql m/^.\0\0\0\xffj\x04.*Host .* is not allowed to connect to this
               MySQL server$/ p/MySQL/ i/unauthorized/
match mysql m|^.\0\0\0\xffj\x04Host hat keine Berechtigung, eine Verbindung
              zu diesem MySQL Server herzustellen\.| p/MySQL/
              i/unauthorized; German/
match mysql m/^.\0\0\0...Al sistema '[-.\w]+' non e` consentita la
              connessione a questo server MySQL$/ p/MySQL/
              i/unauthorized; Italian/
match mysql m|^.\0\0\0\xffi?\x04?Host .* is blocked because of many connection
              errors\.| p/MySQL/ i/blocked - too many connection errors/
match mysql m/^.\0\0\0.(3\.[-.\w]+)\0.*\x08\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0$/s
              p/MySQL/ v/$1/
match mysql m/^.\0\0\0\n(3\.[-.\w]+)\0...\0/s p/MySQL/ v/$1/
match mysql m/^.\0\0\0\n(4\.[-.\w]+)\0.../s p/MySQL/ v/$1/
match mysql m|^.\0\0\0\n(5\.[-.\w]+)\0...\0|s p/MySQL/ v/$1/
match mysql m|^.\0\0\0\xffj\x04'[\d.]+' .* MySQL|s p/MySQL/
Probe TCP GenericLines q|\r\n\r\n|
Probe TCP GetRequest q|GET / HTTP/1.0\r\n\r\n|
Probe TCP HTTPOptions q|OPTIONS / HTTP/1.0\r\n\r\n|
...

```

We see that the `mysql` match lines are designed to be triggered by the NULL
probe so no custom probes are needed to determine which servers allow remote
logins (for that see [the section called “SOLUTION: Hack Version Detection to Suit Custom Needs, such as Open Proxy Detection”](https://nmap.org/book/vscan-hack-it.html)). By looking at
these `mysql` match lines that we discover MySQL
services that don't allow remote logins will result in an info field containing
the word `unauthorized`.

In addition to service types and version numbers, there are many
cases where version detection is able to gather useful information on scan targets.
The probes file is full of such gems that can turn a time-consuming task of protocol
research, script coding, locating test servers, and debugging into a simple
Nmap command. A few interesting tidbits of information that
version detection can sometimes reveal are:

[]()

* SSH protocol versions

* Whether a CVS pserver is properly configured

* The usernames used by popular peer-to-peer file sharing clients

* Whether an X server is accepting connections

* Language and other localization parameters of many services

* The wordsize of the target's CPU

* The configured botnames of popular IRC bots such as eggdrop

* Whether posting is allowed on Internet news (NNTP) servers

The version detection database is constantly growing and being refined thanks
to the amazing Nmap user community and their service
fingerprint submissions. This solution is a good example of how investigating the
capabilities of Nmap's service detection can provide
elegant, sometimes non-obvious solutions to many diverse problems.

---

[Prev](https://nmap.org/book/vscan-community.html)Community Contributions

[Up](https://nmap.org/book/vscan.html)Chapter 7. Service and Application Version Detection

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/vscan-hack-it.html)SOLUTION: Hack Version Detection to Suit Custom Needs, such as Open Proxy Detection