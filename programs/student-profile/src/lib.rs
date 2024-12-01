use anchor_lang::prelude::*;
use anchor_spl::{ associated_token::AssociatedToken, token::* };

declare_id!("FGc5wu8DhfTUDiytpLK8KzEZkf8phcS1MntXKqAq1dgd");

#[program]
pub mod student_profile {
    use super::*;

    pub fn add_student_profile(
        ctx: Context<AddStudentProfile>,
        name: String,
        about: String
    ) -> Result<()> {
        msg!("Adding student profile");
        msg!("About: {}", about);
        msg!("Name: {}", name);
        let student_profile = &mut ctx.accounts.student_profile;
        student_profile.student = ctx.accounts.student.key();
        student_profile.name = name;
        student_profile.about = about;

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
                &[&["mint".as_bytes(), &[ctx.bumps.mint]]]
            ),
            (10 * 10) ^ 6
        )?;

        msg!("Minted tokens");
        Ok(())
    }

    pub fn update_student_profile(
        ctx: Context<UpdateStudentProfile>,
        name: String,
        about: String
    ) -> Result<()> {
        msg!("Updating student profile");
        msg!("About: {}", about);
        msg!("Name: {}", name);
        let student_profile = &mut ctx.accounts.student_profile;
        student_profile.student = ctx.accounts.student.key();
        student_profile.name = name;
        student_profile.about = about;
        Ok(())
    }

    pub fn delete_student_profile(_ctx: Context<DeleteStudentProfile>) -> Result<()> {
        msg!("Deleting student profile");
        Ok(())
    }

    pub fn initialize_token_mint(_ctx: Context<InitializeMint>) -> Result<()> {
        msg!("Initializing mint");
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct StudentProfile {
    pub student: Pubkey,
    #[max_len(32)]
    pub name: String,
    #[max_len(100)]
    pub about: String,
}

#[derive(Accounts)]
#[instruction(name: String, about: String)]
pub struct AddStudentProfile<'info> {
    #[account(
        init,
        seeds = [student.key().as_ref()],
        bump,
        payer = student,
        space = DISC + StudentProfile::INIT_SPACE
    )]
    pub student_profile: Account<'info, StudentProfile>,
    #[account(mut)]
    pub student: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(
        seeds = ["mint".as_bytes()],
        bump,
        mut,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = student,
        associated_token::mint = mint,
        associated_token::authority = student
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(name: String, about: String)]
pub struct UpdateStudentProfile<'info> {
    #[account(
        mut,
        seeds = [student.key().as_ref()],
        bump,
        realloc = DISC + StudentProfile::INIT_SPACE,
        realloc::payer = student,
        realloc::zero = true,
    )]
    pub student_profile: Account<'info, StudentProfile>,
    #[account(mut)]
    pub student: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeleteStudentProfile<'info> {
    #[account(
        mut,
        seeds = [student.key().as_ref()],
        bump,
        close=student,
    )]
    pub student_profile: Account<'info, StudentProfile>,
    #[account(mut)]
    pub student: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        seeds = ["mint".as_bytes()],
        bump,
        payer = user,
        mint::decimals = 6,
        mint::authority = mint
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

const DISC: usize = 8;
