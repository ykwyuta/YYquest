import { spawnSync } from "node:child_process";
import { copyFileSync, mkdirSync } from "node:fs";
import { fileURLToPath } from "node:url";

const root = new URL("../", import.meta.url);
const cargo = spawnSync("cargo", [
  "build", "--locked", "--release", "--lib", "--target", "wasm32-unknown-unknown",
], {
  cwd: fileURLToPath(root),
  env: { ...process.env, CARGO_TARGET_DIR: fileURLToPath(new URL("target/", root)) },
  stdio: "inherit",
});

if (cargo.error) throw cargo.error;
if (cargo.status !== 0) process.exit(cargo.status || 1);

const dist = new URL("dist/", root);
mkdirSync(dist, { recursive: true });
for (const name of ["index.html", "main.js", "game.css", "_headers"]) {
  copyFileSync(new URL(`web/${name}`, root), new URL(name, dist));
}
copyFileSync(
  new URL("target/wasm32-unknown-unknown/release/yyquest.wasm", root),
  new URL("yyquest.wasm", dist),
);
