//! Piroba (პირობა, "promise"): friends stake USDC on their goals.
//!
//! STATUS: skeleton only. Accounts and instruction signatures are sketched out,
//! but every instruction body is a TODO. Nothing here moves funds yet.
//! See docs/ARCHITECTURE.md for the intended design.

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

// Placeholder program ID (Anchor's default template key).
// Replaced with the real key on first deploy via `anchor keys sync`.
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BkZ7FEfcYkg476zPFsLnS");

pub const CHALLENGE_SEED: &[u8] = b"challenge";
pub const VAULT_SEED: &[u8] = b"vault";
pub const PARTICIPANT_SEED: &[u8] = b"participant";
pub const DISPUTE_SEED: &[u8] = b"dispute";

/// Planned protocol fee on forfeited stakes, in basis points (10%).
pub const FORFEIT_FEE_BPS: u16 = 1_000;
/// Planned dispute window after a photo proof is submitted (24h).
pub const DISPUTE_WINDOW_SECS: i64 = 24 * 60 * 60;

#[program]
pub mod piroba {
    use super::*;

    /// Creates a challenge and its program-owned USDC vault.
    /// Will store the goal parameters (stake, start/end, proof method).
    /// Rules become immutable once the first participant joins.
    pub fn create_challenge(
        _ctx: Context<CreateChallenge>,
        _challenge_id: u64,
        _stake_amount: u64,
        _start_ts: i64,
        _end_ts: i64,
        _proof_kind: ProofKind,
    ) -> Result<()> {
        // TODO
        Ok(())
    }

    /// Joins a challenge: transfers `stake_amount` from the user into the vault
    /// and creates their Participant account. Only allowed before `start_ts`.
    pub fn join_challenge(_ctx: Context<JoinChallenge>) -> Result<()> {
        // TODO
        Ok(())
    }

    /// Records a check-in. For `ProofKind::Reclaim` it will verify a zkTLS proof
    /// (pass/fail only); for `ProofKind::PhotoVote` it records a pending photo
    /// check-in identified by `proof_hash` that the group then votes on.
    pub fn submit_proof(_ctx: Context<SubmitProof>, _proof_hash: [u8; 32]) -> Result<()> {
        // TODO
        Ok(())
    }

    /// A participant votes on another participant's photo check-in,
    /// or on an open dispute.
    pub fn vote(_ctx: Context<Vote>, _approve: bool) -> Result<()> {
        // TODO
        Ok(())
    }

    /// Opens a dispute against a check-in within the 24h window.
    /// The opener locks a dispute deposit, returned if the dispute succeeds.
    pub fn open_dispute(_ctx: Context<OpenDispute>, _deposit: u64) -> Result<()> {
        // TODO
        Ok(())
    }

    /// Callable by anyone after `end_ts` (and after the last dispute window).
    /// Will mark the challenge settled and fix the payout per finisher:
    /// stake back + equal share of forfeits, minus the 10% fee on forfeits.
    pub fn settle(_ctx: Context<Settle>) -> Result<()> {
        // TODO
        Ok(())
    }

    /// A finisher withdraws their payout from the vault and receives a
    /// non-transferable Token-2022 badge. Can only be claimed once.
    pub fn claim(_ctx: Context<Claim>) -> Result<()> {
        // TODO
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Instruction accounts
// ---------------------------------------------------------------------------

#[derive(Accounts)]
#[instruction(challenge_id: u64)]
pub struct CreateChallenge<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = 8 + Challenge::INIT_SPACE,
        seeds = [CHALLENGE_SEED, creator.key().as_ref(), &challenge_id.to_le_bytes()],
        bump,
    )]
    pub challenge: Account<'info, Challenge>,

    /// Program-owned escrow. Authority is the challenge PDA, so no person can move funds.
    #[account(
        init,
        payer = creator,
        seeds = [VAULT_SEED, challenge.key().as_ref()],
        bump,
        token::mint = stake_mint,
        token::authority = challenge,
        token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    /// Expected to be USDC.
    pub stake_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinChallenge<'info> {
    /// Fee payer can be a sponsor, so users never need SOL.
    #[account(mut)]
    pub payer: Signer<'info>,
    pub user: Signer<'info>,

    #[account(mut)]
    pub challenge: Account<'info, Challenge>,

    #[account(
        init,
        payer = payer,
        space = 8 + Participant::INIT_SPACE,
        seeds = [PARTICIPANT_SEED, challenge.key().as_ref(), user.key().as_ref()],
        bump,
    )]
    pub participant: Account<'info, Participant>,

    #[account(mut)]
    pub user_token: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, seeds = [VAULT_SEED, challenge.key().as_ref()], bump = challenge.vault_bump)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub stake_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitProof<'info> {
    pub user: Signer<'info>,
    pub challenge: Account<'info, Challenge>,
    #[account(
        mut,
        seeds = [PARTICIPANT_SEED, challenge.key().as_ref(), user.key().as_ref()],
        bump = participant.bump,
    )]
    pub participant: Account<'info, Participant>,
    // TODO: Reclaim verifier accounts for zkTLS proofs.
}

