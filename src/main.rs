mod templates;
use macroquad::{prelude::*, rand::rand};
use std::{env::current_exe, fs};
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

fn get_atom() -> Atome
{
    let fichier_atomes = fs::read_to_string("atomes.json")
        .expect("erreur de lecture");
    return serde_json::from_str(&fichier_atomes).expect("erreur");
}

fn draw()
{
    clear_background(SKYBLUE);
}

fn gen_core(mut protons: i32, mut neutrons: i32, nucl_size: f32)
{
    let mut atomes: Vec<(f32, f32, i32)> = Vec::new();
    match rand::gen_range(0, 2)
    {
        0 => {atomes.push((650.0, 650.0, 0)); protons -= 1;},
        1 => {atomes.push((650.0, 650.0, 1)); neutrons -= 1;},
        _ => {}
    }
    while protons > 0 || neutrons > 0 {
        let wth_ncl = rand::gen_range(0, 1);
        match rand::gen_range(0, 7) {
            0 => {atomes.push((650.0 + nucl_size, 650.0, wth_ncl));},
            1 => {},
            2 => {},
            3 => {},
            4 => {},
            5 => {},
            6 => {},
            7 => {},
            _ => {}
        }
        break;
    }
}

#[macroquad::main(window_conf())]
async fn main()
{

    rand::srand(macroquad::miniquad::date::now() as u64);
    let atome: Atome = get_atom();
        let nucleons_size: f32 = 100.0 / (atome.nb_neutrons as f32 + atome.nb_protons as f32);
        let mut prot_rest: i32 = atome.nb_protons;
        let mut neutr_rest: i32 = atome.nb_neutrons;
        gen_core(prot_rest, neutr_rest, nucleons_size);
    loop {
        next_frame().await
    }
}