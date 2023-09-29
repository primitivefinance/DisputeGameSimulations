# Documentating the state and process of this work

Since this project is large and has a degree of context that is not immediatly obvious if you are not familiar with the dispute game and the dispute resolution process, I have decided to document the process and the state of the project in this file.


## Fraud Proofs and the Dispute Game

Fraud proofs are used to validate and ensure that execution data from a layer roll up commited to L1 is correct. If someone believes the data commited by the L2 is incorrect they should be able to challenge the commitment thus starting an instance of the dispute game. The dispute game is then played between two parties until a valid execution trace is identified. At a high level the desired properties of this game are:

 - That it is imposible for the game to resolve to an invalid execution trace (soundness)
 - There is an incentive for the honest party to win the game (fairness)

Currently the game is played by two parties who take turns bisecting the execution trace until a valid execution trace is identified. The time complexity to resolve a dispute game with an execution trace of size $N$ is $\log_{2}(N)$. In order for this game to played fairly and honestly, all parties need access to the execution environment. Currently there are three implementations of the execution environment (Called Cannon):

 - [Solidity](https://github.com/ethereum-optimism/optimism/blob/develop/packages/contracts-bedrock/src/cannon/MIPS.sol)
 - [Golang](https://github.com/ethereum-optimism/optimism/tree/develop/op-program)
 - [Rust](https://github.com/anton-rs/cannon-rs)

 These run the fault proof program that verifies the rollup state transition to ensure valid L2 inputs and L1 outputs. These programs are used to resolve the dispute game. Spec documentation:

 - [Fault-Proof](https://github.com/ethereum-optimism/optimism/blob/65ec61dde94ffa93342728d324fecf474d228e1f/specs/fault-proof.md)
 - [Dispute Game](https://github.com/ethereum-optimism/optimism/blob/65ec61dde94ffa93342728d324fecf474d228e1f/specs/dispute-game.md)
 - [Dispute Game Interface](https://github.com/ethereum-optimism/optimism/blob/65ec61dde94ffa93342728d324fecf474d228e1f/specs/dispute-game-interface.md)

## Simulations

My approach to this simulation has been motivated and inspired by other simulations designed with arbiter like [Portfolio Simulations](https://github.com/primitivefinance/portfolio_simulations). 

To start, I think the most important thing is to identify a random variable which we want to model. In AMM simulations we have done in the past this has been price paths of assets whith different stocastic processes. 
- In this simulation we are interested in modeling behavior of a dishonest actor in the dispute game. Our goal is to provide evidence for or against the soundness (likelyhood of a dishonest actor winning) of the dispute game.

In this approach we look to manually load in some reasonable starting state for our system. This historically has looked like 
- initializing EOAs, 
- deploying contracts, 
- depositing liquidity into contracts etc. 

Once a reasonable state is reached we can loop over some variation in our random variable on some dispute games and let the speed of wind and revm do it's work. For The dispute game this looks like 
- Identifying the minimal set of contracts needed to play the game
- Identifying the minimal set of EOAs
- Creating these EOAs and deploying these contracts
- Defining an honest and dishonest actor to model each EOA
- Starting a dispute game between two EOAs

So far I have indentified the following contracts that need to be played are
- [FaultDisputeGame](lib/optimism/packages/contracts-bedrock/src/dispute/FaultDisputeGame.sol)
- [L2OutputOracle](lib/optimism/packages/contracts-bedrock/src/L1/L2OutputOracle.sol): The trusted L2OutputOracle contract.
- [BlockOracle](lib/optimism/packages/contracts-bedrock/src/dispute/BlockOracle.sol): The block oracle, used for loading block hashes further back than the `BLOCKHASH` opcode allows as well as their estimated timestamps.
- [IBigStepper (The execution machine)](lib/optimism/packages/contracts-bedrock/src/dispute/interfaces/IBigStepper.sol)

I have currently been able to deploy the [AlphabetVM](lib/optimism/packages/contracts-bedrock/test/FaultDisputeGame.t.sol) which implements [IBigStepper](lib/optimism/packages/contracts-bedrock/src/dispute/interfaces/IBigStepper.sol) and the [L2OutputOracle](lib/optimism/packages/contracts-bedrock/src/L1/L2OutputOracle.sol). I am currently looking into deploying the FaultDisputeGame and BlockOracle contracts, and am working through understanding all the arguments in their constructors.

## Resources 
- [Keys in Mordor Summit: Dispute Games](https://www.youtube.com/watch?v=GaLm4iXOtOo)
- [Sub Game](https://oplabs.notion.site/Sub-game-Reasoning-74c1e434025344fa968d61612b18709d)