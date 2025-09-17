---
title: "Usage and Examples | Nmap Network Scanning"
source_url: https://nmap.org/book/nse-usage.html
fetched_at: 2025-09-17T16:43:22.557067+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 9. Nmap Scripting Engine](https://nmap.org/book/nse.html)
* Usage and Examples

[Prev](https://nmap.org/book/nse.html)

[Next](https://nmap.org/book/nse-script-format.html)

Usage and Examples
----------

 While NSE has a complex implementation for efficiency, it is strikingly easy to use. Simply specify `-sC`[]() to enable the most common scripts. Or specify the `--script`[]() option to choose your own scripts to execute by providing categories, script file names, or the name of directories full of scripts you wish to execute. You can customize some scripts by providing arguments to them via the `--script-args`[]() and `--script-args-file`[]() options. The `--script-help`[]() shows a description of what each selected script does. The two remaining options, `--script-trace`[]() and `--script-updatedb`,[]() are generally only used for script debugging and development. Script scanning is also included as part of the `-A` (aggressive scan) option.

 Script scanning is normally done in combination with a port scan, because scripts may be run or not run depending on the port states found by the scan. With the `-sn` option it is possible to run a script scan without a port scan, only host discovery. In this case only host scripts will be eligible to run. To run a script scan with neither a host discovery nor a port scan, use the `-Pn -sn` options together with `-sC` or `--script`. Every host will be assumed up and still only host scripts will be run. This technique is useful for scripts like `whois-ip`[]() that only use the remote system's address and don't require it to be up.

 Scripts are not run in a sandbox and thus could accidentally or maliciously damage your system or invade your privacy. Never run scripts from third parties unless you trust the authors or have carefully audited the scripts yourself.

### Script Categories ###

[]()

NSE scripts define a list of categories they belong to. Currently defined categories are `auth`, `broadcast`, `brute`, `default`. `discovery`, `dos`, `exploit`, `external`, `fuzzer`, `intrusive`, `malware`, `safe`, `version`, and `vuln`. Category names are not case sensitive. The following list describes each category.

[`auth` ]()

[These scripts deal with authentication credentials (or bypassing them) on the target system. Examples include `x11-access`, `ftp-anon`, and `oracle-enum-users`. Scripts which use brute force attacks to determine credentials are placed in the `brute` category instead.]()

[]()[ `broadcast` ]()

[Scripts in this category typically do discovery of hosts not listed on the command line by broadcasting on the local network. Use the `newtargets`]()[ script argument to allow these scripts to automatically add the hosts they discover to the Nmap scanning queue.]()

[]()[ `brute` ]()

[These scripts use brute force attacks to guess authentication credentials of a remote server. Nmap contains scripts for brute forcing dozens of protocols, including `http-brute`, `oracle-brute`, `snmp-brute`, etc.]()

[]()[ `default` ]()

[These scripts are the default set and are run when using the `-sC` or `-A` options rather than listing scripts with `--script`. This category can also be specified explicitly like any other using `--script=default`. Many factors are considered in deciding whether a script should be run by default:]()

[Speed]()

[A default scan must finish quickly, which excludes brute force authentication crackers, web spiders, and any other scripts which can take minutes or hours to scan a single service.]()

[Usefulness]()

[Default scans need to produce valuable and actionable information. If even the script author has trouble explaining why an average networking or security professional would find the output valuable, the script should not run by default.]()

[Verbosity]()

[Nmap output is used for a wide variety of purposes and needs to be readable and concise. A script which frequently produces pages full of output should not be added to the `default` category. When there is no important information to report, NSE scripts (particularly default ones) should return nothing. Checking for an obscure vulnerability may be OK by default as long as it only produces output when that vulnerability is discovered.]()

[Reliability]()

[Many scripts use heuristics and fuzzy signature matching to reach conclusions about the target host or service. Examples include `sniffer-detect` and `sql-injection`. If the script is often wrong, it doesn't belong in the `default` category where it may confuse or mislead casual users. Users who specify a script or category directly are generally more advanced and likely know how the script works or at least where to find its documentation.]()

[Intrusiveness]()

[Some scripts are very intrusive because they use significant resources on the remote system, are likely to crash the system or service, or are likely to be perceived as an attack by the remote administrators. The more intrusive a script is, the less suitable it is for the `default` category. Default scripts are almost always in the `safe` category too, though we occasionally allow `intrusive` scripts by default when they are only mildly intrusive and score well in the other factors.]()

[Privacy]()

[Some scripts, particularly those in the `external` category described later, divulge information to third parties by their very nature. For example, the `whois` script must divulge the target IP address to regional whois registries. We have also considered (and decided against) adding scripts which check target SSH and SSL key fingerprints against Internet weak key databases. The more privacy-invasive a script is, the less suitable it is for `default` category inclusion.]()

[We don't have exact thresholds for each of these criteria, and many of them are subjective. All of these factors are considered together when making a decision whether to promote a script into the `default` category. A few default scripts are `identd-owners` (determines the username running remote services using identd), `http-auth` (obtains authentication scheme and realm of web sites requiring authentication), and `ftp-anon` (tests whether an FTP server allows anonymous access).]()

[]()[ `discovery` ]()

[These scripts try to actively discover more about the network by querying public registries, SNMP-enabled devices, directory services, and the like. Examples include `html-title` (obtains the title of the root path of web sites), `smb-enum-shares` (enumerates Windows shares), and `snmp-sysdescr` (extracts system details via SNMP).]()

[]()[ `dos` ]()

[Scripts in this category may cause a denial of service. Sometimes this is done to test vulnerability to a denial of service method, but more commonly it is an undesired by necessary side effect of testing for a traditional vulnerability. These tests sometimes crash vulnerable services.]()

[]()[ `exploit` ]()

[These scripts aim to actively exploit some vulnerability. Examples include `jdwp-exec` and `http-shellshock`.]()

[]()[ `external` ]()

[Scripts in this category may send data to a third-party database or other network resource. An example of this is `whois-ip`, which makes a connection to whois]()[ servers to learn about the address of the target. There is always the possibility that operators of the third-party database will record anything you send to them, which in many cases will include your IP address and the address of the target. Most scripts involve traffic strictly between the scanning computer and the client; any that do not are placed in this category.]()

[]()[ `fuzzer` ]()

[This category contains scripts which are designed to send server software unexpected or randomized fields in each packet. While this technique can useful for finding undiscovered bugs and vulnerabilities in software, it is both a slow process and bandwidth intensive. An example of a script in this category is `dns-fuzz`, which bombards a DNS server with slightly flawed domain requests until either the server crashes or a user specified time limit elapses.]()

[]()[ `intrusive` ]()

[These are scripts that cannot be classified in the `safe` category because the risks are too high that they will crash the target system, use up significant resources on the target host (such as bandwidth or CPU time), or otherwise be perceived as malicious by the target's system administrators. Examples are `http-open-proxy` (which attempts to use the target server as an HTTP proxy) and `snmp-brute` (which tries to guess a device's SNMP community string by sending common values such as `public`, `private`, and `cisco`). Unless a script is in the special `version` category, it should be categorized as either `safe` or `intrusive`.]()

[]()[ `malware` ]()

[These scripts test whether the target platform is infected by malware or backdoors. Examples include `smtp-strangeport`, which watches for SMTP servers running on unusual port numbers, and `auth-spoof`, which detects identd spoofing daemons which provide a fake answer before even receiving a query. Both of these behaviors are commonly associated with malware infections.]()

[]()[ `safe` ]()

[Scripts which weren't designed to crash services, use large amounts of network bandwidth or other resources, or exploit security holes are categorized as `safe`. These are less likely to offend remote administrators, though (as with all other Nmap features) we cannot guarantee that they won't ever cause adverse reactions. Most of these perform general network discovery. Examples are `ssh-hostkey` (retrieves an SSH host key) and `html-title` (grabs the title from a web page). Scripts in the `version` category are not categorized by safety, but any other scripts which aren't in `safe` should be placed in `intrusive`.]()

[]()[ ]()[ `version` ]()

[The scripts in this special category are an extension to the version detection feature and cannot be selected explicitly. They are selected to run only if version detection (`-sV`) was requested. Their output cannot be distinguished from version detection output and they do not produce service or host script results. Examples are `skypev2-version`, `pptp-version`, and `iax2-version`.]()

[]()[ `vuln` ]()

[These scripts check for specific known vulnerabilities and generally only report results if they are found. Examples include `realvnc-auth-bypass` and `afp-path-vuln`.]()

### [Script Types and Phases]() ###

[ NSE supports four types of scripts, which are distinguished by the kind of targets they take and the scanning phase in which they are run. Individual scripts may support multiple types of operation. ]()

[Prerule scripts]()

[These scripts run before any of Nmap's scan phases, so Nmap has not collected any information about its targets yet. They can be useful for tasks which don't depend on specific scan targets, such as performing network broadcast requests to query DHCP and DNS SD servers. Some of these scripts can generate new targets for Nmap to scan (only if you specify the ]()[newtargets](https://nmap.org/nsedoc/lib/target.html) NSE argument). For example, [dns-zone-transfer](https://nmap.org/nsedoc/scripts/dns-zone-transfer.html) can obtain a list of IPs in a domain using a zone transfer request and then automatically add them to Nmap's scan target list. Prerule scripts can be identified by containing a `prerule` function (see [the section called “Rules”](https://nmap.org/book/nse-script-format.html#nse-format-rules)).

Host scripts

Scripts in this phase run during Nmap's normal scanning process after Nmap has performed host discovery, port scanning, version detection, and OS detection against the target host. This type of script is invoked once against each target host which matches its `hostrule` function. Examples are [whois-ip](https://nmap.org/nsedoc/scripts/whois-ip.html), which looks up ownership information for a target IP, and [path-mtu](https://nmap.org/nsedoc/scripts/path-mtu.html) which tries to determine the maximum IP packet size which can reach the target without requiring fragmentation.

Service scripts

These scripts run against specific services listening on a target host. For example, Nmap includes more than 15 http service scripts to run against web servers. If a host has web servers running on multiple ports, those scripts may run multiple times (one for each port). These are the most commong Nmap script type, and they are distinguished by containing a `portrule` function for deciding which detected services a script should run against.

Postrule scripts

These scripts run after Nmap has scanned all of its targets. They can be useful for formatting and presenting Nmap output. For example, [ssh-hostkey](https://nmap.org/nsedoc/scripts/ssh-hostkey.html) is best known for its service (portrule) script which connects to SSH servers, discovers their public keys, and prints them. But it also includes a postrule which checks for duplicate keys amongst all of the hosts scanned, then prints any that are found. Another potential use for a postrule script is printing a reverse-index of the Nmap output—showing which hosts run a particular service rather than just listing the services on each host. Postrule scripts are identified by containing a `postrule` function.

Many scripts could potentially run as either a prerule or postrule script. In those cases, we recommend using a prerule for consistency.

### Command-line Arguments ###

 These are the five command-line arguments specific to script scanning:

[`-sC` ]()

[Performs a script scan using the default set of scripts. It is equivalent to `--script=default`. Some of the scripts in this `default` category are considered intrusive and should not be run against a target network without permission. ]()

[]()[ `--script *`<filename>`*|*`<category>`*|*`<directory>`*/|*`<expression>`*[,...]`]()

[Runs a script scan using the comma-separated list of filenames, script
categories, and directories. Each element in the list may also be a
Boolean expression describing a more complex set of scripts. Each
element is interpreted first as an expression, then as a category, and
finally as a file or directory name. The special argument`all` makes every script in Nmap's script database
eligible to run. The `all` argument should be used with caution as NSE may contain dangerous scripts including exploits, brute force authentication crackers, and denial of service attacks.]()

[Each element in the script expression list may be prefixed with a`+` character to force the given script(s) to run
regardless of the conditions in their `portrule` or`hostrule` functions. This is generally only done by
advanced users in special cases. For example, you might want to do a
configuration review on a bunch of MS SQL servers, some of which are
running on nonstandard ports. Rather than slow the Nmap scan by
running extensive version detection (`-sV
--version-all`) so that Nmap will recognize the `ms-sql`service, you can force the `ms-sql-config` script to run against all the
targeted hosts and ports by specifying `--script
+ms-sql-config`.]()

[File and directory names may be relative or absolute. Absolute names are
used directly. Relative paths are searched for in the`scripts` subdirectory of each of the following places until
found:]()[]()[]()

|                                            `--datadir`                                             |
|----------------------------------------------------------------------------------------------------|
|                                           `$NMAPDIR`[]()                                           |
|                              `~/.nmap` (not searched on Windows)[]()                               |
|                             `*`<APPDATA>`*\nmap` (only on Windows)[]()                             |
|                           the directory containing the `nmap`executable                            |
|the directory containing the `nmap`executable, followed by `../share/nmap` (not searched on Windows)|
|                            `NMAPDATADIR`[ (not searched on Windows)]()                             |
|                                       the current directory.                                       |

[]()

[When a directory name ending in `/` is given, Nmap loads every file in the directory
whose name ends with `.nse`. All other files are
ignored and directories are not searched recursively. When a filename is
given, it does not have to have the `.nse` extension;
it will be added automatically if necessary.]()

[See ]()[the section called “Script Selection”](https://nmap.org/book/nse-usage.html#nse-script-selection) for examples and a full
explanation of the `--script` option.

[]()

[Nmap scripts are stored in a `scripts`subdirectory of the Nmap data directory by default (see]()[Chapter 14, *Understanding and Customizing Nmap Data Files*](https://nmap.org/book/data-files.html)). For efficiency, scripts are indexed in
a database stored
in `scripts/script.db`,[which lists the category or categories in which each script belongs.
The argument `all` will execute all scripts in the
Nmap script database, but should be used cautiously since Nmap may contain exploits, denial of service attacks, and other dangerous scripts.]()

[]()[ `--script-args *`<args>`*` ]()

[Provides arguments to the scripts. See ]()[the section called “Arguments to Scripts”](https://nmap.org/book/nse-usage.html#nse-args) for a detailed explanation.

[`--script-args-file *`<filename>`*` ]()

[This option is the same as `--script-args` except that you pass the arguments in a file rather than on the command-line. See ]()[the section called “Arguments to Scripts”](https://nmap.org/book/nse-usage.html#nse-args) for a detailed explanation.

[`--script-help *`<filename>`*|*`<category>`*|*`<directory>`*|*`<expression>`*|all[,...]` ]()

[ Shows help about scripts. For each script matching the given specification, Nmap prints the script name, its categories, and its description. The specifications are the same as those accepted by `--script`; so for example if you want help about the `ssl-enum-ciphers` script, you would run **nmap --script-help ssl-enum-ciphers**. A sample of script help is shown in ]()[Example 9.2, “Script help”](https://nmap.org/book/nse-usage.html#nse-script-help).

Example 9.2. Script help

[

```
$ nmap --script-help "afp-* and discovery"

Starting Nmap 7.40 ( https://nmap.org ) at 2017-04-21 14:15 UTC

afp-ls
Categories: discovery safe
https://nmap.org/nsedoc/scripts/afp-ls.html
  Attempts to get useful information about files from AFP volumes.
  The output is intended to resemble the output of ls.

afp-serverinfo
Categories: default discovery safe
https://nmap.org/nsedoc/scripts/afp-serverinfo.html
  Shows AFP server information. This information includes the server's
  hostname, IPv4 and IPv6 addresses, and hardware type (for example
  Macmini or MacBookPro).

afp-showmount
Categories: discovery safe
https://nmap.org/nsedoc/scripts/afp-showmount.html
  Shows AFP shares and ACLs.

```

]()

[  
]()

[ If the `-oX`]()[ option is used, an XML representation of the script help will be written to the given file. ]()

[]()[ `--script-trace` ]()

[ This option is similar to `--packet-trace`, but works at the application level rather than packet by packet. If this option is specified, all incoming and outgoing communication performed by scripts is printed. The displayed information includes the communication protocol, source and target addresses, and the transmitted data. If more than 5% of transmitted data is unprintable, hex dumps are given instead. Specifying `--packet-trace` enables script tracing too. ]()

[]()[ `--script-updatedb` ]()

[This option updates the script database found in `scripts/script.db` which is used by Nmap to determine the available default scripts and categories. It is only necessary to update the database if you have added or removed NSE scripts from the default `scripts` directory or if you have changed the categories of any script. This option is used by itself without arguments: **nmap --script-updatedb**.]()

[ Some other Nmap options have effects on script scans. The most prominent of these is `-sV`.]()[]() A version scan automatically executes the scripts in the `version` category.[]() The scripts in this category are slightly different from other scripts because their output blends in with the version scan results and they do not produce any script scan output to the screen. If the `-oX`[]() option is used, typical script output will still be available in the XML output file.

 Another option which affects the scripting engine is `-A`.[]() The aggressive Nmap mode implies the `-sC` option.

### Script Selection ###

[]()[]()

 The `--script` option takes a comma-separated list of categories, filenames, and directory names. Some simple examples of its use:

**nmap --script default,safe**

Loads all scripts in the `default` and `safe` categories.

**nmap --script smb-os-discovery**

Loads only the `smb-os-discovery` script. Note that the `.nse` extension is optional.

**nmap --script default,banner,/home/user/customscripts**

Loads the script in the `default` category, the `banner` script, and all `.nse` files in the directory `/home/user/customscripts`.

[]()

 When referring to scripts from `script.db` by name, you can use a shell-style ‘`*`’ wildcard.

**nmap --script "http-\*"**

Loads all scripts whose name starts with `http-`, such as `http-auth` and `http-open-proxy`. The argument to `--script` had to be in quotes to protect the wildcard from the shell.

[]()

 More complicated script selection can be done using the `and`, `or`, and `not` operators to build Boolean expressions. The operators have the same [precedence](https://lua.org/manual/5.4/manual.html#3.4.8) as in Lua: `not` is the highest, followed by `and` and then `or`. You can alter precedence by using parentheses. Because expressions contain space characters it is necessary to quote them.

**nmap --script "not intrusive"**

Loads every script except for those in the `intrusive` category.

**nmap --script "default or safe"**

This is functionally equivalent to **nmap --script "default,safe"**. It loads all scripts that are in the `default` category or the `safe` category or both.

**nmap --script "default and safe"**

Loads those scripts that are in *both* the `default` and `safe` categories.

**nmap --script "(default or safe or intrusive) and not http-\*"**

Loads scripts in the `default`, `safe`, or `intrusive` categories, except for those whose names start with `http-`.

 Names in a Boolean expression may be a category, a filename from `script.db`, or `all`. A name is any sequence of characters not containing ‘` `’, ‘`,`’, ‘`(`’, ‘`)`’, or ‘`;`’, except for the sequences `and`, `or`, and `not`, which are operators.

### Arguments to Scripts ###

[]()

 Arguments may be passed to NSE scripts using the `--script-args` option. The arguments describe a table of key-value pairs and possibly array values. The arguments are provided to scripts as a table in the registry called `nmap.registry.args`, though they are normally accessed through the `stdnse.get_script_args` function.

 The syntax for script arguments is similar to Lua's table constructor syntax. Arguments are a comma-separated list of `name=value` pairs. Names and values may be strings not containing whitespace or the characters ‘`{`’, ‘`}`’, ‘`=`’, or ‘`,`’. To include one of these characters in a string, enclose the string in single or double quotes. Within a quoted string, ‘`\`’ escapes a quote. A backslash is only used to escape quotation marks in this special case; in all other cases a backslash is interpreted literally.

 Values may also be tables enclosed in `{}`, just as in Lua. A table may contain simple string values, for example a list of proxy hosts; or more name-value pairs, including nested tables.

Script arguments are often qualified with the relevant script name so that a user doesn't unintentionally affect multiple scripts with a single generic name. For example, you can set the timeout for responses to the `broadcast-ping` script (and only that script) by setting `broadcast-ping.timeout` to the amount of time you're willing to wait. Sometimes, however, you want a script argument applied more widely. If you remove the qualification and specify just `timeout=250ms`, you will be setting the value for more than a dozen scripts in addition to `broadcast-ping`. You can even combine qualified and unqualified arguments, and the most specific match takes precedence. For example, you could specify `rlogin-brute.timeout=20s,timeout=250ms`. In that case, the timeout will be 20 seconds for the `rlogin-brute` script, and 250 milliseconds for all other scripts which support this variable (`broadcast-ping`, `lltd-discovery`, etc.)

Rather than pass the arguments on the command line with `--script-args`, you may store them in a file (separated by commas or newlines) and specify just the file name with `--script-args-file`. Options specified with `--script-args` on the command-line take precedence over those given in a file. The filename may be given as an absolute path or relative to Nmap's usual search path (NMAPDIR, etc.)

Here is a typical Nmap invocation with script arguments:

[]()

**nmap -sC --script-args 'user=foo,pass=",{}=bar",paths={/admin,/cgi-bin},xmpp-info.server\_name=localhost'**  

 Notice that the script arguments are surrounded in single quotes. For the Bash shell, this prevents the shell from interpreting the double quotes and doing automatic string concatenation. Naturally, different shells may require you to escape quotes or to use different quotes. See your relevant manual. The command results in this Lua table:

```
nmap.registry.args = {
  user = "foo",
  pass = ",{}=bar",
  paths = {
    "/admin",
    "/cgi-bin"
  },
  xmpp-info.server_name="localhost"
}

```

 While you could access the values directly from `nmap.registry.args`, it is normally better to use the `stdnse.get_script_args` function like this:

```
local server_name = stdnse.get_script_args("xmpp-info.server_name")

```

 All script arguments share a global namespace, the `nmap.registry.args` table. For this reason, short or ambiguous names like `user` are not recommended. Some scripts prefix their arguments with their script name, like `smtp-open-relay.domain`. Arguments used by libraries, which can affect many scripts, usually have names beginning with the name of the library, like `smbuser` and `creds.snmp`.

 The online NSE Documentation Portal at [`https://nmap.org/nsedoc/`](https://nmap.org/nsedoc/) lists the arguments that each script accepts, including any library arguments that may influence the script.

### Complete Examples ###

**nmap -sC example.com**

A simple script scan using the default set of scripts.

**nmap -sn -sC example.com**

A script scan without a port scan; only host scripts are eligible to run.

**nmap -Pn -sn -sC example.com**

A script scan without host discovery or a port scan. All hosts are assumed up and only host scripts are eligible to run.

[**nmap --script smb-os-discovery --script-trace example.com**]()

[Execute a specific script with script tracing.]()

[]()[ **nmap --script snmp-sysdescr --script-args creds.snmp=admin example.com**]()

[Run an individual script that takes a script argument.]()

[**nmap --script mycustomscripts,safe example.com**]()

[Execute all scripts in the `mycustomscripts` directory as well as all scripts in the `safe` category.]()

---

[Prev](https://nmap.org/book/nse.html)Chapter 9. Nmap Scripting Engine

[Up](https://nmap.org/book/nse.html)Chapter 9. Nmap Scripting Engine

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/nse-script-format.html)Script Format