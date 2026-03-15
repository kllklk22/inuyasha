# inuyasha

limit order book (lob) matching engine built for high-frequency trading (hft) environments. 

written in rust for absolute zero-cost abstractions and memory safety. uses `BTreeMap` for $O(\log N)$ price level insertions and fast cross-spread matching. 

i drank three energy drinks and wrote this because existing python backtesters were simulating slippage too slowly. this matches limit orders at microsecond latency. 
