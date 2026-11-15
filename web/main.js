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
const keys = new Map([
  ["ArrowUp", 38], ["ArrowDown", 40], ["ArrowLeft", 37], ["ArrowRight", 39],
  ["KeyW", 38], ["KeyS", 40], ["KeyA", 37], ["KeyD", 39],
  ["Enter", 13], ["Space", 13], ["KeyZ", 13], ["KeyJ", 74], ["Escape", 27], ["KeyX", 27],
]);
let wasm;

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

try {
  const response = await fetch(new URL("./yyquest.wasm", import.meta.url));
  if (!response.ok) throw new Error(`Wasm download failed (${response.status})`);
  const { instance } = await WebAssembly.instantiate(await response.arrayBuffer(), { screen });
  wasm = instance.exports;
  wasm.draw();
  status.hidden = true;
  controls.hidden = false;
} catch (error) {
  status.textContent = "ゲームを読み込めませんでした。ページを再読み込みしてください。";
  console.error(error);
}

window.addEventListener("keydown", (event) => {
  if (!wasm || event.ctrlKey || event.altKey || event.metaKey || event.isComposing) return;
  if (event.target instanceof HTMLButtonElement && ["Enter", "Space"].includes(event.code)) return;
  const key = keys.get(event.code);
  if (key === undefined) return;
  event.preventDefault();
  if (!event.repeat || (key >= 37 && key <= 40)) wasm.key(key);
});

controls.addEventListener("click", (event) => {
  const button = event.target.closest("button");
  if (button) wasm?.key(Number(button.dataset.key));
});
