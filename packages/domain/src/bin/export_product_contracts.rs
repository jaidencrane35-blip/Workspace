//! Emit Product Proof TypeScript contracts from Rust domain types.
//!
//! Usage:
//!   cargo run -p workspace-domain --bin export_product_contracts -- <out.ts>

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

use ts_rs::TS;
use workspace_domain::desktop_action::{
    ActionItemOutcome, ActionOperationResult, ActionPlan, ActionPlanItem, ActionTargetDescriptor,
    ItemDisposition, OperationOutcome, ProjectedDisposition, ProposedEffect,
    RestoreCompatibilitySummary, RestoreExecutionSummary, ResumePlanPreview,
};
use workspace_domain::ids::{SavedContextId, WorkspaceId};
use workspace_domain::pilot_measurement::{
    PilotBaseline, PilotConsent, PilotInterviewPhase, PilotInterviewRecord, PilotLeaveResumeRecord,
    PilotMeasurementScope, PilotMeasurementSnapshot, PilotScopeItem,
};
use workspace_domain::product_proof_contracts::ProductProofContractRoot;
use workspace_domain::saved_context::{
    SavedContext, SavedContextCaptureScope, SavedContextMonitor, SavedContextRestoreIdentity,
    SavedContextScopeItem, SavedContextWindow,
};
use workspace_domain::workspace::Workspace;

fn append_decl<T: TS>(out: &mut String) {
    let decl = T::decl();
    // ts-rs emits `type X = ...`; Experience imports need `export type`.
    if let Some(rest) = decl.strip_prefix("type ") {
        out.push_str("export type ");
        out.push_str(rest);
    } else {
        out.push_str(&decl);
    }
    out.push_str("\n\n");
}

fn render_contracts() -> String {
    let _ = ProductProofContractRoot::name();

    let mut body = String::new();
    append_decl::<WorkspaceId>(&mut body);
    append_decl::<SavedContextId>(&mut body);
    append_decl::<ProjectedDisposition>(&mut body);
    append_decl::<ItemDisposition>(&mut body);
    append_decl::<OperationOutcome>(&mut body);
    append_decl::<ProposedEffect>(&mut body);
    append_decl::<PilotInterviewPhase>(&mut body);
    append_decl::<SavedContextScopeItem>(&mut body);
    append_decl::<SavedContextCaptureScope>(&mut body);
    append_decl::<SavedContextRestoreIdentity>(&mut body);
    append_decl::<SavedContextWindow>(&mut body);
    append_decl::<SavedContextMonitor>(&mut body);
    append_decl::<SavedContext>(&mut body);
    append_decl::<Workspace>(&mut body);
    append_decl::<ActionTargetDescriptor>(&mut body);
    append_decl::<ActionPlanItem>(&mut body);
    append_decl::<ActionPlan>(&mut body);
    append_decl::<RestoreCompatibilitySummary>(&mut body);
    append_decl::<ResumePlanPreview>(&mut body);
    append_decl::<ActionItemOutcome>(&mut body);
    append_decl::<RestoreExecutionSummary>(&mut body);
    append_decl::<ActionOperationResult>(&mut body);
    append_decl::<PilotScopeItem>(&mut body);
    append_decl::<PilotMeasurementScope>(&mut body);
    append_decl::<PilotConsent>(&mut body);
    append_decl::<PilotBaseline>(&mut body);
    append_decl::<PilotLeaveResumeRecord>(&mut body);
    append_decl::<PilotInterviewRecord>(&mut body);
    append_decl::<PilotMeasurementSnapshot>(&mut body);
    body
}

fn main() {
    let out = env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../app/src/generated/productContracts.ts")
    });

    let header = r#"/** AUTO-GENERATED — do not edit.
 * Source: packages/domain (Product Proof Experience wire types)
 * Regenerate: pnpm sync:contracts
 * Verify: pnpm verify:contracts
 * Constitution: ARCHITECTURAL_CONSTITUTION_V2.md §4.4 / §4.7
 */

"#;

    let contents = format!("{header}{}", render_contracts());

    if let Some(parent) = out.parent() {
        if let Err(error) = fs::create_dir_all(parent) {
            eprintln!(
                "export_product_contracts: mkdir {}: {error}",
                parent.display()
            );
            process::exit(1);
        }
    }

    if let Err(error) = fs::write(&out, &contents) {
        eprintln!(
            "export_product_contracts: write {}: {error}",
            out.display()
        );
        process::exit(1);
    }

    println!(
        "export_product_contracts: wrote {} ({} bytes)",
        out.display(),
        contents.len()
    );
}
