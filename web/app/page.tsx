"use client";

import { useState, useRef, useCallback, useEffect } from "react";

const DEFAULT_KENYAN_JSON = {
  clinic_id: "KEN-NAIROBI-001",
  patient_number: "12345",
  national_id: "27845612",
  names: { first: "Wanjiru", middle: "Njeri", last: "Kamau" },
  gender: "F",
  date_of_birth: "1985-03-15",
  phone: "+254712345678",
  location: { county: "Nairobi", subcounty: "Westlands" },
  visit: {
    date: "2026-02-15",
    complaint: "Fever and cough",
    vitals: {
      temperature_celsius: 38.5,
      bp_systolic: 120,
      bp_diastolic: 80,
      weight_kg: 65,
    },
    diagnosis: "Upper respiratory tract infection",
    treatment: "Amoxicillin 500mg TDS for 7 days",
  },
};

const DEFAULT_XML = `<patient>
  <clinic_id>KEN-NAIROBI-001</clinic_id>
  <patient_number>12345</patient_number>
  <national_id>27845612</national_id>
  <names>
    <first>Wanjiru</first>
    <middle>Njeri</middle>
    <last>Kamau</last>
  </names>
  <gender>F</gender>
  <date_of_birth>1985-03-15</date_of_birth>
  <phone>+254712345678</phone>
  <location>
    <county>Nairobi</county>
    <subcounty>Westlands</subcounty>
  </location>
  <visit>
    <date>2026-02-15</date>
    <complaint>Fever and cough</complaint>
    <vitals>
      <temperature_celsius>38.5</temperature_celsius>
      <bp_systolic>120</bp_systolic>
      <bp_diastolic>80</bp_diastolic>
      <weight_kg>65.0</weight_kg>
    </vitals>
    <diagnosis>Upper respiratory tract infection</diagnosis>
    <treatment>Amoxicillin 500mg TDS for 7 days</treatment>
  </visit>
</patient>`;

type BundleEntry = {
  resource?: Record<string, unknown>;
  request?: { method?: string; url?: string };
};

type FHIRBundle = {
  resourceType: string;
  id?: string;
  timestamp?: string;
  type: string;
  entry?: BundleEntry[];
  error?: string;
};

type ValidationIssue = {
  severity: "error" | "warning" | "info";
  path: string;
  message: string;
  fix?: string;
};

type ValidationResult = {
  valid: boolean;
  score: number;
  issues: ValidationIssue[];
  stats: {
    totalEntries: number;
    resourceTypes: Record<string, number>;
    checkedRules: number;
    passedRules: number;
  };
};

type BatchResultItem = {
  filename: string;
  ok: boolean;
  bundle?: FHIRBundle;
  error?: string;
};

type BatchResult = {
  succeeded: number;
  failed: number;
  results: BatchResultItem[];
};

function Badge({
  label,
  color,
}: {
  label: string;
  color: "blue" | "green" | "yellow" | "red" | "purple" | "orange";
}) {
  const colors = {
    blue: "bg-blue-900/40 text-blue-300 border border-blue-800",
    green: "bg-green-900/40 text-green-300 border border-green-800",
    yellow: "bg-yellow-900/40 text-yellow-300 border border-yellow-800",
    red: "bg-red-900/40 text-red-300 border border-red-800",
    purple: "bg-purple-900/40 text-purple-300 border border-purple-800",
    orange: "bg-orange-900/40 text-orange-300 border border-orange-800",
  };
  return (
    <span className={`text-xs px-2 py-0.5 rounded font-mono ${colors[color]}`}>
      {label}
    </span>
  );
}

const RESOURCE_CONFIG: Record<
  string,
  { color: "blue" | "green" | "yellow" | "red" | "purple" | "orange"; icon: string }
> = {
  Patient: { color: "blue", icon: "P" },
  Observation: { color: "green", icon: "O" },
  Encounter: { color: "yellow", icon: "E" },
  Condition: { color: "red", icon: "C" },
  MedicationRequest: { color: "purple", icon: "M" },
};

function getResourceSummary(entry: BundleEntry): string | null {
  const r = entry.resource;
  if (!r) return null;
  const rt = r.resourceType as string;

  if (rt === "Observation") {
    const code = r.code as Record<string, unknown> | undefined;
    const text = code?.text as string | undefined;
    const vq = r.valueQuantity as Record<string, unknown> | undefined;
    if (vq && text) return `${text}: ${vq.value} ${vq.unit ?? ""}`.trim();
    if (text) return text;
  }
  if (rt === "Condition") {
    const code = r.code as Record<string, unknown> | undefined;
    return (code?.text as string) ?? null;
  }
  if (rt === "MedicationRequest") {
    const med = r.medicationCodeableConcept as Record<string, unknown> | undefined;
    return (med?.text as string) ?? null;
  }
  if (rt === "Encounter") {
    const rc = r.reasonCode as Array<Record<string, unknown>> | undefined;
    return (rc?.[0]?.text as string | undefined) ?? null;
  }
  if (rt === "Patient") {
    const name = r.name as Array<Record<string, unknown>> | undefined;
    const n = name?.[0];
    if (n) {
      const given = (n.given as string[] | undefined)?.[0] ?? "";
      const family = (n.family as string) ?? "";
      return `${given} ${family}`.trim();
    }
  }
  return null;
}

