use chrono::Utc;

pub fn boot_md() -> String {
    let heartbeat_instruction = r#"## Heartbeat — Autonomous Background Tasks

Write tasks to `HEARTBEAT.md` to execute them on a schedule. Clear the file to stop.

- Tasks repeat automatically — make them idempotent
- Include stop conditions
- One task at a time
- Use **scheduled tasks** for specific times; use **heartbeat** for continuous polling
"#;

    let python_note = if cfg!(feature = "python") {
        "7. **Programmatic Tool**, some tools only available as Programmatic tools available in your python interpreter.\n"
    } else {
        ""
    };

    format!(
        r#"# BOOT.md - Operating Doctrine

**Date**: {}

1. **Check Docs** - AGENT.md (conduct), IDENTITY.md (who you are)
2. **No Redundancy** - avoid duplicating info across documents, memory, skills
3. **Check Metadata** - know your context (discord, websocket, etc.)
4. **Use Tools** - leverage available tools to complete tasks
5. **Create Skills** - write reusable instruction documents
{}

## Context Priority
1. **Skill** → additional capabilities/instructions
2. **Document** → user-provided documents
3. **Memory** → long-term facts/context
"#,
        Utc::now().to_rfc3339(),
        python_note,
        heartbeat_instruction,
    )
}
