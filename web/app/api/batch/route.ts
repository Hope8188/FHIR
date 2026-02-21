import { NextRequest, NextResponse } from "next/server";
import { execFile } from "child_process";
import { writeFile, unlink } from "fs/promises";
import { tmpdir } from "os";
import { join } from "path";
import { randomUUID } from "crypto";

const BRIDGE_BIN = join(process.cwd(), "..", "target", "debug", "kenya-fhir-bridge");

const MAX_FILES = 20;
const MAX_FILE_SIZE = 256 * 1024; // 256 KB per file

type BatchResultItem = {
  filename: string;
  ok: boolean;
  bundle?: unknown;
  error?: string;
};

async function runBridge(content: Buffer, format: "json" | "xml"): Promise<string> {
  const ext = format === "xml" ? ".xml" : ".json";
  const tmpPath = join(tmpdir(), `batch-${randomUUID()}${ext}`);
  await writeFile(tmpPath, content);
  try {
    return await new Promise<string>((resolve, reject) => {
      const args = ["--input", tmpPath, ...(format === "xml" ? ["--format", "xml"] : [])];
      execFile(BRIDGE_BIN, args, { timeout: 10000 }, (err, stdout, stderr) => {
        if (err) reject(new Error((stderr?.trim() || err.message).split("\n")[0]));
        else resolve(stdout);
      });
    });
  } finally {
    unlink(tmpPath).catch(() => {});
  }
}

export async function POST(req: NextRequest) {
  try {
    const form = await req.formData();
    const files = form.getAll("files");

    if (!files.length) {
      return NextResponse.json({ error: "No files provided" }, { status: 400 });
    }
    if (files.length > MAX_FILES) {
      return NextResponse.json({ error: `Maximum ${MAX_FILES} files per batch` }, { status: 400 });
    }

    const results: BatchResultItem[] = await Promise.all(
      files.map(async (f) => {
        if (typeof f === "string") return { filename: "(unknown)", ok: false, error: "Expected file upload" };
        const file = f as File;
        if (file.size > MAX_FILE_SIZE) return { filename: file.name, ok: false, error: "File too large (max 256 KB)" };

        const bytes = await file.arrayBuffer();
        const buf = Buffer.from(bytes);
        const name = file.name.toLowerCase();
        const format: "json" | "xml" = name.endsWith(".xml") ? "xml" : "json";

        // Basic sanity check
        if (format === "xml" && !buf.includes(0x3c)) {
          return { filename: file.name, ok: false, error: "File does not appear to be XML" };
        }

        try {
          const out = await runBridge(buf, format);
          const bundle = JSON.parse(out);
          return { filename: file.name, ok: true, bundle };
        } catch (e) {
          return { filename: file.name, ok: false, error: e instanceof Error ? e.message : "Transform failed" };
        }
      })
    );

    const succeeded = results.filter(r => r.ok).length;
    const failed = results.filter(r => !r.ok).length;

    return NextResponse.json({ succeeded, failed, results });
  } catch {
    return NextResponse.json({ error: "Batch processing failed" }, { status: 500 });
  }
}
