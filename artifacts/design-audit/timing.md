| Run | Step | Seconds | Notes |
|---|---|---:|---|
| audit-0 | cold test build (deps rebuilt) | 137 | copied main target did not reuse deps |
| audit-0 | visual suite, 42 scenarios, 870 captures | 80 | |
| audit-0 | whole visual-test incl. video and contact sheet | 254 | |
| test-0 | warm build (lib and bin, 9.7 s) plus full cargo test: lib 187 tests 7.7 s, bin 475 tests incl. visual suite 42.2 s | 61 | no video |
