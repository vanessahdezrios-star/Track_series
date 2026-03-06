//PROYECTO TITULADO: "TRACKER DE SERIES"
//ELABORADO POR: VANESSA HERNÁNDEZ RÍOS

use anchor_lang::prelude::*; //Importa todo lo que voy a necesitar para usar anchor

declare_id!("FB6Nb9UZDXhPveBu6yqnx7YXTXmR3MjBu7neFpe7ekKk");

//PROGRAMA
#[program] // Todo el código  utilizado para crear el programa en Solana
pub mod track_series {
    use super::*;

    //--SE CREA EL TRACKER DE SERIES
    pub fn crear_tracker(context: Context<NuevoTrackSeries>, nombre: String) -> Result<()> {
        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let series: Vec<Serie> = Vec::new(); //se crea el vector

        // Se guarda
        context.accounts.series.set_inner(TrackSeries {
            owner: owner_id,
            nombre,
            series,
        });
        Ok(())
    }

    //--FUNCIÓN PARA QUE EL USUARIO AGREGUE SERIES
    pub fn agregar_serie(
        context: Context<NuevaSerie>,
        nombre: String,
        temporadas: u16,
        plataforma: String,
        estado: EstadoSerie,
    ) -> Result<()> {
        //MANDA A LLAMAR A LA FUNCIÓN DE ERRORES CON EL MENSAJE DEL OWNER
        require!(
            context.accounts.series.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let serie = Serie {
            nombre,
            temporadas,
            plataforma,
            estado,
        };

        context.accounts.series.series.push(serie);

        Ok(())
    }

    //--FUNCIÓN PARA QUE EL USUARIO VEA LAS SERIES AGREGADAS
    pub fn ver_series(context: Context<NuevaSerie>) -> Result<()> {
        //MANDA A LLAMAR A LA FUNCIÓN DE ERRORES CON EL MENSAJE DEL OWNER
        require!(
            context.accounts.series.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );
        let series = &context.accounts.series.series;
        for (i, serie) in series.iter().enumerate() {
            msg!("-----------------------------");
            msg!("Serie {}:", i + 1);
            msg!("Nombre: {}", serie.nombre);
            msg!("Temporadas: {}", serie.temporadas);
            msg!("Plataforma: {}", serie.plataforma);
            msg!("Estado: {:?}", serie.estado);
        }

        Ok(())
    }

    //--FUNCIÓN PARA QUE EL USUARIO EDITE LAS SERIES
    pub fn editar_serie(
        context: Context<NuevaSerie>,
        nombre: String,
        nuevas_temporadas: u16,
        nuevo_estado: EstadoSerie,
    ) -> Result<()> {
        //MANDA A LLAMAR A LA FUNCIÓN DE ERRORES CON EL MENSAJE DEL OWNER
        require!(
            context.accounts.series.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let series = &mut context.accounts.series.series;

        for i in 0..series.len() {
            if series[i].nombre == nombre {
                //CAMBIAR TEMPORADAS
                series[i].temporadas = nuevas_temporadas;
                //CAMBIAR ESTADOS
                series[i].estado = nuevo_estado;

                msg!(
                    "Serie actualizada: {} | Temporadas: {} | Estado: {:?}",
                    nombre,
                    series[i].temporadas,
                    series[i].estado
                );
                return Ok(());
            }
        }
        //MANDA A LLAMAR LA FUNCIÓN DE ERRORES CON EL MENSAJE DE LA SERIE
        Err(Errores::SerieNoExiste.into())
    }

    //--FUNCIÓN PARA QUE EL USUARIO ELIMINE SERIES
    pub fn eliminar_serie(context: Context<NuevaSerie>, nombre: String) -> Result<()> {
        //MANDA A LLAMAR A LA FUNCIÓN DE ERRORES CON EL MENSAJE DEL OWNER
        require!(
            context.accounts.series.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );
        let series = &mut context.accounts.series.series;

        for i in 0..series.len() {
            if series[i].nombre == nombre {
                series.remove(i);
                msg!("La serie {} fue eliminada", nombre);
                return Ok(());
            }
        }
        //MANDA A LLAMAR LA FUNCIÓN DE ERRORES CON EL MENSAJE DE LA SERIE
        Err(Errores::SerieNoExiste.into())
    }
}

//--CÓDIGO PARA LOS ERRORES
#[error_code]
pub enum Errores {
    #[msg("¡Error!, no eres el propietario de la cuenta")]
    NoEresElOwner,
    #[msg("¡Error!,la serie no existe")]
    SerieNoExiste,
}

//--ESTRCUTURA DE LA CUENTA DONDE SE ALAMACENARAN LAS SERIES
#[account]
#[derive(InitSpace)]
pub struct TrackSeries {
    owner: Pubkey,
    #[max_len(60)] //Longitud del string
    nombre: String,
    #[max_len(30)]
    series: Vec<Serie>,
}

//--DATOS DE LA SERIE
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Serie {
    #[max_len(60)]
    nombre: String,
    temporadas: u16,
    #[max_len(15)]
    plataforma: String,
    #[max_len(15)]
    estado: EstadoSerie,
}

//-- DEFINE ESTADO DE LA SERIE
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, PartialEq, Debug)]
pub enum EstadoSerie {
    Viendo,
    Pausada,
    Encurso,
    Terminada,
}

//--CUENTAS NECESARIAS PARA QUE FUNCIONE EL PROGRAMA
#[derive(Accounts)] //Describe las cuentas
pub struct NuevoTrackSeries<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = TrackSeries::INIT_SPACE + 8,
        seeds = [b"track_series", owner.key().as_ref()],
        bump
    )]
    pub series: Account<'info, TrackSeries>,
    pub system_program: Program<'info, System>,
}

//--CUENTAS PARA CREAR Y MODIFICAR SERIES
#[derive(Accounts)]
pub struct NuevaSerie<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub series: Account<'info, TrackSeries>,
}
