## Scenario with Different Mode
In this tool we assume the target chain is Ethereum, however you can simulate different chain by changing parameters.
All the behavior of relayers, and the parameters are described a in a yaml file.   You can easily load the scenario file to simulate the result.  There are some example scenario files listed in [scenario](./scenario).

There are six different game mode: `relayers-only`, `relayer-challenger`, `relayer-challengers`, `relayers-extend`, `affirmation`, and `affirmation-only`.  We have analysized each mode, their pros and cons.  The winning mode is `affirmation-only` which will be implemented in Darwinia ChainRelay and deployed to Darwinia testnet, aka Crab Network first.

You can quickly jump to [**affirmation-Only mode**](#affirmation-only-mode) section for conclusion otherwise you can read on and see how the solution evolves along the way.

In `relayers-only`, `relayers-extend` and `affirmation` mode, when someone doesn't agree with the block submitted by other relayer, he should submit the correct block to express his opinion, while in `relayer-challenger` mode and `relayer-challengers` mode,  he signals simply by send a `yes` or `no` flag.


Following table shows the main differences between these mode.

| Rule \Mode                        | **Relayers-Only**  | **Relayer-Challenger** | **Relayer-Challengers** | **Relayers-Extend** | **affirmation/affirmation-Only** |
| --------------------------------- | ------------------ | ---------------------- | ----------------------- | ------------------- | -------------------------- |
| Only 1 relayer submit blocks      |                    | :heavy_check_mark:     | :heavy_check_mark:      |                     |                            |
| Allow extend from challenger      |                    |                        | :heavy_check_mark:      | :heavy_check_mark:  | :heavy_check_mark:         |
| Allow extend from initial relayer |                    |                        |                         |                     | :heavy_check_mark:         |
| Once in participate all           | :heavy_check_mark: | :heavy_check_mark:     |                         |                     |                            |
| Once lie drop all                 | :heavy_check_mark: |                        |                         |                     |                            |
| Ensure correct 1st block overall  | :heavy_check_mark: |                        |                         | :white_check_mark:  | :white_check_mark:         |
| Versus mode                       | 1 vs many          | 1 vs 1                 | 1 vs many               | 1 vs many           | many vs many               |
| Possible results                  | slash/reward       | slash/reward           | slash/reward/return     | slash/reward/return | slash/reward/return        |

Note: In most cases, return will no happend.

| Label              | Meaning                        |
|--------------------|--------------------------------|
| :heavy_check_mark: | in any condition               |
| :white_check_mark: | in most condition (Optimistic) |

In all mode, the `sampling function` will point out the next one or many blocks, the relayer(s) should submit on it.  
The `sampling function` is subtle, and should different when the target chain using different consensus mechanism.  
There is a discussion [**sampling function**](#sampling-function) section, but we will explain these modes with a general `half` sampling equation.

There is still a little possibility that the initial submit in from a valid branch chain,
so there is a stage two in the game, after that the blocks from the initial relayer are verified on chain.
There is a discussion in [**Stage two**](#stage-two) section.

If there is only one `[[challengers]]` in scenario file, the scenario will run in relayer-challenger mode.
The `scenario/challenger.yml` is a scenario for one relayer and one challenger, you may run it with `-v` option to know more about this.

If there is more than one `[[challengers]]` in scenario file, the scenario will run in relayer-challengers mode.
The `scenario/challengers.yml` is a scenario for one relayer with multiple challengers, you may run it with `-v` option to know more about this.