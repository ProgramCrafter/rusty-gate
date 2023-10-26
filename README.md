# RustyGate

Proxy application for opening .ton sites of all kinds

## Features to be done

- [ ] Web2, direct HTTP connection
      - problem: proxy needs to replace Host header
- [ ] Web2, direct HTTPS connection
      - problem: browser will send Host header and server won't recognize it at all
- [ ] TON Storage bag-of-files
      - could be based on [ton-blockchain/.../storage-daemon](https://github.com/ton-blockchain/ton/tree/testnet/storage/storage-daemon) save for the fact that TON code is published under GPL
- [ ] RLDP HTTP connection
      - base: https://github.com/xssnick/Tonutils-Proxy
- [ ] (new) HTTP over WebRTC with signalling info sent over the same socket
- [ ] (new) HTTP over RLDP over WebRTC with signalling info sent over the same socket
- [ ] (new) HTTP over RLDP over WebRTC with signalling info at some other link

## Scheme of HTTP [over RLDP] over WebRTC

![изображение](https://github.com/ProgramCrafter/rusty-gate/assets/82749242/f70fa565-b1ab-477f-b62a-88fd0dde171d)

