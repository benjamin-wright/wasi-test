# WASI test

## Prerequisites

- Enable both the containerd image store (general) and WASM support (Features in development) in docker desktop
- `make init` some dependencies

## Performance Results

Native rust binary:

```
     data_received..................: 82 MB  2.7 MB/s
     data_sent......................: 54 MB  1.8 MB/s
     http_req_blocked...............: avg=677ns  min=0s       med=0s     max=2.94ms  p(90)=1µs    p(95)=1µs   
     http_req_connecting............: avg=121ns  min=0s       med=0s     max=1.83ms  p(90)=0s     p(95)=0s    
     http_req_duration..............: avg=2.35ms min=332µs    med=2ms    max=50.23ms p(90)=3.65ms p(95)=4.68ms
       { expected_response:true }...: avg=2.35ms min=332µs    med=2ms    max=50.23ms p(90)=3.65ms p(95)=4.68ms
     http_req_failed................: 0.00%  ✓ 0            ✗ 632147
     http_req_receiving.............: avg=8.33µs min=2µs      med=4µs    max=49.76ms p(90)=11µs   p(95)=19µs  
     http_req_sending...............: avg=2.3µs  min=0s       med=1µs    max=4.87ms  p(90)=2µs    p(95)=4µs   
     http_req_tls_handshaking.......: avg=0s     min=0s       med=0s     max=0s      p(90)=0s     p(95)=0s    
     http_req_waiting...............: avg=2.34ms min=314µs    med=1.99ms max=41.23ms p(90)=3.63ms p(95)=4.66ms
     http_reqs......................: 632147 21070.605847/s
     iteration_duration.............: avg=2.36ms min=371.62µs med=2.01ms max=50.26ms p(90)=3.67ms p(95)=4.7ms 
     iterations.....................: 632147 21070.605847/s
     vus............................: 50     min=50         max=50  
     vus_max........................: 50     min=50         max=50 
```

WASM JIT binary:

```
     data_received..................: 4.4 MB 147 kB/s
     data_sent......................: 2.9 MB 98 kB/s
     http_req_blocked...............: avg=16.54µs min=0s      med=1µs     max=11.71ms p(90)=2µs     p(95)=2µs    
     http_req_connecting............: avg=1.92µs  min=0s      med=0s      max=1.66ms  p(90)=0s      p(95)=0s     
     http_req_duration..............: avg=43.73ms min=14.83ms med=43.06ms max=94.16ms p(90)=46.03ms p(95)=46.98ms
       { expected_response:true }...: avg=43.73ms min=14.83ms med=43.06ms max=94.16ms p(90)=46.03ms p(95)=46.98ms
     http_req_failed................: 0.00%  ✓ 0          ✗ 34281
     http_req_receiving.............: avg=40.79ms min=8µs     med=40.68ms max=49.31ms p(90)=41.74ms p(95)=42.17ms
     http_req_sending...............: avg=4.22µs  min=1µs     med=3µs     max=434µs   p(90)=8µs     p(95)=10µs   
     http_req_tls_handshaking.......: avg=0s      min=0s      med=0s      max=0s      p(90)=0s      p(95)=0s     
     http_req_waiting...............: avg=2.93ms  min=651µs   med=2.38ms  max=53.84ms p(90)=5.18ms  p(95)=6.11ms 
     http_reqs......................: 34281  1140.94234/s
     iteration_duration.............: avg=43.77ms min=26ms    med=43.08ms max=94.18ms p(90)=46.06ms p(95)=47.01ms
     iterations.....................: 34281  1140.94234/s
     vus............................: 50     min=50       max=50 
     vus_max........................: 50     min=50       max=50 
```

WASM AOT binary:

```
     data_received..................: 4.4 MB 147 kB/s
     data_sent......................: 3.0 MB 98 kB/s
     http_req_blocked...............: avg=4.62µs  min=0s      med=1µs     max=2.83ms  p(90)=3µs     p(95)=3µs    
     http_req_connecting............: avg=2.33µs  min=0s      med=0s      max=2.26ms  p(90)=0s      p(95)=0s     
     http_req_duration..............: avg=43.71ms min=30.65ms med=43.05ms max=66.82ms p(90)=46.25ms p(95)=47.03ms
       { expected_response:true }...: avg=43.71ms min=30.65ms med=43.05ms max=66.82ms p(90)=46.25ms p(95)=47.03ms
     http_req_failed................: 0.00%  ✓ 0           ✗ 34303
     http_req_receiving.............: avg=40.82ms min=8µs     med=40.69ms max=50.22ms p(90)=41.77ms p(95)=42.31ms
     http_req_sending...............: avg=6.77µs  min=1µs     med=5µs     max=528µs   p(90)=11µs    p(95)=15µs   
     http_req_tls_handshaking.......: avg=0s      min=0s      med=0s      max=0s      p(90)=0s      p(95)=0s     
     http_req_waiting...............: avg=2.87ms  min=623µs   med=2.37ms  max=66.8ms  p(90)=5.36ms  p(95)=6.29ms 
     http_reqs......................: 34303  1141.783494/s
     iteration_duration.............: avg=43.75ms min=32.17ms med=43.09ms max=70.45ms p(90)=46.3ms  p(95)=47.06ms
     iterations.....................: 34303  1141.783494/s
     vus............................: 50     min=50        max=50 
     vus_max........................: 50     min=50        max=50 
```

