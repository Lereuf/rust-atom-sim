mod drawatomes;
mod templates;
use macroquad::prelude::*;
use std::fs;
use templates::Atome;
use drawatomes::{ActiveAtom, StarField};

fn window_conf() -> Conf {
    Conf {
        window_title: "Atom Sim - Physics Sandbox".to_owned(),
        window_width: 1300,
        window_height: 800,
        window_resizable: true,
        ..Default::default()
    }
}

fn get_atoms() -> Vec<Atome> {
    let fichier_atomes = fs::read_to_string("atomes.json").unwrap_or_else(|_| {
        r#"[
  {
    "nom_atome": "Hydrogène",
    "nb_neutrons": 0,
    "nb_protons": 1,
    "nb_electrons": 1
  },
  {
    "nom_atome": "Hélium",
    "nb_neutrons": 2,
    "nb_protons": 2,
    "nb_electrons": 2
  },
  {
    "nom_atome": "Carbone",
    "nb_neutrons": 6,
    "nb_protons": 6,
    "nb_electrons": 6
  }
]"#.to_string()
    });

    if let Ok(atoms) = serde_json::from_str::<Vec<Atome>>(&fichier_atomes) {
        atoms
    } else if let Ok(atom) = serde_json::from_str::<Atome>(&fichier_atomes) {
        vec![atom]
    } else {
        vec![
            Atome {
                nom_atome: "Hydrogène".to_string(),
                nb_neutrons: 0,
                nb_protons: 1,
                nb_electrons: 1,
            },
            Atome {
                nom_atome: "Hélium".to_string(),
                nb_neutrons: 2,
                nb_protons: 2,
                nb_electrons: 2,
            },
            Atome {
                nom_atome: "Carbone".to_string(),
                nb_neutrons: 6,
                nb_protons: 6,
                nb_electrons: 6,
            },
        ]
    }
}

// GUI helper: Draw custom interactive button
fn draw_button(x: f32, y: f32, w: f32, h: f32, text: &str, is_active: bool) -> bool {
    let mouse_pos = mouse_position();
    let mx = mouse_pos.0;
    let my = mouse_pos.1;
    let hovered = mx >= x && mx <= x + w && my >= y && my <= y + h;

    let bg_color = if is_active {
        Color::from_rgba(0, 140, 140, 255)
    } else if hovered {
        Color::from_rgba(50, 50, 80, 255)
    } else {
        Color::from_rgba(30, 30, 55, 255)
    };

    let border_color = if is_active {
        Color::from_rgba(0, 255, 255, 255)
    } else if hovered {
        WHITE
    } else {
        Color::from_rgba(70, 70, 100, 255)
    };

    draw_rectangle(x, y, w, h, bg_color);
    draw_rectangle_lines(x, y, w, h, 1.5, border_color);

    let text_color = if is_active { WHITE } else { Color::from_rgba(200, 200, 240, 255) };
    let text_size = measure_text(text, None, 14, 1.0);
    draw_text(
        text,
        x + (w - text_size.width) / 2.0,
        y + h / 2.0 + text_size.height / 2.0 - 2.0,
        14.0,
        text_color,
    );

    hovered && is_mouse_button_pressed(MouseButton::Left)
}

// GUI helper: Draw custom interactive toggle switch
fn draw_toggle(x: f32, y: f32, w: f32, h: f32, label: &str, value: bool) -> bool {
    let mouse_pos = mouse_position();
    let mx = mouse_pos.0;
    let my = mouse_pos.1;
    let hovered = mx >= x && mx <= x + w && my >= y && my <= y + h;

    let bg_color = if value {
        Color::from_rgba(46, 204, 113, 220) // Green translucent
    } else {
        Color::from_rgba(30, 30, 55, 255)
    };

    let border_color = if value {
        Color::from_rgba(46, 204, 113, 255)
    } else if hovered {
        WHITE
    } else {
        Color::from_rgba(70, 70, 100, 255)
    };

    draw_rectangle(x, y, w, h, bg_color);
    draw_rectangle_lines(x, y, w, h, 1.5, border_color);

    let text_color = if value { WHITE } else { Color::from_rgba(180, 180, 200, 255) };
    let text_size = measure_text(label, None, 13, 1.0);
    draw_text(
        label,
        x + (w - text_size.width) / 2.0,
        y + h / 2.0 + text_size.height / 2.0 - 2.0,
        13.0,
        text_color,
    );

    hovered && is_mouse_button_pressed(MouseButton::Left)
}

