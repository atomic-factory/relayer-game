### affirmation mode
In the `relayer-take-over` mode, the extend only happened on challengers, but not the initial relayer.
In affirmation mode, each submit from relayer is a affirmation, anyone can against or take-over each other.

Consider the **Case 1** of `relayer-extend` mode.
```
                  G======3a==========2=========3b=========1===>
Initial Relayer                      b                    a      
Challenger 1                                              c     
Challenger 2                         d                    c    
Challenger 3                                              e   
```

If the *Initial Relayer* is not evil, but he run into network issues.
The extned feature only allow for Challenger is not fair for the *Initial Relayer*.
So the same case in affirmation mode it will become following table. 

Note: 
  - following position is block number, the content in brackets is block info help you understand.
  - the content in brackets for affirmation is proposing level
  - level like round before, but in this mode, the affirmation 5 is allowed.  Such that level is more precise for this mode.

```
affirmation(Level) |Chain Status                                 |**Against**|**Extend From**|Disagree       |Agree           |Sample Added    |Allow Samples
----------------|---------------------------------------------|-----------|---------------|---------------|----------------|----------------|-------------
                |G======3a==========2=========3b=========1===>|           |               |               |                |                |             
affirmation 1(1)   |                                        a    |None       |None           |None           |position 1(self)|None            |1            
affirmation 2(1)   |                                        c    |affirmation 1 |None           |position 1(a)  |position G      |Position 2      |1, 2         
affirmation 3(2)   |                   b                    a    |affirmation 2 |affirmation 1(1)  |position 1(c)  |position 2(self)|Position 3b     |1, 2, 3b     
affirmation 4(2)   |                   d                    c    |affirmation 3 |affirmation 2(1)  |position 2(b)  |position G      |Position 3a     |1, 2, 3a, 3b 
affirmation 5(1)   |                                        e    |affirmation 1 |None           |position 1(a,c)|position G      |reuse Position 2|1, 2, 3a, 3b 
```

When every submit become a affirmation, the good guy can extend the honest affirmation and again other lie affirmations.
The sampling function takes 2 parameters(*position 1* and *position G*) and return the *position 2*, which is with some random effect.
When *affirmation 2* submitting on chain, the *position 2* will be calculated.  
Because there is no consensus on *position 1*, he can not say he agree on *position 1*.
There is only one relay block on *position 2*, so he can say agree on *position 2*.
After *position 2* is determined, *affirmation 5* still get the same position for *position 2*.
When *affirmation 3* submitting on chain the *position 3a* will be calculated.
Also, when *affirmation 4* submitting on chain the *position 3a* and *position 3b* will be calculate


Here in, a guy disagree *affirmation 4* may submit *affirmation 6*, and another guy disagree *affirmation 6* as following
```
affirmation(Level) |Chain Status                                  |**Against**|**Extend From**|Disagree      |Agree            |Sample Added|Allow Samples       
----------------|----------------------------------------------|-----------|---------------|--------------|-----------------|------------|--------------------
                |G======3a====4b====2=========3b==========1===>|           |               |              |                 |            |                    
affirmation 6(3)   |       f           b                     a    |affirmation 4 |affirmation 3(2)  |position 2(d) |position 3a(self)|Position 4b |1, 2, 3a ,3b, 4b    
```
```
affirmation(Level) |Chain Status                                  |**Against**|**Extend From**|Disagree      |Agree            |Sample Added|Allow Samples       
----------------|----------------------------------------------|-----------|---------------|--------------|-----------------|------------|--------------------
                |G==4a==3a====4b====2=========3b==========1===>|           |               |              |                 |            |                    
affirmation 6(3)   |       f           b                     a    |affirmation 4 |affirmation 3(2)  |position 2(d) |position 3a(self)|Position 4b |1, 2, 3a ,3b, 4b    
affirmation 7(3)   |       g           b                     a    |affirmation 6 |affirmation 3(2)  |position 3a(f)|position G       |Position 4a |1, 2, 3a ,3b, 4a, 4b
```


