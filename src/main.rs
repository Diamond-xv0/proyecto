pub struct Personaje {
    nombre: String,
    vida_maxima: u32,
    vida_actual: u32,
    energia_maxima: u32,
    energia_actual: u32,
    fuerza: u32,
    resistencia: u32,
    defensa: u32,
    poder_magico: u32,
    elemento_base: Elemento,
    arma_equipada: Option<Arma>,
}

pub struct Arma {
    nombre: String,
    dano_base: u32,
}



pub struct Proyectil {
    tipo: String,
    velocidad: u32,
    dano: u32,
}
pub struct Enemigo {
    nombre: String,
    vida_maxima: u32,
    vida_actual: u32,
    energia_maxima: u32,
    energia_actual: u32,
    fuerza: u32,
    resistencia: u32,
    defensa: u32,
    poder_magico: u32,
    elemento_base: Elemento,
}

pub enum Elemento {
    Fuego { temperatura: u32 },
    Agua { variante: Option<VarianteAgua> },
    Tierra,
    Planta,
    Luz { lux: u32 },
    Oscuridad { sombra: u32 },
    Electricidad { voltaje: u32 },
    Sonido { decibelios: u32 },
    Neutro { ataque: u32 },
}

pub enum VarianteAgua {
    Hielo { frio: u32 },
}
pub enum TipoAtaque {
    Fisico { melee: bool, distancia: bool },
    Magico {elemento: Elemento},
}


fn ataque(personaje: &mut Personaje, enemigo: &mut Enemigo, tipo: TipoAtaque, proyectil: Proyectil) {
    match tipo {
        TipoAtaque::Fisico { melee: true, distancia: false } => {
            let dano = personaje.fuerza.saturating_sub(enemigo.resistencia);
            if let Some(arma) = &personaje.arma_equipada {
                let dano_total = dano.saturating_add(arma.dano_base);
                if dano_total > 0 && dano_total <= enemigo.vida_actual {
                    enemigo.vida_actual = enemigo.vida_actual.saturating_sub(dano_total);
                } else {
                    return;
                }
            } else if dano > 0 && dano <= enemigo.vida_actual {
                enemigo.vida_actual = enemigo.vida_actual.saturating_sub(dano);
            } else {
                return;
            }
        },
        TipoAtaque::Fisico { melee: false, distancia: true } => {
            if let Some(arma) = &personaje.arma_equipada {
                let dano = arma.dano_base + proyectil.dano;
                let dano = dano * proyectil.velocidad;
                let dano = dano.saturating_sub(enemigo.resistencia);
                if dano > 0 && dano <= enemigo.vida_actual {
                    enemigo.vida_actual = enemigo.vida_actual.saturating_sub(dano);
                } else {
                    return;
                }
            } else {
                return;
            }
        },
        TipoAtaque::Fisico { melee: false, distancia: false } => {
            return;
        },
        TipoAtaque::Fisico { melee: true, distancia: true } => {
            return;
        },
    }
}

fn main() {
}
