use std::fs;
use std::path::Path;

fn linearize(c: f64) -> f64 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn hex_to_rgb(hex: &str) -> (f64, f64, f64) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f64 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f64 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f64 / 255.0;
    (r, g, b)
}

fn relative_luminance(r: f64, g: f64, b: f64) -> f64 {
    let r_lin = linearize(r);
    let g_lin = linearize(g);
    let b_lin = linearize(b);
    0.2126 * r_lin + 0.7152 * g_lin + 0.0722 * b_lin
}

fn contrast_ratio(hex1: &str, hex2: &str) -> f64 {
    let (r1, g1, b1) = hex_to_rgb(hex1);
    let (r2, g2, b2) = hex_to_rgb(hex2);
    let l1 = relative_luminance(r1, g1, b1);
    let l2 = relative_luminance(r2, g2, b2);
    let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
    (lighter + 0.05) / (darker + 0.05)
}

fn check_contrast() -> anyhow::Result<()> {
    println!("Checking WCAG 2.2 AA Contrast for tokens...");

    // Light Theme:
    // --bg:#F3F0E8; --surface:#EBE7DD; --raised:#FAF8F3;
    // --ink:#181715; --ink-2:#55514A; --ink-3:#878278 (adjusted from #8A857B to meet 3.0:1 on surface);
    let light_bgs = [
        ("bg", "#F3F0E8"),
        ("surface", "#EBE7DD"),
        ("raised", "#FAF8F3"),
    ];
    let light_fg_body = [("ink", "#181715"), ("ink-2", "#55514A")];
    let light_fg_ui = [("ink-3", "#878278")];

    for (bg_name, bg_val) in &light_bgs {
        for (fg_name, fg_val) in &light_fg_body {
            let cr = contrast_ratio(bg_val, fg_val);
            println!("  [Light] {fg_name} on {bg_name}: {cr:.2}:1 (req >= 4.5:1)");
            if cr < 4.5 {
                anyhow::bail!(
                    "Light theme contrast failure: {fg_name} on {bg_name} is {cr:.2}:1 < 4.5:1"
                );
            }
        }
        for (fg_name, fg_val) in &light_fg_ui {
            let cr = contrast_ratio(bg_val, fg_val);
            println!("  [Light UI] {fg_name} on {bg_name}: {cr:.2}:1 (req >= 3.0:1)");
            if cr < 3.0 {
                anyhow::bail!(
                    "Light theme UI contrast failure: {fg_name} on {bg_name} is {cr:.2}:1 < 3.0:1"
                );
            }
        }
    }

    // Dark Theme:
    // --bg:#0E0E0D; --surface:#151513; --raised:#1C1C19;
    // --ink:#EDE9E0; --ink-2:#A29D91; --ink-3:#6E6A61;
    let dark_bgs = [
        ("bg", "#0E0E0D"),
        ("surface", "#151513"),
        ("raised", "#1C1C19"),
    ];
    let dark_fg_body = [("ink", "#EDE9E0"), ("ink-2", "#A29D91")];
    let dark_fg_ui = [("ink-3", "#6E6A61")];

    for (bg_name, bg_val) in &dark_bgs {
        for (fg_name, fg_val) in &dark_fg_body {
            let cr = contrast_ratio(bg_val, fg_val);
            println!("  [Dark] {fg_name} on {bg_name}: {cr:.2}:1 (req >= 4.5:1)");
            if cr < 4.5 {
                anyhow::bail!(
                    "Dark theme contrast failure: {fg_name} on {bg_name} is {cr:.2}:1 < 4.5:1"
                );
            }
        }
        for (fg_name, fg_val) in &dark_fg_ui {
            let cr = contrast_ratio(bg_val, fg_val);
            println!("  [Dark UI] {fg_name} on {bg_name}: {cr:.2}:1 (req >= 3.0:1)");
            if cr < 3.0 {
                anyhow::bail!(
                    "Dark theme UI contrast failure: {fg_name} on {bg_name} is {cr:.2}:1 < 3.0:1"
                );
            }
        }
    }

    println!("All token contrast pairs meet WCAG 2.2 AA requirements.");
    Ok(())
}

