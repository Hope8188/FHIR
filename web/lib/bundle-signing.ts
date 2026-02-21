import { createHmac, timingSafeEqual } from "crypto";

const DOWNLOAD_SECRET =
  process.env.DOWNLOAD_SECRET ?? "default-dev-secret-change-in-prod";

export function signBundle(data: string): string {
  return createHmac("sha256", DOWNLOAD_SECRET).update(data, "utf-8").digest("hex");
}

/**
 * Verify an HMAC-SHA256 signature using constant-time comparison.
 * Prevents timing attacks on the comparison.
 */
export function verifyBundleSignature(data: string, providedSig: string): boolean {
  const expected = signBundle(data);
  try {
    return timingSafeEqual(
      Buffer.from(expected, "hex"),
      Buffer.from(providedSig, "hex")
    );
  } catch {
    return false;
  }
}
