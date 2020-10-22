### affirmation-Only mode
In the `affirmation` mode, the affirmation including the against infomation, and the extend from information.
Besides, *affirmation 5* of `affirmation` mode is allowed to submit after *affirmation 3* of `affirmation` mode.

In the `affirmation-only` mode, there are only serial blocks in submission, and the relayer game becomes in rounds as the same as `relayers-only` mode. 
Such that the *affirmation 5* of `affirmation` mode (the *affirmation 3* in `affirmation-only` mode) can not submit after affirmation 3 of `affirmation` mode (the *affirmation 5* in `affirmation-only` mode). 
Besides, the simulation of fee and challegne time of `affirmation-only` mode are also can use refit with two relayer in `relayers-only` mode.

```
affirmation(round) |Chain Status                                 |Samples
----------------|---------------------------------------------|-------------
                |G==4a==3a====4b====2====4c===3b====4d===1===>|         
affirmation 1(1)   |                                        a    |1            
affirmation 2(1)   |                                        c    |1         
affirmation 3(1)   |                                        e    |1
                |                                             |1, 2 (challenge time over next round start)
affirmation 4(2)   |                   b                    a    |1, 2
affirmation 5(2)   |                   d                    c    |1, 2
affirmation 6(2)   |                   f                    e    |1, 2
                |                                             |1, 2, 3a, 3b (challenge time over next round start)
affirmation 7(3)   |       f           b         g          a    |1, 2, 3a ,3b,
affirmation 8(3)   |       h           b         i          a    |1, 2, 3a ,3b
                |                                             |1, 2, 3a, 3b, 4a, 4b, 4c, 4d (challenge time over next round start)
```

The samples will be add when each round starts, and the number of samples exponentially increase with game round.
For an honest relayer, he just relayer more correct blocks he observed, but the affair of creating an incorrect block for an evil relayer becomes an overwhelming burden.
Base on the assumption, there alway a honest guy submit correct block in each round, so there will be at least *affirmation 9* in round 4.


#### Pseudo code of affirmation-only mode
The [substrate template](https://github.com/yanganto/substrate-node-template/tree/relayer-game-affirmation-only) shows the basic concept of model.

Here is the basic material for proposing for a initial relayer
- provide a **block**, not exist on chain and not in list of samples

Here is the basic material to propose for a relayer (not initial) find out a evil affirmation
- provide a serial **blocks** in samples
- if the blocks in samples greater than 1, the serial **blocks** should cover one of the submission in before round

Here is the pseudo code on rpc handler of chain 
> if submission round greater than 1  
> &emsp;check the submisstion follows the samples  
> &emsp;check the submisstion cover one of submission in before round  
> else if sample not set  
> &emsp;set the sample of first round
>
> validate blocks  
>
> if affirmation is the first submission of each round  
> &emsp;update the challenge time of the round

Here is the pseudo code for the offchain worker on chain  
> if the last round is over challenge time  
> &emsp;if only one submission in the round  
> &emsp;&emsp;close game  
> &emsp;else  
> &emsp;&emsp;add new samples for the next round  

Here is the pseudo code for the client, this is a POC level client, watching the event and submitting headers.  
In production, the offchain worker push samples chaning event will be more efficence.

> loop  
> &emsp;watch and get info from `SubmitHeaders`  
> &emsp;if first block heigh unseen  
> &emsp;&emsp;add into the current games  
> &emsp;for game in current games  
> &emsp;&emsp;if the samples of game changed and not none  
> &emsp;&emsp;&emsp;submit headers based on samples  
> &emsp;&emsp;else if the samples of game is none  
> &emsp;&emsp;&emsp;remove this game from current game  

##### Close Game
Confirm correct blocks, and there will be a correct relayer in each round. 
The only correct relayer in each round is winner, others will be slashed.
The reward method is simple as winner takes all model.

#### Conclusion of affirmation-only mode
In this model, the number of samples exponentially increase with game round, and the confirm time is easiliy to be calculated by rounds.
The reward method for this model is winner takes all, so it is easy and clear for relayer who participate in.