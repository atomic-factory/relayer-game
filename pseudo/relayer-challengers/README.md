### relayer-challengers mode
In relayer-challengers mode, when someone is not accepted the block submitted by other relayer, he just put a challenge on chain to express his opinion.
With multiple challengers, the challengers can take over the challenge jobs, and relay is obliged to all the challeenge from relayers.

1. Any relayer can relay a ethereum header on the darwinia chain
2. Any challenger can challenge the relayer with the challenge info non-exsist
    - challenger needs to bond some value for each challenge before the half challenge time over
    - relayer needs to submit the sampling headers based on the challenge and the `sampling function`
3. Close game
    1. The challengers stop challenging, the relayer wins and all challengers will be slashed
    2. The relayer can not provided the next sampling block block pass the validation before the next half challenge time over

Here is some visualized example with relayer *Evil* and challengers *Challenger 1*, *Challenger 2*,  *Challenger 3*, *Evil* is a bad guy some times relay incorrect header, challengers may not be a honest.

This is a plot to show Ethereum, `G` is the genesis block or a block already confirmed on Darwinia chain before the game start.
```
             G==========================================>
```

Here is the first submit, *Evil* submit a block `B`, and *Challenger 1* find out this block is not correct and challenge it with `0`, so the game starts.  
In the meanwhile, the chain still can not determine which block is truth or lied, all the information on chain is that there is a block with dispute.
```
             G======================================1===>
Evil                                                B
Challenger 1                                        0
```
Here in *Challenger 1* submit a challenge `0`, that the length of challenge is 1, and the game is opened.

Based on `sampling function`, the *Evil* should submit the block on *position 2*.
```
             G=================2====================1===>
Evil                                                B
Challenger 1                                        0
```

#### From here, the game will become 2 kinds of scenarios, 
  - *Evil* has no response on position 2,
  - *Evil* submit a block on position 2.

##### In the first scenario (*Evil* has no response on position 2),
If *Evil* dose not submit, *Evil* will be slashed, than go to reward stage and reward the correct challenger.  

##### In the second scenario (*Evil* submit block on position 2),
When *Evil* submit a header on position 2, there are also two kind of scenarios.
  - challenger confirm with the submission on *position 2*
  - challenger deny with the submission on *position 2*

The `relayer-challengers` allows multiple challengers with different opinions. 
For example, *Challenger 2* and *Challenger 3* have different opinions on the block at *position 2*.

*Challenger 2* does not confirm the block on *position 2* and submit a new challenge `00`, that the length of challenge is two.
And based on *Challenger 2*'s challenge, the relayer should submit the block on *position 3a*.
```
              G======3a==========2====================1===>
Evil                             B                    B
Challenger 1                                          0
Challenger 2                     0                    0
```


*Challenger 3* confirms the block on *position 2* and submits a new challenge `01`, that the length of challenge is two.
And based on *Challenger 3*'s challenge, the relayer should submit the block on *position 3b*.
```
              G======3a==========2=========3b=========1===>
Evil                             B                    B
Challenger 1                                          0
Challenger 2                     0                    0
Challenger 3                     1                    0
```

If *Evil* submit a block on *position 2*, there can be challenge with `00` and `01`, and the maximum next sampling blocks can be two,
so the max challenge and the maximum next sampling blocks become a tree and follows the following equation.

- `challenge for n round = 2 ^ (submit_round - 1)`
- `total challenge = 2 ^ submit_round - 1`
- `samples for n round = 2 ^ (submit_round - 2)`
- `total samples for n round = 2 ^ (submit_round - 1)`

When there is no new challenge and all the blocks are over the challenge waiting time, all the bond from challengers will be slashed.
If the relayer submit a block can not be validate or contradictory with other submissions, 
the relayer will be slashed, and than go to reward stage and reward the correct challenger. 


#### Reward Stage
The game will be closed when reaching following conditions
- relayer wins
  - There is no new challenger and all challenging time are over
- relayer fail
  - The block can not be verified 
  - The relayer has not response over the challenge waiting time