function ResourceCard({ entry, index }: { entry: BundleEntry; index: number }) {
  const [open, setOpen] = useState(false);
  const rt = entry.resource?.resourceType as string | undefined;
  const id = entry.resource?.id as string | undefined;
  const method = entry.request?.method;
  const cfg = RESOURCE_CONFIG[rt ?? ""] ?? { color: "blue" as const, icon: "?" };
  const summary = getResourceSummary(entry);

  return (
    <div className="border border-[#262626] rounded-lg overflow-hidden">
      <button
        onClick={() => setOpen(!open)}
        className="w-full flex items-center justify-between px-4 py-3 hover:bg-white/5 transition-colors text-left"
      >
        <div className="flex items-center gap-2 min-w-0">
          <span className="text-[#444] text-xs font-mono w-5 flex-shrink-0">#{index + 1}</span>
          {rt && <Badge label={rt} color={cfg.color} />}
          {method && (
            <span className="text-xs font-mono text-[#555] flex-shrink-0">{method}</span>
          )}
          {summary && (
            <span className="text-xs text-[#888] font-mono truncate">{summary}</span>
          )}
          {!summary && id && (
            <span className="text-xs text-[#555] font-mono truncate">id: {id}</span>
          )}
        </div>
        <svg
          className={`w-4 h-4 text-[#444] transition-transform flex-shrink-0 ml-2 ${open ? "rotate-180" : ""}`}
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
      </button>
      {open && (
        <div className="border-t border-[#262626] bg-[#0d0d0d]">
          <pre className="text-xs font-mono text-[#a0a0a0] p-4 overflow-x-auto leading-relaxed">
            {JSON.stringify(entry.resource, null, 2)}
          </pre>
        </div>
      )}
    </div>
  );
}

function StatRow({ label, value }: { label: string; value: string | number }) {
  return (
    <div className="flex items-center justify-between py-1.5 border-b border-[#1a1a1a] last:border-0">
      <span className="text-xs text-[#666]">{label}</span>
      <span className="text-xs font-mono text-[#ccc]">{value}</span>
    </div>
  );
}

function CopyButton({ text }: { text: string }) {
  const [copied, setCopied] = useState(false);
  const handleCopy = () => {
    navigator.clipboard.writeText(text).then(() => {
      setCopied(true);
      setTimeout(() => setCopied(false), 1800);
    }).catch(() => {
      // fallback
      const ta = document.createElement("textarea");
      ta.value = text;
      ta.style.cssText = "position:fixed;opacity:0";
      document.body.appendChild(ta);
      ta.select();
      document.execCommand("copy");
      document.body.removeChild(ta);
      setCopied(true);
      setTimeout(() => setCopied(false), 1800);
    });
  };
  return (
    <button
      onClick={handleCopy}
      className="text-xs font-mono px-3 py-1 rounded border border-[#333] text-[#666] hover:text-[#aaa] hover:border-[#555] transition-colors"
    >
      {copied ? "copied" : "copy"}
    </button>
  );
}

function DownloadButton({ bundle }: { bundle: FHIRBundle }) {
  const [downloading, setDownloading] = useState(false);
  const [sig, setSig] = useState<string | null>(null);

  const handleDownload = async () => {
    setDownloading(true);
    setSig(null);
    try {
      const res = await fetch("/api/download", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(bundle),
      });
      if (!res.ok) throw new Error("Download failed");
      const signature = res.headers.get("X-Bundle-Signature");
      const blob = await res.blob();
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `fhir-bundle-${Date.now()}.json`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
      if (signature) setSig(signature.slice(0, 16) + "…");
    } catch {
      // silent — user notices no download
    } finally {
      setDownloading(false);
    }
  };

  return (
    <div className="flex items-center gap-2">
      <button
        onClick={handleDownload}
        disabled={downloading}
        className="flex items-center gap-1.5 text-xs font-mono px-3 py-1.5 rounded border border-green-800 text-green-400 hover:bg-green-900/20 disabled:opacity-50 transition-colors"
      >
        {downloading ? (
          <svg className="animate-spin w-3 h-3" fill="none" viewBox="0 0 24 24">
            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z" />
          </svg>
        ) : (
          <svg className="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
          </svg>
        )}
        Secure Download
      </button>
      {sig && (
        <span className="text-xs font-mono text-[#555]" title="HMAC-SHA256 signature prefix">
          sig: {sig}
        </span>
      )}
    </div>
  );
}

