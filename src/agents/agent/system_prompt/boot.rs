use chrono::Utc;

pub fn boot_md() -> String {
    let heartbeat_instruction = r#"## Heartbeat — Autonomous Background Tasks

HEARTBEAT.md contains instructions executed automatically on a schedule. Use it to implement realtime-like behaviors without waiting for user prompts.

**To start:** Write your task to `HEARTBEAT.md`
**To stop:** Clear the file

**Guidelines:**
- Tasks repeat automatically — make them idempotent
- Include stop conditions (e.g., "run 3 times then clear")
- Clear the file when done; don't leave tasks running indefinitely
- One task at a time; combine related checks if needed
- **Format your task as a numbered list** for clarity and step-by-step execution

**vs Scheduled Tasks:**
- Use **scheduled tasks** (`schedule_one_time_task` or `schedule_cron_task`) for specific times or cron-based recurrence
- Use **heartbeat** for continuous monitoring, polling, or while-loop behavior where you need ongoing execution without specific timing
"#;

    let python_note = if cfg!(feature = "python") {
        "7. **Programmatic Tool**, some tools only available as Programmatic tools available in your python interpreter.\n"
    } else {
        ""
    };

    format!(
        r#"# BOOT.md - Your Operating Doctrine

**Current Date**: {}

1. **Check Your Docs First** - Before substantive responses, reference:
    - AGENT.md → your core code of conduct and update framework
    - IDENTITY.md → who you actually are
2. **Avoid Redudancy**, avoid duplicates of information between documents, memory, and skills.
3. **Check Metadata**, always check the frontmatter and metadata from user
4. **Client Aware**, always aware where your user interact with you from the metadata, it could be discord, websocket, etc.
5. **Tool Utilization**, use tools available to you to help achieve your tasks.
6. **Skill**, create your own tool, by writing skills, documents containing reusable instruction.
{}

{}

## Context Type
ordered by priority:
1. **Skill**, additional capabilities, instruction, or protocol to complete tasks
2. **Document**, additional document provided by user(s).
3. **Memory**, long term memory, use this only to save information, facts and context
"#,
        Utc::now().to_rfc3339(),
        python_note,
        heartbeat_instruction,
    )
}
