use anchor_lang::{prelude::*, solana_program::{program::invoke, system_instruction::{self}}};


declare_id!("G2g9f9N2RY1pWcoyK2j8U4LQXnucW8664TrkL88DG7D3");

#[program]
pub mod solbet {
    use super::*;

  pub fn initialize(ctx: Context<InitializeEscrow>) -> Result<()> {
    ctx.accounts.initializeescrow(&ctx.bumps)?;

    Ok(())
}

pub fn create_match(ctx: Context<CreateMatch>, match_id: u64) -> Result<()> {
    ctx.accounts.match_account.match_id = match_id;
    Ok(())
}


pub fn placebet(ctx: Context<PlaceBet>, amount:u64, outcome:Outcome, odds: u64 )  -> Result<()> {
    ctx.accounts.placebet(amount, outcome, odds)?;

    Ok(())
}

pub fn update(ctx: Context<UpdateMatchResult>, matchid: u64, outcome:Outcome ) -> Result<()> {
    ctx.accounts.updatematchresults(matchid, outcome)?;

    Ok(())
}



pub fn settlebets(ctx: Context<SettleBets>) -> Result<()> {
    ctx.accounts.settle_bets()?;
    Ok(())
}

}




#[derive(Accounts)]

pub struct InitializeEscrow<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        seeds = [b"escrow".as_ref(),  &0u64.to_le_bytes()],
        bump,
        payer = admin,
        space =  Escrow::INIT_SPACE,
    )]
    escrow: Account<'info, Escrow>,
    #[account(
        init,
        seeds = [b"bets_data".as_ref(),  &0u64.to_le_bytes()], 
        bump,
        payer = admin,
        space = BetsData::INIT_SPACE,
    )]
    pub bets_data: Account<'info, BetsData>,
    #[account(
        seeds = [b"vault".as_ref(), &0u64.to_le_bytes()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
  
    pub system_program: Program<'info, System>,
}


impl <'info> InitializeEscrow<'info> {
    pub fn initializeescrow(&mut self, bumps:  &InitializeEscrowBumps,) 
    -> Result<()> {
   self.escrow.escrow_bump = bumps.escrow;
   self.escrow.vault_bump = bumps.vault;
   self.escrow.total_amount = 0;

     // Initialize BetsData
     self.bets_data.bump = bumps.bets_data;
     self.bets_data.total_bets = 0;
     self.bets_data.total_amount_bet = 0;
     self.bets_data.status = BetStatus::Open;
     self.bets_data.match_id = 0;
     self.bets_data.bet_accounts = Vec::new();
     
        Ok(())
    }  
}

#[derive(Accounts)]
pub struct CreateMatch<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = Match::INIT_SPACE
    )]
    pub match_account: Account<'info, Match>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(amount: u64, outcome: Outcome, odds: u64)]
//pass the instructs above as mandatory inclusions to the context 
pub struct PlaceBet <'info> {
    #[account (mut)]
    pub bettor: Signer<'info>,
    #[account (
        init, 
        payer = bettor, 
        space = Bet::INIT_SPACE
    )]
    pub bet: Account<'info, Bet>,
    #[account(
        mut,
        seeds = [b"bets_data".as_ref(), &0u64.to_le_bytes()], 
        bump = bets_data.bump,
    )]
    pub bets_data: Account<'info, BetsData>,
    #[account(
        mut,
        seeds = [b"escrow".as_ref(), &0u64.to_le_bytes()],
        bump = escrow.escrow_bump,
    )]
    escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub match_account: Account<'info, Match>,
    #[account(
        mut,
        seeds = [b"vault".as_ref(), &0u64.to_le_bytes()],
        bump = escrow.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl <'info> PlaceBet<'info> {

    pub fn placebet(&mut self, amount: u64, outcome: Outcome, odds: u64) 
    -> Result<()> {
        self.bet.bettor = self.bettor.key();
        self.bet.amount = amount;
        self.bet.outcome = outcome;
        self.bet.odds = odds;
        self.bet.match_id = self.match_account.match_id;

        self.bets_data.total_bets += 1;
        self.bets_data.status = BetStatus::Open;
        self.bets_data.match_id = self.match_account.match_id;
        self.bets_data.total_amount_bet += amount;
        self.bets_data.bet_accounts.push(self.bet.key());

        invoke( &system_instruction::transfer(&self.bettor.key, &self.vault.key, amount), &[
            self.bettor.to_account_info(),
            self.vault.to_account_info(),
            self.system_program.to_account_info(),
        ])?;


        Ok(())
    }  
}


