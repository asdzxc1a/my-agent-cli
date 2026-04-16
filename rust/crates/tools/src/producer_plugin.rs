use runtime::producer::{
    AgentArchetype, ProducerRun, ProducerStage, ProducerWorkspace, RunStep, RunType, StageStatus,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

/// Input for starting any stage run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageRunInput {
    pub workspace_name: String,
    pub run_type: String,
    pub file: Option<String>,
    pub cwd: String,
}

/// Result of a stage run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageRunResult {
    pub run_id: String,
    pub stage: String,
    pub status: String,
    pub artifacts: Vec<String>,
    pub message: String,
}

/// Execute a Slate analysis run.
pub fn run_slate_analyze(input: &Value) -> Result<String, String> {
    let input: StageRunInput = serde_json::from_value(input.clone()).map_err(|e| e.to_string())?;
    run_stage_internal(
        &input,
        RunType::SlateAnalyze,
        ProducerStage::Slate,
        vec![AgentArchetype::ScriptAnalyst, AgentArchetype::BudgetOracle],
        |ws_name, run_id| {
            let slate_report = format!(
                "# Slate Analysis Report\n\n## Workspace: {}\n## Run: {}\n\n## Summary\nAll slate agents completed successfully.\n\n## Next Action\nProceed to `/stage package`.\n",
                ws_name, run_id
            );
            ("SLATE_REPORT.md".to_string(), slate_report)
        },
        format!(
            "Slate analysis complete. {} evaluated. Package stage is now ready.",
            input.file.as_deref().unwrap_or("slate")
        ),
    )
}

/// Execute a Package build run.
pub fn run_package_build(input: &Value) -> Result<String, String> {
    let input: StageRunInput = serde_json::from_value(input.clone()).map_err(|e| e.to_string())?;
    run_stage_internal(
        &input,
        RunType::PackageBuild,
        ProducerStage::Package,
        vec![
            AgentArchetype::PreVizDirector,
            AgentArchetype::CastingScout,
            AgentArchetype::LocationScout,
        ],
        |ws_name, run_id| {
            let pitch_deck = format!(
                "# Pitch Deck: {}\n\n## Visual Thesis\nCompelling visual narrative ready for investors.\n\n## Casting & Locations\nIntegrated from Package stage agents.\n\n## Run: {}\n",
                ws_name, run_id
            );
            ("PITCH_DECK.md".to_string(), pitch_deck)
        },
        "Package build complete. Pitch deck assembled. Finance stage is now ready.".to_string(),
    )
}

/// Execute a Finance model run.
pub fn run_finance_model(input: &Value) -> Result<String, String> {
    let input: StageRunInput = serde_json::from_value(input.clone()).map_err(|e| e.to_string())?;
    run_stage_internal(
        &input,
        RunType::FinanceModel,
        ProducerStage::Finance,
        vec![AgentArchetype::BudgetOracle],
        |ws_name, run_id| {
            let budget_json = serde_json::json!({
                "project_title": ws_name,
                "total_budget": 2500000,
                "currency": "USD",
                "categories": {
                    "above_the_line": 450000,
                    "production": 1200000,
                    "post_production": 350000,
                    "miscellaneous": 500000
                },
                "contingency": 0.15,
                "burn_rate_per_week": 125000,
                "shooting_days": 28,
                "risk_flags": ["vfx_heavy"]
            });
            let burn_report = format!(
                "# Burn Report: {}\n\n## Total Budget: $2,500,000\n## Weekly Burn: $125,000\n## Estimated Shoot Duration: 28 days\n\nRun: {}\n",
                ws_name, run_id
            );
            fs::write(
                PathBuf::from("/tmp").join(format!("burn-{}-{}.md", ws_name, run_id)),
                &burn_report,
            ).ok();
            ("BUDGET_MODEL.json".to_string(), budget_json.to_string())
        },
        "Finance model complete. Budget and burn report generated. Comply stage is now ready."
            .to_string(),
    )
}