// GUI helper: Draw custom interactive slider
fn draw_slider(x: f32, y: f32, w: f32, h: f32, label: &str, value: &mut f32, min: f32, max: f32) {
    draw_text(label, x, y - 6.0, 14.0, Color::from_rgba(180, 180, 210, 255));

    // Draw track
    draw_rectangle(x, y, w, h, Color::from_rgba(35, 35, 60, 255));
    draw_rectangle_lines(x, y, w, h, 1.0, Color::from_rgba(70, 70, 110, 255));

    let mouse_pos = mouse_position();
    let mx = mouse_pos.0;
    let my = mouse_pos.1;

    let padding_y = 12.0;
    if is_mouse_button_down(MouseButton::Left)
        && mx >= x - 5.0 && mx <= x + w + 5.0
        && my >= y - padding_y && my <= y + h + padding_y
    {
        let pct = ((mx - x) / w).clamp(0.0, 1.0);
        *value = min + pct * (max - min);
    }

    // Draw fill track
    let fill_w = ((*value - min) / (max - min)) * w;
    draw_rectangle(x, y, fill_w, h, Color::from_rgba(0, 180, 180, 255));

    // Draw handle
    let handle_x = x + fill_w;
    let handle_w = 8.0;
    let handle_h = h + 8.0;
    let handle_y = y - 4.0;

    draw_rectangle(handle_x - handle_w / 2.0, handle_y, handle_w, handle_h, WHITE);
    draw_rectangle_lines(handle_x - handle_w / 2.0, handle_y, handle_w, handle_h, 1.0, Color::from_rgba(0, 255, 255, 255));

    let val_str = format!("{:.1}", *value);
    draw_text(&val_str, x + w + 10.0, y + h + 2.0, 13.0, WHITE);
}

fn get_element_symbol(protons: i32) -> &'static str {
    match protons {
        1 => "H",
        2 => "He",
        3 => "Li",
        4 => "Be",
        5 => "B",
        6 => "C",
        7 => "N",
        8 => "O",
        9 => "F",
        10 => "Ne",
        11 => "Na",
        12 => "Mg",
        13 => "Al",
        14 => "Si",
        15 => "P",
        16 => "S",
        17 => "Cl",
        18 => "Ar",
        _ => "??",
    }
}

fn draw_info_card(x: f32, y: f32, w: f32, h: f32, atom: &ActiveAtom) {
    draw_rectangle(x, y, w, h, Color::from_rgba(25, 25, 45, 255));
    draw_rectangle_lines(x, y, w, h, 1.5, Color::from_rgba(0, 180, 180, 255));

    let symbol = get_element_symbol(atom.nb_protons);
    let sym_color = match atom.nb_protons {
        1 => Color::from_rgba(255, 255, 255, 255),
        2 | 10 | 18 => Color::from_rgba(241, 196, 15, 255),
        3 | 11 => Color::from_rgba(230, 126, 34, 255),
        4 | 12 => Color::from_rgba(155, 89, 182, 255),
        5 | 14 => Color::from_rgba(52, 152, 219, 255),
        6 | 7 | 8 => Color::from_rgba(46, 204, 113, 255),
        _ => Color::from_rgba(149, 165, 166, 255),
    };

    draw_text(symbol, x + 15.0, y + 45.0, 36.0, sym_color);
    draw_text(&atom.name, x + 70.0, y + 30.0, 16.0, WHITE);

    let info_type = match atom.nb_protons {
        1 => "Non-métal",
        2 | 10 | 18 => "Gaz noble",
        3 | 11 => "Métal alcalin",
        4 | 12 => "Métal alc-terreux",
        5 | 14 => "Métalloïde",
        6 | 7 | 8 => "Non-métal",
        _ => "Élément",
    };
    draw_text(info_type, x + 70.0, y + 48.0, 12.0, Color::from_rgba(160, 160, 180, 255));

    draw_line(x + 15.0, y + 60.0, x + w - 15.0, y + 60.0, 1.0, Color::from_rgba(60, 60, 80, 255));

    let text_y_start = y + 80.0;
    let row_h = 16.0;

    let mass_num = atom.nb_protons + atom.nb_neutrons;
    let charge = atom.nb_protons - atom.nb_electrons;
    let charge_str = if charge > 0 {
        format!("+{charge}")
    } else if charge < 0 {
        format!("{charge}")
    } else {
        "Neutre".to_string()
    };

    let details = [
        (format!("Protons (Z):  {}", atom.nb_protons), Color::from_rgba(235, 59, 90, 255)),
        (format!("Neutrons (N): {}", atom.nb_neutrons), Color::from_rgba(56, 173, 169, 255)),
        (format!("Électrons:    {}", atom.nb_electrons), Color::from_rgba(46, 204, 113, 255)),
        (format!("Nombre Masse: {}", mass_num), WHITE),
        (format!("Charge:       {}", charge_str), if charge == 0 { WHITE } else if charge > 0 { Color::from_rgba(235, 59, 90, 255) } else { Color::from_rgba(56, 173, 169, 255) }),
    ];

    for (idx, (text, color)) in details.iter().enumerate() {
        draw_text(&text, x + 15.0, text_y_start + (idx as f32) * row_h, 13.0, *color);
    }
}