On the other hand, a guy disagree *affirmation 3* may submit affirmation 6, and another guy disagree *affirmation 6*  as following
```
affirmation(Level)|Chain Status                                  |**Against**|**Extend From**|Disagree       |Agree            |Sample Added|Allow Samples       
---------------|----------------------------------------------|-----------|---------------|---------------|-----------------|------------|--------------------
               |G======3a==========2=========3b====4d====1===>|           |               |               |                 |            |                    
affirmation 6(3)  |                   d         f           c    |affirmation 3 |affirmation 4(2)  |position 1(a,e)|position 3b(self)|Position 4d |1, 2, 3a ,3b, 4d    
```
```
affirmation(Level)|Chain Status                                  |**Against**|**Extend From**|Disagree       |Agree            |Sample Added|Allow Samples       
---------------|----------------------------------------------|-----------|---------------|---------------|-----------------|------------|--------------------
               |G======3a==========2====4c===3b====4d====1===>|           |               |               |                 |            |                    
affirmation 6(3)  |                   d         f           c    |affirmation 3 |affirmation 4(2)  |position 1(a,e)|position 3b(self)|Position 4d |1, 2, 3a ,3b, 4d    
affirmation 7(3)  |                   d         g           c    |affirmation 6 |affirmation 4(2)  |position 3b(f) |position 2(d)    |Position 4c |1, 2, 3a ,3b, 4c, 4d
```


If the blocks of affirmations in the **Allow Samples**, the affirmations are in the same game, and one affirmation submitting only add one or zero sample.

#### Incentive model for affirmation mode
In the affirmation mode of relayer game, you can find out there is always **against** affirmation for each affirmation excluding the initial affirmation.
Once the largest level affirmation without different opinion and over the challenge time, the affirmation chain base on **extend from** will be confirmed.
These comfirmed affirmations have different **against** affirmations, so the incentive model is really easy to be calculated based on it's **against** affirmation.
The only one affirmation may without against propoal is the initial affirmation, the already paid by the requesting demand from the user using the token bridge.
If you are interesting about the initial affirmation, please refer the [backing pallet](https://github.com/darwinia-network/darwinia-common/tree/master/frame/bridge/eth/backing).
There maybe some incorrect affirmations without other affirmation to against on it, the bond value of these affirmation will be slash and give to treasury.


#### Pseudo code of affirmation mode
The [substrate template](https://github.com/yanganto/substrate-node-template/tree/relayer-game-proposal) shows the basic concept of model.
Please note that, the term "take over"(legacy used) in the sample code is just the same meaning for "extend".

Here is the basic material for proposing for a initial relayer
- provide a **block**, not exist on chain and not in list of allow samples

Here is the basic material to propose for a relayer (not initial) find out a evil affirmation
- provide a **block** in allow samples, **against** affirmation,
- if the level of affirmation greater than 1, the affirmation (level n) should **extend from** the affirmation with level (n-1)

Here is the pseudo code on rpc handler of chain to find out the disagree position and the agree position
- find out the agree position and the disagree position
> if self position first on chain  
> &emsp;agree self.position  
> &emsp;disagree smallest_and_greater_than_self(recursive on positions of against affirmation and its extend from affirmations)  
> else  
> &emsp;agree biggest_and_smaller_than_self(recursive on positions of extend from affirmation and its extend from affirmation and G)  
> &emsp;disagree against_affirmation.position  
- add a challenge_time for the affirmation


Here is the pseudo code for the offchain worker on chain  
If current block is greater the challenge_time of the largest_level_affirmation
> 
> if largest_level_affirmation conflicts with the block confirmed on chain  
> &emsp;slash the affirmation into treasury  
> 
> let correct_affirmation = largest_level_affirmation 
> 
> while correct_affirmation:  
> &emsp;confirm correct_affirmation.position  
> &emsp;slash correct_affirmation.against as reward  
> &emsp;del correct_affirmation.against  
> &emsp;next_affirmation = correct_affirmation.take_over  
> &emsp;del correct_affirmation  

#### Conclusion of affirmation mode
Base on optimistic condition, there is always a good relayer submitting a correct affirmation on each round.
So the affirmation mode, provide a `many-to-many` game, it provided a system let honest guys extend from each other.
Besides, one confirm block can slash more than one evil affirmations provide a better game for honest relayer.
In this model, the working affair and bond entry barrier share to all the relayers. 
Under optimistic condition, the honest relayers are the majority, so the working affair and bond entry barrier is relatively smaller than the evil relayers.