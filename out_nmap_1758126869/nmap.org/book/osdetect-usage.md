---
title: "Usage and Examples | Nmap Network Scanning"
source_url: https://nmap.org/book/osdetect-usage.html
fetched_at: 2025-09-17T16:42:37.311909+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 8. Remote OS Detection](https://nmap.org/book/osdetect.html)
* Usage and Examples

[Prev](https://nmap.org/book/osdetect.html)

[Next](https://nmap.org/book/osdetect-methods.html)

Usage and Examples
----------

The inner workings of OS detection are quite complex, but it is
one of the easiest features to use. Simply add`-O`[]()to your scan options. You may want to also increase the verbosity with`-v` for even more OS-related details. This is shown
in [Example 8.1](https://nmap.org/book/osdetect-usage.html#osdetect-ex-scanme1).

Example 8.1. OS detection with verbosity (`-O` `-v`)

[]()[]()

```
# nmap -O -v scanme.nmap.org

Starting Nmap ( https://nmap.org )
Nmap scan report for scanme.nmap.org (74.207.244.221)
Not shown: 994 closed ports
PORT      STATE    SERVICE
22/tcp    open     ssh
80/tcp    open     http
646/tcp   filtered ldp
1720/tcp  filtered H.323/Q.931
9929/tcp  open     nping-echo
31337/tcp open     Elite
Device type: general purpose
Running: Linux 2.6.X
OS CPE: cpe:/o:linux:linux_kernel:2.6.39
OS details: Linux 2.6.39
Uptime guess: 1.674 days (since Fri Sep  9 12:03:04 2011)
Network Distance: 10 hops
TCP Sequence Prediction: Difficulty=205 (Good luck!)
IP ID Sequence Generation: All zeros

Read data files from: /usr/local/bin/../share/nmap
Nmap done: 1 IP address (1 host up) scanned in 5.58 seconds
           Raw packets sent: 1063 (47.432KB) | Rcvd: 1031 (41.664KB)

```

Including the `-O -v` options caused Nmap to generate the following extra line items:

Device type[]()

[All fingerprints are classified with one or more high-level device types, such as `router`, `printer`, `firewall`, or (as in this case) `general purpose`. These are further described in ]()[the section called “Device and OS classification (`Class` lines)”](https://nmap.org/book/osdetect-fingerprint-format.html#osdetect-class). Several device types may be shown, in which case they will be separated with the pipe symbol as in “`Device Type: router|firewall`”.

Running[]()

[This field is also related to the OS classification scheme described in ]()[the section called “Device and OS classification (`Class` lines)”](https://nmap.org/book/osdetect-fingerprint-format.html#osdetect-class). It shows the OS Family (`Linux` in this case) and OS generation (`2.6.X`) if available. If there are multiple OS families, they are separated by commas. When Nmap can't narrow down OS generations to one specific choice, options are separated by the pipe symbol ('|') Examples include `OpenBSD 3.X, NetBSD 3.X|4.X` and `Linux 2.4.X|2.5.X|2.6.X`.

If Nmap finds too many OS families to print concisely, it will omit this line. When there are no perfect matches, Nmap changes the field to `Running (JUST GUESSING)`[ and adds an accuracy percentage (100% is a perfect match) in parentheses after each candidate family name. If no fingerprints are close matches, the line is omitted.]()

[OS CPE]()[]()

[This shows a Common Platform Enumeration (CPE)]()[ representation of the operating system when available. It may also have a CPE representation of the hardware type. OS CPE begins with `cpe:/o` and hardware CPE begins with `cpe:/h`. For more about CPE see ]()[the section called “Common Platform Enumeration (CPE)”](https://nmap.org/book/output-formats-cpe.html).

OS details[]()

[This line gives the detailed description for each fingerprint that matches. While the `Device type` and `Running` lines are from predefined enumerated lists that are easy to parse by a computer, the OS details line contains free-form data which is useful to a human reading the report. This can include more exact version numbers, device models, and architectures specific to a given fingerprint. In this example, the only matching fingerprint was `Linux 2.6.20-1 (Fedora Core 5)`. When there are multiple exact matches, they are comma-separated. If there aren't any perfect matches, but some close guesses, the field is renamed `Aggressive OS guesses`]()[ and fingerprints are shown followed by a percentage in parentheses which specifies how close each match was.]()

[Uptime guess]()[]()

[As part of OS detection, Nmap receives several SYN/ACK TCP packets in a row and checks the headers for a timestamp option. Many operating systems use a simple counter for this which starts at zero at boot time then increments at a constant rate such as twice per second. By looking at several responses, Nmap can determine the current values and rate of increase. Simple linear extrapolation determines boot time. The timestamp algorithm is used for OS detection too (see ]()[the section called “TCP timestamp option algorithm (`TS`)”](https://nmap.org/book/osdetect-methods.html#osdetect-ts)) since the increment rate on different systems varies from 2 Hz to 1,000 Hz.

The uptime guess is labeled a “guess” because various factors can make it completely inaccurate. Some operating systems do not start the timestamp counter at zero, but initialize it with a random value, making extrapolation to zero meaningless. Even on systems using a simple counter starting at zero, the counter eventually overflows and wraps around. With a 1,000 Hz counter increment rate, the counter resets to zero roughly every 50 days. So a host that has been up for 102 days will appear to have been up only two days. Even with these caveats, the uptime guess is accurate much of the time for most operating systems, so it is printed when available, but only in verbose mode. The uptime guess is omitted if the target gives zeros or no timestamp options in its SYN/ACK packets, or if it does not reply at all. The line is also omitted if Nmap cannot discern the timestamp increment rate or it seems suspicious (like a 30-year uptime).

Network Distance[]()

[A side effect of one of the OS detection tests allows Nmap to compute how many routers are between it and a target host. The distance is zero when you are scanning localhost, and one for a machine on the same network segment. Each additional router on the path adds one to the hop count. The `Network Distance` line is not printed in this example, since Nmap omits the line when it cannot be computed (no reply to the relevant probe).]()

[TCP Sequence Prediction]()

[Systems with poor TCP initial sequence number]()[ generation are vulnerable to blind TCP spoofing attacks.]()[ In other words, you can make a full connection to those systems and send (but not receive) data while spoofing a different IP address. The target's logs will show the spoofed IP, and you can take advantage of any trust relationship between them. This attack was all the rage in the mid-nineties when people commonly used rlogin to allow logins to their account without any password from trusted IP addresses. Kevin Mitnick]()[ is alleged to have used this attack to break into Tsutomu Shimomura's]()[ computers in December 1994.]()

[The good news is that hardly anyone uses rlogin anymore, and many operating systems have been fixed to use unpredictable initial sequence numbers as proposed by ]()[RFC 1948](http://www.rfc-editor.org/rfc/rfc1948.txt). For these reasons, this line is only printed in verbose mode. Sadly, many vendors still ship [vulnerable operating systems and devices](http://lcamtuf.coredump.cx/newtcp/). Even the fixed ones often vary in implementation, which leaves them valuable for OS detection purposes. The class describes the ISN generation algorithm used by the target, and difficulty is a rough estimate of how hard the system makes blind IP spoofing (0 is the easiest).[ The parenthesized comment is based on the difficulty index and ranges from `Trivial joke`]()[ to `Easy`,]()[ `Medium`,]()[ `Formidable`,]()[ `Worthy challenge`,]()[ and finally `Good luck!`]()[ Further details about sequence tests are provided in ]()[the section called “TCP ISN greatest common divisor (`GCD`)”](https://nmap.org/book/osdetect-methods.html#osdetect-gcd).

While the rlogin family is mostly a relic of the past, clever attackers can still find effective uses for blind TCP spoofing. For example, it allows for spoofed HTTP requests. You don't see the results, but just the URL (POST or GET request) can have dramatic side effects. The spoofing allows attackers to hide their identity, frame someone else, or exploit IP address restrictions.

IP ID sequence generation

Many systems unwittingly give away sensitive[ information about their traffic levels based on how they generate the lowly 16-bit ID field in IP packets. This can be abused to spoof a port scan against other systems and for other mischievous purposes discussed in ]()[the section called “TCP Idle Scan (`-sI`)”](https://nmap.org/book/idlescan.html). This field describes the ID generation algorithm that Nmap was able to discern. More information on how it classifies them is available in [the section called “IP ID sequence generation algorithm (`TI`, `CI`, `II`)”](https://nmap.org/book/osdetect-methods.html#osdetect-ti). Note that many systems use a different IP ID space for each host they communicate with. In that case, they may appear vulnerable (such as showing the `Incremental` class) while still being secure against attacks such as the idle scan. For this reason, and because the issue is rarely critical, the IP ID sequence generation line is only printed in verbose mode. If Nmap does not receive sufficient responses during OS detection, it will omit the whole line. The best way to test whether a host is vulnerable to being an idle scan zombie is to test it with `-sI`.

While TCP fingerprinting is a powerful method for OS detection,
interrogating open ports for clues is another effective approach.
Some applications, such as Microsoft IIS, only run on a single platform (thus
giving it away), while many other apps divulge their platform in
overly verbose banner messages. Adding the `-sV`option enables Nmap version detection, which is trained to look for
these clues (among others). In [Example 8.2](https://nmap.org/book/osdetect-usage.html#osdetect-ex-hpux),
Nmap catches the platform details from an FTP server.

Example 8.2. Using version scan to detect the OS

[]()

```
# nmap -sV -O -v 129.128.X.XX

Starting Nmap ( https://nmap.org )
Nmap scan report for [hostname] (129.128.X.XX)
Not shown: 994 closed ports
PORT      STATE    SERVICE      VERSION
21/tcp    open     ftp          HP-UX 10.x ftpd 4.1
22/tcp    open     ssh          OpenSSH 3.7.1p1 (protocol 1.99)
111/tcp   open     rpc
445/tcp   filtered microsoft-ds
1526/tcp  open     oracle-tns   Oracle TNS Listener
32775/tcp open     rpc
No exact OS matches for host
TCP Sequence Prediction: Class=truly random
                         Difficulty=9999999 (Good luck!)
IP ID Sequence Generation: Incremental
Service Info: OS: HP-UX

```

In this example, the line “`No exact OS
matches for host`”[]()means that TCP/IP
fingerprinting failed to find an exact match. Fortunately, the`Service Info`[]()field a few lines down discloses that
the OS is HP-UX. If several operating systems were detected (which
can happen with NAT gateway boxes that redirect ports to several
different machines), the field would be `OSs` and the values would be
comma separated. The `Service Info` line can also
contain hostnames and device types found during the version scan. The
focus of this chapter is on TCP/IP fingerprinting though, since version
detection was covered in [Chapter 7, *Service and Application Version Detection*](https://nmap.org/book/vscan.html).

With two effective OS detection methods available, which one
should you use? The best answer is usually both. In some cases, such
as a
proxy firewall[]()forwarding to an application on another host, the answers may
legitimately differ. TCP/IP fingerprinting will identify the proxy
while version scanning will generally detect the server running the
proxied application. Even when no proxying or port forwarding is
involved, using both techniques is beneficial. If they come out the
same, that makes the results more credible. If they come out wildly
different, investigate further to determine what is going on before
relying on either. Since OS and version detection go together so
well, the `-A` option enables them both.

OS detection is far more effective if at least one open
and one closed TCP port are found. Set the`--osscan-limit`[]()option and Nmap will not even try OS detection against hosts which do
not meet this criteria. This can save substantial time, particularly
on `-Pn` scans against many hosts. You still need to enable OS
detection with `-O` (or `-A`) for the `--osscan-limit` option
to have any effect.

Another OS detection option is`--osscan-guess`.[]()When Nmap is unable to detect a
perfect OS match, it sometimes offers up near-matches as
possibilities. The match has to be very close for Nmap to do this by
default. If you specify this option (or the equivalent`--fuzzy`[]()option), Nmap will guess more aggressively.
Nmap still tells you when an imperfect match is found and
display its confidence level (percentage) for each guess.

When Nmap performs OS detection against a target and fails to
find a perfect match, it usually repeats the attempt. By default,
Nmap tries five times if conditions are favorable for OS fingerprint
submission, and twice when conditions aren't so good. The`--max-os-tries`[]()option lets you change this maximum
number of OS detection tries. Lowering it (usually to 1) speeds Nmap
up, though you miss out on retries which could potentially identify
the OS. Alternatively, a high value may be set to allow even more
retries when conditions are favorable. This is rarely done, except to
generate better fingerprints for submission and integration into the
Nmap OS database.

Like just about every other part of Nmap, results ultimately
come from the target machine itself. While rare, systems are
occasionally configured to confuse or mislead Nmap. Several programs
have even been developed specifically to trick Nmap OS detection (see[the section called “OS Spoofing”](https://nmap.org/book/nmap-defenses-trickery.html#nmap-defenses-os-spoofing)). Your best bet is to use
numerous reconnaissance methods to explore a network, and don't trust
any one of them.

TCP/IP fingerprinting requires collecting detailed information
about the target's IP stack. The most commonly useful results, such as
TTL[]()[]()information, are printed to Nmap output whenever they are
obtained. Slightly less pertinent information, such as IP ID sequence
generation[]()and TCP sequence prediction[]()difficulty, is only printed in
verbose mode. But if you want all of the IP stack details that Nmap
collected, you can find it in a compact form called a*subject fingerprint*.[]()Nmap sometimes prints this
(for user submission purposes) when it doesn't recognize a host. You
can also force Nmap to print it (in normal, interactive, and XML
formats) by enabling debugging with
(`-d`).[]()Then read [the section called “Understanding an Nmap Fingerprint”](https://nmap.org/book/osdetect-fingerprint-format.html) to interpret it.

---

[Prev](https://nmap.org/book/osdetect.html)Chapter 8. Remote OS Detection

[Up](https://nmap.org/book/osdetect.html)Chapter 8. Remote OS Detection

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/osdetect-methods.html)TCP/IP Fingerprinting Methods Supported by Nmap