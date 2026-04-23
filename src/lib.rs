use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod bodega {
    use super::*;

    pub fn crear_bodega(context: Context<NuevaBodega>, nombre: String) -> Result<()> {
        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let cohetes: Vec<Cohete> = Vec::new();

        context.accounts.bodega.set_inner(Bodega { 
            owner: owner_id,
            nombre,
            cohetes,
        });
        Ok(())
    }

    pub fn agregar_cohete(context: Context<NuevoCohete>, nombre: String, modelo: String) -> Result<()> {
        require!(
            context.accounts.bodega.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        ); 

        let cohete = Cohete {
            nombre,
            modelo,
            disponible: true,
        };

        context.accounts.bodega.cohetes.push(cohete);
        Ok(())
    }

    pub fn eliminar_cohete(context: Context<NuevoCohete>, nombre: String) -> Result<()> {
        require!(
            context.accounts.bodega.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let cohetes = &mut context.accounts.bodega.cohetes;

        for i in 0..cohetes.len() {
            if cohetes[i].nombre == nombre {
                cohetes.remove(i);
                msg!("Cohete {} eliminado!", nombre);
                return Ok(());
            }
        }
        Err(Errores::CoheteNoExiste.into())
    }

    pub fn ver_cohete(context: Context<NuevoCohete>) -> Result<()> {
        require!(
            context.accounts.bodega.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!("La lista de cohetes actualmente es: {:#?}", context.accounts.bodega.cohetes);
        Ok(())
    }

    pub fn alternar_estado(context: Context<NuevoCohete>, nombre: String) -> Result<()> {
        require!(
            context.accounts.bodega.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let cohetes = &mut context.accounts.bodega.cohetes;
        for i in 0..cohetes.len() {
            let estado = cohetes[i].disponible;

            if cohetes[i].nombre == nombre {
                let nuevo_estado = !estado;
                cohetes[i].disponible = nuevo_estado;
                msg!("El cohete: {} ahora tiene un valor de disponibilidad: {}", nombre, nuevo_estado);
                return Ok(());
            }
        }

        Err(Errores::CoheteNoExiste.into())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la bodega que deseas modificar")]
    NoEresElOwner,
    #[msg("Error, el cohete con el que deseas interactuar no existe")]
    CoheteNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct Bodega {
    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    cohetes: Vec<Cohete>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Cohete {
    #[max_len(60)]
    nombre: String,

    #[max_len(60)]
    modelo: String,

    disponible: bool,
}

#[derive(Accounts)]
pub struct NuevaBodega<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Bodega::INIT_SPACE + 8,
        seeds = [b"bodega", owner.key().as_ref()],
        bump
    )]
    pub bodega: Account<'info, Bodega>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoCohete<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub bodega: Account<'info, Bodega>,
}