function ValidationPanel({ bundle }: { bundle: FHIRBundle }) {
  const [result, setResult] = useState<ValidationResult | null>(null);
  const [loading, setLoading] = useState(false);
  const [ran, setRan] = useState(false);

  const validate = async () => {
    setLoading(true);
    try {
      const res = await fetch("/api/validate", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(bundle),
      });
      const data = await res.json();
      setResult(data);
    } catch {
      // silent
    } finally {
      setLoading(false);
      setRan(true);
    }
  };

  const errors = result?.issues.filter(i => i.severity === "error") ?? [];
  const warnings = result?.issues.filter(i => i.severity === "warning") ?? [];

  return (
    <div className="bg-[#111] border border-[#1e1e1e] rounded-lg p-4">
      <div className="flex items-center justify-between mb-3">
        <h3 className="text-xs font-semibold text-[#555] uppercase tracking-wider">
          FHIR R4 Validation
        </h3>
        <button
          onClick={validate}
          disabled={loading}
          className="text-xs font-mono px-3 py-1 rounded border border-blue-800 text-blue-400 hover:bg-blue-900/20 disabled:opacity-50 transition-colors"
        >
          {loading ? "Validating…" : ran ? "Re-validate" : "Validate"}
        </button>
      </div>

      {result && (
        <div className="space-y-2">
          <div className="flex items-center gap-3">
            <span
              className={`text-xs font-mono font-bold ${result.valid ? "text-green-400" : "text-red-400"}`}
            >
              {result.valid ? "PASS" : "FAIL"}
            </span>
            <div className="flex-1 bg-[#1a1a1a] rounded-full h-1.5">
              <div
                className={`h-1.5 rounded-full transition-all ${result.score >= 80 ? "bg-green-600" : result.score >= 50 ? "bg-yellow-600" : "bg-red-600"}`}
                style={{ width: `${result.score}%` }}
              />
            </div>
            <span className="text-xs font-mono text-[#666]">{result.score}/100</span>
          </div>
          <div className="text-xs font-mono text-[#555]">
            {result.stats.passedRules}/{result.stats.checkedRules} rules passed
            {errors.length > 0 && <span className="text-red-500 ml-2">{errors.length} error{errors.length > 1 ? "s" : ""}</span>}
            {warnings.length > 0 && <span className="text-yellow-500 ml-2">{warnings.length} warning{warnings.length > 1 ? "s" : ""}</span>}
          </div>

          {result.issues.length > 0 && (
            <div className="mt-2 space-y-1.5 max-h-[160px] overflow-y-auto">
              {result.issues.map((issue, i) => (
                <div key={i} className={`text-xs rounded p-2 border ${
                  issue.severity === "error"
                    ? "bg-red-950/30 border-red-900 text-red-300"
                    : "bg-yellow-950/30 border-yellow-900 text-yellow-300"
                }`}>
                  <div className="font-mono text-[0.65rem] text-[#555] mb-0.5">{issue.path}</div>
                  <div>{issue.message}</div>
                  {issue.fix && (
                    <div className="text-[#666] mt-0.5">Fix: {issue.fix}</div>
                  )}
                </div>
              ))}
            </div>
          )}
        </div>
      )}

      {!ran && !loading && (
        <p className="text-xs text-[#444]">Run to check FHIR R4 compliance</p>
      )}
    </div>
  );
}

type AfyaLinkResult = {
  success: boolean;
  status?: number;
  responseBody?: unknown;
  endpoint: string;
  live: boolean;
  error?: string;
};

function AfyaLinkPanel({ bundle }: { bundle: FHIRBundle }) {
  const [result, setResult] = useState<AfyaLinkResult | null>(null);
  const [loading, setLoading] = useState(false);
  const [expanded, setExpanded] = useState(false);

  const post = async () => {
    setLoading(true);
    setResult(null);
    try {
      const res = await fetch("/api/afyalink-post", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(bundle),
      });
      const data: AfyaLinkResult = await res.json();
      setResult(data);
      setExpanded(false);
    } catch (e) {
      setResult({
        success: false,
        error: e instanceof Error ? e.message : "Unknown error",
        endpoint: "https://uat.dha.go.ke/v1/shr-med/bundle",
        live: false,
      });
    } finally {
      setLoading(false);
    }
  };

  const entryCount = ((result?.responseBody as { entry?: unknown[] })?.entry ?? []).length;

  return (
    <div className="bg-[#111] border border-[#1e1e1e] rounded-lg p-4">
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <h3 className="text-xs font-semibold text-[#555] uppercase tracking-wider">
            AfyaLink SHR Submit
          </h3>
          {result && (
            <span className={`text-xs font-mono px-1.5 py-0.5 rounded border ${
              result.live
                ? "border-green-800 text-green-400 bg-green-950/30"
                : "border-yellow-800 text-yellow-400 bg-yellow-950/30"
            }`}>
              {result.live ? "LIVE UAT" : "MOCK"}
            </span>
          )}
        </div>
        <button
          onClick={post}
          disabled={loading}
          className="flex items-center gap-1.5 text-xs font-mono px-3 py-1 rounded border border-orange-800 text-orange-400 hover:bg-orange-900/20 disabled:opacity-50 transition-colors"
        >
          {loading ? (
            <svg className="animate-spin w-3 h-3" fill="none" viewBox="0 0 24 24">
              <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
              <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z" />
            </svg>
          ) : (
            <svg className="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
          )}
          POST to /v1/shr-med/bundle
        </button>
      </div>

      {!result && !loading && (
        <p className="text-xs text-[#444]">
          POST bundle to AfyaLink SHR. Set <span className="font-mono text-[#666]">AFYALINK_TOKEN</span> env var for live UAT; otherwise runs with realistic mock response.
        </p>
      )}

      {result && (
        <div className="space-y-2">
          {/* Status line */}
          <div className="flex items-center gap-3 text-xs font-mono">
            <span className={result.success ? "text-green-400 font-bold" : "text-red-400 font-bold"}>
              {result.success ? (result.status === 200 ? "200 OK" : `${result.status}`) : "ERROR"}
            </span>
            <span className="text-[#555] truncate">{result.endpoint}</span>
          </div>

          {result.error && (
            <div className="bg-red-950/30 border border-red-900 rounded p-2 text-xs text-red-300 font-mono">
              {result.error}
            </div>
          )}

          {result.success && result.responseBody && (
            <div className="space-y-1">
              <div className="flex items-center justify-between">
                <span className="text-xs text-[#555] font-mono">
                  {entryCount > 0 && `${entryCount} entries acknowledged`}
                  {(result.responseBody as { extension?: unknown[] })?.extension?.[0] && (
                    <span className="ml-2 text-green-500">
                      ACCEPTED
                    </span>
                  )}
                </span>
                <button
                  onClick={() => setExpanded(!expanded)}
                  className="text-xs font-mono text-[#555] hover:text-[#888] transition-colors"
                >
                  {expanded ? "hide response" : "view response"}
                </button>
              </div>
              {expanded && (
                <pre className="bg-[#0d0d0d] border border-[#1e1e1e] rounded p-3 text-xs font-mono text-[#a0a0a0] overflow-x-auto max-h-[200px] leading-relaxed">
                  {JSON.stringify(result.responseBody, null, 2)}
                </pre>
              )}
            </div>
          )}

          {!result.live && (
            <p className="text-xs text-[#444] font-mono">
              Mock mode — set AFYALINK_TOKEN + AFYALINK_BASE_URL env vars to hit real UAT
            </p>
          )}
        </div>
      )}
    </div>
  );
}

