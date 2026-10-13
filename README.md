<p align="center">
  <img src="assets/logo.svg" alt="Piroba logo" width="140" />
</p>

<h1 align="center">Piroba</h1>

<p align="center"><b>Put money on your promises. Friends stake USDC on their goals, and finishers take the pot.</b></p>

> **Status: idea stage.** This repo contains the design and a program skeleton; no logic is implemented yet.

*Piroba* (პირობა) is Georgian for "promise". We're building it for the Solana Colosseum hackathon.

## The problem

Most goals fail quietly. People say they'll run three times a week, study every day or quit something, and nobody keeps track. Habit apps rely on streaks and notifications, which are easy to ignore because missing a day costs nothing. Putting money on a bet with friends works better, but in practice someone has to hold the cash, decide who actually did it and chase people to pay up. That takes trust and effort, so most groups never do it.

## How it works

1. **Create.** Pick a goal, a duration, a stake (e.g. 20 USDC) and a proof method. The rules lock when the first person joins.
2. **Join.** Friends open a link (a Solana Blink) and stake with a wallet or an email/passkey wallet. The app sponsors transaction fees, so nobody needs SOL.
3. **Prove.** Daily or weekly check-ins, using either:
   - a **zkTLS proof** from a fitness account like Strava via Reclaim Protocol, which reveals only pass/fail, or
   - a **photo** the group votes on, with a 24h dispute window and a dispute deposit.
4. **Settle.** After the deadline anyone can call `settle`. Finishers get their stake back plus a share of the forfeited stakes, minus a 10% fee on forfeits. Each finisher gets a non-transferable Token-2022 badge.

## How it uses Solana

Solana plays three roles:

| Role | What it does | How |
|---|---|---|
| **The bank** | Holds every stake so nobody can move it | Program-owned USDC escrow (a PDA token account) |
| **The referee** | Decides who kept their promise | Verifies proofs and records votes and disputes on-chain |
| **The scoreboard** | Keeps a permanent record of who finished | Non-transferable Token-2022 badges |

Fast, cheap transactions make daily check-ins and per-person payouts practical. Blinks mean a challenge can be joined straight from a chat link.

## Planned architecture

```
 Telegram Mini App / Next.js web app ──► Solana Action (Blink) endpoints
                │                                   │
                ▼                                   ▼
  Embedded email/passkey wallet  ──►  fee-sponsored transactions
                                                    │
                                                    ▼
                                        Piroba Anchor program
                         ┌──────────────┬───────────┴───────┬──────────────┐
                    Challenge PDA   Participant PDAs    Dispute PDAs   USDC vault PDA
                                                    │
                       Reclaim zkTLS proofs ──► submit_proof
                                                    │
                                     claim ──► Token-2022 badge
```

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for accounts, PDAs and the instruction flow. The program skeleton is in [programs/piroba/src/lib.rs](programs/piroba/src/lib.rs).

## Planned stack

- **Anchor** for the on-chain program
- **Reclaim Protocol zkTLS** for private, verifiable fitness proofs
- **Token-2022 non-transferable tokens** for finisher badges
- **Solana Actions / Blinks** so people can join from a link
- **Embedded email wallets with fee sponsorship**, so nobody needs SOL
- **USDC** for stakes
- **Next.js** for the web app
- **Telegram Mini App**, where friend groups already are

## Business model

- **Sponsored challenges** from gyms, schools and brands
- **A 10% fee on forfeited stakes.** Finishers never pay a fee on their own stake.
- **Interest on idle stakes** while challenges run
- **Premium features**

## Roadmap

1. **Validate** with real Telegram groups before writing much code
2. **Prove** that a Reclaim proof works end to end on devnet
3. **Build** the escrow program (create, join, prove, settle, claim)
4. **Add** Blink + email onboarding with sponsored fees
5. **Audit** before mainnet

## Team

- [@Besika40k](https://github.com/Besika40k)
- Rezo Darsavelidze

## License

[MIT](LICENSE)
