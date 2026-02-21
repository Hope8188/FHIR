import { NextRequest, NextResponse } from "next/server";
import { signBundle } from "@/lib/bundle-signing";

/**
 * POST /api/download
 *
 * Accepts a FHIR Bundle JSON body, signs it with HMAC-SHA256, and returns
 * it as a downloadable file with the signature in X-Bundle-Signature header.
 *
 * Security:
 *  - HMAC key from DOWNLOAD_SECRET env var (server-side only, never client)
 *  - Content-Disposition forces file download, not inline render
 *  - Cache-Control: no-store prevents caching of PHI
 *  - No PHI in logs or error responses
 */

/**
 * GET /api/download?bundle=<base64url>
 * Accepts bundle as base64url query param, signs and returns as download.
 */
export async function GET(req: NextRequest) {
  const bundleParam = req.nextUrl.searchParams.get("bundle");
  if (!bundleParam) {
    return NextResponse.json({ error: "Missing bundle parameter" }, { status: 400 });
  }
  try {
    const decoded = Buffer.from(bundleParam, "base64url").toString("utf-8");
    const bundle = JSON.parse(decoded);
    return buildDownloadResponse(bundle);
  } catch {
    return NextResponse.json({ error: "Invalid bundle data" }, { status: 400 });
  }
}

/**
 * POST /api/download
 * Accepts FHIR Bundle in request body, signs and returns as download.
 */
export async function POST(req: NextRequest) {
  try {
    const bundle = await req.json();
    return buildDownloadResponse(bundle);
  } catch {
    return NextResponse.json({ error: "Invalid request body" }, { status: 400 });
  }
}

function buildDownloadResponse(bundle: unknown): NextResponse {
  const canonical = JSON.stringify(bundle, null, 2);
  const signature = signBundle(canonical);
  const filename = `fhir-bundle-${Date.now()}.json`;

  return new NextResponse(canonical, {
    status: 200,
    headers: {
      "Content-Type": "application/fhir+json; charset=utf-8",
      "Content-Disposition": `attachment; filename="${filename}"`,
      "X-Bundle-Signature": signature,
      "X-Bundle-Algorithm": "HMAC-SHA256",
      "Cache-Control": "no-store",
    },
  });
}
