const coordinateScale = 2;
const canvas = document.querySelector("#game");
const view = canvas.getContext("2d", { alpha: false });
let image;
const decoder = new TextDecoder();
const encoder = new TextEncoder();
const status = document.querySelector("#status");
const controls = document.querySelector("#controls");
const stage = document.querySelector(".game-stage");
const touchLayout = document.querySelector("#touch-layout");
const layoutPreference = "yyquest-touch-controls";
const saveKey = "yyquest-adventure";
let lastSaved;
let saveFailed = false;
const desktopControls = matchMedia("(hover: hover) and (pointer: fine) and (min-width: 701px)");
const refreshButton = document.querySelector("#cache-refresh");
const mapButton = document.querySelector("#map-control");
const soundButton = document.querySelector("#sound-toggle");
const soundPreference = "yyquest-sound";
let audio;
let soundEnabled = true;
const refreshParam = "_yyrefresh";
const keys = new Map([
  ["ArrowUp", 38], ["ArrowDown", 40], ["ArrowLeft", 37], ["ArrowRight", 39],
  ["KeyW", 87], ["KeyS", 83], ["KeyA", 65], ["KeyD", 68],
  ["Enter", 13], ["Space", 13], ["KeyZ", 13], ["KeyJ", 74], ["KeyM", 77], ["Escape", 27], ["KeyX", 27],
]);
let wasm;
let previousTrafficFrame = 0;
let gamepadButtons = 0;
let readGamepad = typeof navigator.getGamepads === "function";
const keyboardSource = 1;
const gamepadSource = 2;
const touchDirections = new Map();
const directions = new Set([37, 38, 39, 40, 65, 68, 83, 87]);
let preferredLayout;

function setControlLayout(layout) {
  stage.dataset.controls = layout;
  touchLayout.textContent = layout === "outside" ? "操作を画面に戻す" : "操作を画面の外へ";
}

try {
  const saved = localStorage.getItem(layoutPreference);
  if (saved === "outside" || saved === "overlay") preferredLayout = saved;
  soundEnabled = localStorage.getItem(soundPreference) !== "off";
} catch {
  // Touch controls still work when browser storage is unavailable.
}

soundButton.setAttribute("aria-pressed", soundEnabled);
soundButton.addEventListener("click", () => {
  soundEnabled = !soundEnabled;
  soundButton.setAttribute("aria-pressed", soundEnabled);
  if (soundEnabled) unlockSound();
  try {
    localStorage.setItem(soundPreference, soundEnabled ? "on" : "off");
  } catch {
    // Keep the selection for the current visit.
  }
});

function unlockSound() {
  if (!soundEnabled || !window.AudioContext) return;
  try {
    audio ??= new AudioContext();
  } catch {
    return;
  }
  if (audio.state === "suspended") audio.resume().catch(() => {});
}

const soundNotes = [[], [587], [196, 147], [523, 659, 784], [392, 523, 659, 784]];

function playSound() {
  const cue = wasm.take_sound();
  if (!cue || !soundEnabled || audio?.state !== "running") return;
  const now = audio.currentTime;
  for (const [index, hz] of soundNotes[cue].entries()) {
    const at = now + index * 0.065;
    const oscillator = audio.createOscillator();
    const volume = audio.createGain();
    oscillator.type = "triangle";
    oscillator.frequency.value = hz;
    volume.gain.setValueAtTime(0.065, at);
    volume.gain.exponentialRampToValueAtTime(0.001, at + 0.085);
    oscillator.connect(volume).connect(audio.destination);
    oscillator.start(at);
    oscillator.stop(at + 0.09);
  }
}

function applyControlLayout() {
  setControlLayout(preferredLayout ?? (desktopControls.matches ? "outside" : "overlay"));
  resizeViewport();
}

applyControlLayout();
desktopControls.addEventListener("change", applyControlLayout);

touchLayout.addEventListener("click", () => {
  preferredLayout = stage.dataset.controls === "outside" ? "overlay" : "outside";
  applyControlLayout();
  try {
    localStorage.setItem(layoutPreference, preferredLayout);
  } catch {
    // Keep the selection for the current visit.
  }
});

function resizeViewport() {
  if (!wasm?.set_view_width) return;
  let width = 320;
  if (matchMedia("(orientation: landscape)").matches) {
    const page = getComputedStyle(stage.parentElement);
    const height = (window.visualViewport?.height ?? window.innerHeight)
      - parseFloat(page.paddingTop) - parseFloat(page.paddingBottom)
      - (parseFloat(page.getPropertyValue("--footer-height")) || 0);
    let available = stage.clientWidth;
    if (stage.dataset.controls === "outside" && getComputedStyle(controls).display === "contents") {
      available -= stage.querySelector(".directions").offsetWidth + stage.querySelector(".actions").offsetWidth;
      available -= 2 * (parseFloat(getComputedStyle(stage).columnGap) || 0);
    }
    if (height > 0) width = Math.floor(240 * available / height);
  }
  wasm.set_view_width(Math.max(320, width));
}

