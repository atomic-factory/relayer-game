### relayers-only mode
There are 3 rules in relayers-only mode.
1. Anyone can relay a ethereum header on the darwinia chain
2. Confirmed or open a round of game
    - if there is only one block (or same blocks) over the challenge time, this block is confirmed.  (2-1)
    - else (there are different blocks in the same block height) the game is starts
      - everyone in the game should submit the block based on the `sampling function` until closed (2-2-1)
        - Once the a block according sampling function submit the next round of gamae is started, and become recursive in 2
    - anyone can get in the game in any round but need to participate until the game closed
3. Close game
    - In following two condition, that all of the submission in the game from a relayer are ignored. (3-1)
      - if someone in the game did not keep submit the blocks in following rounds in the game
      - if the block of someone in the game is verified fail by hash
    - Because some submission are ignored, if all blocks in each round become the only or the same, this block is confirmed, and game is closed. (3-2)

Here is some visualized example with two relayer *Evil* and *Honest*, *Evil* is a bad guy some times relay incorrect header, *Honest* is always relay a correct header.

This is a plot to show Ethereum, `G` is the genesis block or a block already confirmed on Darwinia chain before the game start.
```
         G==========================================>
```

Here is the first submit, *Evil* relay a block with lie(`L`), and *Honest* find out this and relay a truth block with the same ehereum block height,  
so the game starts.  In the meanwhile, the chain can not determine which block is truth or lied, all the information on chain is that there are two different blocks with same block height, at least one of the block with incorrect information.
```
         G======================================1===>
Evil                                            L
Honest                                          H
```
Based on `sampling function`, the *Evil* and *Honest* should submit the header on the *position 2* (adopted rule 2-2-1.).
```
         G==================2===================1===>
Evil                                            L
Honest                                          H
```
#### From here, the game will become 3 kinds of scenarios, 
  - *Evil* has no response on *position 2*,
  - *Evil* submit a correct block on *position 2* honestly.
  - *Evil* submit a block still with lie on *position 2*.

##### In the first scenario (*Evil* has no response on *position 2*),
the *Honest* will submit a header on *position 2*
```
         G==================2===================1===>
Evil                                            L
Honest                      H                   H
```
And waiting the challenge time over, the lie block from *Evil* will be removed (adopted rule 3-1), 
and the only block in each round will be confirmed (denote with **C**) (adopted rule 3-2).
```
         G==================2===================1===>
Evil                                            -
Honest                      C                   C
```

##### In the second scenario (*Evil* submit a block on *position 2* honestly),
the `Honest` will submit a header on *position 2*
```
         G==================2===================1===>
Evil                        H                   L
Honest                      H                   H
```

And waiting the challenge time over, 
the blocks (the same) in submit round 2 are all confirmed. (adopted rule 2-1)
And *Evil* and `Honest` are still in the game and base on `sampling_function`, 
they should submit headers on *position 3*(adopted rule 2-2-1.).

```
         G==================2=========3=========1===>
Evil                        C                   L
Honest                      C                   H
```

##### In the third scenario (*Evil* submit a block still with lie on *position 2*),
the `Honest` will submit a correct header on *position 2*.
```
         G==================2===================1===>
Evil                        L                   L
Honest                      H                   H
```
And there is nothing confirmed without different opinions, 
so base on the `sampling_function` the *position 3* should be submit by *Evil* and *Honest*.

```
         G=======3==========2===================1===>
Evil                        L                   L
Honest                      H                   H
```
`Evil` and `Honest` can start to submit the block on *position 3* when they have different opinion on *position 2*, 
but the challenge time of submit round 3 will be star counting after run out the challenge time of submit round 2.

#### Pseudo code of relayers-only mode
Here is the [pseudo code](./pseudo/relayers-only/chain.md) of chain, help you to comprehensive this model with multiple relayers in one game.  

The RPC handlers on chain allow anyone to submit headers to challenge blocks still in challenge time, or submit the header according to the sampling function.  
The offchain worker keeps updating the next sampling block.

Here is the [pseudo code](./pseudo/relayers-only/initial-relayer.md) for the client as the initial relayer. 
The client first submits the initial header, and than keeps watching the `next_sampling_block`, and submits header of `next_sampling_block`.

> submit the initial header   
> while `next_sampling_block`  
> &emsp;submit `next_sampling_block`

Here is the [pseudo code](./pseudo/relayers-only/validating-relayer.md) for the client validating submitting block on chain. 
The client first finds out a incorrect initial header, and than keeps watching the `next_sampling_block`, and submits header of `next_sampling_block`.

> while submit headers  
> &emsp;if verify fail   
> &emsp;&emsp;submit correct block  
>
> while next sample block submit changed  
> &emsp;if the block not correct  
> &emsp;&emsp;submit new correct block  

#### Conclusion of relayers-only mode
- In the first scenario, the game is closed.  
- In the second and third scenario, the game is still going and will be convergence some where between `G` and `1`.

Once the *Evil* goes into contradictory. All of the bond from *Evil* will be slashed, and the slash to reward can be distributed with different functions.  
In the model, no mater there are how many malicious relayers, one honest relayer always response correct information will win the game.  
For a honest relayer, the optimistic bond entry barrier is `log2(first submit block - blocks_from_last_comfirm) * bond` and the max game round is `first submit block - blocks_from_last_comfirm`.
However, in the worst case, the bond entry barrier may up to `O(n)`, please refer this [issue](https://github.com/darwinia-network/relayer-game/issues/1).