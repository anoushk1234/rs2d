# rs2d
A crate for encoding and decoding data in a 2D Reed-Solomon matrix.

Inspired by the [Mustafa Al-Bassam's Paper](https://arxiv.org/pdf/1809.09044.pdf).


## Algorithm

```
sd = digest
cr = columnRoot
r = rowRoot
shard = [0u8;1280]
block = [shard;23]
s = data shard
p = horizontal parity
s' = veritical data parity
p' = vertical erasure parity
```
### Data Matrix
```
s1  s2  s3  s4  s5 -> d1
s6  s7  s8  s9  s10 -> d2
s11 s12 s13 s14 s15 -> d3
s16 s17 s18 s19 s20 -> d4
s21 s22 s23 sd  sd -> d5
```
### Erasure Code Matrix
```
s1  s2   s3   s4   s5  -> p1  p2  p3  p4  p5 ->  r1
s6  s7   s8   s9   s10 -> p6  p7  p8  p9  p10 -> r2
s11 s12  s13  s14  s15 -> p11 p12 p13 p14 p15 -> r3
s16 s17  s18  s19  s20 -> p16 p17 p18 p19 p20 -> r4
s21 s22  s23  sd   sd  -> p21 p22 p23 p24 p25 -> r5
 |   |    |    |    |
 v   v    v    v    v  
s1'  s2'  s3'  s4'  s5'  -> p1'  p2'  p3'  p4'  p5' ->  r6
s6'  s7'  s8'  s9'  s10' -> p6'  p7'  p8'  p9'  p10' -> r7
s11' s12' s13' s14' s15' -> p11' p12' p13' p14' p15' -> r8
s16' s17' s18' s19' s20' -> p16' p17' p18' p19' p20' -> r9
s21' s22' s23' sd'  sd'  -> p21' p22' p23' p24' p25' -> r10
|    |    |    |    |       |    |    |    |    |
cr1  cr2  cr3  cr4  cr5     cr6  cr7  cr8  cr9  cr10
```

## Current Limitations
- Only works when n and k are the same.
- No performance optimizations.
- Data Shard has to be a byte array.

# TODO
- [x] Encode function
- [ ] Decode/Verify function
- [ ] Test Cases
- [x] Benchmarks