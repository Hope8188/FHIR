import { NextRequest, NextResponse } from "next/server";

const AFYALINK_BASE = process.env.AFYALINK_BASE_URL ?? "https://uat.dha.go.ke";
const AFYALINK_TOKEN = process.env.AFYALINK_TOKEN ?? "";

export type AfyaLinkResult =
  | { success: true; status: number; responseBody: unknown; endpoint: string; live: boolean }
  | { success: false; error: string; endpoint: string; live: boolean };

export async function POST(req: NextRequest): Promise<NextResponse> {
  const bundle = await req.json();
  const endpoint = `${AFYALINK_BASE}/v1/shr-med/bundle`;
  const live = Boolean(AFYALINK_TOKEN);

  // ── Live path: real AfyaLink UAT ─────────────────────────────────────────
  if (live) {
    try {
      const controller = new AbortController();
      const timer = setTimeout(() => controller.abort(), 15_000);

      const res = await fetch(endpoint, {
        method: "POST",
        headers: {
          "Content-Type": "application/fhir+json",
          Accept: "application/fhir+json",
          Authorization: `Bearer ${AFYALINK_TOKEN}`,
        },
        body: JSON.stringify(bundle),
        signal: controller.signal,
      });

      clearTimeout(timer);

      let responseBody: unknown;
      try {
        responseBody = await res.json();
      } catch {
        responseBody = { rawStatus: res.status };
      }

      return NextResponse.json({
        success: res.ok,
        status: res.status,
        responseBody,
        endpoint,
        live: true,
      } satisfies AfyaLinkResult);
    } catch (e) {
      const msg = e instanceof Error ? e.message : "Request failed";
      return NextResponse.json({
        success: false,
        error: msg,
        endpoint,
        live: true,
      } satisfies AfyaLinkResult);
    }
  }

  // ── Mock path: sandbox not configured ────────────────────────────────────
  // Simulate the AfyaLink SHR response shape to demonstrate the full flow
  // without requiring credentials.
  const bundleId = (bundle as { id?: string }).id ?? "unknown";
  const entryCount = ((bundle as { entry?: unknown[] }).entry ?? []).length;

  const mockResponse = {
    resourceType: "Bundle",
    id: bundleId,
    type: "transaction-response",
    timestamp: new Date().toISOString(),
    // Simulate per-entry responses — 200 for each PUT/POST
    entry: Array.from({ length: entryCount }, (_, i) => ({
      response: {
        status: i === entryCount - 1 && entryCount > 7 ? "201 Created" : "200 OK",
        location: `urn:uuid:mock-${i}`,
        etag: `W/"1"`,
        lastModified: new Date().toISOString(),
      },
    })),
    // AfyaLink-specific extension: processing status
    extension: [
      {
        url: "http://afyalink.dha.go.ke/fhir/StructureDefinition/bundle-processing-status",
        valueCode: "ACCEPTED",
      },
    ],
  };

  return NextResponse.json({
    success: true,
    status: 200,
    responseBody: mockResponse,
    endpoint,
    live: false,
  } satisfies AfyaLinkResult);
}
