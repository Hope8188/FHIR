import { NextRequest, NextResponse } from "next/server";

// Required LOINC codes for vital-sign Observations
const VITAL_LOINC = new Set(["8310-5", "29463-7", "85354-9"]);
const BP_COMPONENT_LOINC = new Set(["8480-6", "8462-2"]);

type Coding = { system?: string; code?: string; display?: string };
type CodeableConcept = { coding?: Coding[]; text?: string };
type Reference = { reference?: string };
type Quantity = { value?: number; unit?: string; system?: string };
type ObservationComponent = { code?: CodeableConcept; valueQuantity?: Quantity };

type Resource = {
  resourceType: string;
  id?: string;
  status?: string;
  class?: Coding;
  code?: CodeableConcept;
  category?: CodeableConcept[];
  subject?: Reference;
  identifier?: Array<{ system?: string; value?: string }>;
  name?: Array<{ use?: string; family?: string; given?: string[] }>;
  component?: ObservationComponent[];
  valueQuantity?: Quantity;
  clinicalStatus?: CodeableConcept;
  intent?: string;
  medicationCodeableConcept?: CodeableConcept;
};

type BundleEntry = {
  fullUrl?: string;
  resource?: Resource;
  request?: { method?: string; url?: string };
};

type Bundle = {
  resourceType?: string;
  id?: string;
  type?: string;
  timestamp?: string;
  entry?: BundleEntry[];
};

export type ValidationIssue = {
  severity: "error" | "warning" | "info";
  path: string;
  message: string;
  fix?: string;
};

export type ValidationResult = {
  valid: boolean;
  score: number; // 0–100
  issues: ValidationIssue[];
  stats: {
    totalEntries: number;
    resourceTypes: Record<string, number>;
    checkedRules: number;
    passedRules: number;
  };
};

function codings(cc?: CodeableConcept): Coding[] {
  return cc?.coding ?? [];
}

