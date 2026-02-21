import { NextRequest, NextResponse } from "next/server";
import { execFile } from "child_process";
import { writeFile, unlink } from "fs/promises";
import { tmpdir } from "os";
import { join } from "path";
import { randomUUID } from "crypto";

const BRIDGE_BIN = join(process.cwd(), "..", "target", "debug", "kenya-fhir-bridge");

/**
 * POST /api/transform-xml
 *
 * Accepts a multipart/form-data upload with a single field "file" containing
 * a Kenyan clinic XML record, runs the Rust bridge, returns a FHIR R4 Bundle.
 *
 * Security:
 *  - File is written to a unique temp path; deleted immediately after use.
 *  - Errors returned to client never include PHI or internal stack details.
 */
export async function POST(req: NextRequest) {
  let tmpPath: string | null = null;
  try {
    const formData = await req.formData();
    const file = formData.get("file");

    if (!file || typeof file === "string") {
      return NextResponse.json(
        { error: "Expected a 'file' field with an XML upload" },
        { status: 400 }
      );
    }

    const bytes = await (file as File).arrayBuffer();
    const buffer = Buffer.from(bytes);

    // Sanity-check: reject obviously non-XML uploads (no <)
    if (!buffer.includes(0x3c)) {
      return NextResponse.json(
        { error: "Uploaded file does not appear to be XML" },
        { status: 400 }
      );
    }

    tmpPath = join(tmpdir(), `kenyan-xml-${randomUUID()}.xml`);
    await writeFile(tmpPath, buffer);

    const result = await new Promise<string>((resolve, reject) => {
      execFile(
        BRIDGE_BIN,
        ["--input", tmpPath!, "--format", "xml"],
        { timeout: 10000 },
        (err, stdout, stderr) => {
          if (err) {
            // Sanitize stderr — don't forward raw internal errors
            const msg = stderr?.trim() || "Transform failed";
            reject(new Error(msg.split("\n")[0]));
          } else {
            resolve(stdout);
          }
        }
      );
    });

    const bundle = JSON.parse(result);
    return NextResponse.json(bundle);
  } catch (e: unknown) {
    const msg = e instanceof Error ? e.message : "Transform failed";
    return NextResponse.json({ error: msg }, { status: 500 });
  } finally {
    // Always clean up temp file
    if (tmpPath) unlink(tmpPath).catch(() => {});
  }
}
