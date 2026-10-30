// REAL FIXED CODE
// RustSec ID: RUSTSEC-2025-0035 (FIXED)
// Crate: macroquad
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: not-fl3/macroquad
// Commit: ae59b47
// File: src/lib.rs


}
 
     fn window_minimized_event(&mut self) {
        let context = get_context();

         #[cfg(target_os = "android")]
        context.audio_context.pause();

        // Clear held down keys and button and announce them as released
        context.mouse_released.extend(context.mouse_down.drain());
        context.keys_released.extend(context.keys_down.drain());

        // Announce all touches as released
        for (_, touch) in context.touches.iter_mut() {
            touch.phase = input::TouchPhase::Ended;
        }
     }
 
     fn quit_requested_event(&mut self) {