#[derive(Accounts)]
pub struct Vote<'info> {
    pub voter: Signer<'info>,
    pub challenge: Account<'info, Challenge>,
    /// The voter must be a participant in the same challenge.
    #[account(
        seeds = [PARTICIPANT_SEED, challenge.key().as_ref(), voter.key().as_ref()],
        bump = voter_participant.bump,
    )]
    pub voter_participant: Account<'info, Participant>,
    /// The participant whose check-in is being voted on.
    #[account(mut)]
    pub target: Account<'info, Participant>,
}

#[derive(Accounts)]
pub struct OpenDispute<'info> {
    #[account(mut)]
    pub opener: Signer<'info>,
    pub challenge: Account<'info, Challenge>,
    pub target: Account<'info, Participant>,

    #[account(
        init,
        payer = opener,
        space = 8 + Dispute::INIT_SPACE,
        seeds = [DISPUTE_SEED, target.key().as_ref(), &target.checkins.to_le_bytes()],
        bump,
    )]
    pub dispute: Account<'info, Dispute>,

    #[account(mut)]
    pub opener_token: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, seeds = [VAULT_SEED, challenge.key().as_ref()], bump = challenge.vault_bump)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Settle<'info> {
    /// Anyone can crank settlement.
    pub caller: Signer<'info>,
    #[account(mut)]
    pub challenge: Account<'info, Challenge>,
    #[account(seeds = [VAULT_SEED, challenge.key().as_ref()], bump = challenge.vault_bump)]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    // TODO: treasury account for the forfeit fee.
}

#[derive(Accounts)]
pub struct Claim<'info> {
    pub user: Signer<'info>,
    pub challenge: Account<'info, Challenge>,
    #[account(
        mut,
        seeds = [PARTICIPANT_SEED, challenge.key().as_ref(), user.key().as_ref()],
        bump = participant.bump,
    )]
    pub participant: Account<'info, Participant>,

    #[account(mut, seeds = [VAULT_SEED, challenge.key().as_ref()], bump = challenge.vault_bump)]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub user_token: InterfaceAccount<'info, TokenAccount>,

    pub stake_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    // TODO: Token-2022 badge mint (NonTransferable extension) + user badge account.
}

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

#[account]
#[derive(InitSpace)]
pub struct Challenge {
    pub creator: Pubkey,
    /// Used in the PDA seeds so one creator can run many challenges.
    pub challenge_id: u64,
    pub stake_mint: Pubkey,
    pub stake_amount: u64,
    pub start_ts: i64,
    pub end_ts: i64,
    pub proof_kind: ProofKind,
    pub participant_count: u32,
    pub settled: bool,
    pub bump: u8,
    pub vault_bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Participant {
    pub challenge: Pubkey,
    pub user: Pubkey,
    pub checkins: u16,
    pub completed: bool,
    pub claimed: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Dispute {
    pub challenge: Pubkey,
    /// Participant account whose check-in is disputed.
    pub target: Pubkey,
    pub opener: Pubkey,
    pub deposit: u64,
    pub opened_ts: i64,
    pub votes_uphold: u16,
    pub votes_reject: u16,
    pub resolved: bool,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum ProofKind {
    /// zkTLS proof via Reclaim Protocol (e.g. Strava). Only pass/fail is revealed.
    Reclaim,
    /// Photo check-in approved by group vote, with a 24h dispute window.
    PhotoVote,
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[error_code]
pub enum PirobaError {
    #[msg("End time must be after start time")]
    InvalidTimeRange,
    #[msg("Stake amount must be greater than zero")]
    InvalidStakeAmount,
    #[msg("Challenge has already started; joining is closed")]
    ChallengeStarted,
    #[msg("Challenge is not active")]
    ChallengeNotActive,
    #[msg("Challenge has not ended yet")]
    ChallengeNotEnded,
    #[msg("Challenge is already settled")]
    AlreadySettled,
    #[msg("Challenge is not settled yet")]
    NotSettled,
    #[msg("Proof is invalid")]
    InvalidProof,
    #[msg("Wrong proof method for this challenge")]
    WrongProofKind,
    #[msg("Voter is not a participant in this challenge")]
    NotParticipant,
    #[msg("Dispute window has closed")]
    DisputeWindowClosed,
    #[msg("Dispute deposit is too low")]
    DepositTooLow,
    #[msg("Participant did not complete the challenge")]
    NotCompleted,
    #[msg("Payout already claimed")]
    AlreadyClaimed,
    #[msg("Arithmetic overflow")]
    MathOverflow,
}
