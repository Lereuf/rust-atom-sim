use macroquad::prelude::*;
use crate::templates::Atome;

pub struct Star {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub size: f32,
    pub brightness: f32,
}

pub struct StarField {
    pub stars: Vec<Star>,
}

impl StarField {
    pub fn new(count: usize, min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> Self {
        let mut stars = Vec::new();
        for _ in 0..count {
            stars.push(Star {
                x: rand::gen_range(min_x, max_x),
                y: rand::gen_range(min_y, max_y),
                speed: rand::gen_range(5.0, 25.0),
                size: rand::gen_range(0.5, 2.5),
                brightness: rand::gen_range(0.3, 1.0),
            });
        }
        Self { stars }
    }

    pub fn update(&mut self, dt: f32, min_x: f32, max_x: f32, min_y: f32, max_y: f32) {
        for star in &mut self.stars {
            // Drift left
            star.x -= star.speed * dt;
            if star.x < min_x {
                star.x = max_x;
                star.y = rand::gen_range(min_y, max_y);
            }
        }
    }

    pub fn draw(&self) {
        for star in &self.stars {
            let color = Color::from_rgba(
                255,
                255,
                255,
                (star.brightness * 255.0) as u8,
            );
            draw_circle(star.x, star.y, star.size, color);
        }
    }
}

pub struct ActiveAtom {
    pub name: String,
    pub nb_protons: i32,
    pub nb_neutrons: i32,
    pub nb_electrons: i32,
    pub nucleons: Vec<(f32, f32, i32)>, // (x_offset, y_offset, type) where type 0 = proton, 1 = neutron
    pub nucleon_size: f32,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub mass: f32,
}

impl ActiveAtom {
    pub fn new(template: &Atome, x: f32, y: f32, vx: f32, vy: f32) -> Self {
        let total_nucleons = template.nb_protons + template.nb_neutrons;
        let base_size = 40.0;
        let nucleon_size = base_size / (total_nucleons as f32).max(1.0).sqrt();
        let nucleons = gen_core_nucleons(template.nb_protons, template.nb_neutrons);
        let mass = (total_nucleons as f32).max(1.0);

        Self {
            name: template.nom_atome.clone(),
            nb_protons: template.nb_protons,
            nb_neutrons: template.nb_neutrons,
            nb_electrons: template.nb_electrons,
            nucleons,
            nucleon_size,
            x,
            y,
            vx,
            vy,
            mass,
        }
    }

    // Get the maximum radius of the nucleus
    pub fn nucleus_radius(&self) -> f32 {
        let max_dist = self.nucleons.iter()
            .map(|&(nx, ny, _)| (nx*nx + ny*ny).sqrt())
            .fold(0.0f32, |max, d| max.max(d));
        (max_dist * self.nucleon_size) + (self.nucleon_size / 2.0)
    }

    // Get the radius of the outermost electron shell
    pub fn outer_orbit_radius(&self) -> f32 {
        let n_nucl = self.nucleus_radius();
        let mut max_r = n_nucl;

        let mut electrons = self.nb_electrons;
        if electrons > 0 {
            max_r += 30.0; // Shell 1
            electrons -= electrons.min(2);
        }
        if electrons > 0 {
            max_r += 30.0; // Shell 2
            electrons -= electrons.min(8);
        }
        if electrons > 0 {
            max_r += 30.0; // Shell 3
            electrons -= electrons.min(18);
        }
        if electrons > 0 {
            max_r += 30.0; // Shell 4
        }
        max_r
    }

    // Update position and bounce off boundaries
    pub fn update(&mut self, dt: f32, min_x: f32, max_x: f32, min_y: f32, max_y: f32, friction: f32) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        // Apply friction
        self.vx *= 1.0 - friction * dt;
        self.vy *= 1.0 - friction * dt;

        // Edge bounce
        let r = self.outer_orbit_radius();

        if self.x - r < min_x {
            self.x = min_x + r;
            self.vx = self.vx.abs() * 0.8;
        } else if self.x + r > max_x {
            self.x = max_x - r;
            self.vx = -self.vx.abs() * 0.8;
        }

        if self.y - r < min_y {
            self.y = min_y + r;
            self.vy = self.vy.abs() * 0.8;
        } else if self.y + r > max_y {
            self.y = max_y - r;
            self.vy = -self.vy.abs() * 0.8;
        }
    }

