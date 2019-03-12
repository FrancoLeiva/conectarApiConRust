const VEL_BICI: f32 = 25.0; // KM por hora
const ANIO_LUZ: f32 = 9460730472580.8; // km

fn cuanto_en_bici(anios_luz: f32) -> f32 {
    let km_al_anio : f32 = VEL_BICI * 24.0 * 365.0;
    let distancia_total : f32 = ANIO_LUZ * anios_luz;
    distancia_total / km_al_anio
}

fn main(){
    println!("Wn bici te vas a tardar {} años.", cuanto_en_bici(40.0));
}
