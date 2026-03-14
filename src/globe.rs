use std::error::Error;
use std::fs;
use std::path::Path;
use txtplot::canvas::BrailleCanvas;
use txtplot::three_d::{Vec3, ZBuffer, project_to_screen, rotate_x, rotate_y, plot_z, line_z};
use colored::Color;

/// State used to calculate and render project drift and navigation POIs.
#[derive(Debug, Default)]
pub struct ProjectDrift {
    pub client_lat: f64,
    pub client_lon: f64,
    pub model_lat: f64,
    pub model_lon: f64,
    pub total_stories: usize,
    pub done_stories: usize,
    pub verified_stories: usize,
    pub destinations: Vec<Destination>,
    pub lighthouse: Option<Destination>,
}

#[derive(Debug, Clone)]
pub struct Destination {
    pub id: String,
    pub title: String,
    pub lat: f64,
    pub lon: f64,
    pub completed: bool,
}

/// Probe the .keel board and test results to calculate current project drift.
pub fn probe_project_drift() -> ProjectDrift {
    let mut drift = ProjectDrift::default();

    // 1. Identify the Active Mission and its associated Epic
    let mut active_mission_id = String::new();
    let missions_path = Path::new(".keel/missions");
    if let Ok(entries) = fs::read_dir(missions_path) {
        let mut latest_time = String::new();
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let readme_path = entry.path().join("README.md");
                if let Ok(content) = fs::read_to_string(readme_path) {
                    if content.contains("status: active") || content.contains("status: achieved") {
                        let id = entry.file_name().to_string_lossy().to_string();
                        let updated = content.lines()
                            .find(|l| l.starts_with("updated_at: "))
                            .map(|l| l[12..].to_string())
                            .unwrap_or_default();
                        
                        if updated >= latest_time {
                            latest_time = updated;
                            active_mission_id = id;
                        }
                    }
                }
            }
        }
    }

    // 2. Count Stories and Verification
    let stories_path = Path::new(".keel/stories");
    if let Ok(entries) = fs::read_dir(stories_path) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                drift.total_stories += 1;
                let readme_path = entry.path().join("README.md");
                if let Ok(content) = fs::read_to_string(readme_path) {
                    if content.lines().any(|l| l.trim() == "status: done") {
                        drift.done_stories += 1;
                        if entry.path().join("manifest.yaml").exists() {
                            drift.verified_stories += 1;
                        }
                    }
                }
            }
        }
    }

    if drift.total_stories > 0 {
        let completion = drift.done_stories as f64 / drift.total_stories as f64;
        drift.client_lat = (completion - 0.5) * std::f64::consts::PI;
        drift.client_lon = 0.0;

        if drift.done_stories > 0 {
            let verification_rate = drift.verified_stories as f64 / drift.done_stories as f64;
            drift.model_lat = (completion * verification_rate - 0.5) * std::f64::consts::PI;
            drift.model_lon = (1.0 - verification_rate) * (std::f64::consts::PI / 4.0);
        }
    }

    // 3. Map Epics to Destinations
    let epics_path = Path::new(".keel/epics");
    if let Ok(entries) = fs::read_dir(epics_path) {
        for (i, entry) in entries.flatten().enumerate() {
            if entry.path().is_dir() {
                let id = entry.file_name().to_string_lossy().to_string();
                let readme_path = entry.path().join("README.md");
                if let Ok(content) = fs::read_to_string(readme_path) {
                    let title = content.lines()
                        .find(|l| l.starts_with("# "))
                        .map(|l| l[2..].to_string())
                        .unwrap_or_else(|| id.clone());
                    
                    let completed = content.contains("(done)");
                    
                    // Spread locations
                    let lat = ((i as f64 * 1.7).sin()) * (std::f64::consts::PI / 2.5);
                    let lon = ((i as f64 * 2.9).cos()) * std::f64::consts::PI;
                    
                    let dest = Destination { id, title, lat, lon, completed };
                    
                    if !active_mission_id.is_empty() && content.contains(&format!("mission: {}", active_mission_id)) {
                        drift.lighthouse = Some(dest.clone());
                    } else {
                        drift.destinations.push(dest);
                    }
                }
            }
        }
    }

    drift
}

