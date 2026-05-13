Async:

ui.asyncs.spawn(async { ... }, |result, ui| {}) — spawns an async task, calls closure with result when done
ui.asyncs.timer(duration, |ui| {}) — spawns a timer that calls closure after duration seconds
