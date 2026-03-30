use std::error::Error;
use std::fs;
use std::path::Path;

use serde::Deserialize;
use txtplot::ChartContext;

#[derive(Debug, Deserialize)]
struct FlowHistory {
    weekly: Vec<WeeklyStats>,
}

#[derive(Debug, Deserialize)]
struct WeeklyStats {
    week_start: String,
    stories_done: u32,
    voyages_done: u32,
}

/// Generate a terminal visualization of project progress.
pub fn render_stats() -> Result<String, Box<dyn Error>> {
    let history_path = Path::new(".keel/flow_history.json");
    if !history_path.exists() {
        return Ok("No flow history found in .keel/flow_history.json".to_string());
    }

    let content = fs::read_to_string(history_path)?;
    let history: FlowHistory = serde_json::from_str(&content)?;

    // Chronological order: from oldest to newest.
    let weekly_data: Vec<&WeeklyStats> = history.weekly.iter().rev().collect();

    let mut cumulative_stories = 0.0;
    let stories_points: Vec<(f64, f64)> = weekly_data
        .iter()
        .enumerate()
        .map(|(i, w)| {
            cumulative_stories += w.stories_done as f64;
            (i as f64, cumulative_stories)
        })
        .collect();

    let mut cumulative_voyages = 0.0;
    let voyages_points: Vec<(f64, f64)> = weekly_data
        .iter()
        .enumerate()
        .map(|(i, w)| {
            cumulative_voyages += w.voyages_done as f64;
            (i as f64, cumulative_voyages)
        })
        .collect();

    if stories_points.is_empty() {
        return Ok("No weekly stats available to plot.".to_string());
    }

    // Setup chart with braille renderer.
    let mut chart = ChartContext::new(60, 15);

    // Draw cumulative stories as a line.
    chart.line_chart(&stories_points, None);
    
    // Overlay cumulative voyages as a line too.
    chart.line_chart(&voyages_points, None);

    // Add labels.
    let mut output = String::new();
    output.push_str("atxt project progress: cumulative stories and voyages\n");
    output.push_str(&chart.canvas.render_no_color());
    output.push_str("\n");
    
    // Add legend for the x-axis (weeks).
    if let Some(first) = weekly_data.first() {
        if let Some(last) = weekly_data.last() {
             output.push_str(&format!("{} -> {} ({} weeks)\n", first.week_start, last.week_start, weekly_data.len()));
        }
    }

    // Add summary stats.
    output.push_str(&format!("\nTotal: {} stories, {} voyages completed.\n", cumulative_stories, cumulative_voyages));

    Ok(output)
}
