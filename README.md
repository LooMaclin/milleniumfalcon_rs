11 threads - simple echo server - i7-4720HQ:

```
[loomaclin@localhost hyper_test]$ wrk -d1m -c10000 -t11 -s ~/IdeaProjects/hyper_test/planets.lua --latency http://127.0.0.1:1337/planets.json
Running 1m test @ http://127.0.0.1:1337/planets.json
  11 threads and 10000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.22ms    4.19ms  78.36ms   87.05%
    Req/Sec    13.29k     6.08k   28.31k    58.32%
  Latency Distribution
     50%    6.95ms
     75%    8.48ms
     90%   13.60ms
     99%   24.78ms
  7139506 requests in 1.00m, 45.22GB read
  Socket errors: connect 8989, read 0, write 5358508, timeout 0
Requests/sec: 118891.37
Transfer/sec:    771.12MB

```

11 threads - simple echo server with deserealization and serialization - i7-4720HQ:

```

```