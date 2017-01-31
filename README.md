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

##Test 2 (01.02.2017): 4 threads - echo server with ser/deser json, with tarantool select one tuple from db (my sync connector):

```
[loomaclin@localhost milleniumfalcon-rs]$ wrk -d1m -c7168 -t4 -s ~/IdeaProjects/milleniumfalcon-rs/planets.lua --latency http://127.0.0.1:1337/planets.json
Running 1m test @ http://127.0.0.1:1337/planets.json
  4 threads and 7168 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    17.64ms    7.51ms  85.90ms   77.46%
    Req/Sec     4.98k     2.80k   11.67k    58.92%
  Latency Distribution
     50%   14.56ms
     75%   20.97ms
     90%   28.90ms
     99%   41.64ms
  1187319 requests in 1.00m, 5.60GB read
  Socket errors: connect 6151, read 297, write 13482535, timeout 0
Requests/sec:  19765.89
Transfer/sec:     95.46MB
```