fn draw_background_grid(min_x: f32, max_x: f32, min_y: f32, max_y: f32) {
    let grid_color = Color::from_rgba(20, 20, 42, 255);
    let step = 60.0;

    let mut start_x = min_x - (min_x % step);
    if start_x < min_x { start_x += step; }
    let mut x = start_x;
    while x <= max_x {
        draw_line(x, min_y, x, max_y, 1.0, grid_color);
        x += step;
    }

    let mut y = min_y;
    while y <= max_y {
        draw_line(min_x, y, max_x, y, 1.0, grid_color);
        y += step;
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);

    let templates = get_atoms();
    let mut active_atoms: Vec<ActiveAtom> = Vec::new();

    // Spawn default setup of atoms
    if !templates.is_empty() {
        active_atoms.push(ActiveAtom::new(&templates[0], 500.0, 300.0, 60.0, -30.0));
    }
    if templates.len() > 1 {
        active_atoms.push(ActiveAtom::new(&templates[1], 850.0, 350.0, -50.0, 60.0));
    }
    if templates.len() > 5 {
        active_atoms.push(ActiveAtom::new(&templates[5], 750.0, 550.0, 20.0, -20.0));
    }

    let mut star_field = StarField::new(80, 320.0, 1300.0, 0.0, 800.0);

    let mut selected_template_idx: Option<usize> = None;
    let mut dragged_atom_idx: Option<usize> = None;
    let mut drag_offset = (0.0, 0.0);

    let mut paused = false;
    let mut collisions_enabled = true;
    let mut gravity_enabled = false;
    let mut show_orbitals = true;
    let mut sim_speed = 1.0;
    let mut gravity_strength = 100.0;

    loop {
        let dt = get_frame_time().min(0.05);
        let screen_w = screen_width();
        let screen_h = screen_height();
        let mouse_pos = mouse_position();
        let mx = mouse_pos.0;
        let my = mouse_pos.1;

        // --- UPDATE ---
        star_field.update(dt, 320.0, screen_w, 0.0, screen_h);

        // Update physics
        if !paused {
            // Update individual atoms
            for (idx, atom) in active_atoms.iter_mut().enumerate() {
                // If it is dragged, we update its velocity but not its position through normal physics
                if Some(idx) != dragged_atom_idx {
                    atom.update(dt * sim_speed, 320.0, screen_w, 0.0, screen_h, 0.15);
                }
            }

            // Gravity/attraction force
            if gravity_enabled {
                drawatomes::apply_gravity(&mut active_atoms, dt * sim_speed, gravity_strength);
            }

            // Elastic collisions
            if collisions_enabled {
                drawatomes::resolve_collisions(&mut active_atoms);
            }
        }

        // Handle dragging
        if let Some(idx) = dragged_atom_idx {
            if idx < active_atoms.len() {
                let prev_x = active_atoms[idx].x;
                let prev_y = active_atoms[idx].y;

                active_atoms[idx].x = mx + drag_offset.0;
                active_atoms[idx].y = my + drag_offset.1;

                if dt > 0.0 {
                    let inst_vx = (active_atoms[idx].x - prev_x) / dt;
                    let inst_vy = (active_atoms[idx].y - prev_y) / dt;
                    active_atoms[idx].vx = active_atoms[idx].vx * 0.4 + inst_vx.clamp(-1500.0, 1500.0) * 0.6;
                    active_atoms[idx].vy = active_atoms[idx].vy * 0.4 + inst_vy.clamp(-1500.0, 1500.0) * 0.6;
                }
            }
        }

        // Hover detection
        let mut hovered_atom_idx = None;
        if mx > 320.0 {
            let mut min_dist = f32::MAX;
            for (idx, atom) in active_atoms.iter().enumerate() {
                let dx = mx - atom.x;
                let dy = my - atom.y;
                let dist = (dx*dx + dy*dy).sqrt();
                let hover_r = atom.outer_orbit_radius().max(25.0);

                if dist < hover_r {
                    if dist < min_dist {
                        min_dist = dist;
                        hovered_atom_idx = Some(idx);
                    }
                }
            }
        }

        // Mouse click inputs
        if is_mouse_button_pressed(MouseButton::Left) && mx > 320.0 {
            if let Some(idx) = hovered_atom_idx {
                // Clicked an atom -> start dragging
                dragged_atom_idx = Some(idx);
                drag_offset = (active_atoms[idx].x - mx, active_atoms[idx].y - my);
            } else if let Some(t_idx) = selected_template_idx {
                // Clicked empty space with spawn tool -> spawn atom
                let new_atom = ActiveAtom::new(
                    &templates[t_idx],
                    mx,
                    my,
                    rand::gen_range(-120.0, 120.0),
                    rand::gen_range(-120.0, 120.0),
                );
                active_atoms.push(new_atom);
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            dragged_atom_idx = None;
        }

        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        // --- DRAW ---
        clear_background(Color::from_rgba(8, 8, 18, 255));

        // Draw deep space playground background
        star_field.draw();
        draw_background_grid(320.0, screen_w, 0.0, screen_h);

        // Draw active atoms
        let t = get_time() as f32;
        for (idx, atom) in active_atoms.iter().enumerate() {
            let is_h = hovered_atom_idx == Some(idx);
            let is_d = dragged_atom_idx == Some(idx);
            atom.draw(t, is_h, is_d, show_orbitals);
        }

        // Draw UI HUD Side Panel (Glassmorphism style)
        draw_rectangle(0.0, 0.0, 320.0, screen_h, Color::from_rgba(15, 15, 27, 245));
        draw_line(320.0, 0.0, 320.0, screen_h, 1.5, Color::from_rgba(50, 50, 75, 255));

        // Title
        draw_text("A T O M   S I M", 20.0, 35.0, 22.0, Color::from_rgba(0, 255, 255, 255));
        draw_text("Multi-Atoms Sandbox", 20.0, 52.0, 12.0, Color::from_rgba(150, 150, 175, 255));
        draw_line(20.0, 65.0, 300.0, 65.0, 1.0, Color::from_rgba(40, 40, 60, 255));

        // Templates grid
        draw_text("GÉNÉRER UN ATOME", 20.0, 85.0, 12.0, Color::from_rgba(0, 255, 255, 200));

        let col_w = 135.0;
        let col_h = 30.0;
        let spacing_y = 35.0;
        let grid_y_start = 100.0;

        for (idx, template) in templates.iter().enumerate() {
            let col = idx % 2;
            let row = idx / 2;
            let tx = 20.0 + (col as f32) * (col_w + 10.0);
            let ty = grid_y_start + (row as f32) * spacing_y;

            let sym = get_element_symbol(template.nb_protons);
            let label = format!("[{}] {}", sym, template.nom_atome);
            let active = selected_template_idx == Some(idx);

            if draw_button(tx, ty, col_w, col_h, &label, active) {
                if active {
                    selected_template_idx = None; // toggle off
                } else {
                    selected_template_idx = Some(idx);
                }
            }
        }

        // Drag/Select Tool button
        let drag_tool_y = 285.0;
        let is_drag_tool_active = selected_template_idx.is_none();
        if draw_button(20.0, drag_tool_y, 280.0, 32.0, "Outil : Sélectionner & Lancer", is_drag_tool_active) {
            selected_template_idx = None;
        }

        draw_line(20.0, 335.0, 300.0, 335.0, 1.0, Color::from_rgba(40, 40, 60, 255));

        // Simulation controls
        draw_text("CONTRÔLES SIMULATION", 20.0, 355.0, 12.0, Color::from_rgba(0, 255, 255, 200));

        // Row 1
        let ctrl_y1 = 370.0;
        if draw_toggle(20.0, ctrl_y1, 135.0, 30.0, if paused { "Reprendre" } else { "Pause" }, paused) {
            paused = !paused;
        }
        if draw_toggle(165.0, ctrl_y1, 135.0, 30.0, "Orbites", show_orbitals) {
            show_orbitals = !show_orbitals;
        }

        // Row 2
        let ctrl_y2 = 410.0;
        if draw_toggle(20.0, ctrl_y2, 135.0, 30.0, "Collisions", collisions_enabled) {
            collisions_enabled = !collisions_enabled;
        }
        if draw_toggle(165.0, ctrl_y2, 135.0, 30.0, "Attraction", gravity_enabled) {
            gravity_enabled = !gravity_enabled;
        }

        // Speed slider
        let slider_y1 = 470.0;
        draw_slider(20.0, slider_y1, 230.0, 6.0, "Vitesse simulation", &mut sim_speed, 0.0, 3.0);

        // Gravity slider
        let slider_y2 = 520.0;
        if gravity_enabled {
            draw_slider(20.0, slider_y2, 230.0, 6.0, "Force Attraction", &mut gravity_strength, 10.0, 400.0);
        }

        // Clear and Reset buttons
        let btn_y = 565.0;
        if draw_button(20.0, btn_y, 135.0, 30.0, "Vider l'écran", false) {
            active_atoms.clear();
        }
        if draw_button(165.0, btn_y, 135.0, 30.0, "Réinitialiser", false) {
            active_atoms.clear();
            if !templates.is_empty() {
                active_atoms.push(ActiveAtom::new(&templates[0], 500.0, 300.0, 60.0, -30.0));
            }
            if templates.len() > 1 {
                active_atoms.push(ActiveAtom::new(&templates[1], 850.0, 350.0, -50.0, 60.0));
            }
            if templates.len() > 5 {
                active_atoms.push(ActiveAtom::new(&templates[5], 750.0, 550.0, 20.0, -20.0));
            }
        }

        draw_line(20.0, 615.0, 300.0, 615.0, 1.0, Color::from_rgba(40, 40, 60, 255));

        // Hovered atom details or instructions
        draw_text("FICHE TECHNIQUE", 20.0, 635.0, 12.0, Color::from_rgba(0, 255, 255, 200));

        if let Some(idx) = hovered_atom_idx {
            if idx < active_atoms.len() {
                draw_info_card(20.0, 650.0, 280.0, 135.0, &active_atoms[idx]);
            }
        } else if let Some(idx) = dragged_atom_idx {
            if idx < active_atoms.len() {
                draw_info_card(20.0, 650.0, 280.0, 135.0, &active_atoms[idx]);
            }
        } else {
            // Draw instruction card
            draw_rectangle(20.0, 650.0, 280.0, 135.0, Color::from_rgba(20, 20, 38, 255));
            draw_rectangle_lines(20.0, 650.0, 280.0, 135.0, 1.0, Color::from_rgba(50, 50, 75, 255));

            draw_text("Aucun atome survolé", 35.0, 680.0, 13.0, Color::from_rgba(150, 150, 175, 255));
            draw_text("Astuces :", 35.0, 705.0, 13.0, Color::from_rgba(0, 200, 200, 255));
            draw_text("- Cliquez & glissez pour lancer un atome.", 35.0, 725.0, 11.0, Color::from_rgba(160, 160, 180, 255));
            draw_text("- Sélectionnez un élément en haut puis", 35.0, 742.0, 11.0, Color::from_rgba(160, 160, 180, 255));
            draw_text("  cliquez dans le vide pour le générer.", 35.0, 757.0, 11.0, Color::from_rgba(160, 160, 180, 255));
        }

        // Draw small status overlay in simulation area
        let status_str = format!("Atomes: {} | FPS: {}", active_atoms.len(), get_fps());
        draw_text(&status_str, 340.0, 25.0, 14.0, Color::from_rgba(100, 100, 120, 255));

        next_frame().await
    }
}