window.addEventListener("resize", resizeViewport);
window.visualViewport?.addEventListener("resize", resizeViewport);

function animateTraffic(now) {
  pollGamepad();
  const elapsed = Math.floor(now - previousTrafficFrame);
  if (elapsed >= 16) {
    previousTrafficFrame = now;
    if (wasm.animate_traffic(elapsed)) updateControls();
  }
  requestAnimationFrame(animateTraffic);
}

function pollGamepad() {
  if (!readGamepad || document.hidden || !document.hasFocus()) return;
  let pad;
  try {
    pad = Array.from(navigator.getGamepads()).find((device) => device?.mapping === "standard");
  } catch {
    readGamepad = false;
  }
  // Standard Gamepad mapping uses buttons 12–15 for the directional pad.
  const buttons = [[12, 38], [13, 40], [14, 37], [15, 39], [1, 27], [0, 13]];
  let held = 0;
  for (const [index, code] of buttons) {
    const wasPressed = Boolean(gamepadButtons & (1 << index));
    const pressed = Boolean(pad?.buttons[index]?.pressed);
    if (pressed) held |= 1 << index;
    if (pressed && !wasPressed) {
      if (directions.has(code)) pressDirection(code, gamepadSource);
      else sendKey(code);
    } else if (!pressed && wasPressed && directions.has(code)) {
      wasm.release_direction?.(code, gamepadSource);
    }
  }
  gamepadButtons = held;
}

const screen = {
  bitmap(address, width, height) {
    if (!image || image.width !== width || image.height !== height) {
      canvas.width = width;
      canvas.height = height;
      stage.style.setProperty("--canvas-ratio", width / height);
      stage.style.setProperty("--canvas-limit", width > 640 ? "100%" : "840px");
      image = view.createImageData(width, height);
    }
    const pixels = new Uint32Array(wasm.memory.buffer, address, width * height);
    for (let i = 0; i < pixels.length; i++) {
      const rgb = pixels[i];
      image.data[i * 4] = rgb >> 16;
      image.data[i * 4 + 1] = rgb >> 8 & 255;
      image.data[i * 4 + 2] = rgb & 255;
      image.data[i * 4 + 3] = 255;
    }
    view.putImageData(image, 0, 0);
  },
  text(x, y, size, color, address, length, serif) {
    const bytes = new Uint8Array(wasm.memory.buffer, address, length);
    const face = serif
      ? '"Yu Mincho", "Hiragino Mincho ProN", "Noto Serif CJK JP", serif'
      : '"Yu Gothic UI", "Hiragino Kaku Gothic ProN", sans-serif';
    view.font = `600 ${size * coordinateScale}px ${face}`;
    view.fillStyle = `#${color.toString(16).padStart(6, "0")}`;
    view.textBaseline = "top";
    view.fillText(decoder.decode(bytes), x * coordinateScale, y * coordinateScale);
  },
};

function wasmUrl(page) {
  const url = new URL("./yyquest.wasm", import.meta.url);
  const token = page.searchParams.get(refreshParam);
  if (token) url.searchParams.set(refreshParam, token);
  return url;
}

function showStatus(message) {
  status.textContent = message;
  status.hidden = false;
}

refreshButton.addEventListener("click", async () => {
  clearHeldDirections();
  refreshButton.disabled = true;
  showStatus("ゲームファイルを取得し直しています…");

  try {
    const next = new URL(location.href);
    next.searchParams.set(refreshParam, Date.now().toString(36));
    const files = [
      next,
      new URL("./index.html", import.meta.url),
      new URL("./main.js", import.meta.url),
      new URL("./game.css", import.meta.url),
      wasmUrl(next),
    ];
    if ("caches" in window) {
      try {
        const paths = new Set([...files, new URL("./", import.meta.url)]
          .map(({ origin, pathname }) => origin + pathname));
        for (const name of await caches.keys()) {
          const cache = await caches.open(name);
          for (const request of await cache.keys()) {
            const { origin, pathname } = new URL(request.url);
            if (paths.has(origin + pathname)) await cache.delete(request);
          }
        }
      } catch (error) {
        console.warn("ゲームのキャッシュを開けませんでした。", error);
      }
    }

    await Promise.all(files.map(async (url) => {
      const response = await fetch(url, { cache: "reload" });
      if (!response.ok) throw new Error(`Reload failed (${response.status}): ${url.pathname}`);
      await response.arrayBuffer();
    }));
    location.replace(next.href);
  } catch (error) {
    refreshButton.disabled = false;
    showStatus("再読み込みできませんでした。通信状態を確認して、もう一度お試しください。");
    console.warn(error);
  }
});

function sendKey(key, repeat = false) {
  if (!wasm || refreshButton.disabled) return;
  if (key === 13) saveAdventure();
  wasm.key(key, repeat);
  playSound();
  updateControls();
  if (!directions.has(key)) saveAdventure();
}

