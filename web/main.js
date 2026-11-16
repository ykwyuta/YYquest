const coordinateScale = 2;
const canvas = document.querySelector("#game");
const view = canvas.getContext("2d", { alpha: false });
let image;
const decoder = new TextDecoder();
const status = document.querySelector("#status");
const controls = document.querySelector("#controls");
const stage = document.querySelector(".game-stage");
const touchLayout = document.querySelector("#touch-layout");
const layoutPreference = "yyquest-touch-controls";
const refreshButton = document.querySelector("#cache-refresh");
const mapButton = document.querySelector("#map-control");
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

function setControlLayout(layout) {
  stage.dataset.controls = layout;
  touchLayout.textContent = layout === "outside" ? "操作を画面に戻す" : "操作を画面の外へ";
}

try {
  if (localStorage.getItem(layoutPreference) === "outside") setControlLayout("outside");
} catch {
  // Touch controls still work when browser storage is unavailable.
}

touchLayout.addEventListener("click", () => {
  const layout = stage.dataset.controls === "outside" ? "overlay" : "outside";
  setControlLayout(layout);
  resizeViewport();
  try {
    localStorage.setItem(layoutPreference, layout);
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
      - parseFloat(page.paddingTop) - parseFloat(page.paddingBottom);
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
      wasm.release_direction(code, gamepadSource);
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

function sendKey(key) {
  if (!wasm || refreshButton.disabled) return;
  wasm.key(key, false);
  updateControls();
}

function pressDirection(code, source) {
  if (!wasm || refreshButton.disabled) return;
  wasm.press_direction(code, source);
  updateControls();
}

function clearHeldDirections() {
  touchDirections.clear();
  gamepadButtons = 0;
  wasm?.clear_held_direction();
}

function updateControls() {
  stage.dataset.reading = Boolean(wasm.reading_dialogue?.());
  refreshButton.hidden = !wasm.is_title();
  mapButton.textContent = wasm.debug_back_control() ? "戻る" : "地図";
  mapButton.hidden = !wasm.has_map_control();
  if (refreshButton.hidden) status.hidden = true;
}

try {
  const response = await fetch(wasmUrl(new URL(location.href)));
  if (!response.ok) throw new Error(`Wasm download failed (${response.status})`);
  const { instance } = await WebAssembly.instantiate(await response.arrayBuffer(), { screen });
  wasm = instance.exports;
  wasm.draw();
  mapButton.hidden = !wasm.has_map_control();
  status.hidden = true;
  controls.hidden = false;
  touchLayout.hidden = false;
  resizeViewport();
  previousTrafficFrame = performance.now();
  requestAnimationFrame(animateTraffic);
} catch (error) {
  showStatus("ゲームを読み込めませんでした。「キャッシュを消して再読み込み」をお試しください。");
  console.error(error);
}
refreshButton.hidden = false;

window.addEventListener("keydown", (event) => {
  if (!wasm || event.ctrlKey || event.altKey || event.metaKey || event.isComposing) return;
  if (event.target instanceof HTMLButtonElement && ["Enter", "Space"].includes(event.code)) return;
  const key = keys.get(event.code);
  if (key === undefined) return;
  event.preventDefault();
  if (event.repeat) return;
  if (directions.has(key)) pressDirection(key, keyboardSource);
  else sendKey(key);
});

window.addEventListener("keyup", (event) => {
  const key = keys.get(event.code);
  if (directions.has(key)) wasm?.release_direction(key, keyboardSource);
});

window.addEventListener("blur", clearHeldDirections);
document.addEventListener("visibilitychange", () => {
  if (document.hidden) clearHeldDirections();
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
  wasm?.release_direction(code, 3 + event.pointerId);
}

for (const type of ["pointerup", "pointercancel", "lostpointercapture"]) {
  dpad.addEventListener(type, releasePointer);
}
dpad.addEventListener("contextmenu", (event) => event.preventDefault());

controls.addEventListener("click", (event) => {
  const button = event.target.closest("button[data-key]");
  if (!button) return;
  const key = Number(button.dataset.key);
  if (!directions.has(key) || event.detail === 0) sendKey(key);
});