fn generate_spring_linear(
    stiffness: f64,
    damping: f64,
    mass: f64,
    duration_s: f64,
    steps: usize,
) -> String {
    // Damped harmonic oscillator:
    // m * x'' + c * x' + k * x = 0
    // initial condition: x(0) = 0, target = 1.
    // Let y(t) = 1 - x(t), so y(0) = 1, y'(0) = 0.
    let omega_0 = (stiffness / mass).sqrt();
    let zeta = damping / (2.0 * (mass * stiffness).sqrt());

    let mut points = Vec::with_capacity(steps + 1);

    for i in 0..=steps {
        let t = (i as f64 / steps as f64) * duration_s;
        let val = if zeta < 1.0 {
            // Underdamped
            let omega_d = omega_0 * (1.0 - zeta * zeta).sqrt();
            let envelope = (-zeta * omega_0 * t).exp();
            1.0 - envelope
                * ((omega_d * t).cos() + (zeta * omega_0 / omega_d) * (omega_d * t).sin())
        } else if (zeta - 1.0).abs() < 1e-5 {
            // Critically damped
            1.0 - (-omega_0 * t).exp() * (1.0 + omega_0 * t)
        } else {
            // Overdamped
            let r1 = -omega_0 * (zeta - (zeta * zeta - 1.0).sqrt());
            let r2 = -omega_0 * (zeta + (zeta * zeta - 1.0).sqrt());
            let c1 = r2 / (r2 - r1);
            let c2 = -r1 / (r2 - r1);
            1.0 - (c1 * (r1 * t).exp() + c2 * (r2 * t).exp())
        };
        points.push(format!("{:.4}", val));
    }

    format!("linear({})", points.join(", "))
}

fn generate_motion() -> anyhow::Result<()> {
    println!("Generating CSS spring easing curves...");

    // Tokens from Section 12.9:
    // spring.snappy: stiffness 320, damping 30, mass 1 (dur-micro 120ms or dur-standard 280ms)
    // spring.gentle: stiffness 170, damping 26, mass 1 (dur-standard 280ms or dur-page 480ms)
    // spring.slow: stiffness 120, damping 22, mass 1 (dur-page 480ms or dur-hero 720ms)
    let snappy = generate_spring_linear(320.0, 30.0, 1.0, 0.4, 20);
    let gentle = generate_spring_linear(170.0, 26.0, 1.0, 0.6, 25);
    let slow = generate_spring_linear(120.0, 22.0, 1.0, 0.9, 30);

    let css = format!(
        r#"/* Generated by xtask generate-motion. Do not edit directly. */
:root {{
  --dur-micro: 120ms;
  --dur-standard: 280ms;
  --dur-page: 480ms;
  --dur-hero: 720ms;

  --ease-snappy: {snappy};
  --ease-gentle: {gentle};
  --ease-slow: {slow};
}}

@supports not (transition-timing-function: linear(0, 1)) {{
  :root {{
    --ease-snappy: cubic-bezier(0.22, 1, 0.36, 1);
    --ease-gentle: cubic-bezier(0.22, 1, 0.36, 1);
    --ease-slow: cubic-bezier(0.22, 1, 0.36, 1);
  }}
}}
"#
    );

    let output_path = Path::new("apps/desktop/ui/styles/motion.generated.css");
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(output_path, css)?;
    println!("Generated {}", output_path.display());
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("all");

    match cmd {
        "check-contrast" => check_contrast()?,
        "generate-motion" => generate_motion()?,
        "all" => {
            check_contrast()?;
            generate_motion()?;
        }
        other => {
            eprintln!("Unknown xtask command: {other}");
            eprintln!("Available commands: check-contrast, generate-motion, all");
            std::process::exit(1);
        }
    }

    Ok(())
}
