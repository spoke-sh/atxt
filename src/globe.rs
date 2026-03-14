use std::error::Error;
use txtplot::canvas::BrailleCanvas;
use txtplot::three_d::{Vec3, ZBuffer, project_to_screen, rotate_x, rotate_y, plot_z, line_z};
use colored::Color;

/// Render a 3D prototype of the Drift Globe.
pub fn render_drift_globe(angle_x: f64, angle_y: f64) -> Result<String, Box<dyn Error>> {
    let width_cells = 60;
    let height_cells = 30;
    let mut canvas = BrailleCanvas::new(width_cells, height_cells);
    let mut zbuf = ZBuffer::from_canvas(&canvas);

    let radius = 1.2;
    let camera_dist = 4.0;
    let scale = 80.0;

    // 1. Draw the Globe Perimeter (The Atmosphere)
    draw_circle(&mut canvas, &mut zbuf, radius, camera_dist, scale, Color::BrightBlack);

    // 2. Render Latitude/Longitude Grid with Torchlight
    render_globe_grid(&mut canvas, &mut zbuf, radius, angle_x, angle_y, camera_dist, scale);

    // 3. Render Drift Markers (Boats)
    // Client Boat: Actual Board (Intent)
    let client_lat = 0.4;
    let client_lon = 0.5;
    let client_pos = spherical_to_cartesian(client_lat, client_lon, radius);
    render_marker(&mut canvas, &mut zbuf, client_pos, angle_x, angle_y, camera_dist, scale, Color::Green);

    // Model Boat: Predicted Reality (Source)
    let model_lat = -0.2;
    let model_lon = -0.3;
    let model_pos = spherical_to_cartesian(model_lat, model_lon, radius);
    render_marker(&mut canvas, &mut zbuf, model_pos, angle_x, angle_y, camera_dist, scale, Color::Red);

    // 4. Calculate Drift Metrics
    let drift_vec = client_pos - model_pos;
    let drift_dist = drift_vec.norm();

    let mut output = String::new();
    output.push_str("\x1b[1matext Drift Globe Prototype\x1b[0m\n");
    output.push_str(&canvas.render_with_options(true, None));
    output.push_str(&format!(
        "\nDrift Magnitude: \x1b[1;33m{:.4}\x1b[0m (keel units)\n",
        drift_dist
    ));
    output.push_str("Status: \x1b[32m⛴ Intent\x1b[0m vs \x1b[31m⛵ Reality\x1b[0m\n");
    output.push_str("Use arrow keys to rotate (simulated static view)\n");

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

    // Latitudes
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

    // Longitudes
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
        // Draw a distinct cross marker for the boat
        for dx in -2..=2 {
            plot_z(canvas, zbuf, x + dx, y, z - 0.1, color);
        }
        for dy in -2..=2 {
            plot_z(canvas, zbuf, x, y + dy, z - 0.1, color);
        }
    }
}
