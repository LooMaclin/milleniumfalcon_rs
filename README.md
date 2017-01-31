#Environment:
```
[loomaclin@localhost milleniumfalcon-rs]$ uname -a
Linux localhost 4.8.13-1-ARCH #1 SMP PREEMPT Fri Dec 9 07:24:34 CET 2016 x86_64 GNU/Linux

[loomaclin@localhost milleniumfalcon-rs]$ archey3

               +                OS: Arch Linux x86_64
               #                Hostname: localhost
              ###               Kernel Release: 4.8.13-1-ARCH
             #####              Uptime: 11 days, 2:37
             ######             WM: None
            ; #####;            DE: Cinnamon
           +##.#####            Packages: 869
          +##########           RAM: 7199 MB / 15927 MB
         #############;         Processor Type: Intel(R) Core(TM) i5-6400 CPU @ 2.70GHz
        ###############+        $EDITOR: None
       #######   #######        Root: 44G / 118G (37%) (ext4)
     .######;     ;###;`".      
    .#######;     ;#####.       
    #########.   .########`     
   ######'           '######    
  ;####                 ####;   
  ##'                     '##   
 #'                         `#  

```
#Test command:
```
wrk -d1m -c7168 -t4 -s ~/IdeaProjects/milleniumfalcon-rs/planets.lua --latency http://127.0.0.1:1337/planets.json
```

#Tests:

##Test 1: 4 threads - simple echo server based on hyper-async-master-branch with deserealization and serialization based on serde:

```
Running 1m test @ http://127.0.0.1:1337/planets.json
  4 threads and 7168 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    21.37ms    8.96ms 237.36ms   76.05%
    Req/Sec    11.77k     2.63k   22.07k    69.00%
  Latency Distribution
     50%   18.92ms
     75%   27.15ms
     90%   32.41ms
     99%   48.21ms
  2811605 requests in 1.00m, 13.26GB read
  Socket errors: connect 6151, read 28, write 3137818, timeout 0
Requests/sec:  46791.60
Transfer/sec:    225.98MB
```

##Test 2 (01.02.2017): 8 threads - i7-4720hq - echo server with ser/deser json, with tarantool select one tuple from db (my sync connector):

```
Running 1m test @ http://127.0.0.1:1337/planets.json
  4 threads and 7168 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    12.79ms    7.03ms 107.36ms   72.27%
    Req/Sec     6.22k     1.86k   11.51k    62.05%
  Latency Distribution
     50%   11.76ms
     75%   16.54ms
     90%   21.56ms
     99%   34.82ms
  1484519 requests in 1.00m, 7.00GB read
  Socket errors: connect 6151, read 757, write 11130087, timeout 0
Requests/sec:  24713.61
Transfer/sec:    119.35MB
```

##Test 2 (01.02.2017): 8 threads - i7-4720hq - echo server with ser/deser json, with postgres select one record from db (sfackler sync connector):

```
Running 1m test @ http://127.0.0.1:1337/planets.json
  4 threads and 7168 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    51.80ms   22.73ms 232.01ms   72.23%
    Req/Sec     4.38k     1.39k    8.93k    62.83%
  Latency Distribution
     50%   48.39ms
     75%   66.36ms
     90%   80.03ms
     99%  123.94ms
  1042202 requests in 1.00m, 4.92GB read
  Socket errors: connect 6151, read 314, write 6811891, timeout 0
Requests/sec:  17352.75
Transfer/sec:     83.80MB
```