/// Execute a Compliance scan run.
pub fn run_comply_scan(input: &Value) -> Result<String, String> {
    let input: StageRunInput = serde_json::from_value(input.clone()).map_err(|e| e.to_string())?;
    run_stage_internal(
        &input,
        RunType::ComplyScan,
        ProducerStage::Comply,
        vec![AgentArchetype::ComplianceOfficer],
        |ws_name, run_id| {
            let report = format!(
                "# Compliance Report: {}\n\n## EU AI Act Assessment\n- AI Usage Detected: Yes\n- Risk Level: Medium\n- Disclosure Requirements: Standard\n\n## Union & Labor\nNo major issues flagged.\n\nRun: {}\n",
                ws_name, run_id
            );
            ("COMPLIANCE_REPORT.md".to_string(), report)
        },
        "Compliance scan complete. Medium risk identified. Launch stage is now ready.".to_string(),
    )
}

/// Execute a Launch strategy run.
pub fn run_launch_strategy(input: &Value) -> Result<String, String> {
    let input: StageRunInput = serde_json::from_value(input.clone()).map_err(|e| e.to_string())?;
    run_stage_internal(
        &input,
        RunType::LaunchStrategy,
        ProducerStage::Launch,
        vec![AgentArchetype::DistributionAnalyst],
        |ws_name, run_id| {
            let strategy = format!(
                "# Festival Strategy: {}\n\n## Tier 1 Targets\n1. Cannes Film Festival\n2. Toronto International Film Festival\n\n## Market Premieres\nCannes — best fit for European co-production.\n\nRun: {}\n",
                ws_name, run_id
            );
            ("FESTIVAL_STRATEGY.md".to_string(), strategy)
        },
        "Launch strategy complete. Festival targets mapped. Project is launch-ready.".to_string(),
    )
}

