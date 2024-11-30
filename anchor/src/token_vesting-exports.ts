// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'
import IDL from '../target/idl/token_vesting.json'
import type { TokenVesting } from '../target/types/token_vesting'

// Re-export the generated IDL and type
export { TokenVesting, IDL }

// The programId is imported from the program IDL.
export const TOKEN_VESTING_PROGRAM_ID = new PublicKey(IDL.address)

// This is a helper function to get the Tokenvesting Anchor program.
export function getTokenVestingProgram(provider: AnchorProvider) {
  return new Program(IDL as TokenVesting, provider)
}

// This is a helper function to get the program ID for the Tokenvesting program depending on the cluster.
export function getTokenVestingProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
      // This is the program ID for the Tokenvesting program on devnet and testnet.
      return new PublicKey('BhE35ptgBkeJMPo9zo6YLzPmWKuck88BHGekC3h6XiyE')
    case 'mainnet-beta':
    default:
      return TOKEN_VESTING_PROGRAM_ID
  }
}
