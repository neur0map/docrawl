---
title: "nmap-service-probes File Format | Nmap Network Scanning"
source_url: https://nmap.org/book/vscan-fileformat.html
fetched_at: 2025-09-17T16:42:22.058915+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 7. Service and Application Version Detection](https://nmap.org/book/vscan.html)
* nmap-service-probes File Format

[Prev](https://nmap.org/book/vscan-post-processors.html)

[Next](https://nmap.org/book/vscan-community.html)

`nmap-service-probes` File Format
----------

[]()

As with remote OS detection (`-O`),
Nmap uses a flat file to store the version
detection probes and match strings. While the version of`nmap-services` distributed with
Nmap is sufficient for most users,
understanding the file format allows advanced
Nmap hackers to add their own services to
the detection engine. Like many Unix files,`nmap-service-probes` is line-oriented. Lines
starting with a hash (#) are treated as
comments[]()and ignored by the
parser. Blank lines are ignored as well. Other lines must contain
one of the directives described below. Some readers prefer to peek at
the examples in [the section called “Putting It All Together”](https://nmap.org/book/vscan-fileformat.html#vscan-fileformat-example)before tackling the following dissection.

### `Exclude` Directive ###

[]()

Syntax: `Exclude` *`<port specification>`*

Examples:

```
Exclude 53,T:9100,U:30000-40000

```

This directive excludes the specified ports from the version
scan. It can only be used once and should be near the top of the file,
above any Probe directives. The Exclude directive uses the same
format as the Nmap `-p` switch, so ranges and comma
separated lists of ports are supported. In the`nmap-service-probes` included with
Nmap the only ports excluded are TCP port 9100 through 9107.
These are common ports for
printers[]()to listen on and they often print
any data sent to them. So a version detection scan can cause
them to print many pages full of probes that Nmap sends, such as
SunRPC requests, help statements, and X11 probes.

This behavior is often undesirable, especially when a scan is
meant to be stealthy. However, Nmap's default behavior of avoiding
scanning this port can make it easier for a sneaky user to hide a
service: simply run it on an excluded port such as 9100 and it is less
likely to be identified by name. The port scan will still show it as
open. Users can override the Exclude directive with the`--allports`[]() option. This causes version detection to
interrogate all open ports.

### `Probe` Directive ###

[]()

Syntax: `Probe` *`<protocol>`* *`<probename>`* *`<probestring>`* [`no-payload`]

Examples:

```
Probe TCP GetRequest q|GET / HTTP/1.0\r\n\r\n|
Probe UDP DNSStatusRequest q|\0\0\x10\0\0\0\0\0\0\0\0\0|
Probe TCP NULL q||
Probe UDP Sqlping q|\x02| no-payload

```

The Probe directive tells Nmap what string to send to recognize various services. All of the directives discussed later operate on the most recent `Probe` statement. The arguments are as follows:

*`<protocol>`*

This must be either `TCP` or `UDP`. Nmap only uses probes that match the protocol of the service it is trying to scan.

*`<probename>`*

This is a plain English name for the probe. It is used in service fingerprints to describe which probes elicited responses.

*`<probestring>`*[]()

[Tells Nmap what to send.
It must start with a `q`, then a delimiter
character which begins and ends the string. Between the delimiter
characters is the string that is actually sent. It is formatted
similarly to a C or Perl string in that it allows the following
standard escape characters: `\\``\0`, `\a`,`\b`, `\f`,`\n`, `\r`,`\t`, `\v`,
and `\xHH` (where `H` is any hexadecimal digit). One `Probe` line in`nmap-service-probes` has an empty probe string, as shown in the third
example above. This is the TCP
NULL probe]()[which just listens for the
initial banners that many services send. If your delimiter character (`|` in these examples) is needed for your probe string, you need to choose a different delimiter.]()

[[`no-payload`]]()

[This keyword is used to instruct Nmap not to use the given probe as a ]()[ protocol-specific payload during UDP port scanning. The `Sqlping` probe, for instance, sometimes triggers IDS alerts for "MS-SQL ping attempt."]()

### [`match` Directive]() ###

[]()

Syntax: `match` *`<service>`* *`<pattern>`* [*`<versioninfo>`*]

Examples:

```
match ftp m/^220.*Welcome to .*Pure-?FTPd (\d\S+\s*)/ p/Pure-FTPd/ v/$1/ cpe:/a:pureftpd:pure-ftpd:$1/
match ssh m/^SSH-([\d.]+)-OpenSSH[_-]([\w.]+)\r?\n/i p/OpenSSH/ v/$2/ i/protocol $1/ cpe:/a:openbsd:openssh:$2/
match mysql m|^\x10\0\0\x01\xff\x13\x04Bad handshake$| p/MySQL/ cpe:/a:mysql:mysql/
match chargen m|@ABCDEFGHIJKLMNOPQRSTUVWXYZ|
match uucp m|^login: login: login: $| p/NetBSD uucpd/ o/NetBSD/ cpe:/o:netbsd:netbsd/a
match printer m|^([\w-_.]+): lpd: Illegal service request\n$| p/lpd/ h/$1/
match afs m|^[\d\D]{28}\s*(OpenAFS)([\d\.]{3}[^\s\0]*)\0| p/$1/ v/$2/

```

The match directive tells Nmap how to recognize services based on responses to the string sent by the previous `Probe` directive. A single `Probe` line may be followed by dozens or hundreds of `match` statements. If the given pattern matches, an optional version specifier builds the application name, version number, and additional info for Nmap to report. The arguments to this directive follow:

*`<service>`*

This is simply the service name that the pattern matches. Examples would be `ssh`, `smtp`,`http`, or `snmp`. As a special case, you can prefix the service name with `ssl/`, as in `ssl/vmware-auth`. In that case, the service would be stored as `vmware-auth` tunneled by SSL. This is useful for services which can be fully recognized without the overhead of making an SSL connection.

*`<pattern>`*

This pattern is used to determine whether the response
received matches the service given in the previous parameter. The
format is like Perl, with the syntax being `m/[regex]/[opts]`. The“m” tells Nmap that a match string is beginning. The forward slash
(`/`) is a delimiter, which can be substituted by almost any printable
character as long as the second slash is also replaced to match. The
regex[is a ]()[Perl-style regular expression](http://www.perl.com/doc/manual/html/pod/perlre.html). This is
made possible by the excellent Perl
Compatible Regular Expressions (PCRE)[]()[library (]()[`http://www.pcre.org`](http://www.pcre.org/)). The only options
currently supported are '`i`', which makes a match case-insensitive and
'`s`' which includes newlines in the '.' specifier. As you might
expect, these two options have the same semantics as in
Perl. Subexpressions to be captured (such as version numbers) are surrounded by parentheses as shown in most of the examples above.

*`<versioninfo>`*

The `*`<versioninfo>`*` section actually
contains several optional fields. Each field begins with an identifying
letter (such as `h` for“hostname”). Next comes a delimiter character which
the signature writer chooses. The preferred delimiter is slash
(`‘/’`) unless that is used in the field
itself. Next comes the field value, followed by the delimiter
character. The following table describes the six fields:

Table 7.1. `versioninfo` field formats and values

[

|         Field format         |                                                                                                                                                                                                                                          Value description                                                                                                                                                                                                                                           |
|------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|  [`p/vendorproductname/`]()  |                                                                                                                                                                                       Includes the vendor and often service name and is of the form “Sun Solaris rexecd”, “ISC BIND named”, or“Apache httpd”.                                                                                                                                                                                        |
|       [`v/version/`]()       |                                                                                                                                                                                                 The application version “number”, which may include non-numeric characters and even multiple words.                                                                                                                                                                                                  |
|         [`i/info/`]()        |                                                                                                                                              Miscellaneous further information which was immediately available and might be useful. Examples include whether an X server is open to unauthenticated connections, or the protocol number of SSH servers.                                                                                                                                              |
|       [`h/hostname/`]()      |                                                                                                                               The hostname (if any) offered up by a service. This is common for protocols such as SMTP and POP3 and is useful because these hostnames may be for internal networks or otherwise differ from the straightforward reverse DNS responses.                                                                                                                               |
| []()[ `o/operatingsystem/`]()|                                       The operating system the service is running on. This may legitimately be different than the OS reported by Nmap IP stack based OS detection. For example, the target IP might be a Linux box which uses network address translation to forward requests to an Microsoft IIS server in the DMZ. In this case, stack OS detection should report the OS as Linux, while service detection reports port 80 as being Windows.                                       |
|      [`d/devicetype/`]()     |                                                               The type of device the service is running on, a string like “print server” or “webcam”. Some services disclose this information, and it can be inferred in many more cases. For example, the HP-ChaiServer web server only runs on printers. For a full list of device types, see [the section called “Device Types”](https://nmap.org/book/osdetect-device-types.html).                                                               |
|    [`cpe:/cpename/[a]`]()    |A CPE name for some aspect of the service. This may be used multiple times; it's conceivable to be able to identify not only the service (`cpe:/a` names) but also the operating system (`cpe:/o` names) and hardware platform (`cpe:/h` names) as well. The trailing slash is not part of CPE syntax but is included to match the format of other fields. See [the section called “Common Platform Enumeration (CPE)”](https://nmap.org/book/output-formats-cpe.html) for more information about CPE.|
]()

[  

Any of the fields can be omitted. In fact, all of the fields
can be omitted if no further information on the service is available.
Any of the version fields can include numbered strings such as $1 or
$2, which are replaced (in a Perl-like fashion) with the corresponding
parenthesized substring in the *`<pattern>`*.
Within `cpe://` templates, such substitutions are
transformed as follows: certain characters such as the colon are
escaped; spaces are converted to underscores, and all characters are
made lower case.

]()

[In rare cases, a*helper function*]()[can be applied to the replacement
text before insertion. The following table describes the three helper functions available:]()

[Table 7.2. `versioninfo` helper functions]()

[

|Helper function |                                                                                                                                                                                                                                                                                                               Function description                                                                                                                                                                                                                                                                                                               |
|----------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|   [`$P()` ]()  |                                                                                                                                                          Filters out unprintable characters. This is useful for converting<br/>Unicode UTF-16 encoded strings such as `W\0O\0R\0K\0G\0R\0O\0U\0P\0`into the ASCII approximation `WORKGROUP`. It can<br/>be used in any `versioninfo` field by passing it the<br/>number of the match you want to make printable, like this: `i/$P(3)/`.                                                                                                                                                          |
| [`$SUBST()` ]()|Makes substitutions in matches before they are printed. It takes three<br/>arguments. The first is the substitution number in the pattern, just as you<br/>would use in a normal replacement variable such as `$1` or`$3`. The second and third arguments specify a substring you<br/>wish to find and replace, respectively. All instances of the match string<br/>found in the substring are replaced, not just the first one. For example, the<br/>VanDyke VShell sshd gives its version number in a format such as`2_2_3_578`. We use the versioninfo field`v/$SUBST(1,"_",".")/` to convert it to the more conventional<br/>form `2.2.3.578`.|
|   [`$I()` ]()  |                                                                                                                        Unpacks an unsigned integer from the captured bytes. Given a captured string of<br/>up to 8 bytes, it will treat them as an unsigned integer and convert it to<br/>decimal format. It takes two arguments. The first is the substitution number in<br/>the pattern. The second is the string `">"` to indicate<br/>that the bytes are in big-endian order, or `"<"` to<br/>indicate little-endian.                                                                                                                        |
]()

[  
]()

### [`softmatch` Directive]() ###

[]()

Syntax: `softmatch` *`<service>`* *`<pattern>`*

Examples:

```
softmatch ftp m/^220 [-.\w ]+ftp.*\r\n$/i
softmatch smtp m|^220 [-.\w ]+SMTP.*\r\n|
softmatch pop3 m|^\+OK [-\[\]\(\)!,/+:<>@.\w ]+\r\n$|

```

The softmatch directive is similar in format to the match directive
discussed above. The main difference is that scanning continues after
a softmatch, but it is limited to probes that are known to match the
given service. This allows for a normal (“hard”) match to be found
later, which may provide useful version information. See [the section called “Technique Described”](https://nmap.org/book/vscan-technique.html) for more details on how
this works. Arguments are not defined here because they are the same
as for `match` above, except that there is never a *`<versioninfo>`* argument. Also as with `match`, many `softmatch` statements can exist within a single `Probe` section.

### `ports` and `sslports` Directives ###

[]()[]()[]()

Syntax: `ports` *`<portlist>`*

Examples:

```
ports 21,43,110,113,199,505,540,1248,5432,30444
ports 111,4045,32750-32810,38978

```

This line tells Nmap what ports the services identified by this
probe are commonly found on. It should only be used once within each `Probe` section. The syntax is a slightly simplified
version of that taken by the Nmap `-p` option. See the examples above.
More details on how this works are in [the section called “Technique Described”](https://nmap.org/book/vscan-technique.html).

Syntax: `sslports` *`<portlist>`*

Example:

```
sslports 443

```

This is the same as '`ports`' directive described above, except that these ports are often used to wrap a service in SSL. For example, the HTTP
probe declares “`sslports 443`” and SMTP-detecting
probes have an “`sslports 465`” line because those
are the standard ports for HTTPS and SMTPS respectively. The *`<portlist>`* format is the same as with `ports`. This optional directive cannot appear more than once per `Probe`.

### `totalwaitms` Directive ###

[]()

Syntax: `totalwaitms` *`<milliseconds>`*

Example:

```
totalwaitms 6000

```

This rarely necessary directive specifies the amount of time Nmap
should wait before giving up on the most recently defined `Probe` against a
particular service. The Nmap default is usually fine.

### `tcpwrappedms` Directive ###

[]()

Syntax: `tcpwrappedms` *`<milliseconds>`*

Example:

```
tcpwrappedms 3000

```

This directive is only used for the Null probe. If a service closes the TCP connection before this timer runs out, then the service is labeled `tcpwrapped`. Otherwise, matching continues as usual.

### `rarity` Directive ###

[]()

Syntax: `rarity` *`<value between 1 and 9>`*

Example:

```
rarity 6

```

The rarity directive roughly corresponds to how infrequently this probe can
be expected to return useful results. The higher the number, the more
rare the probe is considered and the less likely it is to be tried against
a service. More details can be found in[the section called “Probe Selection and Rarity”](https://nmap.org/book/vscan-technique.html#vscan-selection-and-rarity).

### `fallback` Directive ###

[]()

Syntax: `fallback` *`<Comma separated list of probes>`*

Example:

```
fallback GetRequest,GenericLines

```

This optional directive specifies which probes should be used as fallbacks
for if there are no matches in the current Probe section. For more information on fallbacks
see [the section called “Cheats and Fallbacks”](https://nmap.org/book/vscan-technique.html#vscan-cheats-and-fallbacks). For TCP probes without a fallback directive, Nmap first tries match lines in the probe itself and then does an implicit fallback to the NULL probe.[]()If the fallback directive is present, Nmap first tries
match lines from the probe itself, then those from the
probes specified in the fallback directive (from left to right). Finally,
Nmap will try the NULL probe. For UDP the behavior is identical except that
the NULL probe is never tried.

### Putting It All Together ###

Here are some examples from `nmap-service-probes` which put this all together (to save space many lines have been skipped). After reading this far into the section, the following should be understood.[]()

```
# The Exclude directive takes a comma separated list of ports.
# The format is exactly the same as the -p switch.
Exclude T:9100-9107

# This is the NULL probe that just compares any banners given to us
##############################NEXT PROBE##############################
Probe TCP NULL q||
# Wait for at least 5 seconds for data.  Otherwise an Nmap default is used.
totalwaitms 5000
# Windows 2003
match ftp m/^220[ -]Microsoft FTP Service\r\n/ p/Microsoft ftpd/
match ftp m/^220 ProFTPD (\d\S+) Server/ p/ProFTPD/ v/$1/
softmatch ftp m/^220 [-.\w ]+ftp.*\r\n$/i
match ident m|^flock\(\) on closed filehandle .*midentd| p/midentd/ i/broken/
match imap m|^\* OK Welcome to Binc IMAP v(\d[-.\w]+)| p/Binc IMAPd/ v$1/
softmatch imap m/^\* OK [-.\w ]+imap[-.\w ]+\r\n$/i
match lucent-fwadm m|^0001;2$| p/Lucent Secure Management Server/
match meetingmaker m/^\xc1,$/ p/Meeting Maker calendaring/
# lopster 1.2.0.1 on Linux 1.1
match napster m|^1$| p/Lopster Napster P2P client/

Probe UDP Help q|help\r\n\r\n|
rarity 3
ports 7,13,37
match chargen m|@ABCDEFGHIJKLMNOPQRSTUVWXYZ|
match echo m|^help\r\n\r\n$|

```

[]()

---

[Prev](https://nmap.org/book/vscan-post-processors.html)Post-processors

[Up](https://nmap.org/book/vscan.html)Chapter 7. Service and Application Version Detection

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/vscan-community.html)Community Contributions