function BatchPanel() {
  const [files, setFiles] = useState<File[]>([]);
  const [dragging, setDragging] = useState(false);
  const [result, setResult] = useState<BatchResult | null>(null);
  const [loading, setLoading] = useState(false);
  const [expandedIdx, setExpandedIdx] = useState<number | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const addFiles = (newFiles: File[]) => {
    const valid = newFiles.filter(f => f.name.match(/\.(json|xml)$/i));
    setFiles(prev => {
      const existing = new Set(prev.map(f => f.name));
      const unique = valid.filter(f => !existing.has(f.name));
      return [...prev, ...unique].slice(0, 20);
    });
  };

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setDragging(false);
    const dropped = Array.from(e.dataTransfer.files);
    addFiles(dropped);
  }, []);

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    setDragging(true);
  };

  const handleDragLeave = () => setDragging(false);

  const runBatch = async () => {
    if (!files.length) return;
    setLoading(true);
    setResult(null);
    try {
      const form = new FormData();
      files.forEach(f => form.append("files", f));
      const res = await fetch("/api/batch", { method: "POST", body: form });
      const data = await res.json();
      setResult(data);
    } catch {
      // silent
    } finally {
      setLoading(false);
    }
  };

  const downloadBundle = async (bundle: FHIRBundle, filename: string) => {
    const res = await fetch("/api/download", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(bundle),
    });
    if (!res.ok) return;
    const blob = await res.blob();
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `fhir-${filename.replace(/\.(json|xml)$/i, "")}-${Date.now()}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  return (
    <div className="bg-[#111] border border-[#1e1e1e] rounded-lg p-4 flex flex-col gap-3">
      <h3 className="text-xs font-semibold text-[#555] uppercase tracking-wider">
        Batch Mode — up to 20 files
      </h3>

      {/* Drop zone */}
      <div
        onDrop={handleDrop}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onClick={() => fileInputRef.current?.click()}
        className={`border-2 border-dashed rounded-lg p-6 flex flex-col items-center justify-center gap-2 cursor-pointer transition-colors ${
          dragging ? "border-blue-500 bg-blue-900/10" : "border-[#2a2a2a] hover:border-[#444]"
        }`}
      >
        <input
          ref={fileInputRef}
          type="file"
          accept=".json,.xml,application/json,application/xml,text/xml"
          multiple
          className="hidden"
          onChange={e => {
            addFiles(Array.from(e.target.files ?? []));
            e.target.value = "";
          }}
        />
        <svg className="w-8 h-8 text-[#333]" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5}
            d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
        </svg>
        <p className="text-xs text-[#555]">
          {dragging ? "Drop files here" : "Drag & drop or click to select"}
        </p>
        <p className="text-xs text-[#444] font-mono">.json and .xml files</p>
      </div>

      {files.length > 0 && (
        <div className="space-y-1 max-h-[120px] overflow-y-auto">
          {files.map((f, i) => (
            <div key={i} className="flex items-center justify-between text-xs font-mono">
              <span className="text-[#888] truncate">{f.name}</span>
              <div className="flex items-center gap-2 flex-shrink-0 ml-2">
                <span className="text-[#555]">{(f.size / 1024).toFixed(1)}KB</span>
                <button
                  onClick={() => setFiles(prev => prev.filter((_, j) => j !== i))}
                  className="text-[#444] hover:text-red-400 transition-colors"
                >
                  x
                </button>
              </div>
            </div>
          ))}
        </div>
      )}

      <div className="flex items-center gap-2">
        <button
          onClick={runBatch}
          disabled={loading || files.length === 0}
          className="flex-1 text-xs font-mono py-2 px-4 rounded bg-blue-600 hover:bg-blue-500 disabled:bg-blue-900 disabled:text-blue-700 text-white font-semibold transition-colors"
        >
          {loading ? "Processing…" : `Transform ${files.length || ""} file${files.length !== 1 ? "s" : ""}`}
        </button>
        {files.length > 0 && (
          <button
            onClick={() => { setFiles([]); setResult(null); }}
            className="text-xs font-mono px-3 py-2 rounded border border-[#333] text-[#555] hover:text-[#888] transition-colors"
          >
            Clear
          </button>
        )}
      </div>

      {result && (
        <div className="space-y-2">
          <div className="flex gap-3 text-xs font-mono">
            <span className="text-green-400">{result.succeeded} succeeded</span>
            {result.failed > 0 && <span className="text-red-400">{result.failed} failed</span>}
          </div>
          <div className="space-y-1.5 max-h-[200px] overflow-y-auto">
            {result.results.map((r, i) => (
              <div key={i} className={`border rounded-lg overflow-hidden ${r.ok ? "border-[#262626]" : "border-red-900/50"}`}>
                <div className="flex items-center justify-between px-3 py-2">
                  <div className="flex items-center gap-2 min-w-0">
                    <span className={`text-xs ${r.ok ? "text-green-500" : "text-red-500"}`}>
                      {r.ok ? "OK" : "ERR"}
                    </span>
                    <span className="text-xs font-mono text-[#888] truncate">{r.filename}</span>
                  </div>
                  <div className="flex items-center gap-2 flex-shrink-0">
                    {r.ok && r.bundle && (
                      <>
                        <button
                          onClick={() => setExpandedIdx(expandedIdx === i ? null : i)}
                          className="text-xs font-mono text-[#555] hover:text-[#888] transition-colors"
                        >
                          {expandedIdx === i ? "hide" : "view"}
                        </button>
                        <button
                          onClick={() => downloadBundle(r.bundle!, r.filename)}
                          className="text-xs font-mono text-green-500 hover:text-green-400 transition-colors"
                        >
                          download
                        </button>
                      </>
                    )}
                  </div>
                </div>
                {r.error && (
                  <div className="px-3 pb-2 text-xs text-red-400 font-mono">{r.error}</div>
                )}
                {expandedIdx === i && r.bundle && (
                  <div className="border-t border-[#262626] bg-[#0d0d0d]">
                    <pre className="text-xs font-mono text-[#a0a0a0] p-3 overflow-x-auto max-h-[160px] leading-relaxed">
                      {JSON.stringify(r.bundle, null, 2)}
                    </pre>
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

const MAPPING_RULES: Array<[string, string]> = [
  ["national_id → IPRS/CR", "Patient.identifier (CR-SYNTH-*, cr.dha.go.ke)"],
  ["national_id", "Patient.identifier (secondary, digitalhealth.go.ke)"],
  ["names.last + first + middle", "Patient.name (HumanName, official)"],
  ["patient_number", "Patient.identifier (facility number)"],
  ["phone", "Patient.telecom (mobile)"],
  ["location.county", "Patient.address.district"],
  ["location.subcounty", "Patient.address.line"],
  ["visit → encounter", "Encounter (class: OP, finished) ← AfyaLink SHR 2025"],
  ["visit.attending_puid", "Practitioner (hwr.dha.go.ke) + Encounter.participant"],
  ["visit.complaint", "Encounter.reasonCode (text)"],
  ["visit.diagnosis", "Condition (ICD-11 primary + ICD-10 backward-compat)"],
  ["visit.treatment", "MedicationRequest (order, free-text dosage)"],
  ["vitals.temperature_celsius", "Observation LOINC 8310-5"],
  ["vitals.bp (panel)", "Observation LOINC 85354-9 + components 8480-6 / 8462-2"],
  ["vitals.weight_kg", "Observation LOINC 29463-7"],
  ["vitals.pulse_rate", "Observation LOINC 8867-4 (optional)"],
  ["vitals.o2_saturation", "Observation LOINC 59408-5 (optional)"],
  ["visit.sha_member_number", "Coverage (CAT-SHA-001) + Claim (preauthorization)"],
  ["visit.sha_intervention_code", "Claim.item.productOrService (SHA intervention)"],
  ["clinic_id", "Organization (facility-registry.dha.go.ke FID)"],
];

export default function Home() {
  const [inputMode, setInputMode] = useState<"json" | "xml-text" | "xml-file">("json");
  const [inputJson, setInputJson] = useState(JSON.stringify(DEFAULT_KENYAN_JSON, null, 2));
  const [inputXml, setInputXml] = useState(DEFAULT_XML);
  const [xmlFile, setXmlFile] = useState<File | null>(null);
  const [xmlFileDragging, setXmlFileDragging] = useState(false);
  const [bundle, setBundle] = useState<FHIRBundle | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [jsonError, setJsonError] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<"tree" | "raw" | "batch">("tree");
  const [showBatch, setShowBatch] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [mounted, setMounted] = useState(false);

  useEffect(() => { setMounted(true); }, []);

  const handleInputChange = (val: string) => {
    setInputJson(val);
    try {
      JSON.parse(val);
      setJsonError(null);
    } catch {
      setJsonError("Invalid JSON");
    }
  };

  const transformXmlFile = async (file: File) => {
    const form = new FormData();
    form.append("file", file);
    const res = await fetch("/api/transform-xml", { method: "POST", body: form });
    const data = await res.json();
    if (!res.ok || data.error) {
      setError(data.error ?? "XML transform failed");
    } else {
      setBundle(data);
    }
  };

  const transform = async () => {
    setError(null);
    setBundle(null);
    setLoading(true);
    try {
      if (inputMode === "json") {
        const parsed = JSON.parse(inputJson);
        const res = await fetch("/api/transform", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify(parsed),
        });
        const data = await res.json();
        if (!res.ok || data.error) {
          setError(data.error ?? "Transform failed");
        } else {
          setBundle(data);
          setActiveTab("tree");
        }
      } else if (inputMode === "xml-text") {
        const blob = new Blob([inputXml], { type: "application/xml" });
        const file = new File([blob], "record.xml", { type: "application/xml" });
        await transformXmlFile(file);
        if (!error) setActiveTab("tree");
      } else if (inputMode === "xml-file") {
        if (!xmlFile) { setError("Please select an XML file"); return; }
        await transformXmlFile(xmlFile);
        if (!error) setActiveTab("tree");
      }
    } catch (e) {
      setError(e instanceof Error ? e.message : "Unknown error");
    } finally {
      setLoading(false);
    }
  };

  const handleXmlFileDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setXmlFileDragging(false);
    const dropped = e.dataTransfer.files[0];
    if (dropped && (dropped.name.endsWith(".xml") || dropped.type.includes("xml"))) {
      setXmlFile(dropped);
      setError(null);
    }
  }, []);

  const isTransformDisabled =
    loading ||
    (inputMode === "json" && !!jsonError) ||
    (inputMode === "xml-file" && !xmlFile);

  const entries = bundle?.entry ?? [];
  const resourceCounts = entries.reduce<Record<string, number>>((acc, e) => {
    const rt = (e.resource?.resourceType as string) ?? "Unknown";
    acc[rt] = (acc[rt] ?? 0) + 1;
    return acc;
  }, {});
  const rawJson = bundle ? JSON.stringify(bundle, null, 2) : "";

  if (!mounted) return null;

  return (
    <div className="min-h-screen bg-[#0a0a0a] text-[#ededed]">
      {/* Header */}
      <header className="border-b border-[#1a1a1a] px-6 py-4">
        <div className="max-w-7xl mx-auto flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-lg bg-blue-600 flex items-center justify-center flex-shrink-0">
              <svg className="w-4 h-4 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
                  d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
              </svg>
            </div>
            <div>
              <h1 className="text-lg font-semibold tracking-tight">FHIR Builder</h1>
              <p className="text-xs text-[#555]">Kenya-to-FHIR R4 Bridge</p>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <Badge label="AfyaLink-Ready" color="blue" />
            <Badge label="FHIR R4" color="green" />
            <button
              onClick={() => setShowBatch(!showBatch)}
              className={`text-xs font-mono px-3 py-1 rounded border transition-colors ${
                showBatch
                  ? "border-purple-700 text-purple-300 bg-purple-900/20"
                  : "border-[#333] text-[#555] hover:text-[#888]"
              }`}
            >
              Batch
            </button>
          </div>
        </div>
      </header>

      <div className="max-w-7xl mx-auto px-6 py-8">

        {/* Batch Panel */}
        {showBatch && (
          <div className="mb-6">
            <BatchPanel />
          </div>
        )}

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* Left: Input */}
          <div className="flex flex-col gap-4">
            <div className="flex items-center justify-between">
              <h2 className="text-sm font-semibold text-[#888] uppercase tracking-wider">
                Kenyan Clinic Record
              </h2>
              {inputMode === "json" && jsonError && (
                <span className="text-xs text-red-400 font-mono">{jsonError}</span>
              )}
            </div>

            {/* Input mode tabs */}
            <div className="flex gap-1 bg-[#111] border border-[#1e1e1e] rounded-lg p-1">
              {(["json", "xml-text", "xml-file"] as const).map((mode) => (
                <button
                  key={mode}
                  onClick={() => setInputMode(mode)}
                  className={`flex-1 text-xs font-mono py-1.5 px-2 rounded transition-colors ${
                    inputMode === mode
                      ? "bg-blue-600 text-white"
                      : "text-[#555] hover:text-[#888]"
                  }`}
                >
                  {mode === "json" ? "JSON" : mode === "xml-text" ? "XML (paste)" : "XML (file)"}
                </button>
              ))}
            </div>

            {/* JSON editor */}
            {inputMode === "json" && (
              <textarea
                value={inputJson}
                onChange={(e) => handleInputChange(e.target.value)}
                className="w-full h-[420px] bg-[#111] border border-[#262626] rounded-lg p-4 font-mono text-sm text-[#ccc] resize-none focus:outline-none focus:border-blue-600 transition-colors leading-relaxed"
                spellCheck={false}
              />
            )}

            {/* XML paste editor */}
            {inputMode === "xml-text" && (
              <textarea
                value={inputXml}
                onChange={(e) => setInputXml(e.target.value)}
                className="w-full h-[420px] bg-[#111] border border-[#262626] rounded-lg p-4 font-mono text-sm text-[#ccc] resize-none focus:outline-none focus:border-blue-600 transition-colors leading-relaxed"
                spellCheck={false}
              />
            )}

            {/* XML file upload with drag-and-drop */}
            {inputMode === "xml-file" && (
              <div
                onDrop={handleXmlFileDrop}
                onDragOver={(e) => { e.preventDefault(); setXmlFileDragging(true); }}
                onDragLeave={() => setXmlFileDragging(false)}
                onClick={() => fileInputRef.current?.click()}
                className={`h-[420px] bg-[#111] border-2 border-dashed rounded-lg flex flex-col items-center justify-center gap-4 cursor-pointer transition-colors ${
                  xmlFileDragging
                    ? "border-blue-500 bg-blue-900/10"
                    : "border-[#262626] hover:border-blue-700"
                }`}
              >
                <input
                  ref={fileInputRef}
                  type="file"
                  accept=".xml,application/xml,text/xml"
                  className="hidden"
                  onChange={(e) => {
                    const f = e.target.files?.[0] ?? null;
                    setXmlFile(f);
                    setError(null);
                  }}
                />
                {xmlFile ? (
                  <>
                    <svg className="w-10 h-10 text-blue-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5}
                        d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                    </svg>
                    <div className="text-center">
                      <p className="text-sm font-mono text-[#ccc]">{xmlFile.name}</p>
                      <p className="text-xs text-[#555] mt-1">{(xmlFile.size / 1024).toFixed(1)} KB</p>
                    </div>
                    <button
                      onClick={(e) => { e.stopPropagation(); setXmlFile(null); }}
                      className="text-xs font-mono px-3 py-1 rounded border border-[#333] text-[#555] hover:text-red-400 hover:border-red-800 transition-colors"
                    >
                      Remove
                    </button>
                  </>
                ) : (
                  <>
                    <svg className="w-10 h-10 text-[#333]" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5}
                        d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
                    </svg>
                    <div className="text-center">
                      <p className="text-sm text-[#666]">
                        {xmlFileDragging ? "Drop your XML file here" : "Drag & drop or click to select"}
                      </p>
                      <p className="text-xs text-[#444] font-mono mt-1">.xml — Kenyan clinic record</p>
                    </div>
                  </>
                )}
              </div>
            )}

            <button
              onClick={transform}
              disabled={isTransformDisabled}
              className="flex items-center justify-center gap-2 bg-blue-600 hover:bg-blue-500 disabled:bg-blue-900 disabled:text-blue-700 text-white font-semibold py-3 px-6 rounded-lg transition-colors"
            >
              {loading ? (
                <>
                  <svg className="animate-spin w-4 h-4" fill="none" viewBox="0 0 24 24">
                    <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                    <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z" />
                  </svg>
                  Transforming…
                </>
              ) : (
                <>
                  <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
                  </svg>
                  Transform to FHIR R4 Bundle
                </>
              )}
            </button>

            {/* Mapping reference */}
            <div className="bg-[#111] border border-[#1e1e1e] rounded-lg p-4">
              <h3 className="text-xs font-semibold text-[#555] uppercase tracking-wider mb-3">
                Mapping Rules ({MAPPING_RULES.length} fields)
              </h3>
              <div className="space-y-1.5 text-xs font-mono max-h-[180px] overflow-y-auto pr-1">
                {MAPPING_RULES.map(([from, to]) => (
                  <div key={from} className="flex items-start gap-2 text-[#555]">
                    <span className="text-green-700 flex-shrink-0 mt-0.5">+</span>
                    <span className="text-[#888] min-w-[200px]">{from}</span>
                    <span className="text-blue-700 flex-shrink-0">-&gt;</span>
                    <span className="text-[#666]">{to}</span>
                  </div>
                ))}
              </div>
            </div>
          </div>

          {/* Right: Output */}
          <div className="flex flex-col gap-4">
            <div className="flex items-center justify-between">
              <h2 className="text-sm font-semibold text-[#888] uppercase tracking-wider">
                FHIR R4 Bundle Output
              </h2>
              {bundle && (
                <div className="flex items-center gap-2">
                  {(["tree", "raw"] as const).map((t) => (
                    <button
                      key={t}
                      onClick={() => setActiveTab(t)}
                      className={`text-xs px-3 py-1 rounded font-mono transition-colors ${
                        activeTab === t
                          ? "bg-blue-600 text-white"
                          : "text-[#555] hover:text-[#888]"
                      }`}
                    >
                      {t}
                    </button>
                  ))}
                  {activeTab === "raw" && <CopyButton text={rawJson} />}
                </div>
              )}
            </div>

            {error && (
              <div className="bg-red-950/40 border border-red-800 rounded-lg p-4">
                <p className="text-sm text-red-300 font-mono">{error}</p>
              </div>
            )}

            {!bundle && !error && !loading && (
              <div className="flex-1 flex flex-col items-center justify-center h-[520px] border border-dashed border-[#1e1e1e] rounded-lg text-[#333]">
                <svg className="w-12 h-12 mb-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1}
                    d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                </svg>
                <p className="text-sm">Click Transform to generate a FHIR Bundle</p>
                <p className="text-xs text-[#2a2a2a] mt-1">Supports JSON, XML paste, or XML file drag-and-drop</p>
              </div>
            )}

            {loading && (
              <div className="flex-1 flex items-center justify-center h-[520px] border border-[#1e1e1e] rounded-lg">
                <div className="flex flex-col items-center gap-3 text-[#444]">
                  <svg className="animate-spin w-8 h-8" fill="none" viewBox="0 0 24 24">
                    <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                    <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z" />
                  </svg>
                  <span className="text-sm font-mono">Running Rust bridge…</span>
                </div>
              </div>
            )}

            {bundle && activeTab === "tree" && (
              <div className="flex flex-col gap-3 overflow-y-auto max-h-[680px] pr-1">
                {/* Bundle metadata */}
                <div className="bg-[#111] border border-[#1e1e1e] rounded-lg p-4">
                  <div className="flex items-center justify-between mb-3">
                    <h3 className="text-xs font-semibold text-[#555] uppercase tracking-wider">
                      Bundle Metadata
                    </h3>
                    <DownloadButton bundle={bundle} />
                  </div>
                  <StatRow label="resourceType" value={bundle.resourceType} />
                  <StatRow label="type" value={bundle.type} />
                  {bundle.id && <StatRow label="id" value={bundle.id} />}
                  {bundle.timestamp && <StatRow label="timestamp" value={bundle.timestamp} />}
                  <StatRow label="total entries" value={entries.length} />
                  {Object.entries(resourceCounts).map(([rt, count]) => (
                    <StatRow key={rt} label={rt} value={`${count} resource${count > 1 ? "s" : ""}`} />
                  ))}
                </div>

                {/* Validation panel */}
                <ValidationPanel bundle={bundle} />

                {/* Resource entries */}
                <div className="flex flex-col gap-2">
                  {entries.map((entry, i) => (
                    <ResourceCard key={i} entry={entry} index={i} />
                  ))}
                </div>

                <div className="flex items-center gap-2 text-xs text-green-400 font-mono bg-green-950/30 border border-green-900 rounded-lg px-4 py-3">
                  <svg className="w-4 h-4 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                  </svg>
                  FHIR R4 Bundle generated — ready for AfyaLink SHR · use &quot;POST to AfyaLink&quot; below
                </div>
              </div>
            )}

            {bundle && activeTab === "raw" && (
              <div className="flex flex-col gap-2">
                <div className="flex justify-end">
                  <DownloadButton bundle={bundle} />
                </div>
                <pre className="bg-[#0d0d0d] border border-[#1e1e1e] rounded-lg p-4 text-xs font-mono text-[#a0a0a0] overflow-auto max-h-[680px] leading-relaxed">
                  {rawJson}
                </pre>
              </div>
            )}
          </div>
        </div>

        {/* Bottom info bar */}
        <div className="mt-8 grid grid-cols-1 sm:grid-cols-4 gap-4">
          {[
            {
              title: "ICD-11 + ICD-10 Dual Coding",
              desc: "Kenya DHA 2025 mandates ICD-11 MMS as primary. ICD-10 retained for backward-compat with KenyaEMR/older SHR. 12 diagnoses mapped.",
              badge: "DHA 2025",
              color: "green" as const,
            },
            {
              title: "AfyaLink SHR Compliance",
              desc: "Encounter.class=OP · FID facility URI · HWR PUID Practitioner · SHA Coverage+Claim (preauthorization) · CR-SYNTH fallback.",
              badge: "AfyaLink-Ready",
              color: "blue" as const,
            },
            {
              title: "LOINC Vitals Panel",
              desc: "Temp: 8310-5 · BP panel: 85354-9 (systolic 8480-6 + diastolic 8462-2 as components) · Weight: 29463-7 · Pulse: 8867-4 · SpO2: 59408-5",
              badge: "R4 Spec",
              color: "yellow" as const,
            },
            {
              title: "Offline-First + Secure",
              desc: "SQLite 7-day retry queue for rural facilities. HMAC-SHA256 signed downloads. Synthetic CR-ID fallback when AfyaLink UAT unreachable.",
              badge: "Signed",
              color: "purple" as const,
            },
          ].map((card) => (
            <div key={card.title} className="bg-[#111] border border-[#1e1e1e] rounded-lg p-4">
              <div className="flex items-center justify-between mb-2">
                <h3 className="text-sm font-semibold text-[#ccc]">{card.title}</h3>
                <Badge label={card.badge} color={card.color} />
              </div>
              <p className="text-xs text-[#555] leading-relaxed">{card.desc}</p>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