#[derive(Accounts)]
pub struct UpdateMatchResult<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub match_account: Account<'info, Match>,
    pub system_program: Program<'info, System>,
}

impl <'info> UpdateMatchResult<'info> {

    pub fn updatematchresults(&mut self, match_id : u64 , result: Outcome ) 
    -> Result<()> {
        self.match_account.match_id = match_id;
        self.match_account.result = result;
        Ok(())
    }  
}



#[derive(Accounts)]
pub struct SettleBets<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub match_account: Account<'info, Match>,
    #[account(
        mut,
        seeds = [b"escrow".as_ref(), &0u64.to_le_bytes()],
        bump = escrow.escrow_bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        seeds = [b"bets_data".as_ref(), &0u64.to_le_bytes()],
        bump = bets_data.bump,
    )]
    pub bets_data: Account<'info, BetsData>,
    #[account(
        mut,
        seeds = [b"vault".as_ref(), &0u64.to_le_bytes()],
        bump = escrow.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> SettleBets<'info> {
    pub fn settle_bets(&mut self) -> Result<()> {
        let bet_pubkeys = &self.bets_data.bet_accounts;
        msg!("Bet Account Public Keys: {:?}", bet_pubkeys);

        let account_infos: Vec<AccountInfo<'info>> = vec![
            self.escrow.to_account_info(),
            self.vault.to_account_info(),
            self.system_program.to_account_info(),
            self.admin.to_account_info(),
            self.match_account.to_account_info(),
            self.bets_data.to_account_info()
        ];
        for bet_pubkey in bet_pubkeys {
            let bet_account_info = account_infos
                .iter()
                .find(|info| info.key == bet_pubkey)
                .ok_or(ErrorCode::BetAccountNotFound)?;

            let bet_account = Bet::try_from_slice(&bet_account_info.try_borrow_data()?)?;

            if bet_account.outcome == self.match_account.result {
                let payout_amount = bet_account.amount * bet_account.odds;

                invoke(
                    &system_instruction::transfer(
                        &self.vault.key(),
                        &bet_account.bettor,
                        payout_amount,
                    ),
                    &[
                        self.vault.to_account_info(),
                        bet_account_info.clone(),
                        self.system_program.to_account_info(),
                    ],
                )?;

                self.escrow.total_amount -= payout_amount;
            }
        }

        if self.escrow.total_amount > 0 {
            invoke(
                &system_instruction::transfer(
                    &self.vault.key(),
                    &self.admin.key(),
                    self.escrow.total_amount,
                ),
                &[
                    self.vault.to_account_info(),
                    self.admin.to_account_info(),
                    self.system_program.to_account_info(),
                ],
            )?;
        }

        Ok(())
    }

  
}





// Define accounts
#[account]
pub struct Bet {
    pub bettor: Pubkey,
    pub amount: u64,
    pub outcome: Outcome,
    pub odds: u64,
    pub match_id: u64,
}

impl Space for Bet {
    const INIT_SPACE: usize = 8 + 32 + 8 + 1 + 8 + 8  ;
}


#[account]
pub struct Match {
    pub match_id: u64,
    pub result: Outcome,
}

impl Space for Match {
    const INIT_SPACE: usize = 8 + 8 + 1 ;
}


#[account]

pub struct Escrow {
    pub escrow_bump: u8,
    pub vault_bump: u8,
    pub total_amount: u64,
}

impl Space for Escrow {
    const INIT_SPACE: usize = 8 + 1 + 1 + 8  ; // Add 6 bytes for padding to align to 8-byte boundary
}


// Define enums
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum Outcome {
    Win,
    Lose,
}

#[account]
pub struct BetsData {
    pub bump: u8,
    pub total_bets: u64,
    pub total_amount_bet: u64,
    pub status: BetStatus,
    pub match_id: u64,
    pub bet_accounts: Vec<Pubkey>, // Store Pubkeys of Bet accounts
}


impl Space for BetsData {
    const INIT_SPACE: usize = 8 + 1 + 8  + 8 + 1 + 8 + 4  + 320;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum BetStatus {
    Open,
    Closed,
    Settled,
}


// Error Codes
#[error_code]
pub enum ErrorCode {
    #[msg("Bet account not found.")]
    BetAccountNotFound,
}