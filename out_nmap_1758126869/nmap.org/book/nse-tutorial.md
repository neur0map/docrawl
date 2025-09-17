---
title: "Script Writing Tutorial | Nmap Network Scanning"
source_url: https://nmap.org/book/nse-tutorial.html
fetched_at: 2025-09-17T16:43:59.305170+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 9. Nmap Scripting Engine](https://nmap.org/book/nse.html)
* Script Writing Tutorial

[Prev](https://nmap.org/book/nse-api.html)

[Next](https://nmap.org/book/nsedoc.html)

Script Writing Tutorial
----------

[]()

 Suppose that you are convinced of the power of NSE. How do you go about writing your own script? Let's say that you want to extract information from an identification server[]() to determine the owner of the process listening on a TCP port. This is not really the purpose of identd (it is meant for querying the owner of outgoing connections, not listening daemons), but many identd servers allow it anyway. Nmap used to have this functionality (called ident scan), but it was removed while transitioning to a new scan engine architecture. The protocol identd uses is pretty simple, but still too complicated to handle with Nmap's version detection language. First, you connect to the identification server and send a query of the form `*`<port-on-server>`*, *`<port-on-client>`*` and terminated with a newline character. The server should then respond with a string containing the server port, client port, response type, and address information. The address information is omitted if there is an error. More details are available in [RFC 1413](http://www.rfc-editor.org/rfc/rfc1413.txt), but this description is sufficient for our purposes. The protocol cannot be modeled in Nmap's version detection language for two reasons. The first is that you need to know both the local and the remote port of a connection. Version detection does not provide this data. The second, more severe obstacle, is that you need two open connections to the target—one to the identification server and one to the listening port you wish to query. Both obstacles are easily overcome with NSE.

 The anatomy of a script is described in [the section called “Script Format”](https://nmap.org/book/nse-script-format.html). In this section we will show how the described structure is utilized.

### The Head ###

 The head of the script is essentially its meta information. This includes the fields: `description`, `categories`, `dependencies`, `author`, and `license` as well as initial NSEDoc information such as usage, args, and output tags (see [the section called “Writing Script Documentation (NSEDoc)”](https://nmap.org/book/nsedoc.html)).

 The description field should contain a paragraph or more describing what the script does. If anything about the script results might confuse or mislead users, and you can't eliminate the issue by improving the script or results text, it should be documented in the `description`. If there are multiple paragraphs, the first is used as a short summary where necessary. Make sure that first paragraph can serve as a stand alone abstract. This description is short because it is such a simple script:

[]()[]()

```
description = [[
Attempts to find the owner of an open TCP port by querying an auth
(identd - port 113) daemon which must also be open on the target system.
]]

```

Next comes NSEDoc information. This script is missing the
common `@usage` and `@args` tags
since it is so simple, but it does have an
NSEDoc `@output` tag:

```
---
--@output
-- 21/tcp   open     ftp       ProFTPD 1.3.1
-- |_ auth-owners: nobody
-- 22/tcp   open     ssh       OpenSSH 4.3p2 Debian 9etch2 (protocol 2.0)
-- |_ auth-owners: root
-- 25/tcp   open     smtp      Postfix smtpd
-- |_ auth-owners: postfix
-- 80/tcp   open     http      Apache httpd 2.0.61 ((Unix) PHP/4.4.7 ...)
-- |_ auth-owners: dhapache
-- 113/tcp  open     auth?
-- |_ auth-owners: nobody
-- 587/tcp  open     submission Postfix smtpd
-- |_ auth-owners: postfix
-- 5666/tcp open     unknown
-- |_ auth-owners: root

```

 Next come the `author`, `license`, and `categories` tags. This script belongs to the `safe`[]() because we are not using the service for anything it was not intended for. Because this script is one that should run by default it is also in the `default`[]() category. Here are the variables in context:

[]()

```
author = "Diman Todorov"

license = "Same as Nmap--See https://nmap.org/book/man-legal.html"

categories = {"default", "safe"}

```

### The Rule ###

 The rule section is a Lua method which decides whether to skip or execute the script's action. This decision is usually based on the type of the rule and the host and port information passed to it. A `prerule` or a `postrule` will always evaluate to true. In the case of the identification script, it is slightly more complicated than that. To decide whether to run the identification script against a given port we need to know if there is an auth server running on the target machine. In other words, the script should be run only if the currently scanned TCP port is open and TCP port 113 is also open. For now we will rely on the fact that identification servers listen on TCP port 113. Unfortunately NSE only gives us information about the currently scanned port.

To find out if port 113 is open, we use the `nmap.get_port_state` function. If the auth port was not scanned, the `get_port_state` function returns `nil`. So we check that the table is not `nil`. We also check that both ports are in the `open` state. If this is the case, the action is executed, otherwise we skip the action.

[]()

```
portrule = function(host, port)
	local auth_port = { number=113, protocol="tcp" }
	local identd = nmap.get_port_state(host, auth_port)

	return identd ~= nil
		and identd.state == "open"
		and port.protocol == "tcp"
		and port.state == "open"
end

```

### The Action ###

 At last we implement the actual functionality! The script first connects to the port on which we expect to find the identification server, then it will connect to the port we want information about. Doing so involves first creating two socket options by calling `nmap.new_socket`. Next we define an error-handling `catch` function which closes those sockets if failure is detected. At this point we can safely use object methods such as `open`, `close`, `send` and `receive` to operate on the network socket. In this case we call `connect` to make the connections. NSE's exception handling mechanism[]() is used to avoid excessive error-handling code. We simply wrap the networking calls in a `try` call which will in turn call our `catch` function if anything goes wrong.

If the two connections succeed, we construct a query string and parse the response. If we received a satisfactory response, we return the retrieved information.

[]()

```
action = function(host, port)
        local owner = ""

        local client_ident = nmap.new_socket()
        local client_service = nmap.new_socket()

        local catch = function()
                client_ident:close()
                client_service:close()
        end

        local try = nmap.new_try(catch)

        try(client_ident:connect(host.ip, 113))
        try(client_service:connect(host.ip, port.number))

        local localip, localport, remoteip, remoteport =
                try(client_service:get_info())

        local request = port.number .. ", " .. localport .. "\r\n"

        try(client_ident:send(request))

        owner = try(client_ident:receive_lines(1))

        if string.match(owner, "ERROR") then
                owner = nil
        else
                owner = string.match(owner,
                        "%d+%s*,%s*%d+%s*:%s*USERID%s*:%s*.+%s*:%s*(.+)\r?\n")
        end

        try(client_ident:close())
        try(client_service:close())

        return owner
end

```

Note that because we know that the remote port is stored
in `port.number`, we could have ignored the last two
return values of `client_service:get_info()` like
this:

```
local localip, localport = try(client_service:get_info())

```

In this example we exit quietly if the service responds with an error. This is done by assigning `nil` to the `owner` variable which will be returned. NSE scripts generally only return messages when they succeed, so they don't flood the user with pointless alerts.

[]()

---

[Prev](https://nmap.org/book/nse-api.html)Nmap API

[Up](https://nmap.org/book/nse.html)Chapter 9. Nmap Scripting Engine

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/nsedoc.html)Writing Script Documentation (NSEDoc)