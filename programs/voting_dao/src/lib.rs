use anchor_lang::prelude::*;

declare_id!("HHuZj7VnCbpGMJ9Y9ARFRmk6WT6eAKVXETekMbPw2bsq");

#[program]
pub mod dao_voting {
    use super::*;

    pub fn create_proposal(ctx: Context<CreateProposal>, title: String, description: String) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.title = title;
        proposal.description = description;
        proposal.creator = *ctx.accounts.user.key;
        proposal.votes_for = 0;
        proposal.votes_against = 0;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, vote_for: bool) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        if vote_for {
            proposal.votes_for += 1;
        } else {
            proposal.votes_against += 1;
        }
        // let user = &mut ctx.accounts.useq;
        // user.reward_points += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(init, payer = user, space = 8 + 64 + 256 + 32 + 8 + 8)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct Proposal {
    pub title: String,
    pub description: String,
    pub creator: Pubkey,
    pub votes_for: u64,
    pub votes_against: u64,
}


#[account]
pub struct User {
    pub reward_points: u64,
}