Golang binary:

```
     data_received..................: 76 MB  2.5 MB/s
     data_sent......................: 44 MB  1.5 MB/s
     http_req_blocked...............: avg=750ns  min=0s       med=0s     max=3.2ms   p(90)=1µs    p(95)=1µs   
     http_req_connecting............: avg=149ns  min=0s       med=0s     max=1.93ms  p(90)=0s     p(95)=0s    
     http_req_duration..............: avg=2.93ms min=459µs    med=2.57ms max=66.1ms  p(90)=4.35ms p(95)=5.21ms
       { expected_response:true }...: avg=2.93ms min=459µs    med=2.57ms max=66.1ms  p(90)=4.35ms p(95)=5.21ms
     http_req_failed................: 0.00%  ✓ 0            ✗ 507807
     http_req_receiving.............: avg=8.47µs min=2µs      med=5µs    max=3.36ms  p(90)=15µs   p(95)=23µs  
     http_req_sending...............: avg=2.46µs min=0s       med=2µs    max=3.28ms  p(90)=4µs    p(95)=6µs   
     http_req_tls_handshaking.......: avg=0s     min=0s       med=0s     max=0s      p(90)=0s     p(95)=0s    
     http_req_waiting...............: avg=2.92ms min=455µs    med=2.56ms max=66.09ms p(90)=4.34ms p(95)=5.2ms 
     http_reqs......................: 507807 16925.556111/s
     iteration_duration.............: avg=2.95ms min=466.29µs med=2.58ms max=66.13ms p(90)=4.37ms p(95)=5.22ms
     iterations.....................: 507807 16925.556111/s
     vus............................: 50     min=50         max=50  
     vus_max........................: 50     min=50         max=50  
```

Node binary:

```
     data_received..................: 87 MB  2.9 MB/s
     data_sent......................: 40 MB  1.3 MB/s
     http_req_blocked...............: avg=749ns  min=0s       med=0s     max=3.12ms  p(90)=1µs    p(95)=1µs   
     http_req_connecting............: avg=188ns  min=0s       med=0s     max=2.21ms  p(90)=0s     p(95)=0s    
     http_req_duration..............: avg=3.23ms min=524µs    med=2.62ms max=73.33ms p(90)=5.47ms p(95)=6.85ms
       { expected_response:true }...: avg=3.23ms min=524µs    med=2.62ms max=73.33ms p(90)=5.47ms p(95)=6.85ms
     http_req_failed................: 0.00%  ✓ 0            ✗ 461348
     http_req_receiving.............: avg=8.35µs min=2µs      med=5µs    max=3.26ms  p(90)=14µs   p(95)=22µs  
     http_req_sending...............: avg=2.34µs min=0s       med=2µs    max=3.36ms  p(90)=3µs    p(95)=5µs   
     http_req_tls_handshaking.......: avg=0s     min=0s       med=0s     max=0s      p(90)=0s     p(95)=0s    
     http_req_waiting...............: avg=3.22ms min=503µs    med=2.61ms max=73.31ms p(90)=5.46ms p(95)=6.84ms
     http_reqs......................: 461348 15374.712033/s
     iteration_duration.............: avg=3.24ms min=539.66µs med=2.63ms max=77.08ms p(90)=5.48ms p(95)=6.87ms
     iterations.....................: 461348 15374.712033/s
     vus............................: 50     min=50         max=50  
     vus_max........................: 50     min=50         max=50  
```

| Platform | Avg Duration   | rps              | Image size | RAM usage (at rest) |
| ---      | ---            | ---              | ---        | ---                 |
| WASM     | `43.73ms`      | `1140.94234/s`   | `2.77MB`   | `21MB`              |
| WASM AOT | `43.71ms`      | `1141.783494/s`  | `5.24MB`   | `21MB`              |
| Rust     | `2.35ms`       | `21070.605847/s` | `7.84MB`   | `0MB`               |
| Node     | `3.23ms`       | `15374.712033/s` | `271MB`    | `18MB`              |
| Go       | `2.93ms`       | `16925.556111/s` | `9.6MB`    | `15MB`              |

higher memory usage too, probably the extra load from the wasm runtime?

- WASM image size: 2.76MB
- RUST image size: 7.31MB

WASM image is smaller, so that stands up to the promise at least. Didn't try REALLY hard to make those super small, but they're closer than I expected for a super-simple stateless web server.
