### relayer-challenger mode
In relayer-challenger mode, when someone is not accepted the block submitted by other relayers, he just puts a challenge on chain to express his opinion.
And currently, we do not consider the scenario that the challenger do evil.
However, there is still a bond for the challenger to challenge.
1. Any relayer can relay a ethereum header on the darwinia chain
2. Any challenger can challenge the relayer and open the game
    - challenger needs to bond some value for each challenge before the half challenge time over
    - relayer needs to submit the next specified block by `sampling function`
3. Close game
    1. The challenger stops challenging, the relayer wins and challenger will be slashed
    2. The relayer can not provided the next sampling block block pass the validation before the next half challenge time over

Here is some visualized example with *Evil* relayer and *Challenger* challenger, *Evil* is a bad guy some times relay incorrect header, *Challenger* is honest challenger challenge with the correct opinion.

This is a plot to show Ethereum, `G` is the genesis block or a block already confirmed on Darwinia chain before the game start.
```
             G==========================================>
```

Here is the first submission, *Evil* relays a block with lie(`L`), and *Challenger* finds out this block is not correct and challenge it with `0`, so the game starts.  
In the meanwhile, the chain still can not determine which block is truth or lied, all the information on chain is that there is a block with dispute.
```
             G======================================1===>
Evil                                                L
Challenger                                          0
```

Based on `sampling function`, the *Evil* should submit the block on position 2.
```
             G=================2====================1===>
Evil                                                L
Challenger                                          0
```
#### From here, the game will become 3 kinds of scenarios, 
  - *Evil* has no response on *position 2*,
  - *Evil* submit a block on *position 2* honestly.
  - *Evil* submit a block still with lie on *position 2*.

##### In the first scenario (*Evil* has no response on *position 2*),
If *Evil* is not response before the half challenge time over, 
the *Challenger* will win the game and the bond of *Evil* in position 1 will become the be slashed and become reward for *Challenger*.

##### In the second scenario (`Evil` submit a block on *position 2* honestly),
If `Evil` submit a correct block in *position 2*, the challenger will challenge with `0` on *position 1* and '1' on *position 2*.
```
             G=================2====================1===>
Evil                           H                    L
Challenger                     1                    0
```
Such that, based on `sampling function`, the next sampling block will be between the *position 1* and *position 2*.
```
             G=================2==========3=========1===>
Evil                           H                    L
Challenger                     1                    0
```
##### In the third scenario (*Evil* submit a block still with lie on *position 2*),
If *Evil* submit a correct block in *position 2*, the challenger will challenge with `0` on position 1 and '0' on *position 2*.
```
             G=================2====================1===>
Evil                           L                    L
Challenger                     0                    0
```
Such that, based on `sampling function`, the next sampling block will be between the genesis and *position 2*.
```
             G=======3=========2====================1===>
Evil                           L                    L
Challenger                     0                    0
```

#### Pseudo code of relayer-challenger mode
Here is the [pseudo code](./pseudo/relayer-challenger/chain.md), help you to comprehensive this model with one relayer and one challenger.
Once challenger determine a block pending on chain is correct or not, he will not change his idea.  

The rpc on chain allow relayer to submit headers, and any one to challenge blocks still in challenge time.  
The offchain worker keep updating the next sampling tartget.

Here is the [pseudo code](./pseudo/relayer-challenger/relayer.md) for the relayer, this code is the same with the initial relayer in `relayers-only` model
The client first submits the initial header, and than keeps watching the `next_sampling_block`, and submits header of `next_sampling_block`.

> submit the initial header   
> while `next_sampling_block`  
> &emsp;submit `next_sampling_block`  

Here is the [pseudo code](./pseudo/relayer-challenger/challenger.md) for challenger
> The client first finds out a incorrect initial header and submits a challenge info , and than keeps watching the `next_sampling_block`, and keeps submitting the challenge info base on the relayer's new submission.
>
> while submit headers  
> &emsp;if verify fail  
> &emsp;&emsp;challenge  
> while next sample block submit changed  
> &emsp;verify the block  
> &emsp;submit new challenge  

#### Conclusion of relayer-challenger mode
- In the first scenario, the game is closed.  
- In the second and third scenario, the game is still going and will be convergence some where between `G` and `1`.
  - Following are the assumption, that challenger will beat the evil relayer
    - The fake blocks are not easy to pass validation blocks when near by
    - If challenger is not collusion with the evil relayer.

Once the *Evil* goes into contradictory. All of the bond from `Evil` will be slashed, and the game is closed. 
Please note there is no correct block on position 1 after the game closed, so there may be multiple parallel relayer-challenger games on chain to keep the bridge works. 
For a honest challenger or relayer, the bond entry barrier is `log2(first submit block - blocks_from_last_comfirm) * bond` and the max game round is `log2(first submit block - blocks_from_last_comfirm)`. 
In this model, there is an assumption that the challenger will be honest to keep the bridge secure, so it is required some legal enforcement or high value staking for challenger, 
such that it is not truly decentralized for this model. 
In the worst case, the bond entry barrier may up to `O(n)`, please refer this [issue](https://github.com/darwinia-network/relayer-game/issues/1). 