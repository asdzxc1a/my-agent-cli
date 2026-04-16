# Next Steps: Nova Producer OS

## Completed (Phase 5)

- ✅ Fix pre-existing `CARGO_BIN_EXE_claw` → `CARGO_BIN_EXE_nova` test regression
- ✅ Add animated progress bars to `/run` with real-time agent completion and synthesis bars
- ✅ Finalize demo workspace with realistic artifacts (SLATE_REPORT.md, PITCH_DECK.md, BUDGET_MODEL.json)
- ✅ Update `docs/cannes-demo.md` to match pre-staged artifacts and demo flow
- ✅ Full CI verification: `cargo check --workspace`, `cargo build -p nova-cli`, `cargo test --workspace` all green
- ✅ Commit and push Phase 5 completion

## Medium Priority (Post-Cannes)

### 1. Retry Logic
- Implement real `/run retry` behavior: reload last run JSON, identify failed/blocked steps, re-run only those agents, re-synthesize

### 2. Approval Resolution
- Add `/approvals approve <id>` and `/approvals reject <id>` commands
- Wire approval resolution into compliance scan so approved runs proceed to synthesis and stage completion

### 3. Agent Prompt Injection
- Move agent system prompts from stubs to real, production-ready prompts in `agent_prompts.rs`
- Integrate with actual LLM calls via the existing `Agent` tool mechanism (currently agents are simulated threads)

### 4. External Integrations
- Connect Distribution Analyst to web search
- Connect Pre-Viz Director to image generation MCP tools

## Architecture Reminders

- **Always run tests after changes:** `cargo test -p runtime --test producer_domain_tests && cargo test -p tools --test producer_slate_e2e && cargo test -p tools --test producer_pipeline_e2e && cargo test -p tools --test producer_approval_e2e`
- **Commit message format:** `feat(producer): <description>`
- **Git remote:** `https://github.com/asdzxc1a/my-agent-cli.git`
- **Project root:** `my-agent-cli/`
- **Build command:** `cd my-agent-cli/rust && cargo build -p nova-cli`
- **Binary path:** `my-agent-cli/rust/target/debug/nova`
