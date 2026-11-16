const width = 320;
const height = 240;
const scale = 2;
const canvas = document.querySelector("#game");
const view = canvas.getContext("2d", { alpha: false });
const buffer = Object.assign(document.createElement("canvas"), { width, height }).getContext("2d", { alpha: false });
const image = buffer.createImageData(width, height);
const decoder = new TextDecoder();
const status = document.querySelector("#status");
const controls = document.querySelector("#controls");
const refreshButton = document.querySelector("#cache-refresh");
const mapButton = document.querySelector("#map-control");
const refreshParam = "_yyrefresh";
const keys = new Map([
  ["ArrowUp", 38], ["ArrowDown", 40], ["ArrowLeft", 37], ["ArrowRight", 39],
  ["KeyW", 38], ["KeyS", 40], ["KeyA", 37], ["KeyD", 39],
  ["Enter", 13], ["Space", 13], ["KeyZ", 13], ["KeyJ", 74], ["KeyM", 77], ["Escape", 27], ["KeyX", 27],
]);
let wasm;
let previousTrafficFrame = 0;

function animateTraffic(now) {
  const elapsed = Math.floor(now - previousTrafficFrame);
  if (elapsed >= 65) {
    previousTrafficFrame = now;
    wasm.animate_traffic(elapsed);
  }
  requestAnimationFrame(animateTraffic);
}

const screen = {
  bitmap(address) {
    const pixels = new Uint32Array(wasm.memory.buffer, address, width * height);
    for (let i = 0; i < pixels.length; i++) {
      const rgb = pixels[i];
      image.data[i * 4] = rgb >> 16;
      image.data[i * 4 + 1] = rgb >> 8 & 255;
      image.data[i * 4 + 2] = rgb & 255;
      image.data[i * 4 + 3] = 255;
    }
    buffer.putImageData(image, 0, 0);
    view.imageSmoothingEnabled = false;
    view.drawImage(buffer.canvas, 0, 0, width * scale, height * scale);
  },
  text(x, y, size, color, address, length) {
    const bytes = new Uint8Array(wasm.memory.buffer, address, length);
    view.font = `600 ${size * scale}px "Yu Gothic UI", "Hiragino Kaku Gothic ProN", sans-serif`;
    view.fillStyle = `#${color.toString(16).padStart(6, "0")}`;
    view.textBaseline = "top";
    view.fillText(decoder.decode(bytes), x * scale, y * scale);
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
  wasm.key(key);
  refreshButton.hidden = !wasm.is_title();
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
  if (!event.repeat || (key >= 37 && key <= 40)) sendKey(key);
});

controls.addEventListener("click", (event) => {
  const button = event.target.closest("button");
  if (button) sendKey(Number(button.dataset.key));
});
