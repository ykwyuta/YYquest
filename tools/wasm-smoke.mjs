import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

const binary = await readFile(new URL("../dist/yyquest.wasm", import.meta.url));
const decoder = new TextDecoder();
let wasm;
let labels = [];
let center = 0;
const { instance } = await WebAssembly.instantiate(binary, {
  screen: {
    bitmap(address, width, height) {
      assert.equal(width, 640);
      assert.equal(height, 480);
      labels = [];
      const pixels = new Uint32Array(wasm.memory.buffer, address, width * height);
      center = pixels[200 * width + width / 2];
    },
    text(_x, _y, _size, _color, address, length) {
      labels.push(decoder.decode(new Uint8Array(wasm.memory.buffer, address, length)));
    },
  },
});
wasm = instance.exports;

function press(...keys) {
  for (const key of keys) wasm.key(key);
}
function has(fragment) {
  assert.ok(labels.some((text) => text.includes(fragment)), labels.join(" / "));
}
function hasTitle() {
  assert.equal(wasm.is_title(), 1);
  has("残響の");
  has("アルカディア");
}
function hasDialogue(line) {
  has(line);
  assert.ok(!labels.some((text) => text.includes("WASD") || text.includes("東の森へ。") || text.includes("リリスと北の石碑へ。")),
    labels.join(" / "));
}

const enter = 13, left = 37, up = 38, right = 39, down = 40, journal = 74;
wasm.draw();
assert.equal(wasm.animate_traffic(70), 0);
assert.equal(wasm.is_title(), 1);
hasTitle();
assert.notEqual(center, 0);

const map = 77, escape = 27;
assert.equal(wasm.has_map_control(), 1);
press(map);
assert.equal(wasm.is_title(), 0);
has("黄昏の辺境領ノクス");
has("虚無の海");
press(enter);
has("霧灯の港町");
assert.equal(wasm.animate_traffic(70), 1);
press(right, right, right, ...Array(32).fill(up), left, left, left);
has("灰灯りの村");
has("街・村");
press(enter);
has("島街道への出口");
press(journal);
has("街の手記");
has("炉の名札");
has("灰の中の晩餐");
press(down, journal);
has("精霊　ガウェイン旧宅へ");
press(journal, up, journal);
has("精霊　共同炉へ");
press(...Array(4).fill(left), ...Array(74).fill(up));
has("共同炉");
press(enter, enter);
has("警備隊が避難者の名前");
press(enter);
has("精霊　冬越し小屋へ");
press(escape);
has("街・村");
press(enter);
has("精霊　冬越し小屋へ");
press(escape, escape, right, enter);
has("黒鉄港町");
press(enter);
has("虚無の海");
press(escape);
assert.equal(wasm.is_title(), 1);
hasTitle();
press(enter);
assert.equal(wasm.is_title(), 0);
hasDialogue("新聖律歴30年");
assert.equal(wasm.has_map_control(), 0);
press(enter);
has("案内精霊　東の森へ。");
press(map);
has("機工連邦ヴァルカン");
press(map);
has("案内精霊　東の森へ。");
press(journal);
has("父の最後の言葉");
press(journal);

// Find the first encounter, use rune magic, then read the changed journal.
press(down, up, down, up, down, up, down, up, down, up, down);
has("霧のスライム");
press(down, enter);
has("父の最後の言葉");
press(enter, journal);
has("記憶が風化した");
assert.ok(!labels.includes("父の最後の言葉"));

// Follow the road, recruit Lilith, and use spirit magic at the monument.
press(journal, up, ...Array(10).fill(right), down, down);
hasDialogue("リリスが仲間になった");
press(enter);
has("案内精霊　リリスと北の石碑へ");
press(up, up, up, right, up);
has("忘却の獣");
press(up, up, enter, enter, enter, enter, enter, enter, enter, enter);
has("序章　完");
has("残った記憶  2/3");
press(enter, left);
assert.equal(wasm.is_title(), 1);
hasTitle();
// Open the debug menu via the directional pad, then jump to a town and a story.
press(up, up, down, down, left, right, left, right);
assert.equal(wasm.is_title(), 1);
press(enter);
has("デバッグモード");
assert.equal(wasm.debug_back_control(), 1);
press(enter, down, down, down, enter, enter);
has("灰灯りの村");
has("島街道への出口");
press(escape, escape, escape);
has("デバッグモード");
press(escape, escape, down, enter, enter, enter, down, enter);
has("炉の名札");
has("名札のない薪束");
press(enter);
has("寄合小屋へ");
press(escape, escape, escape);
has("デバッグモード");

console.log("Wasm smoke passed (title, island, town story, encounter, journal, companion, ending, debug menu)");