/// Render a 3D Navigation Chart with dynamic sizing and clipping.
pub fn render_drift_globe(
    angle_x: f64, 
    angle_y: f64, 
    drift: &ProjectDrift,
    width_cells: u16,
    height_cells: u16,
) -> Result<String, Box<dyn Error>> {
    let mut canvas = BrailleCanvas::new(width_cells as usize, height_cells as usize);
    let mut zbuf = ZBuffer::from_canvas(&canvas);

    let radius = 1.2;
    let camera_dist = 4.5;
    
    // Scale globe to fit the smaller terminal dimension
    let scale = (width_cells.min(height_cells * 2) as f64 * 1.5).min(150.0);

    render_axis(&mut canvas, &mut zbuf, radius, angle_x, angle_y, camera_dist, scale);
    draw_circle(&mut canvas, &mut zbuf, radius * 1.05, camera_dist, scale, Color::Blue);
    render_terrain_and_grid(&mut canvas, &mut zbuf, radius, angle_x, angle_y, camera_dist, scale);

    for dest in &drift.destinations {
        let pos = spherical_to_cartesian(dest.lat, dest.lon, radius);
        let color = if dest.completed { Color::Cyan } else { Color::BrightBlack };
        render_poi(&mut canvas, &mut zbuf, pos, angle_x, angle_y, camera_dist, scale, color);
    }

    if let Some(ref lh) = drift.lighthouse {
        let pos = spherical_to_cartesian(lh.lat, lh.lon, radius);
        render_lighthouse(&mut canvas, &mut zbuf, pos, angle_x, angle_y, camera_dist, scale);
    }

    let client_pos = spherical_to_cartesian(drift.client_lat, drift.client_lon, radius);
    let model_pos = spherical_to_cartesian(drift.model_lat, drift.model_lon, radius);
    let is_docked = (client_pos - model_pos).norm() < 0.05;

    if is_docked {
        render_marker(&mut canvas, &mut zbuf, client_pos, angle_x, angle_y, camera_dist, scale, Color::Yellow);
    } else {
        render_marker(&mut canvas, &mut zbuf, client_pos, angle_x, angle_y, camera_dist, scale, Color::Green);
        render_marker(&mut canvas, &mut zbuf, model_pos, angle_x, angle_y, camera_dist, scale, Color::Red);
        
        let c_rot = rotate_x(rotate_y(client_pos, angle_y), angle_x);
        let m_rot = rotate_x(rotate_y(model_pos, angle_y), angle_x);
        if c_rot.z < 0.0 && m_rot.z < 0.0 {
             if let (Some(s1), Some(s2)) = (
                 project_and_clip(&mut canvas, Vec3::new(c_rot.x, c_rot.y, c_rot.z + camera_dist), scale),
                 project_and_clip(&mut canvas, Vec3::new(m_rot.x, m_rot.y, m_rot.z + camera_dist), scale)
             ) {
                 line_z(&mut canvas, &mut zbuf, s1, s2, Color::Yellow);
             }
        }
    }

    let mut output = String::new();
    output.push_str("\x1b[1matext Navigation Chart\x1b[0m\n");
    output.push_str(&canvas.render_with_options(true, None));
    
    let drift_vec = client_pos - model_pos;
    let drift_dist = drift_vec.norm();
    output.push_str(&format!(
        "\nDrift: \x1b[1;{}m{:.4}\x1b[0m | ",
        if drift_dist < 0.1 { "32" } else if drift_dist < 0.5 { "33" } else { "31" },
        drift_dist
    ));
    
    if is_docked {
        output.push_str("\x1b[1;33mDOCKING ACHIEVED\x1b[0m\n");
    } else {
        output.push_str("\x1b[32m⛴ Intent\x1b[0m -> \x1b[31m⛵ Reality\x1b[0m\n");
    }

    if let Some(ref lh) = drift.lighthouse {
        output.push_str(&format!("\x1b[1;37m🔦 Lighthouse (Current Mission): {}\x1b[0m\n", lh.title));
    }

    Ok(output)
}

fn spherical_to_cartesian(lat: f64, lon: f64, r: f64) -> Vec3 {
    let x = r * lat.cos() * lon.sin();
    let y = r * lat.sin();
    let z = r * lat.cos() * lon.cos();
    Vec3::new(x, y, z)
}

fn is_land(lat: f64, lon: f64) -> bool {
    let val = (lat * 6.0).sin() * (lon * 4.0).cos() + (lat * 3.0).cos() * (lon * 8.0).sin();
    val > 0.7
}

fn project_and_clip(canvas: &BrailleCanvas, v: Vec3, scale: f64) -> Option<(isize, isize, f64)> {
    if let Some((x, y, z)) = project_to_screen(v, canvas.pixel_width() as f64, canvas.pixel_height() as f64, scale) {
        if x >= 0 && x < canvas.pixel_width() as isize && y >= 0 && y < canvas.pixel_height() as isize {
            return Some((x, y, z));
        }
    }
    None
}

fn plot_clipped(canvas: &mut BrailleCanvas, zbuf: &mut ZBuffer, x: isize, y: isize, z: f64, color: Color) {
    if x >= 0 && x < canvas.pixel_width() as isize && y >= 0 && y < canvas.pixel_height() as isize {
        plot_z(canvas, zbuf, x, y, z, color);
    }
}

