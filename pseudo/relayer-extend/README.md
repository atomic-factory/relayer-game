### relayers-extend mode
The `relayer-extend` mode is similar to the `relayer-challengers` mode, and the challenger need to provide headers to express the different opinions.
In this mode the challengers should submit header to prevent the evil challengers to mal-response easy and DoS the system.
However, there is still no the rule `Once in participate all` for `Once lie drop all`, so there is some rare case without confirm block at all.
The simulation of fee and challenge times of this mode are similar to `relayer-challengers` mode.

Here in, the plots are converted from the second scenario (*Evil* submit block on *position 2*) in `relayer-challengers` mode, 
that relayers submit the blocks `a` to `e`, and the *Evil* decides to quit the game without response on *position 3a* and *position 3b*.

```
              G======3a==========2=========3b=========1===>
Evil                             b                    a      Slash
Challenger 1                                          c      Return
Challenger 2                     d                    c      Return  (extend from Challenger 1)
Challenger 3                     b                    e      Return  
```
The game is closed and `c` is **not** confirmed, because of `e`.

The results are 3 status, following 2 cases help you to know more about this.

**Case 1**
```
              G======3a==========2=========3b=========1===>
Evil                             b                    a      
Challenger 1                                          c     
Challenger 2                     d                    c    
Challenger 3                                          e   
```
Only *Challenger 2* beat *Evil*, so we can deem the result from *Challenger 3* is correct.
So *Challenger 1* and *Challenger 2* got the reward, and the `c` is confirmed as following plot.
```
              G======3a==========2=========3b=========1===>
Evil                             -                    -      Slash
Challenger 1                                          C      Reward
Challenger 2                                          -      Slash
Challenger 3                     C                    C      Reward
```

**Case 2**
```
              G======3a==========2=========3b=========1===>
Evil                             b                    a      Slash
Challenger 1                                          c      Return
Challenger 2                     d                    c      Return   (extend from Challenger 1)
Challenger 3                                          e      Return
Challenger 4                     b                    e      Return   (extend from Challenger 3)
```
*Challenger 2* and *Challenger 4* beat *Evil*.
Without `Once in participate all` and `Once lie drop all`, the possible blocks in *position 1* are `C`, `E`.
`A` is eliminated, because the initial relayer having responsibility to keep relaying the sampling blocks.
There is no rule to eliminate blocks `C` or `E`, so there is no confirm block.

And let us using Honest(`H`) and Lie(`L`) symbols to show the four possible for **Case 2**.

**Case 2-1**
```
              G======3a==========2=========3b=========1===>
Evil                             H                    L      Slash
Challenger 1                                          H      Return
Challenger 2                     L                    H      Return   (extend from Challenger 1)
Challenger 3                                          L      Return
Challenger 4                     H                    L      Return   (extend from Challenger 3)
```
**Case 2-2**
```
              G======3a==========2=========3b=========1===>
Evil                             H                    L      Slash
Challenger 1                                          L      Return
Challenger 2                     L                    L      Return   (extend from Challenger 1)
Challenger 3                                          H      Return
Challenger 4                     H                    H      Return   (extend from Challenger 3)
```
**Case 2-3**
```
              G======3a==========2=========3b=========1===>
Evil                             L                    L      Slash
Challenger 1                                          L      Return
Challenger 2                     H                    L      Return   (extend from Challenger 1)
Challenger 3                                          H      Return
Challenger 4                     L                    H      Return   (extend from Challenger 3)
```
**Case 2-4**
```
              G======3a==========2=========3b=========1===>
Evil                             L                    L      Slash
Challenger 1                                          H      Return
Challenger 2                     H                    H      Return   (extend from Challenger 1)
Challenger 3                                          L      Return
Challenger 4                     L                    L      Return   (extend from Challenger 3)
```

Therefor, if the game rarely stop as the status show in the **Case 2**, we just slash Evil and return the bond for challengers as following plot.
```
              G======3a==========2=========3b=========1===>
Evil                             -                    -      Slash
Challenger 1                                          -      Return
Challenger 2                     -                    -      Return   (extend from Challenger 1)
Challenger 3                                          -      Return
Challenger 4                     -                    -      Return   (extend from Challenger 3)
```

#### Conclusion of relayer-extend mode
**Case 2** is the worst case for this mode, nothing is confirmed, and the good news is there is not bad block relay on chain.
However, in optimistic game, there is always a good guy in each round.  
Such that the good guy will return in *position 3a* or *position 3b* to beat *Challenger 2* or *Challenger 4*.
In this model, the working affair for challenging is sharing to more than one challengers.  
But the affair for the relayer is the same as aforementioned models.