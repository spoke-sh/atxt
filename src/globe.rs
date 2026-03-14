use std::error::Error;
use std::fs;
use std::path::Path;
use txtplot::canvas::BrailleCanvas;
use txtplot::three_d::{Vec3, ZBuffer, project_to_screen, rotate_x, rotate_y, plot_z, line_z};
use colored::Color;

/// State used to calculate and render project drift.
#[derive(Debug, Default)]
pub struct ProjectDrift {
    pub client_lat: f64,
    pub client_lon: f64,
    pub model_lat: f64,
    pub model_lon: f64,
}

/// Probe the .keel board and test results to calculate current project drift.
pub fn probe_project_drift() -> ProjectDrift {
    let mut drift = ProjectDrift::default();

    // 1. Calculate Client Positioning (Intent) from Keel Board
    let stories_path = Path::new(".keel/stories");
    if let Ok(entries) = fs::read_dir(stories_path) {
        let mut total_stories = 0;
        let mut done_stories = 0;

        for entry in entries.flatten() {
            if entry.path().is_dir() {
                total_stories += 1;
                let readme_path = entry.path().join("README.md");
                if let Ok(content) = fs::read_to_string(readme_path) {
                    if content.contains("status: done") {
                        done_stories += 1;
                    }
                }
            }
        }

        if total_stories > 0 {
            // Latitude maps to story completion (0% = South Pole, 100% = North Pole)
            let completion = done_stories as f64 / total_stories as f64;
            drift.client_lat = (completion - 0.5) * std::f64::consts::PI;
            // Longitude maps to relative progress (placeholder for now)
            drift.client_lon = 0.0; 
        }
    }

    // 2. Calculate Model Positioning (Reality) from Verification Manifests
    // We'll use the ratio of stories with passing verification manifests.
    if let Ok(entries) = fs::read_dir(stories_path) {
        let mut total_done = 0;
        let mut verified_done = 0;

        for entry in entries.flatten() {
            let readme_path = entry.path().join("README.md");
            if let Ok(content) = fs::read_to_string(readme_path) {
                if content.contains("status: done") {
                    total_done += 1;
                    if entry.path().join("manifest.yaml").exists() {
                        verified_done += 1;
                    }
                }
            }
        }

        if total_done > 0 {
            let verification_rate = verified_done as f64 / total_done as f64;
            drift.model_lat = drift.client_lat * verification_rate;
            // Introduce a subtle longitude shift based on drift
            drift.model_lon = (1.0 - verification_rate) * std::f64::consts::PI / 2.0;
        }
    }

    drift
}

/// Render a 3D prototype of the Drift Globe.
pub fn render_drift_globe(angle_x: f64, angle_y: f64, drift: &ProjectDrift) -> Result<String, Box<dyn Error>> {
    let width_cells = 60;
    let height_cells = 30;
    let mut canvas = BrailleCanvas::new(width_cells, height_cells);
    let mut zbuf = ZBuffer::from_canvas(&canvas);

    let radius = 1.2;
    let camera_dist = 4.0;
    let scale = 80.0;

    // 1. Draw the Globe Perimeter
    draw_circle(&mut canvas, &mut zbuf, radius, camera_dist, scale, Color::BrightBlack);

    // 2. Render Latitude/Longitude Grid
    render_globe_grid(&mut canvas, &mut zbuf, radius, angle_x, angle_y, camera_dist, scale);

    // 3. Render Drift Markers (Boats)
    let client_pos = spherical_to_cartesian(drift.client_lat, drift.client_lon, radius);
    render_marker(&mut canvas, &mut zbuf, client_pos, angle_x, angle_y, camera_dist, scale, Color::Green);

    let model_pos = spherical_to_cartesian(drift.model_lat, drift.model_lon, radius);
    render_marker(&mut canvas, &mut zbuf, model_pos, angle_x, angle_y, camera_dist, scale, Color::Red);

    // 4. Calculate Drift Metrics
    let drift_vec = client_pos - model_pos;
    let drift_dist = drift_vec.norm();

    let mut output = String::new();
    output.push_str("\x1b[1mDrift Globe\x1b[0m\n");
    output.push_str(&canvas.render_with_options(true, None));
    output.push_str(&format!(
        "\nDrift Magnitude: \x1b[1;{}m{:.4}\x1b[0m (keel units)\n",
        if drift_dist < 0.3 { "32" } else if drift_dist < 0.6 { "33" } else { "31" },
        drift_dist
    ));
    output.push_str("Status: \x1b[32m⛴ Intent\x1b[0m vs \x1b[31m⛵ Reality\x1b[0m\n");

    Ok(output)
}