function loadAdventure() {
  try {
    const saved = localStorage.getItem(saveKey);
    if (!saved) return;
    const bytes = encoder.encode(saved);
    const address = wasm.prepare_save_load(bytes.length);
    if (address) {
      new Uint8Array(wasm.memory.buffer, address, bytes.length).set(bytes);
      if (wasm.offer_continue()) {
        lastSaved = saved;
        return;
      }
    }
    showStatus("冒険の記録を読み込めませんでした。");
  } catch (error) {
    showStatus("ブラウザの保存領域を開けませんでした。");
    console.warn(error);
  }
}

function saveAdventure() {
  if (!wasm?.checkpoint_size) return;
  const size = wasm.checkpoint_size();
  if (!size) return;
  const saved = decoder.decode(new Uint8Array(wasm.memory.buffer, wasm.checkpoint_ptr(), size));
  if (saved === lastSaved) return;
  try {
    localStorage.setItem(saveKey, saved);
    lastSaved = saved;
    if (saveFailed) status.hidden = true;
    saveFailed = false;
  } catch (error) {
    if (!saveFailed) console.warn(error);
    saveFailed = true;
    showStatus("冒険の記録を保存できませんでした。");
  }
}

function pressDirection(code, source) {
  if (!wasm || refreshButton.disabled) return;
  if (!wasm.press_direction) {
    sendKey(code);
    return;
  }
  wasm.press_direction(code, source);
  updateControls();
}

function clearHeldDirections() {
  touchDirections.clear();
  gamepadButtons = 0;
  wasm?.clear_held_direction?.();
}

function updateControls() {
  stage.dataset.reading = Boolean(wasm.reading_dialogue?.());
  const refreshWasHidden = refreshButton.hidden;
  refreshButton.hidden = !wasm.is_title();
  mapButton.textContent = wasm.debug_back_control() ? "戻る" : "地図";
  mapButton.hidden = !wasm.has_map_control();
  if (refreshButton.hidden && !saveFailed) status.hidden = true;
  if (refreshButton.hidden !== refreshWasHidden) resizeViewport();
}

try {
  const response = await fetch(wasmUrl(new URL(location.href)));
  if (!response.ok) throw new Error(`Wasm download failed (${response.status})`);
  const { instance } = await WebAssembly.instantiate(await response.arrayBuffer(), { screen });
  wasm = instance.exports;
  status.hidden = true;
  loadAdventure();
  wasm.draw();
  mapButton.hidden = !wasm.has_map_control();
  controls.hidden = false;
  soundButton.hidden = !window.AudioContext;
  touchLayout.hidden = false;
  refreshButton.hidden = !wasm.is_title();
  resizeViewport();
  previousTrafficFrame = performance.now();
  requestAnimationFrame(animateTraffic);
} catch (error) {
  refreshButton.hidden = false;
  showStatus("ゲームを読み込めませんでした。「キャッシュを消して再読み込み」をお試しください。");
  console.error(error);
}

window.addEventListener("keydown", (event) => {
  if (!wasm || event.ctrlKey || event.altKey || event.metaKey || event.isComposing) return;
  if (event.target instanceof HTMLButtonElement && ["Enter", "Space"].includes(event.code)) return;
  const key = keys.get(event.code) ?? keys.get(event.key);
  if (key === undefined) return;
  unlockSound();
  event.preventDefault();
  if (event.repeat) {
    if (!wasm.press_direction && directions.has(key)) sendKey(key, true);
    return;
  }
  if (directions.has(key)) pressDirection(key, keyboardSource);
  else sendKey(key);
});

window.addEventListener("keyup", (event) => {
  const key = keys.get(event.code) ?? keys.get(event.key);
  if (directions.has(key)) wasm?.release_direction?.(key, keyboardSource);
});

window.addEventListener("blur", clearHeldDirections);
window.addEventListener("pagehide", saveAdventure);
document.addEventListener("visibilitychange", () => {
  if (document.hidden) {
    clearHeldDirections();
    saveAdventure();
  }
});

const dpad = controls.querySelector(".directions");
dpad.addEventListener("pointerdown", (event) => {
  const button = event.target.closest("button[data-key]");
  if (!button || event.button !== 0 || !wasm || refreshButton.disabled) return;
  event.preventDefault();
  button.setPointerCapture(event.pointerId);
  const code = Number(button.dataset.key);
  touchDirections.set(event.pointerId, code);
  pressDirection(code, 3 + event.pointerId);
});

function releasePointer(event) {
  const code = touchDirections.get(event.pointerId);
  if (code === undefined) return;
  touchDirections.delete(event.pointerId);
  wasm?.release_direction?.(code, 3 + event.pointerId);
}

for (const type of ["pointerup", "pointercancel", "lostpointercapture"]) {
  dpad.addEventListener(type, releasePointer);
}
dpad.addEventListener("contextmenu", (event) => event.preventDefault());

controls.addEventListener("click", (event) => {
  const button = event.target.closest("button[data-key]");
  if (!button) return;
  unlockSound();
  const key = Number(button.dataset.key);
  if (!directions.has(key) || event.detail === 0) sendKey(key);
});