    // Draw the atom
    pub fn draw(&self, t: f32, is_hovered: bool, is_dragged: bool, show_orbitals: bool) {
        let nucl_r = self.nucleon_size / 2.0;

        // Glow effect if hovered/dragged
        if is_dragged {
            draw_circle(self.x, self.y, self.outer_orbit_radius() + 8.0, Color::from_rgba(0, 255, 255, 35));
        } else if is_hovered {
            draw_circle(self.x, self.y, self.outer_orbit_radius() + 5.0, Color::from_rgba(255, 255, 255, 25));
        }

        // Draw nucleus (protons and neutrons)
        for &(ox, oy, wth_ncl) in &self.nucleons {
            let px = self.x + ox * self.nucleon_size;
            let py = self.y + oy * self.nucleon_size;

            let color = match wth_ncl {
                0 => Color::from_rgba(235, 59, 90, 255),  // Beautiful Coral Red
                1 => Color::from_rgba(56, 173, 169, 255), // Beautiful Cyan/Teal
                _ => GRAY,
            };
            draw_glossy_circle(px, py, nucl_r, color);
        }

        // Draw electron shells
        if show_orbitals && self.nb_electrons > 0 {
            let mut rem_electrons = self.nb_electrons;
            let n_nucl = self.nucleus_radius();

            // Shell configurations: (max_electrons, offset_radius, speed_multiplier, clockwise)
            let shells = [
                (2, 30.0, 1.5, true),
                (8, 60.0, 1.0, false),
                (18, 90.0, 0.7, true),
                (999, 120.0, 0.5, false),
            ];

            for &(max_e, offset, speed_mult, clockwise) in &shells {
                if rem_electrons <= 0 { break; }

                let shell_e = rem_electrons.min(max_e);
                rem_electrons -= shell_e;

                let orbit_r = n_nucl + offset;

                // Draw orbit path line
                let path_color = if is_hovered {
                    Color::from_rgba(0, 255, 255, 80)
                } else {
                    Color::from_rgba(100, 220, 255, 30)
                };
                draw_circle_lines(self.x, self.y, orbit_r, 1.0, path_color);

                // Draw electrons
                let dir = if clockwise { 1.0 } else { -1.0 };
                let speed = 2.0 * speed_mult * dir;
                let e_size = (nucl_r / 2.0).max(4.0);

                for i in 0..shell_e {
                    let angle = (i as f32 * std::f32::consts::PI * 2.0 / shell_e as f32) + t * speed;
                    let ex = self.x + orbit_r * angle.cos();
                    let ey = self.y + orbit_r * angle.sin();

                    // Glow circle
                    draw_circle(ex, ey, e_size + 2.0, Color::from_rgba(46, 204, 113, 80));
                    // Solid glossy circle
                    draw_glossy_circle(ex, ey, e_size, Color::from_rgba(46, 204, 113, 255));
                }
            }
        }
    }
}

// 3D glossy circle helper
fn draw_glossy_circle(cx: f32, cy: f32, r: f32, base_color: Color) {
    draw_circle(cx, cy, r, base_color);
    draw_circle_lines(cx, cy, r, 1.0, Color::from_rgba(0, 0, 0, 160));
    // Gloss highlight (top-left)
    draw_circle(cx - r * 0.3, cy - r * 0.3, r * 0.25, Color::from_rgba(255, 255, 255, 140));
}

// Spirale de Fermat pour agencer les nucléons de manière dense
pub fn gen_core_nucleons(mut protons: i32, mut neutrons: i32) -> Vec<(f32, f32, i32)> {
    let mut nucleons = Vec::new();
    let total = protons + neutrons;
    let golden_angle = std::f32::consts::PI * (3.0 - 5.0_f32.sqrt());

    for i in 0..total {
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

        let radius = 0.45 * (i as f32).sqrt();
        let theta = i as f32 * golden_angle;

        let offset_x = radius * theta.cos();
        let offset_y = radius * theta.sin();

        nucleons.push((offset_x, offset_y, wth_ncl));
    }
    nucleons
}

// Gère le rebond élastique 2D entre atomes en respectant la conservation de l'énergie et des masses
pub fn resolve_collisions(atoms: &mut [ActiveAtom]) {
    let len = atoms.len();
    for i in 0..len {
        for j in (i + 1)..len {
            let dx = atoms[j].x - atoms[i].x;
            let dy = atoms[j].y - atoms[i].y;
            let dist = (dx*dx + dy*dy).sqrt();

            let r_i = atoms[i].outer_orbit_radius();
            let r_j = atoms[j].outer_orbit_radius();
            // Permet un léger chevauchement d'orbites avant de collisionner (0.85)
            let min_dist = (r_i + r_j) * 0.85;

            if dist < min_dist && dist > 0.0 {
                let overlap = min_dist - dist;

                // Vecteur normal
                let nx = dx / dist;
                let ny = dy / dist;

                // 1. Résolution de la pénétration (séparation physique)
                let separation_x = nx * overlap * 0.5;
                let separation_y = ny * overlap * 0.5;

                atoms[i].x -= separation_x;
                atoms[i].y -= separation_y;
                atoms[j].x += separation_x;
                atoms[j].y += separation_y;

                // 2. Résolution des vitesses (collision élastique)
                let m_i = atoms[i].mass;
                let m_j = atoms[j].mass;

                // Vitesse relative
                let rvx = atoms[j].vx - atoms[i].vx;
                let rvy = atoms[j].vy - atoms[i].vy;

                // Vitesse le long de la normale
                let vel_along_normal = rvx * nx + rvy * ny;

                // Ne collisionner que s'ils se rapprochent
                if vel_along_normal < 0.0 {
                    let e = 0.85; // Coefficient de restitution (1.0 = élastique parfait)
                    let impulse_scalar = -(1.0 + e) * vel_along_normal / (1.0 / m_i + 1.0 / m_j);

                    let impulse_x = impulse_scalar * nx;
                    let impulse_y = impulse_scalar * ny;

                    atoms[i].vx -= impulse_x / m_i;
                    atoms[i].vy -= impulse_y / m_i;
                    atoms[j].vx += impulse_x / m_j;
                    atoms[j].vy += impulse_y / m_j;
                }
            }
        }
    }
}

// Applique une force d'attraction mutuelle de type gravitationnel
pub fn apply_gravity(atoms: &mut [ActiveAtom], dt: f32, g_constant: f32) {
    let len = atoms.len();
    for i in 0..len {
        for j in 0..len {
            if i == j { continue; }
            let dx = atoms[j].x - atoms[i].x;
            let dy = atoms[j].y - atoms[i].y;
            let dist_sq = dx*dx + dy*dy;
            let dist = dist_sq.sqrt();

            if dist > 40.0 { // Évite les accélérations infinies à très courte portée
                // Accélération = G * m_j / d^2
                let force_magnitude = g_constant * atoms[j].mass / dist_sq;
                let ax = (dx / dist) * force_magnitude;
                let ay = (dy / dist) * force_magnitude;

                atoms[i].vx += ax * dt;
                atoms[i].vy += ay * dt;
            }
        }
    }
}
