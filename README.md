# WASI test

## Prerequisites

- Enable both the containerd image store (general) and WASM support (Features in development) in docker desktop
- `make init` some dependencies

## Performance Results

Native rust binary:

```
     data_received..................: 152 MB  2.5 MB/s
     data_sent......................: 102 MB  1.7 MB/s
     http_req_blocked...............: avg=572ns  min=0s       med=0s     max=2.88ms  p(90)=1µs    p(95)=1µs   
     http_req_connecting............: avg=72ns   min=0s       med=0s     max=1.98ms  p(90)=0s     p(95)=0s    
     http_req_duration..............: avg=2.52ms min=454µs    med=2.11ms max=55.71ms p(90)=3.76ms p(95)=4.9ms 
       { expected_response:true }...: avg=2.52ms min=454µs    med=2.11ms max=55.71ms p(90)=3.76ms p(95)=4.9ms 
     http_req_failed................: 0.00%   ✓ 0            ✗ 1180877
     http_req_receiving.............: avg=7.42µs min=2µs      med=4µs    max=10.98ms p(90)=11µs   p(95)=18µs  
     http_req_sending...............: avg=2.17µs min=0s       med=1µs    max=10.07ms p(90)=2µs    p(95)=4µs   
     http_req_tls_handshaking.......: avg=0s     min=0s       med=0s     max=0s      p(90)=0s     p(95)=0s    
     http_req_waiting...............: avg=2.51ms min=448µs    med=2.1ms  max=55.69ms p(90)=3.75ms p(95)=4.89ms
     http_reqs......................: 1180877 19680.469546/s
     iteration_duration.............: avg=2.53ms min=463.37µs med=2.12ms max=55.97ms p(90)=3.78ms p(95)=4.92ms
     iterations.....................: 1180877 19680.469546/s
     vus............................: 50      min=50         max=50   
     vus_max........................: 50      min=50         max=50  
```

WASM-compiled binary:

```
     data_received..................: 8.8 MB 146 kB/s
     data_sent......................: 5.9 MB 98 kB/s
     http_req_blocked...............: avg=3.12µs  min=0s      med=1µs     max=2.56ms  p(90)=3µs     p(95)=3µs    
     http_req_connecting............: avg=967ns   min=0s      med=0s      max=1.58ms  p(90)=0s      p(95)=0s     
     http_req_duration..............: avg=43.98ms min=29.7ms  med=43.07ms max=89.71ms p(90)=46.81ms p(95)=47.26ms
       { expected_response:true }...: avg=43.98ms min=29.7ms  med=43.07ms max=89.71ms p(90)=46.81ms p(95)=47.26ms
     http_req_failed................: 0.00%  ✓ 0           ✗ 68150
     http_req_receiving.............: avg=41ms    min=6µs     med=40.74ms max=70.83ms p(90)=41.94ms p(95)=43.05ms
     http_req_sending...............: avg=7.83µs  min=1µs     med=5µs     max=10.3ms  p(90)=13µs    p(95)=17µs   
     http_req_tls_handshaking.......: avg=0s      min=0s      med=0s      max=0s      p(90)=0s      p(95)=0s     
     http_req_waiting...............: avg=2.97ms  min=605µs   med=2.31ms  max=62.41ms p(90)=5.49ms  p(95)=6.32ms 
     http_reqs......................: 68150  1135.226252/s
     iteration_duration.............: avg=44.02ms min=32.26ms med=43.11ms max=89.83ms p(90)=46.85ms p(95)=47.3ms 
     iterations.....................: 68150  1135.226252/s
     vus............................: 50     min=50        max=50 
     vus_max........................: 50     min=50        max=50 
```

19680.469546/s for rust, 1135.226252/s for wasm, unexpectedly slow!


![alt text](docs/image.png)

higher memory usage too, probably the extra load from the wasm runtime?

- WASM image size: 2.76MB
- RUST image size: 7.31MB

WASM image is smaller, so that stands up to the promise at least. Didn't try REALLY hard to make those super small, but they're closer than I expected for a super-simple stateless web server.

## TODO
- add golang