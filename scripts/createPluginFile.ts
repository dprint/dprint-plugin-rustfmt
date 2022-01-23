import { createHash } from "https://deno.land/std@0.122.0/hash/mod.ts";
import * as hex from "https://deno.land/std@0.122.0/encoding/hex.ts";

const packageText = Deno.readTextFileSync("Cargo.toml");
// version = "x.x.x"
const version = packageText.match(/version\s*=\s*\"(\d+\.\d+\.\d+)\"/)![1];

if (!/^\d+\.\d+\.\d+$/.test(version)) {
    throw new Error("Error extracting version.");
}

const outputFile = {
    schemaVersion: 1,
    name: "dprint-plugin-rustfmt",
    version,
    "mac-x86_64": getPlatformObject("dprint-plugin-rustfmt-x86_64-apple-darwin.zip"),
    "linux-x86_64": getPlatformObject("dprint-plugin-rustfmt-x86_64-unknown-linux-gnu.zip"),
    "windows-x86_64": getPlatformObject("dprint-plugin-rustfmt-x86_64-pc-windows-msvc.zip"),
};
Deno.writeTextFile("rustfmt.exe-plugin", JSON.stringify(outputFile, undefined, 2) + "\n");

function getPlatformObject(zipFileName: string) {
    const fileBytes = Deno.readFileSync(zipFileName);
    const hash = createHash("sha256");
    hash.update(fileBytes);
    const checksum = hex.encodeToString(new Uint8Array(hash.digest()));
    console.log(zipFileName + ": " + checksum);
    return {
        "reference": `https://github.com/dprint/dprint-plugin-rustfmt/releases/download/${version}/${zipFileName}`,
        "checksum": checksum,
    };
}