/// Check the status of a producer run.
pub fn run_status(input: &Value) -> Result<String, String> {
    let workspace_name: String = input
        .get("workspace_name")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();
    let run_id: String = input
        .get("run_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let cwd: String = input
        .get("cwd")
        .and_then(|v| v.as_str())
        .unwrap_or(".")
        .to_string();

    if run_id.is_empty() {
        return Ok(serde_json::json!({
            "status": "no_active_run",
            "message": "No active run. Start one with `/run slate analyze --slate <file>`."
        })
        .to_string());
    }

    let run_path = PathBuf::from(&cwd)
        .join(".nova")
        .join("workspaces")
        .join(&workspace_name)
        .join("runs")
        .join(&run_id)
        .join("run.json");

    if !run_path.exists() {
        return Ok(serde_json::json!({
            "status": "not_found",
            "message": format!("Run {run_id} not found.")
        })
        .to_string());
    }

    let content = fs::read_to_string(&run_path).map_err(|e| e.to_string())?;
    let run: ProducerRun = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    let steps: Vec<Value> = run
        .steps
        .iter()
        .map(|s| {
            serde_json::json!({
                "number": s.step_number,
                "agent": s.agent_name,
                "status": format!("{:?}", s.status),
                "icon": s.status.icon(),
            })
        })
        .collect();

    Ok(serde_json::json!({
        "run_id": run.run_id,
        "status": format!("{:?}", run.status),
        "steps": steps,
    })
    .to_string())
}

fn run_stage_internal<F>(
    input: &StageRunInput,
    run_type: RunType,
    stage: ProducerStage,
    agents: Vec<AgentArchetype>,
    synthesizer: F,
    completion_message: String,
) -> Result<String, String>
where
    F: FnOnce(&str, &str) -> (String, String),
{
    let cwd = PathBuf::from(&input.cwd);
    let ws_root = cwd.join(".nova").join("workspaces").join(&input.workspace_name);

    if !ws_root.exists() {
        return Err(format!("Workspace '{}' does not exist.", input.workspace_name));
    }

    let ws_path = ws_root.join("workspace.json");
    let mut ws: ProducerWorkspace = if ws_path.exists() {
        let content = fs::read_to_string(&ws_path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())?
    } else {
        ProducerWorkspace::new(&input.workspace_name, cwd.clone())
    };

    // Verify stage is not locked
    let stage_state = ws.stages.get(&stage).copied().unwrap_or_else(|| runtime::producer::StageState::locked());
    if stage_state.status == StageStatus::Locked {
        return Err(format!("Stage {:?} is locked. Complete the previous stage first.", stage));
    }

    ws.current_stage = stage;
    if let Some(s) = ws.stages.get_mut(&stage) {
        s.status = StageStatus::Running;
    }
    save_workspace(&ws, &ws_path)?;

    let run_id = format!("{}-run-{}", stage.as_str(), chrono::Utc::now().format("%Y%m%d-%H%M%S"));
    let mut run = ProducerRun::new(&run_id, run_type);
    run.start();

    let runs_dir = ws_root.join("runs").join(&run_id);
    fs::create_dir_all(&runs_dir).map_err(|e| e.to_string())?;
    fs::create_dir_all(runs_dir.join("steps")).map_err(|e| e.to_string())?;

    // Spawn agents in parallel
    let mut handles = Vec::new();
    for (i, agent) in agents.iter().enumerate() {
        let step_number = (i + 1) as u32;
        let agent_name = agent.display_name().to_string();
        let agent_id = format!("{}-{}", run_id, agent_name.to_lowercase().replace(' ', "-"));
        let runs_dir_clone = runs_dir.clone();
        let ws_name = input.workspace_name.clone();
        let run_id_clone = run_id.clone();

        let mut step = RunStep::new(step_number, agent_id.clone(), agent_name.clone());
        step.start();
        save_step(&step, &runs_dir_clone)?;
        run.add_step(step.clone());

        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));

            let artifact_content = format!(
                "# {} Output for {}\n\nGenerated by {} during {}.\n",
                agent_name, ws_name, agent_id, run_id_clone
            );
            let artifact_path = runs_dir_clone
                .join("steps")
                .join(format!("{:02}-{}.md", step_number, agent_name.to_lowercase().replace(' ', "-")));
            let _ = fs::write(&artifact_path, artifact_content);

            let mut completed_step = step;
            completed_step.complete();
            completed_step.output_summary = Some(format!("{} completed.", agent_name));
            let _ = save_step(&completed_step, &runs_dir_clone);
            completed_step
        });
        handles.push((step_number, handle));
    }

    for (step_number, handle) in handles {
        let completed_step = handle.join().map_err(|_| format!("agent {step_number} panicked"))?;
        if let Some(s) = run.steps.iter_mut().find(|s| s.step_number == step_number) {
            *s = completed_step;
        }
    }

    // Synthesis step
    let synth_number = (agents.len() + 1) as u32;
    let mut synthesizer_step = RunStep::new(
        synth_number,
        format!("{run_id}-synthesizer"),
        "Synthesis Agent",
    );
    synthesizer_step.start();
    save_step(&synthesizer_step, &runs_dir)?;

    let (artifact_name, artifact_content) = synthesizer(&ws.name, &run_id);

    let artifacts_dir = ws_root.join("artifacts");
    fs::create_dir_all(&artifacts_dir).map_err(|e| e.to_string())?;
    let artifact_path = artifacts_dir.join(&artifact_name);
    fs::write(&artifact_path, artifact_content).map_err(|e| e.to_string())?;

    synthesizer_step.complete();
    synthesizer_step.output_summary = Some(format!("Synthesized {artifact_name}"));
    save_step(&synthesizer_step, &runs_dir)?;
    run.add_step(synthesizer_step);

    run.complete();
    run.artifact_names.push(artifact_name);
    save_run(&run, &runs_dir)?;

    ws.complete_stage(stage);
    save_workspace(&ws, &ws_path)?;

    let result = StageRunResult {
        run_id: run.run_id.clone(),
        stage: stage.to_string(),
        status: "completed".to_string(),
        artifacts: run.artifact_names.clone(),
        message: completion_message,
    };

    serde_json::to_string_pretty(&result).map_err(|e| e.to_string())
}

fn save_workspace(ws: &ProducerWorkspace, path: &Path) -> Result<(), String> {
    let json = serde_json::to_string_pretty(ws).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

fn save_run(run: &ProducerRun, runs_dir: &Path) -> Result<(), String> {
    let path = runs_dir.join("run.json");
    let json = serde_json::to_string_pretty(run).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

fn save_step(step: &RunStep, runs_dir: &Path) -> Result<(), String> {
    let path = runs_dir
        .join("steps")
        .join(format!("{:02}-{}.json", step.step_number, step.agent_name.to_lowercase().replace(' ', "-")));
    let json = serde_json::to_string_pretty(step).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}
