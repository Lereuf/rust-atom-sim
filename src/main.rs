mod templates;
use macroquad::prelude::*;
use std::fs;
use templates::Atome;

fn window_conf() -> Conf {
    Conf {
        window_title: "Atom sim".to_owned(),
        window_width: 1300,
        window_height: 1300,
        window_resizable: false,
        ..Default::default()
    }
}

fn get_atom() -> Atome {
    let fichier_atomes = fs::read_to_string("atomes.json").expect("erreur de lecture");
    return serde_json::from_str(&fichier_atomes).expect("erreur");
}

static MAP: std::sync::Mutex<Vec<(f32, f32, i32)>> = std::sync::Mutex::new(Vec::new());
static mut NUCLEONS_SIZE: f32 = 100.0;
static mut NB_ELECTRONS: i32 = 0;

fn draw() {
    clear_background(SKYBLUE);
    let map = MAP.lock().unwrap();
    let r = unsafe { NUCLEONS_SIZE } / 2.0;
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    // Dessin du noyau (protons et neutrons)
    for nucleon in map.iter().rev() {
        match nucleon.2 {
            0 => {
                draw_circle(center_x + nucleon.0, center_y + nucleon.1, r, RED);
                draw_circle_lines(center_x + nucleon.0, center_y + nucleon.1, r, 2.0, BLACK);
            }
            1 => {
                draw_circle(center_x + nucleon.0, center_y + nucleon.1, r, BLUE);
                draw_circle_lines(center_x + nucleon.0, center_y + nucleon.1, r, 2.0, BLACK);
            }
            _ => {}
        }
    }

    // Dessin de l'orbite et des électrons en mouvement
    let nb_electrons = unsafe { NB_ELECTRONS };
    if nb_electrons > 0 {
        let radius = 250.0; // Distance fixe par rapport au centre du noyau
        // Dessin de l'orbite (cercle vert fluo)
        draw_circle_lines(center_x, center_y, radius, 1.0, LIME);
        
        // Dessin des électrons en rotation
        let t = get_time() as f32;
        let speed = 1.5; // vitesse de rotation (radians par seconde)
        let re = (r / 3.0).max(5.0); // Les électrons sont petits mais toujours visibles
        for i in 0..nb_electrons {
            let theta = (i as f32 * std::f32::consts::PI * 2.0 / nb_electrons as f32) + t * speed;
            let offset_x = radius * theta.cos();
            let offset_y = radius * theta.sin();
            
            draw_circle(center_x + offset_x, center_y + offset_y, re, GREEN);
            draw_circle_lines(center_x + offset_x, center_y + offset_y, re, 1.0, BLACK);
        }
    }
}

fn gen_core(mut protons: i32, mut neutrons: i32, nucl_size: f32) {
    let mut map = MAP.lock().unwrap();
    let total = protons + neutrons;

    // Angle d'or en radians (environ 137.5 degrés)
    let golden_angle = std::f32::consts::PI * (3.0 - 5.0_f32.sqrt());

    for i in 0..total {
        // Choix aléatoire : 0 pour proton, 1 pour neutron
        let wth_ncl = if protons > 0 && neutrons > 0 {
            rand::gen_range(0, 2)
        } else if protons > 0 {
            0
        } else {
            1
        };

        if wth_ncl == 0 {
            protons -= 1;
        } else {
            neutrons -= 1;
        }

        // Spirale de Fermat pour un agencement dense et naturel
        // Le facteur 0.7 permet aux nucléons de se toucher/chevaucher plus étroitement
        let radius = nucl_size * 0.4 * (i as f32).sqrt();
        let theta = i as f32 * golden_angle;

        let offset_x = radius * theta.cos();
        let offset_y = radius * theta.sin();

        map.push((offset_x, offset_y, wth_ncl));
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);
    let atome: Atome = get_atom();
    unsafe {
        NUCLEONS_SIZE = 100.0 / (atome.nb_neutrons as f32 + atome.nb_protons as f32).sqrt();
        NB_ELECTRONS = atome.nb_electrons;
    }
    let prot_rest: i32 = atome.nb_protons;
    let neutr_rest: i32 = atome.nb_neutrons;
    gen_core(prot_rest, neutr_rest, unsafe { NUCLEONS_SIZE });
    loop {
        draw();
        next_frame().await
    }
}
