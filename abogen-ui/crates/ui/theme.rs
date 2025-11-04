/// Global cyberpunk theme CSS styles
pub fn get_theme_css() -> &'static str {
    r#"
@import url('https://fonts.googleapis.com/css2?family=Rajdhani:wght@300;400;500;600;700&display=swap');

:root {
  --bg: #070A12;
  --panel-bg: #0A0F1A;
  --text: #D7E1FF;
  --muted: #8BA1C7;
  --neon-cyan: #27E8FF;
  --neon-magenta: #FF3AD4;
  --neon-amber: #FFB300;
  --neon-lime: #39FF14;
  --error: #FF3355;
  --ok: #55FFCC;
  --radius: 14px;
  --glow: 0 0 12px rgba(39,232,255,.45);
}

* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body {
  font-family: 'Rajdhani', sans-serif;
  background: radial-gradient(1200px 800px at 20% 0%, #0C1222 0%, #05070B 60%), #05070B;
  color: var(--text);
  overflow-x: hidden;
}

.panel {
  background: var(--panel-bg);
  border: 1px solid rgba(255,58,212,.35);
  box-shadow: 0 0 0 1px rgba(255,58,212,.2) inset, 0 0 24px rgba(255,58,212,.25);
  border-radius: var(--radius);
  padding: 20px;
}

.dashed {
  border: 2px dashed var(--neon-magenta);
  border-radius: 18px;
}

.neon-btn {
  text-transform: uppercase;
  letter-spacing: .5px;
  padding: 12px 18px;
  border-radius: 12px;
  border: 1px solid rgba(255,58,212,.6);
  background: linear-gradient(180deg, #1A1022, #0E0B16);
  box-shadow: 0 0 12px rgba(255,58,212,.35), inset 0 0 10px rgba(255,58,212,.2);
  transition: transform .06s ease, box-shadow .2s;
  color: var(--text);
  cursor: pointer;
  font-family: 'Rajdhani', sans-serif;
  font-weight: 600;
  font-size: 14px;
  outline: none;
}

.neon-btn:hover {
  transform: translateY(-1px);
  box-shadow: 0 0 16px rgba(255,58,212,.45), inset 0 0 12px rgba(255,58,212,.25);
}

.neon-btn:active {
  transform: translateY(0);
}

.neon-btn:focus-visible {
  outline: 2px solid var(--neon-magenta);
  outline-offset: 2px;
}

.neon-btn.primary {
  border-color: var(--neon-cyan);
  box-shadow: 0 0 14px rgba(39,232,255,.55), inset 0 0 10px rgba(39,232,255,.25);
}

.neon-btn.primary:hover {
  box-shadow: 0 0 18px rgba(39,232,255,.65), inset 0 0 12px rgba(39,232,255,.3);
}

.neon-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none;
}

.slider-track {
  height: 4px;
  background: linear-gradient(90deg, #FFB300, #27E8FF);
  border-radius: 999px;
}

.progress {
  height: 22px;
  border-radius: 14px;
  background: rgba(255,255,255,.05);
  border: 1px solid rgba(39,232,255,.25);
  position: relative;
  overflow: hidden;
}

.progress > .bar {
  position: absolute;
  top: 0;
  left: 0;
  height: 100%;
  background: linear-gradient(90deg, #39FF14, #FFB300);
  box-shadow: 0 0 18px rgba(57,255,20,.45);
  transition: width 0.2s ease;
}

.container {
  max-width: 1280px;
  width: 100%;
  margin: 0 auto;
  padding: 20px;
}

@media (max-width: 768px) {
  .container {
    padding: 12px;
  }
  
  .neon-btn {
    padding: 10px 16px;
    font-size: 12px;
  }
}

.log-entry {
  font-family: 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.6;
  padding: 4px 0;
}

.log-entry.info {
  color: var(--neon-cyan);
}

.log-entry.notice {
  color: var(--neon-amber);
}

.log-entry.error {
  color: var(--error);
}

.combo {
  position: relative;
}

.combo-input {
  width: 100%;
  padding: 10px 14px;
  background: rgba(10, 15, 26, 0.8);
  border: 1px solid rgba(255,58,212,.4);
  border-radius: 10px;
  color: var(--text);
  font-family: 'Rajdhani', sans-serif;
  outline: none;
  transition: all 0.2s;
}

.combo-input:focus {
  border-color: var(--neon-magenta);
  box-shadow: 0 0 10px rgba(255,58,212,.3);
}

.checkbox-wrapper {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
}

.checkbox-input {
  width: 20px;
  height: 20px;
  accent-color: var(--neon-cyan);
  cursor: pointer;
}

.select {
  padding: 10px 14px;
  background: rgba(10, 15, 26, 0.8);
  border: 1px solid rgba(255,58,212,.4);
  border-radius: 10px;
  color: var(--text);
  font-family: 'Rajdhani', sans-serif;
  outline: none;
  cursor: pointer;
  transition: all 0.2s;
}

.select:focus {
  border-color: var(--neon-magenta);
  box-shadow: 0 0 10px rgba(255,58,212,.3);
}

/* Teleprompter Styles */
.teleprompter-container {
  position: relative;
  width: 100%;
  height: 400px;
  background: rgba(10, 15, 26, 0.9);
  border: 1px solid rgba(39,232,255,.35);
  border-radius: 14px;
  box-shadow: 0 0 20px rgba(39,232,255,.25);
  overflow: hidden;
  margin: 20px 0;
}

.teleprompter-text {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  padding: 30px;
  color: var(--text);
  font-family: 'Courier New', monospace;
  font-size: 24px;
  line-height: 1.5;
  text-align: center;
  transform: translateY(0px);
  transition: transform 0.1s linear;
}

.current-line-highlight {
  position: absolute;
  top: 50%;
  left: 0;
  width: 100%;
  height: 30px;
  background: rgba(39,232,255,.1);
  border-top: 1px solid rgba(39,232,255,.3);
  border-bottom: 1px solid rgba(39,232,255,.3);
  transform: translateY(-50%);
  pointer-events: none;
}

.teleprompter-controls {
  position: absolute;
  bottom: 20px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  gap: 10px;
  z-index: 10;
}

.mirror-indicator {
  position: absolute;
  top: 10px;
  right: 10px;
  background: rgba(39,232,255,.2);
  border: 1px solid rgba(39,232,255,.5);
  border-radius: 8px;
  padding: 4px 8px;
  font-size: 12px;
  color: var(--neon-cyan);
  z-index: 10;
}

.gesture-control-panel {
  background: rgba(10, 15, 26, 0.8);
  border: 1px solid rgba(255,58,212,.35);
  border-radius: 14px;
  padding: 20px;
  margin: 20px 0;
  box-shadow: 0 0 15px rgba(255,58,212,.2);
}

@keyframes pulse {
  0% { opacity: 1; }
  50% { opacity: 0.5; }
  100% { opacity: 1; }
}
"#
}

