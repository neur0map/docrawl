---
title: "Script Parallelism in NSE | Nmap Network Scanning"
source_url: https://nmap.org/book/nse-parallelism.html
fetched_at: 2025-09-17T16:43:59.553703+00:00
---

Blocked for possible web abuse

Blocked for possible web abuse
==========

The IP address you are coming from has requested an inordinately large number of pages in a short amount of time and has been temporarily blocked to conserve our resources. This often happens when people try to use web spidering programs to download large portions of the site. The block will be removed 24 hours after the latest period of high traffic. If you feel this IP ban was made in error, you can email fyodor@nmap.org.

* [Nmap Network Scanning](https://nmap.org/book/toc.html)
* [Chapter 9. Nmap Scripting Engine](https://nmap.org/book/nse.html)
* Script Parallelism in NSE

[Prev](https://nmap.org/book/nsedoc.html)

[Next](https://nmap.org/book/nse-vscan.html)

Script Parallelism in NSE
----------

 In [the section called “Network I/O API”](https://nmap.org/book/nse-api.html#nse-api-networkio), it was mentioned that NSE automatically parallelizes network operations. Usually this process is transparent to a script author, but there are some advanced techniques that require knowledge of how it works. The techniques covered in this section are controlling how multiple scripts interact in a library, using multiple threads in parallel, and disabling parallelism for special cases.

 The standard mechanism for parallel execution is a thread. A thread encapsulates the execution flow and data of a script. Lua thread may be yielded at arbitrary locations to continue work on another script. Typically, these yield locations are blocking socket operations in the `nmap`[]() library. The yield back to the script is also transparent, a side effect of the socket operation.

 Let's go over some common terminology. A *script* is analogous to a binary executable; it holds the information necessary to execute a script. A *thread* (a Lua coroutine) is analogous to a process; it runs a script against a host and possibly port. Sometimes we abuse terminology and refer to a running thread as a running “script”, but what this really means is an instantiation of a script, in the same way that a process is the instantiation of an executable.

 NSE provides the bare-bone essentials needed to expand parallelism basic model of one thread per script: new independent threads, mutexes, and condition variables.

### Worker Threads ###

 There are several instances where a script needs finer control with respect to parallel execution beyond what is offered by default with a generic script. A common need is to read from multiple sockets concurrently. For example, an HTTP spidering script may want to have multiple Lua threads querying web server resources in parallel. To answer this need, NSE offers the function `stdnse.new_thread` to create worker threads. These worker threads have all the power of independent scripts with the only restriction that they may not report script output.

 Each worker thread launched by a script is given a main function and a variable number of arguments to be passed to the main function by NSE:

`worker_thread, status_function = stdnse.new_thread(main, ...)`

`stdnse.new_thread` returns two values: the Lua thread (coroutine) that uniquely identifies your worker thread, and a status query function that queries the status of your new worker. The status query function returns two values:

`status, error_object = status_function()`

 The first return value is simply the return value of `coroutine.status` run on the worker thread coroutine. (More precisely, the `base` coroutine. Read more about `base` coroutine in [the section called “The base thread”](https://nmap.org/book/nse-parallelism.html#nse-parallelism-base).) The second return value contains an error object that caused the termination of the worker thread, or `nil` if no error was thrown. This object is typically a string, like most Lua errors. However, any Lua type can be an error object, even `nil`. Therefore inspect the error object, the second return value, only if the status of the worker is `"dead"`.

 NSE discards all return values from the main function when the worker thread finishes execution. You should communicate with your worker through the use of `main` function parameters, upvalues, or function environments. See [Example 9.10](https://nmap.org/book/nse-parallelism.html#nse-worker-example) for an example.

 Finally, when using worker threads you should always use condition variables or mutexes to coordinate them. Nmap is single-threaded so there are no memory synchronization issues to worry about; but there *is* contention for resources. These resources include usually network bandwidth and sockets. Condition variables are also useful if the work for any single thread is dynamic. For example, a web server spider script with a pool of workers will initially have a single root HTML document. Following the retrieval of the root document, the set of resources to be retrieved (the worker's work) may become very large as each new document adds new URLs to fetch.

Example 9.10. Worker threads

```
local requests = {"/", "/index.html", --[[ long list of objects ]]}

function thread_main (host, port, responses, ...)
  local condvar = nmap.condvar(responses);
  local what = {n = select("#", ...), ...};
  local allReqs = nil;
  for i = 1, what.n do
    allReqs = http.pGet(host, port, what[i], nil, nil, allReqs);
  end
  local p = assert(http.pipeline(host, port, allReqs));
  for i, response in ipairs(p) do responses[#responses+1] = response end
  condvar "signal";
end

function many_requests (host, port)
  local threads = {};
  local responses = {};
  local condvar = nmap.condvar(responses);
  local i = 1;
  repeat
    local j = math.min(i+10, #requests);
    local co = stdnse.new_thread(thread_main, host, port, responses,
        unpack(requests, i, j));
    threads[co] = true;
    i = j+1;
  until i > #requests;
  repeat
    for thread in pairs(threads) do
      if coroutine.status(thread) == "dead" then threads[thread] = nil end
    end
    if ( next(threads) ) then
      condvar "wait"
    end
  until next(threads) == nil;
  return responses;
end

```

 For brevity, this example omits typical behavior of a traditional web spider. The requests table is assumed to contain enough objects to warrant the use of worker threads. The code in this example dispatches a new thread with as many as 11 relative URLs. Worker threads are cheap, so don't be afraid to create a lot of them. After dispatching all these threads, the code waits on a condition variable until every thread has finished, then finally return the responses table.

 You may have noticed that we did not use the status function returned by `stdnse.new_thread`. You will typically use this for debugging or if your program must stop based on the error thrown by one of your worker threads. Our simple example did not require this but a more fault-tolerant library may.

### Mutexes ###

[]()[]()

 Recall from the beginning of this section that each script execution thread (e.g. `ftp-anon` running against an FTP server on a target host) yields to other scripts whenever it makes a call on network objects (sending or receiving data). Some scripts require finer concurrency control over thread execution. An example is the `whois`[]() script which queries whois[]() servers for each target IP address. Because many concurrent queries can get your IP banned for abuse, and because a single query may return the same information another instance of the script is about to request, it is useful to have other threads pause while one thread performs a query.

 To solve this problem, NSE includes a `mutex` function which provides a [mutex](http://en.wikipedia.org/wiki/Mutual_exclusion) (mutual exclusion object) usable by scripts. The mutex allows for only one thread to be working on an object at a time. Competing threads waiting to work on this object are put in the waiting queue until they can get a “lock” on the mutex. A solution for the `whois` problem above is to have each thread block on a mutex using a common string, ensuring that only one thread at a time is querying a server. When finished querying the remote servers, the thread can store results in the NSE registry and unlock the mutex. Other scripts waiting to query the remote server can then obtain a lock, check for the cache for a usable result from a previous query, make their own queries, and unlock the mutex. This is a good example of serializing access to a remote resource.

 The first step in using a mutex is to create one with a call to `nmap.mutex`.

```
mutexfn = nmap.mutex(object)

```

 The `mutexfn` returned is a function which works as a mutex for the `object` passed in. This object can be any [Lua data type](https://lua.org/manual/5.4/manual.html#2.1) except `nil`, Boolean, and number. The returned function allows you to lock, try to lock, and release the mutex. Its sole argument must be one of the following:

`"lock"`

 Makes a blocking lock on the mutex. If the mutex is busy (another thread has a lock on it), then the thread will yield and wait. The function returns with the mutex locked.

`"trylock"`

 Makes a non-blocking lock on the mutex. If the mutex is busy then it immediately returns with a return value of `false`. Otherwise, locks the mutex and returns `true`.

`"done"`

 Releases the mutex and allows another thread to lock it. If the thread does not have a lock on the mutex, an error will be raised.

`"running"`

 Returns the thread locked on the mutex or `nil` if the mutex is not locked. This should only be used for debugging as it interferes with garbage collection of finished threads.

 NSE maintains a weak reference to the mutex so other calls to `nmap.mutex` with the same object will return the same mutex function. However, if you discard your reference to the mutex then it may be collected and subsequent calls to `nmap.mutex` with the object will return a different function. Therefore save your mutex to a (local) variable that persists as long as you need it.

 A simple example of using the API is provided in [Example 9.11](https://nmap.org/book/nse-parallelism.html#nse-mutex-handling). For real-life examples, read the `asn-query`[]() and `whois`[]() scripts in the Nmap distribution.

Example 9.11. Mutex manipulation

```
local mutex = nmap.mutex("My Script's Unique ID");
function action(host, port)
  mutex "lock";
  -- Do critical section work - only one thread at a time executes this.
  mutex "done";
  return script_output;
end

```

### Condition Variables ###

 Condition variables arose out of a need to coordinate with worker threads created by the `stdnse.new_thread` function. A condition variable allows many threads to wait on one object, and one or all of them to be awakened when some condition is met. Said differently, multiple threads may unconditionally `block` on the condition variable by *waiting*. Other threads may use the condition variable to wake up the waiting threads.

 For example, consider the earlier [Example 9.10, “Worker threads”](https://nmap.org/book/nse-parallelism.html#nse-worker-example). Until all the workers finish, the controller thread must sleep. Note that we cannot `poll` for results like in a traditional operating system thread because NSE does not preempt Lua threads. Instead, we use a condition variable that the controller thread *waits* on until awakened by a worker. The controller will continually wait until all workers have terminated.

 The first step in using a condition variable is to create one with a call to `nmap.condvar`.

`condvarfn = nmap.condvar(object)`

 The semantics for condition variables are similar to those of mutexes. The `condvarfn` returned is a function which works as a condition variable for the `object` passed in. This object can be any [Lua data type](https://lua.org/manual/5.4/manual.html#2.1) except `nil`, Boolean, and number. The returned function allows you to wait, signal, and broadcast on the condition variable. Its sole argument must be one of the following:

`"wait"`

 Wait on the condition variable. This adds the current thread to the waiting queue for the condition variable. It will resume execution when another thread signals or broadcasts on the condition variable.

`"signal"`

 Signal the condition variable. One of the threads in the condition variable's waiting queue will be resumed.

`"broadcast"`

 Resume all the threads in the condition variable's waiting queue.

 Like with mutexes, NSE maintains a weak reference to the condition variable so other calls to `nmap.condvar` with the same object will return the same function. However, if you discard your reference to the condition variable then it may be collected and subsequent calls to `nmap.condvar` with the object will return a different function. Therefore save your condition variable to a (local) variable that persists as long as you need it.

 When using condition variables, it is important to check the predicate before and after waiting. A predicate is a test on whether to continue doing work within a worker or controller thread. For worker threads, this will at the very least include a test to see if the controller is still alive. You do not want to continue doing work when there's no thread to use your results. A typical test before waiting may be: Check whether the controller is still running; if not, then quit. Check if is work to be done; if not, then wait.

 A thread waiting on a condition variable may be resumed without any other thread having called `"signal"` or `"broadcast"` on the condition variable (a spurious wakeup). The usual, but not only, reason that this may happen is the termination of one of the threads using the condition variable. This is an important guarantee NSE makes that allows you to avoid deadlock where a worker or controller waits for a thread to wake them up that ended without signaling the condition variable.

### Collaborative Multithreading ###

 One of Lua's least-known features is collaborative multithreading through *coroutines*. A coroutine provides an independent execution stack that can be yielded and resumed. The standard `coroutine` table provides access to the creation and manipulation of coroutines. Lua's online first edition of [*Programming in Lua*](https://lua.org/pil/) contains an excellent introduction to coroutines. What follows is an overview of the use of coroutines here for completeness, but this is no replacement for the definitive reference.

 We have mentioned coroutines throughout this section as *threads*. This is the *type* (`"thread"`) of a coroutine in Lua. They are not the preemptive threads that programmers may be expecting. Lua threads provide the basis for parallel scripting but only one thread is ever running at a time.

 A Lua function executes on top of a Lua thread. The thread maintains a stack of active functions, local variables, and the current instruction pointer. We can switch between coroutines by explicitly yielding the running thread. The coroutine which resumed the yielded thread resumes operation. [Example 9.12](https://nmap.org/book/nse-parallelism.html#nse-cm-coroutines) shows a brief use of coroutines to print numbers.

Example 9.12. Basic Coroutine Use

```
local function main ()
  coroutine.yield(1)
  coroutine.yield(2)
  coroutine.yield(3)
end
local co = coroutine.create(main)
for i = 1, 3 do
  print(coroutine.resume(co))
end
--> true    1
--> true    2
--> true    3

```

 Coroutines are the facility that enables NSE to run scripts in parallel. All scripts are run as coroutines that yield whenever they make a blocking socket function call. This enables NSE to run other scripts and later resume the blocked script when its I/O operation has completed.

 Sometimes coroutines are the best tool for a job within a single script. One common use in socket programming is filtering data. You may write a function that generates all the links from an HTML document. An iterator using `string.gmatch` can catches only a single pattern. Because some complex matches may take many different Lua patterns, it is more appropriate to use a coroutine. [Example 9.13](https://nmap.org/book/nse-parallelism.html#nse-cm-links) shows how to do this.

Example 9.13. Link Generator

```
function links (html_document)
  local function generate ()
    for m in string.gmatch(html_document, "url%((.-)%)") do
      coroutine.yield(m) -- css url
    end
    for m in string.gmatch(html_document, "href%s*=%s*\"(.-)\"") do
      coroutine.yield(m) -- anchor link
    end
    for m in string.gmatch(html_document, "src%s*=%s*\"(.-)\"") do
      coroutine.yield(m) -- img source
    end
  end
  return coroutine.wrap(generate)
end

function action (host, port)
  -- ... get HTML document and store in html_document local
  for link in links(html_document) do
    links[#links+1] = link; -- store it
  end
  -- ...
end

```

#### The base thread ####

 Because scripts may use coroutines for their own multithreading, it is important to be able to identify the *owner* of a resource or to establish whether the script is still alive. NSE provides the function `stdnse.base` for this purpose.

 Particularly when writing a library that attributes ownership of a cache or socket to a script, you can use the base thread to establish whether the script is still running. `coroutine.status` on the base thread will give the current state of the script. In cases where the script is `"dead"`, you will want to release the resource. Be careful with keeping references to these threads; NSE may discard a script even though it has not finished executing. The thread will still report a status of `"suspended"`. You should keep a weak reference to the thread in these cases so that it may be collected.

---

[Prev](https://nmap.org/book/nsedoc.html)Writing Script Documentation (NSEDoc)

[Up](https://nmap.org/book/nse.html)Chapter 9. Nmap Scripting Engine

[Home](https://nmap.org/book/toc.html)

[Next](https://nmap.org/book/nse-vscan.html)Version Detection Using NSE