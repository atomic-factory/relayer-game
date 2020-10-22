# Relayers' Optimistic Verification Game

## Overview

Relayers' Optimistic Verification Game (ROVG) is a module of Darwinia ChainRelay, which is a super-light client with a sub-linear foreign blockchain header stored.  Each block header contains a particular pre-calculated field `mmr_root` of previous block header hashes using the Merkle Mountain Range algorithm.  Thus, each block header is "history aware," making it possible to verify whether a lower height block belongs to the chain by checking the header hash and Merkle Proof to the `mmr_root`.  Darwinia ChainRelay also includes difficulty transitions for PoW consensus, and validators set changes for PoS consensus into `mmr_root`to detect malicious fork.  With this cryptographic enforcement, it's sturdy and economically infeasible for one or a group of adversaries to forge a block header on a fork and be accepted by Darwinia ChainRelay without detection.  Please check out Darwinia ChainRelay for more detail.

RROVG is here to provide a mechanism for faster resolution and confirmation of recent block header.  It will reward honest relayer while punishing malicious relayer.  False block header submission will be eventually detected via rounds of challenges if necessary.  The result is deterministic that the adversary will fail and be slashed; therefore, the process is optimistic.

## Prerequisites

There're some essential prerequisites and assumptions:

- At least one honest relayer can monitor the latest state on both chains, detect malicious data submission, and react by submitting objectively observed data within a reasonable timeframe.
- There's enough incentive for an honest relayer to perform.
- It is permissionless for anyone to act as a relayer and participate in the verification game; there's no entry barrier except some bonding capital.

## Theory

Darwinia ChainRelay is a sub-linear light client, which means it does not store every block header of the blockchain it monitors.  When initialized, it contains only one block, which is the genesis block.  When a relayer submits a new block header, it might be the block header of height 10,000 or even higher.  There are huge blanks in-between.  If another relayer does not agree and submits a different block header claiming that's the block header data at the height of 10,000.  How does ChainRelay resolve this conflict, and who is going to be the judge?

ROVG is a commit-reveal process.  Once a block header is submitted, it provides block header hash and its `mmr_root` of all previous block header hashes till genesis block.  Therefore, if a block header submission is in question, ChainRelay will challenge the relayer for a prior block header data or more specified by a `sampling function`.  That block header hashes must be a leaf or leaves of previously submitted `mmr_root`.  In this way, if an adversary tries to fool ChainRelay, he must prepare the whole chain, while each block must comply with consensus rule.  The attack difficulty equals attacking the original blockchain network.

To reduce spam and faster resolution, each data submission shall include some pledge specified by `bond_function`, which takes challenging rounds as a parameter and may be variable.  If data is honest and taken as confirmed, the pledge is returned in full, and the relayer is entitled to additional rewards coming from verification fees.  If failed in the challenge, the pledge is confiscated and goes to those who challenge and win.  This incentive model greatly discourages malicious relayers and encourages honest relayers to actively participate and guard the ChainRelay.

In the chapters below, we discuss the different game modes and how we choose the one we believe to be most efficient.

## Tools

There are several tools in this project, and also a lot of thought of relayer verification game as different mode.  
It is very helpful to know there are more possibilities to do relayer games through this document.

- `refit` is a **re**layer **f**ee **i**nference **t**ool to simulate and optimized the game for relayers in Darwinia Network.  

  Parameters are organized in scenario config files.  You can fine-tune three important equations and load these scenario config files to simulate the verification game.  You can evaluate the efficiency of the parameter setting.  If you are only interesting in this part, please go to [Refit section](#refit---a-relayer-fee-inference-tool)

- The `chain`, `relayer`, `challenger` in `/scenario/<model>` folder can read the scenario file and simulate with more detail.


## Sampling Function
Sampling function is an equation to provide the block height numbers, that relayer should submit the blocks at that block height.
Sampling function is the key part to prevent the attacker, and also determine the total consuming time in relayer game.
And it is reasonable for using different sample equation for different target chain with different consensus algorithm.
Following listed are the design philosophy.  

- Transparent and with ambiguous part
  - The sample equation should be clear and transparent for people, and there will be also some ambiguous part provided by random number, such that the attacker need much affair to making fake headers.  
  - Once the sample calculated, it will reuse at all.  
    - Take affirmation mode for example, if same agree position and same disagree position will get the same sample block number as output
- Sampling the tail at first
  - By nature, the **PoW** consensus mechanism, the branch will occur and not greater than a reasonable length, for example 6.  To accelerate the process of relayer verification game, the sampling function will label the *position N-6* to *position N-1* blocks at the second round, such that the nature branch point can be find out as soon as possible.
- Confirm blocks affinity
  - If the sampling block is in the `ConfrimBlockAttractRange` range of confirmed blocks, the sampling blocks will change to the block near by the confirmed blocks
  - Such that it is easy to find out the counterfeit block which is near by a confirmed block

### Substrate example of sampling function
Here is a [sample pallet](https://github.com/yanganto/substrate-node-template/blob/relayer-game-affirmation/pallets/sample/src/lib.rs) shows to sample a block 
with the features aforementioned.
And the pseudo code for relay pallet, sample pallet listed here help to know more about what is thing going on chain.

**relay pallet**

The RPC handlers on chain allow anyone to submit headers to challenge blocks still in challenge time, or submit the header according to the sampling function.  
The offchain worker keeps updating the next sampling block.

> fn `submit`  
> &emsp;find out the agree position and the disagree position  
> &emsp;call `gen_sampling_blocks` of sample pallet   
>
> fn `offchain_worker`  
> &emsp;find out the blocks over challenge time  
> &emsp;store the block as confirmed and call `confirm` of  sample pallet   


**sample pallet**
> fn `confirm`  
> &emsp;store the block height of confirmed blocks  
>
> fn `gen_sampling_blocks`  
> &emsp;generat the sampling block base on disagree block, agree block,  
> &emsp;and also consider the concensuse of target chain, confirmed blocks  

## Stage Two
In stage two of the relayer verification game, the nature branch will be solved.
When a relayer with dispute on chain but all blocks is correct, the challenger or second relayer can ask to open the stage two of the game.
Here is status when the stage two opening in `relayers-only` mode.

```
                G==============================nnnn1=====>
Initial Relayer                                Caaaa
Relayer 2                                      Cbbbb
```
**C**: is the latest confirm block

Only the longest validated chain will be accepted in block chain network.
*Relayer 2* start to provide the chain as long as possible after the point of first submission to open the stage two.

```
                G==============================nnnn1=====>
Initial Relayer                                Caaaa
Relayer 2                                      Cbbbbbbbb
```

Then, *Initial Relayer* should provide a longer chain to prove that he is the longest validated chain in the challenge time as following.
```
                G==============================nnnn1=====>
Initial Relayer                                Caaaaaaaaa
Relayer 2                                      Cbbbbbbbb
```

If *Relayer 2* can still challenge by providing more headers, and so on.

The stage two of game should be rare, because all relayers should submit a block already finalized. 
However, Stage Two is designed to solve the branch issue just in case. 

## Reference

| Item               | Material                                           |
|--------------------|----------------------------------------------------|
| EMeetup 2020/06/23 | [Slides](https://slides.com/yanganto/relayer-game) |