##### In the first scenario (relayer wins),
All of the challengers will be slash, and reward the relayer.

##### In the second scenario (relayer fail),
The relayer will be slash, all the bond of challenger will be returned, 
and the leaf of challenge make the relayer fail should be rewarded,  also the roots and parents derived from the wining leaf should be rewarded.
These reward are based on the slash value of relayer on each submit round.
However there may be still some challengers we can not check their behavior, so the value they bond will be returned without rewards.

For example, the block height of *position 1* is `G+4`, and the block on *position 3b* can not be verified with the block in *position 1*.
Following chart show the challenge from three challengers.

```
              G  3a 2  3b 1==>
Evil                B  B  B
Challenger 1              0
Challenger 2        1     0
Challenger 3        0     0
```

Following are the actions for the relayer and challengers
- *Evil* is slashed
- All bond of challengers are returned
- *Challenger 1* is rewarded from the relayer's bond of *position 1*
- *Challenger 2* is rewarded from the relayer's bond of *position 2*
- There is no reward for *Challenger 3*.

Another example, the block height of *position 1* is `G+4`, and all the blocks are valid.
Following chart show the challenge from three challengers.


```
              G  3a 2  3b 1==>
Honest           B  B  B  B
Challenger 1              0
Challenger 2        1     0
Challenger 3        0     0
```

Following are the actions for the relayer and challengers
- All bond from *Honest* relayer is returned
- All challengers are be slashed as reward to relayer

The slash and reward can be related as following:
- The value slashed from *Challenger 1* is as reward for the submission of block in *position 2*
- The value slashed from *Challenger 2* is as reward for the submission of block in *position 3b*
- The value slashed from *Challenger 3* is as reward for the submission of block in *position 3a*

#### Pseudo code of relayer-challengers mode
Here is the [pseudo code](./pseudo/relayer-challengers/chain.md), help you to comprehensive this model with one relayer and one challenger,
Once challenger determine a block pending on chain is correct or not, he will not change his idea.

The rpc on chain allow relayer to submit headers, and any one to challenge blocks still in challenge time.  
The offchain worker keep updating the next sampling blocks.

Here is the [pseudo code](./pseudo/relayer-challengers/relayer.md) for the relayer, this code is the same with the initial relayer in `relayers-only` model
> The client first submit the initial header, and than keep watch the list of `next_sampling_blocks`, and submit each header listed in `next_sampling_blocks`.
>
> submit the initial header  
> while `next_sampling_blocks`  
> &emsp;submit each header listed in `next_sampling_blocks`  

Here is the [pseudo code](./pseudo/relayer-challengers/challenger.md) for challenger
> The client first findout an uncorrect initial header and submit a challenge info , and than keep watch the `next_sampling_blocks`, and keep submit the challenge info base on the relayer's new submit.
>
> while submit headers  
> &emsp;if verify fail  
> &emsp;&emsp;challenge  
> while submit headers  
> &emsp;if next sample block submit  
> &emsp;&emsp;verify the block  
> &emsp;&emsp;submit new challenge  


#### Conclusion of relayer-challengers mode
In this model, we are not determine each block in different round is correct or not. 
We just make sure we have a solution which can always to challenge a evil relayer and let him to provide more information on chain. 
Once the relayer contradictory itself the relay is slashed and the game is close. 
On the other hand, the honest relayer can get the corresponding rewards for each block from the corresponding slash of challenge. 
For a honest relayer, the bond entry barrier is `blocks_from_last_comfirm * bond` and the max game round is `first submit block - blocks_from_last_comfirm`. 
For a honest challenger, the bond entry barrier is `log2(first submit block - blocks_from_last_comfirm) * bond` and the max game round is `log2(first submit block - blocks_from_last_comfirm)`. 
In the worst case, the bond entry barrier may up to `O(n)`, please refer this [issue](https://github.com/darwinia-network/relayer-game/issues/1). 
The challenging time of block may be extended with `graceful period` for relayer only. 
The `graceful period` will be calculate by `graceful_function` when implementing.