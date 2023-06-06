use anchor_lang::prelude::*;

declare_id!("6o2c3QS8xZYFZ7fjWpfgMtdmDXEJZ9MC2LUpCHjGV2Dn");

#[program]
mod hello_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: String) -> Result<()> {
        //Deserialize the JSON data from data
        //Then assign them to their variables in the account

        //Vital Signs [To be deserialized from the incoming JSON]
        ctx.accounts.patient_account.blood_preasure = 120;
        ctx.accounts.patient_account.heart_rate = 80;
        ctx.accounts.patient_account.temperature = 37;
        ctx.accounts.patient_account.oxygen_level = 100;

        //Do some checks with the data and check If patients vital signs are below or above the healthy limits

        if (ctx.accounts.patient_account.blood_preasure > 170
            || ctx.accounts.patient_account.blood_preasure < 80)
        {
            msg!("In emergency, immidiately call the doctor");
            //In emergency, immidiately call the family doctor
            //OR
            //Get in touch with a doctor
        }

        if (ctx.accounts.patient_account.oxygen_level < 85) {
            msg!("In emergency, immidiately call the doctor");
            //In emergency, immidiately call the family doctor
            //OR
            //Get in touch with a doctor
        }

        if (ctx.accounts.patient_account.heart_rate > 100
            || ctx.accounts.patient_account.heart_rate < 60)
        {
            msg!("In emergency, immidiately call the doctor");
            //In emergency, immidiately call the family doctor
            //OR
            //Get in touch with a doctor
        }

        //In Fahrenheit
        if (ctx.accounts.patient_account.temperature > 38
            || ctx.accounts.patient_account.temperature < 35)
        {
            msg!("In emergency, immidiately call the doctor");
            //In emergency, immidiately call the family doctor
            //OR
            //Get in touch with a doctor
        }

        Ok(())
    }

    pub fn update_patient_data(ctx: Context<UpdateData>, data: String) -> Result<()> {
        //Deserialize the JSON data from data
        //Then assign them to their variables in the account

        //Vital Signs [To be deserialized from the incoming JSON]
        ctx.accounts.patient_account.blood_preasure = 120;
        ctx.accounts.patient_account.heart_rate = 80;
        ctx.accounts.patient_account.temperature = 37;
        ctx.accounts.patient_account.oxygen_level = 100;

        //Do some checks with the data and check If patients vital signs are below or above the healthy limits

        if (ctx.accounts.patient_account.blood_preasure > 170
            || ctx.accounts.patient_account.blood_preasure < 80)
        {
            msg!("In emergency, immidiately call the doctor");
            //In emergency, immidiately call the family doctor
            //OR
            //Get in touch with a doctor
        }

        if (ctx.accounts.patient_account.oxygen_level < 85) {
            msg!("In emergency, immidiately call the doctor");
            //In emergency, immidiately call the family doctor
            //OR
            //Get in touch with a doctor
        }

        if (ctx.accounts.patient_account.heart_rate > 100
            || ctx.accounts.patient_account.heart_rate < 60)
        {
            msg!("In emergency, immidiately call the doctor");
            //In emergency, immidiately call the family doctor
            //OR
            //Get in touch with a doctor
        }

        //In Fahrenheit
        if (ctx.accounts.patient_account.temperature > 38
            || ctx.accounts.patient_account.temperature < 35)
        {
            msg!("In emergency, immidiately call the doctor");
            //In emergency, immidiately call the family doctor
            //OR
            //Get in touch with a doctor
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 8 + 8 + 8)]
    pub patient_account: Account<'info, PatientAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut, payer = signer, space = 8 + 8 + 8 + 8 + 8)]
    pub patient_account: Account<'info, PatientAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PatientAccount {
    blood_preasure: u8,
    heart_rate: u8,
    temperature: u8,
    oxygen_level: u8,
}