fn render_axis(
    canvas: &mut BrailleCanvas,
    zbuf: &mut ZBuffer,
    r: f64,
    angle_x: f64,
    angle_y: f64,
    camera_dist: f64,
    scale: f64,
) {
    let p1 = Vec3::new(0.0, r * 1.3, 0.0);
    let r1 = rotate_x(rotate_y(p1, angle_y), angle_x);
    let v1 = Vec3::new(r1.x, r1.y, r1.z + camera_dist);
    if let Some(s1) = project_and_clip(canvas, v1, scale) {
        plot_clipped(canvas, zbuf, s1.0, s1.1, s1.2 - 0.1, Color::White);
    }
}

fn draw_circle(
    canvas: &mut BrailleCanvas,
    zbuf: &mut ZBuffer,
    r: f64,
    camera_dist: f64,
    scale: f64,
    color: Color,
) {
    let steps = 128;
    let mut prev: Option<(isize, isize, f64)> = None;
    for i in 0..=steps {
        let angle = (i as f64) * std::f64::consts::TAU / steps as f64;
        let p = Vec3::new(r * angle.cos(), r * angle.sin(), 0.0);
        let v_cam = Vec3::new(p.x, p.y, p.z + camera_dist);
        if let Some(curr) = project_and_clip(canvas, v_cam, scale) {
            if let Some(p) = prev {
                line_z(canvas, zbuf, p, curr, color);
            }
            prev = Some(curr);
        } else {
            prev = None;
        }
    }
}

fn render_terrain_and_grid(
    canvas: &mut BrailleCanvas,
    zbuf: &mut ZBuffer,
    r: f64,
    angle_x: f64,
    angle_y: f64,
    camera_dist: f64,
    scale: f64,
) {
    let lat_steps = 30;
    let lon_steps = 60;

    for i in 0..lat_steps {
        let lat = (i as f64 / (lat_steps - 1) as f64 - 0.5) * std::f64::consts::PI;
        for j in 0..lon_steps {
            let lon = (j as f64 / lon_steps as f64 - 0.5) * std::f64::consts::TAU;
            let p = spherical_to_cartesian(lat, lon, r);
            let rotated = rotate_x(rotate_y(p, angle_y), angle_x);
            
            if rotated.z < 0.0 {
                let v_cam = Vec3::new(rotated.x, rotated.y, rotated.z + camera_dist);
                if let Some((x, y, z)) = project_and_clip(canvas, v_cam, scale) {
                    if is_land(lat, lon) {
                        plot_clipped(canvas, zbuf, x, y, z, Color::BrightBlack);
                    } else if i % 6 == 0 || j % 6 == 0 {
                        plot_clipped(canvas, zbuf, x, y, z, Color::Black);
                    }
                }
            }
        }
    }
}

fn render_poi(
    canvas: &mut BrailleCanvas,
    zbuf: &mut ZBuffer,
    pos: Vec3,
    angle_x: f64,
    angle_y: f64,
    camera_dist: f64,
    scale: f64,
    color: Color,
) {
    let rotated = rotate_x(rotate_y(pos, angle_y), angle_x);
    if rotated.z < 0.0 {
        let v_cam = Vec3::new(rotated.x, rotated.y, rotated.z + camera_dist);
        if let Some((x, y, z)) = project_and_clip(canvas, v_cam, scale) {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    plot_clipped(canvas, zbuf, x + dx, y + dy, z - 0.1, color);
                }
            }
        }
    }
}

fn render_lighthouse(
    canvas: &mut BrailleCanvas,
    zbuf: &mut ZBuffer,
    pos: Vec3,
    angle_x: f64,
    angle_y: f64,
    camera_dist: f64,
    scale: f64,
) {
    let rotated = rotate_x(rotate_y(pos, angle_y), angle_x);
    if rotated.z < 0.0 {
        let v_cam = Vec3::new(rotated.x, rotated.y, rotated.z + camera_dist);
        if let Some((x, y, z)) = project_and_clip(canvas, v_cam, scale) {
            for dy in -6..=0 {
                plot_clipped(canvas, zbuf, x, y + dy, z - 0.15, Color::White);
            }
            for dx in 1..15 {
                plot_clipped(canvas, zbuf, x + dx, y - 6, z - 0.15, Color::Yellow);
            }
        }
    }
}

fn render_marker(
    canvas: &mut BrailleCanvas,
    zbuf: &mut ZBuffer,
    pos: Vec3,
    angle_x: f64,
    angle_y: f64,
    camera_dist: f64,
    scale: f64,
    color: Color,
) {
    let rotated = rotate_x(rotate_y(pos, angle_y), angle_x);
    if rotated.z < 0.0 {
        let v_cam = Vec3::new(rotated.x, rotated.y, rotated.z + camera_dist);
        if let Some((x, y, z)) = project_and_clip(canvas, v_cam, scale) {
            for dx in -2..=2 {
                plot_clipped(canvas, zbuf, x + dx, y, z - 0.2, color);
            }
            for dy in -4..=0 {
                plot_clipped(canvas, zbuf, x, y + dy, z - 0.2, color);
            }
            for i in 1..=3 {
                plot_clipped(canvas, zbuf, x + i, y - 4 + i, z - 0.2, color);
            }
        }
    }
}
