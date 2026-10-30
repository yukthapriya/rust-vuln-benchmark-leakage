// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0396 (FIXED)
// Crate: conrod_core
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: PistonDevelopers/conrod
// Commit: 956421f
// File: conrod_core/src/mesh.rs


let (w, h) = rect.w_h();
             let left = (rect.left() * dpi_factor + half_viewport_w).round() as i32;
             let top = (rect.top() * dpi_factor - half_viewport_h).round().abs() as i32;
            let width = ((w * dpi_factor).round() as u32).min(viewport_w as u32);
            let height = ((h * dpi_factor).round() as u32).min(viewport_h as u32);
            if width == 0 || height == 0 {
                None
            } else {
                Some(Scizzor {
                    top_left: [left.max(0), top.max(0)],
                    dimensions: [width, height],
                })
             }
         };
 
 
                 // Update the scizzor and produce a command.
                 current_scizzor = new_scizzor;
                if let Some(scizzor) = new_scizzor {
                    commands.push(PreparedCommand::Scizzor(scizzor));
                }
 
                 // Set the state back to plain drawing.
                 current_state = State::Plain {
                     start: vertices.len(),
                 };
             }
 
            // If the scizzor is `None`, then nothing is viewable, so don't draw anything.
            if current_scizzor.is_none() {
                continue;
            }

             match kind {
                 render::PrimitiveKind::Rectangle { color } => {
                     switch_to_plain_state!();