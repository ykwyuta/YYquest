import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

const binary = await readFile(new URL("../dist/yyquest.wasm", import.meta.url));
const decoder = new TextDecoder();
let wasm;
let labels = [];
let center = 0;
let expectedWidth = 640;
const imports = {
  screen: {
    bitmap(address, width, height) {
      assert.equal(width, expectedWidth);
      assert.equal(height, 480);
      labels = [];
      const pixels = new Uint32Array(wasm.memory.buffer, address, width * height);
      center = pixels[200 * width + width / 2];
    },
    text(_x, _y, _size, _color, address, length) {
      labels.push(decoder.decode(new Uint8Array(wasm.memory.buffer, address, length)));
    },
  },
};
wasm = (await WebAssembly.instantiate(binary, imports)).instance.exports;

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
  assert.ok(labels.join("").includes(line), labels.join(" / "));
  assert.ok(!labels.some((text) => text.startsWith("案内精霊：") || text.includes("移動　　決定　会話・調査")),
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
has("警備隊が避難してきた人");
press(enter);
has("精霊　冬越し小屋へ");
press(...Array(20).fill(up), ...Array(65).fill(right), down);
has("決定で刈り取る");
press(enter, enter);
has("灰葉草を3個刈り取った");
has("再び採れるまで 96 歩");
press(escape);
has("街・村");
press(journal);
has("持ち物");
has("24銀環");
has("灰葉草 ×3");
has("灰葉の傷薬");
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
// Enter the redesigned tutorial and verify the wide field and overview widths.
press(enter);
assert.equal(wasm.is_title(), 0);
hasDialogue("灰灯りの村へ水が来なくなった");
assert.equal(wasm.has_map_control(), 0);
press(enter);
has("炉番に水門");
expectedWidth = 1060;
wasm.set_view_width(530);
has("炉番に水門");
expectedWidth = 640;
press(map);
has("機工連邦ヴァルカン");
expectedWidth = 1060;
press(map);
has("炉番に水門");
expectedWidth = 640;
wasm.set_view_width(320);

wasm.press_direction(down, 1);
hasDialogue("光のあとをたどって");
press(enter);
has("炉番に水門");
wasm.release_direction(down, 1);
wasm.press_direction(up, 1);
assert.equal(wasm.animate_traffic(80), 1);
assert.equal(wasm.animate_traffic(64), 1);
wasm.release_direction(up, 1);
press(enter);
hasDialogue("東の森から笛の音");
has("炉番");
assert.equal(wasm.reading_dialogue(), 1);
press(enter, down, down, ...Array(10).fill(right), down, down, enter);
hasDialogue("さっき笛を吹いた");
has("リリス");
assert.ok(!labels.some((text) => text.includes("仲間になった")));
press(enter, down, left, left, left, enter);
hasDialogue("魔物が飛び出した");
has("状況");
press(enter);
has("霧のスライム");
has("HP 12/12");
press(down, enter);
has("HP 0/12");
has("父の最後の言葉");
press(enter);
hasDialogue("水路が通ったね");
press(enter, journal);
has("記憶が風化した");
assert.ok(!labels.includes("父の最後の言葉"));

press(journal, right, right, right, up, up, up, up, up, right, enter, enter);
has("忘却の獣");
has("記憶を使わずリリスと協力");
press(down, down, enter, enter, enter, enter, enter, enter, enter, enter);
hasDialogue("三つの灯");
has("石碑から響く過去の声");
press(enter);
hasDialogue("一緒に来てくれないか");
press(enter);
hasDialogue("私も行く");
has("リリス");
press(enter);
hasDialogue("リリスが仲間になった");
press(enter, left, down, down, ...Array(10).fill(left), up, up, enter, enter);
has("灰灯りの村");
has("精霊　ガウェイン旧宅へ");

const savedSize = wasm.checkpoint_size();
assert.ok(savedSize > 0);
const checkpoint = new Uint8Array(wasm.memory.buffer, wasm.checkpoint_ptr(), savedSize).slice();
const resumed = await WebAssembly.instantiate(binary, imports);
wasm = resumed.instance.exports;
const destination = wasm.prepare_save_load(checkpoint.length);
assert.ok(destination > 0);
new Uint8Array(wasm.memory.buffer, destination, checkpoint.length).set(checkpoint);
assert.equal(wasm.offer_continue(), 1);
wasm.draw();
has("つづきから");
press(enter);
has("灰灯りの村");
has("精霊　ガウェイン旧宅へ");
assert.equal(wasm.take_sound(), 1);

const restarted = await WebAssembly.instantiate(binary, imports);
wasm = restarted.instance.exports;
wasm.draw();
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
has("私の薪束には刻み目");
has("夜番");
assert.ok(labels.some((text) => text.startsWith("「")));
press(enter);
has("寄合小屋へ");
press(escape, escape, escape);
has("デバッグモード");

console.log("Wasm smoke passed (title, island, town story, tutorial, journal, companion, Nox start, save and resume, debug menu)");
