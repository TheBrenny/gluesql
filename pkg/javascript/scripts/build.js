/**
 * A simple build helper script designed to be called from NPM (npm run build).
 */

const {spawnSync} = require("child_process");
const fs = require("fs");
const path = require("path");

const buildForBrowser = process.argv.includes("--browser");
const buildForNode = process.argv.includes("--node");

const workspaceRoot = path.join(__dirname, "..", "..", "..");
const packageRoot = path.join(workspaceRoot, "pkg", "javascript");
const cargoConfigPath = path.join(workspaceRoot, "Cargo.toml");

// Step 1: Modify the Cargo.toml to uncomment the opt-level setting
let cargoData = fs.readFileSync(cargoConfigPath, "utf-8");
cargoData = cargoData.replace(`#? ?[profile.release]\n#? ?opt-level ?= ?"s"`, `[profile.release]\nopt-level = "s"`);
fs.writeFileSync(cargoConfigPath, cargoData, "utf-8");

// Step 2: Run wasm-build
const commands = {
    browser: ["wasm-pack", "build", "--no-pack", "--target", "web", "--no-typescript", "--release", "--out-dir", "./dist_web"],
    node: ["wasm-pack", "build", "--no-pack", "--target", "nodejs", "--no-typescript", "--release", "--out-dir", "./dist_nodejs", "--", "--no-default-features", "--features", "nodejs"]
}
try {
    if(buildForBrowser) spawnSync(commands.browser[0], commands.browser.slice(1), {
        cwd: packageRoot,
        stdio: "inherit"
    });
} catch(e) {
    console.error(e);
}
try {
    if(buildForNode) spawnSync(commands.node[0], commands.node.slice(1), {
        cwd: packageRoot,
        stdio: "inherit",
    });
} catch(e) {
    console.error(e);
}

// Step 3: Re-comment the opt-level setting in the Cargo.toml
cargoData = cargoData.replace(`[profile.release]\nopt-level = "s"`, `# [profile.release]\n# opt-level = "s"`);
fs.writeFileSync(cargoConfigPath, cargoData, "utf-8");