function validateBundle(bundle: Bundle): ValidationResult {
  const issues: ValidationIssue[] = [];
  let checkedRules = 0;
  let passedRules = 0;

  const pass = () => { checkedRules++; passedRules++; };
  const fail = (issue: ValidationIssue) => { checkedRules++; issues.push(issue); };
  const warn = (issue: ValidationIssue) => { issues.push(issue); }; // warnings don't count against score

  // ── Bundle-level checks ─────────────────────────────────────────────────
  checkedRules++;
  if (bundle.resourceType === "Bundle") passedRules++;
  else fail({ severity: "error", path: "Bundle.resourceType", message: "resourceType must be 'Bundle'" });

  checkedRules++;
  if (bundle.type === "transaction" || bundle.type === "document" || bundle.type === "collection") passedRules++;
  else fail({ severity: "error", path: "Bundle.type", message: `Bundle.type '${bundle.type ?? "(missing)"}' is not a valid FHIR bundle type`, fix: "Use 'transaction' for POSTing to a FHIR server" });

  checkedRules++;
  if (bundle.id) passedRules++;
  else fail({ severity: "warning", path: "Bundle.id", message: "Bundle.id is missing — server will assign one, but it aids traceability" });

  const entries = bundle.entry ?? [];
  const resourceCounts: Record<string, number> = {};

  entries.forEach((entry, i) => {
    const prefix = `Bundle.entry[${i}]`;
    const r = entry.resource;
    const rt = r?.resourceType ?? "(unknown)";
    resourceCounts[rt] = (resourceCounts[rt] ?? 0) + 1;

    // fullUrl
    checkedRules++;
    if (entry.fullUrl?.startsWith("urn:uuid:") || entry.fullUrl?.startsWith("http")) passedRules++;
    else fail({ severity: "error", path: `${prefix}.fullUrl`, message: "fullUrl is missing or not in urn:uuid: format — required for cross-references in transaction bundles", fix: `Set to "urn:uuid:${r?.id ?? "<resource-id>"}"` });

    // request
    checkedRules++;
    if (entry.request?.method && entry.request?.url) passedRules++;
    else fail({ severity: "error", path: `${prefix}.request`, message: "request.method and request.url are required for transaction bundles" });

    if (!r) return;

    // ── Patient ───────────────────────────────────────────────────────────
    if (rt === "Patient") {
      checkedRules++;
      if (r.identifier?.length) passedRules++;
      else fail({ severity: "error", path: `${prefix}.resource.identifier`, message: "Patient.identifier is required — must include national ID or clinic number", fix: "Add Kenya DHA national-id identifier" });

      checkedRules++;
      if (r.name?.length && r.name[0].family) passedRules++;
      else fail({ severity: "error", path: `${prefix}.resource.name`, message: "Patient.name with family name is required" });

      checkedRules++;
      const gender = (r as unknown as { gender?: string }).gender;
      if (["male", "female", "other", "unknown"].includes(gender ?? "")) passedRules++;
      else fail({ severity: "error", path: `${prefix}.resource.gender`, message: `Patient.gender '${gender ?? "(missing)"}' is not a valid FHIR gender code`, fix: "Use male | female | other | unknown" });
    }

    // ── Encounter ─────────────────────────────────────────────────────────
    if (rt === "Encounter") {
      checkedRules++;
      if (r.status) passedRules++;
      else fail({ severity: "error", path: `${prefix}.resource.status`, message: "Encounter.status is required" });

      checkedRules++;
      if (r.class?.code) passedRules++;
      else fail({ severity: "error", path: `${prefix}.resource.class`, message: "Encounter.class is required (FHIR R4 mandatory field)", fix: "Set to {system: v3-ActCode, code: AMB}" });

      checkedRules++;
      if (r.subject?.reference?.startsWith("Patient/")) passedRules++;
      else fail({ severity: "error", path: `${prefix}.resource.subject`, message: "Encounter.subject must reference a Patient" });
    }

    // ── Observation ───────────────────────────────────────────────────────
    if (rt === "Observation") {
      checkedRules++;
      if (r.status === "final" || r.status === "preliminary" || r.status === "amended") passedRules++;
      else fail({ severity: "error", path: `${prefix}.resource.status`, message: `Observation.status '${r.status ?? "(missing)"}' must be final | preliminary | amended` });

      checkedRules++;
      const hasCat = r.category?.some(c => codings(c).some(cd => cd.code === "vital-signs"));
      if (hasCat) passedRules++;
      else fail({ severity: "error", path: `${prefix}.resource.category`, message: "Vital-sign Observation must have category 'vital-signs'", fix: "Add category with system observation-category, code vital-signs" });

      checkedRules++;
      const obsCode = codings(r.code).find(cd => cd.system === "http://loinc.org");
      if (obsCode?.code && VITAL_LOINC.has(obsCode.code)) passedRules++;
      else fail({ severity: "error", path: `${prefix}.resource.code`, message: `Observation.code LOINC '${obsCode?.code ?? "(missing)"}' is not a recognised vital-signs LOINC`, fix: "Use 8310-5 (temp), 29463-7 (weight), or 85354-9 (BP panel)" });

      checkedRules++;
      if (r.subject?.reference?.startsWith("Patient/")) passedRules++;
      else fail({ severity: "error", path: `${prefix}.resource.subject`, message: "Observation.subject must reference a Patient" });

      // BP panel-specific
      const isBpPanel = codings(r.code).some(c => c.code === "85354-9");
      if (isBpPanel) {
        checkedRules++;
        const compCodes = (r.component ?? []).flatMap(c => codings(c.code));
        const hasSystolic = compCodes.some(c => c.code === "8480-6");
        const hasDiastolic = compCodes.some(c => c.code === "8462-2");
        if (hasSystolic && hasDiastolic) passedRules++;
        else fail({ severity: "error", path: `${prefix}.resource.component`, message: "BP panel (85354-9) must have components for systolic (8480-6) and diastolic (8462-2)", fix: "Add component[] with LOINC codes 8480-6 and 8462-2" });

        // BP panel should NOT have a top-level valueQuantity
        checkedRules++;
        if (!r.valueQuantity) passedRules++;
        else fail({ severity: "warning", path: `${prefix}.resource.valueQuantity`, message: "BP panel should not have a top-level valueQuantity — values belong in components" });

        // Range check on BP values
        r.component?.forEach((comp, ci) => {
          const compCode = codings(comp.code).find(c => BP_COMPONENT_LOINC.has(c.code ?? ""));
          if (compCode && comp.valueQuantity?.value !== undefined) {
            const v = comp.valueQuantity.value;
            checkedRules++;
            if (compCode.code === "8480-6" && (v < 30 || v > 300)) {
              fail({ severity: "error", path: `${prefix}.resource.component[${ci}].valueQuantity.value`, message: `Systolic BP ${v} is outside plausible clinical range (30–300 mmHg)` });
            } else if (compCode.code === "8462-2" && (v < 20 || v > 200)) {
              fail({ severity: "error", path: `${prefix}.resource.component[${ci}].valueQuantity.value`, message: `Diastolic BP ${v} is outside plausible clinical range (20–200 mmHg)` });
            } else {
              passedRules++;
            }
          }
        });
      } else {
        // Non-panel observations must have valueQuantity
        checkedRules++;
        if (r.valueQuantity?.value !== undefined) passedRules++;
        else fail({ severity: "error", path: `${prefix}.resource.valueQuantity`, message: "Observation must have valueQuantity for scalar vital signs" });
      }

      // UCUM units
      checkedRules++;
      const allQuantities: Quantity[] = [
        ...(r.valueQuantity ? [r.valueQuantity] : []),
        ...(r.component?.flatMap(c => c.valueQuantity ? [c.valueQuantity] : []) ?? []),
      ];
      const allHaveUcum = allQuantities.every(q => q.system === "http://unitsofmeasure.org");
      if (allQuantities.length === 0 || allHaveUcum) passedRules++;
      else fail({ severity: "warning", path: `${prefix}.resource.valueQuantity.system`, message: "Observation quantity units should use UCUM (http://unitsofmeasure.org)" });
    }

    // ── Condition ─────────────────────────────────────────────────────────
    if (rt === "Condition") {
      checkedRules++;
      const cs = codings(r.clinicalStatus);
      if (cs.length > 0 && cs[0].code) passedRules++;
      else fail({ severity: "error", path: `${prefix}.resource.clinicalStatus`, message: "Condition.clinicalStatus is required with a coded value" });

      checkedRules++;
      if (r.subject?.reference?.startsWith("Patient/")) passedRules++;
      else fail({ severity: "error", path: `${prefix}.resource.subject`, message: "Condition.subject must reference a Patient" });
    }

    // ── MedicationRequest ──────────────────────────────────────────────────
    if (rt === "MedicationRequest") {
      checkedRules++;
      if (r.status) passedRules++;
      else fail({ severity: "error", path: `${prefix}.resource.status`, message: "MedicationRequest.status is required" });

      checkedRules++;
      if (r.intent) passedRules++;
      else fail({ severity: "error", path: `${prefix}.resource.intent`, message: "MedicationRequest.intent is required" });

      checkedRules++;
      if (r.medicationCodeableConcept) passedRules++;
      else fail({ severity: "error", path: `${prefix}.resource.medication[x]`, message: "MedicationRequest must have medication[x] (e.g. medicationCodeableConcept)" });

      checkedRules++;
      if (r.subject?.reference?.startsWith("Patient/")) passedRules++;
      else fail({ severity: "error", path: `${prefix}.resource.subject`, message: "MedicationRequest.subject must reference a Patient" });
    }
  });

  // Warn about missing expected resources
  const expected = ["Patient", "Encounter", "Observation", "Condition", "MedicationRequest"];
  expected.forEach(rt => {
    if (!resourceCounts[rt]) {
      warn({ severity: "warning", path: "Bundle.entry", message: `No ${rt} resource found in bundle — expected for a clinic visit record` });
    }
  });

  const errors = issues.filter(i => i.severity === "error").length;
  const score = checkedRules === 0 ? 0 : Math.round((passedRules / checkedRules) * 100);
  const valid = errors === 0;

  return {
    valid,
    score,
    issues,
    stats: {
      totalEntries: entries.length,
      resourceTypes: resourceCounts,
      checkedRules,
      passedRules,
    },
  };
}

export async function POST(req: NextRequest) {
  try {
    const bundle = await req.json();
    const result = validateBundle(bundle);
    return NextResponse.json(result, { status: result.valid ? 200 : 422 });
  } catch {
    return NextResponse.json({ error: "Invalid JSON body" }, { status: 400 });
  }
}
