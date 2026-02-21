import { NextRequest, NextResponse } from "next/server";
import { execFile } from "child_process";
import { writeFile, unlink } from "fs/promises";
import { tmpdir } from "os";
import { join } from "path";
import { randomUUID } from "crypto";

const BRIDGE_BIN = join(process.cwd(), "..", "target", "debug", "kenya-fhir-bridge");

export async function POST(req: NextRequest) {
  try {
    const body = await req.json();
    const tmpPath = join(tmpdir(), `kenyan-${randomUUID()}.json`);

    await writeFile(tmpPath, JSON.stringify(body));

    const result = await new Promise<string>((resolve, reject) => {
      execFile(BRIDGE_BIN, ["--input", tmpPath], { timeout: 10000 }, (err, stdout, stderr) => {
        unlink(tmpPath).catch(() => {});
        if (err) {
          reject(new Error(stderr || err.message));
        } else {
          resolve(stdout);
        }
      });
    });

    const bundle = JSON.parse(result);
    return NextResponse.json(bundle);
  } catch (e: unknown) {
    const msg = e instanceof Error ? e.message : "Unknown error";
    return NextResponse.json({ error: msg }, { status: 500 });
  }
}