fn spherical_to_cartesian(lat: f64, lon: f64, r: f64) -> Vec3 {
    let x = r * lat.cos() * lon.sin();
    let y = r * lat.sin();
    let z = r * lat.cos() * lon.cos();
    Vec3::new(x, y, z)
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
        if let Some(curr) = project_to_screen(v_cam, canvas.pixel_width() as f64, canvas.pixel_height() as f64, scale) {
            if let Some(p) = prev {
                line_z(canvas, zbuf, p, curr, color);
            }
            prev = Some(curr);
        }
    }
}

fn render_globe_grid(
    canvas: &mut BrailleCanvas,
    zbuf: &mut ZBuffer,
    r: f64,
    angle_x: f64,
    angle_y: f64,
    camera_dist: f64,
    scale: f64,
) {
    let steps = 32;

    for lat_idx in -3..=3 {
        let lat = (lat_idx as f64) * std::f64::consts::PI / 8.0;
        let mut prev: Option<(isize, isize, f64)> = None;
        for lon_idx in 0..=steps {
            let lon = (lon_idx as f64) * std::f64::consts::TAU / steps as f64;
            let p = spherical_to_cartesian(lat, lon, r);
            let rotated = rotate_x(rotate_y(p, angle_y), angle_x);
            let v_cam = Vec3::new(rotated.x, rotated.y, rotated.z + camera_dist);
            
            if rotated.z < 0.0 {
                if let Some(curr) = project_to_screen(v_cam, canvas.pixel_width() as f64, canvas.pixel_height() as f64, scale) {
                    if let Some(p) = prev {
                        let color = if lat_idx == 0 { Color::Cyan } else { Color::BrightBlack };
                        line_z(canvas, zbuf, p, curr, color);
                    }
                    prev = Some(curr);
                } else {
                    prev = None;
                }
            } else {
                prev = None;
            }
        }
    }

    for lon_idx in -4..4 {
        let lon = (lon_idx as f64) * std::f64::consts::PI / 4.0;
        let mut prev: Option<(isize, isize, f64)> = None;
        for lat_idx in -steps/2..=steps/2 {
            let lat = (lat_idx as f64) * std::f64::consts::PI / steps as f64;
            let p = spherical_to_cartesian(lat, lon, r);
            let rotated = rotate_x(rotate_y(p, angle_y), angle_x);
            let v_cam = Vec3::new(rotated.x, rotated.y, rotated.z + camera_dist);
            
            if rotated.z < 0.0 {
                if let Some(curr) = project_to_screen(v_cam, canvas.pixel_width() as f64, canvas.pixel_height() as f64, scale) {
                    if let Some(p) = prev {
                        let color = if lon_idx == 0 { Color::Yellow } else { Color::BrightBlack };
                        line_z(canvas, zbuf, p, curr, color);
                    }
                    prev = Some(curr);
                } else {
                    prev = None;
                }
            } else {
                prev = None;
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
    let v_cam = Vec3::new(rotated.x, rotated.y, rotated.z + camera_dist);

    if let Some((x, y, z)) = project_to_screen(
        v_cam,
        canvas.pixel_width() as f64,
        canvas.pixel_height() as f64,
        scale,
    ) {
        for dx in -2..=2 {
            plot_z(canvas, zbuf, x + dx, y, z - 0.1, color);
        }
        for dy in -2..=2 {
            plot_z(canvas, zbuf, x, y + dy, z - 0.1, color);
        }
    }
}
