# Feature Specification: Portable Memory as Open Documents

**Feature Branch**: `004-memory-open-format`

**Created**: 2026-08-25

**Status**: Draft

**Input**: User description: "currently memory is saved a regular row in db thus it is not portable and shareable, can we instead saved it as regular documents using open knowledge format"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Move an agent's memory to another deployment (Priority: P1)

An operator running Vizier wants to move an agent (or its memory) from one machine or environment to another — for example, migrating from a laptop to a server, or restoring from a backup. Today the memory lives as rows inside an internal database and cannot simply be copied out. The operator needs to be able to take an agent's memory along as a self-contained set of files and have it work correctly in the new place, with nothing lost.

**Why this priority**: This is the core complaint driving the request — memory is currently locked inside a database row and isn't portable. Solving this alone already delivers the primary value even before sharing or external editing are addressed.

**Independent Test**: Copy an agent's memory documents from one Vizier workspace to a fresh one and confirm the agent can query, browse, and traverse its memory graph exactly as before, with all documents present.

**Acceptance Scenarios**:

1. **Given** an agent with an established set of memories, **When** its memory documents are copied to a new Vizier workspace/deployment, **Then** the agent's memory search, memory listing, and related-memory graph all reflect the same content as the original, with no memories missing or corrupted.
2. **Given** an agent's memory has been copied to a new deployment, **When** the agent writes a new memory there, **Then** the new memory coexists correctly alongside the migrated ones (no id/slug collisions, no broken links).

---

### User Story 2 - Read and edit a memory document with standard tools (Priority: P2)

A developer or operator wants to inspect or correct a specific memory without going through Vizier's UI or API — for example, opening it in a text editor, a Markdown viewer, or a personal note-taking tool, reading its content and metadata (title, tags, when it was written), and optionally fixing a mistake directly in the file.

**Why this priority**: This is what "open knowledge format" is for — memory that any standard tool can open — but it depends on Story 1's document format existing first.

**Independent Test**: Open a single memory document outside of Vizier (e.g., in a plain text or Markdown editor) and confirm its title, tags, and content are legible and correctly structured without any Vizier-specific decoding step. Edit the file directly, then confirm the agent picks up the correction the next time it accesses that memory.

**Acceptance Scenarios**:

1. **Given** a memory document on disk, **When** it is opened in a standard text/Markdown editor, **Then** its title, tags, and content are human-readable without needing Vizier running.
2. **Given** a memory document edited directly on disk (outside Vizier), **When** the owning agent next reads or queries that memory, **Then** the corrected content is reflected.

---

### User Story 3 - Share a specific memory with another person or team (Priority: P3)

An operator wants to hand a single memory (or a curated set of them) to a colleague, or publish it into another knowledge base, without exporting or granting access to the entire agent database.

**Why this priority**: This is the "shareable" half of the request. It's valuable but naturally follows once memories exist as individually addressable documents (Story 1) in an open format (Story 2).

**Independent Test**: Select one memory document, copy just that file (and its attachments, if any) to someone else, and confirm the recipient can read its full content and metadata using only standard tools — no access to the source Vizier instance required.

**Acceptance Scenarios**:

1. **Given** a memory document with no attachments, **When** the operator copies that single file to another person, **Then** the recipient can read the complete title, tags, and content without any other files or the running Vizier instance.
2. **Given** a memory document that has attachments, **When** the operator shares that memory, **Then** the attachments travel with it and remain associated with the correct document.

### Edge Cases

- What happens when two memories for the same agent end up with the same title/slug (e.g., after a copy or a manual edit)? The system must not silently overwrite one with the other.
- What happens when a memory document referenced by another memory's link is missing, moved, or renamed (e.g., after a partial copy)? Related-memory lookups must degrade gracefully rather than erroring out.
- What happens when a memory document is deleted directly on disk while Vizier is running? The next query/listing must not surface a stale entry or crash.
- What happens when a memory marked private (or shared only to specific agents) is copied out of the app as a plain file? The file itself carries no enforcement once outside Vizier — this must be a documented, expected limitation, not a silent security gap.
- What happens when an agent accumulates a very large number of memory documents? Search, listing, and graph traversal must remain responsive.
- What happens when a memory document's metadata (frontmatter) is malformed after a manual external edit? The system must handle it without crashing, and should surface a clear error rather than losing the memory.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST persist each memory entry as an individual, self-contained document (metadata plus content) rather than as an opaque row in an internal database.
- **FR-002**: The document format MUST be plain-text and human-readable, using a widely recognized, openly documented convention (not a proprietary or Vizier-specific binary encoding), so it can be opened and understood with standard, commonly available text/Markdown tools.
- **FR-003**: Each memory document MUST carry all metadata currently associated with that memory — title, slug, tags, visibility, shared-to list, creation/update time, and read count — legibly within or alongside the document, not in a separate hidden store.
- **FR-004**: System MUST continue to support all existing memory capabilities (semantic search/query, filtered listing, memory detail lookup, related-memory graph traversal, read-count tracking) using the document store as the source of truth.
- **FR-005**: Links between related memory documents MUST remain resolvable when the referenced documents are present, and MUST fail gracefully (not crash or corrupt other data) when a linked document is missing.
- **FR-006**: System MUST allow an agent's full set of memory documents to be copied to a different Vizier workspace/deployment and load there with no data loss and no id/slug collisions with memories already present.
- **FR-007**: System MUST continue enforcing memory visibility rules (private vs. shared-to-specific-agents) for all access performed through Vizier itself, while documenting that a document copied outside the app is no longer protected by that enforcement.
- **FR-008**: System MUST keep each memory's attachments associated with their owning document such that copying or sharing the document also identifies which attachment files belong with it.
- **FR-009**: System MUST detect and reject (or safely rename) a new memory document whose title/slug would collide with an existing one for the same agent, rather than overwriting the existing document.
- **FR-010**: System MUST provide a migration path so memories already stored under the previous (row-based) mechanism are carried forward and remain accessible after the system adopts document-based storage — no existing memory should become unreadable or vanish as a result of this change.
- **FR-011**: System MUST tolerate a memory document being edited or removed directly on disk between Vizier operations, reflecting the change (or its absence) on next access rather than serving stale or corrupted data.

### Key Entities

- **Memory Document**: A single unit of an agent's long-term memory, stored as one self-contained, human-readable file combining metadata (title, slug, tags, visibility, shared-to list, timestamps, read count) and free-form content. Replaces the current database-row representation.
- **Memory Link**: A reference from one memory document to another (used for related-memory lookups and the memory graph), expressed in a way that survives being read outside of Vizier.
- **Attachment**: A supplementary file associated with a specific memory document (e.g., an image or file the memory refers to), which must stay identifiable as belonging to that document when the document is copied or shared.
- **Agent**: The owner and primary reader/writer of a scoped collection of memory documents; memory remains partitioned per agent.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of an agent's memory documents survive a copy to a new deployment with content, tags, and links intact, verified by comparing the memory listing before and after.
- **SC-002**: A person with no access to a running Vizier instance can read the full title, tags, and content of a shared memory document using only a standard text or Markdown viewer.
- **SC-003**: Memory search and related-memory lookups continue to return results in under 1 second for an agent with up to 5,000 memory documents, matching current responsiveness.
- **SC-004**: 100% of memories that existed before the change to document-based storage remain queryable and readable after upgrade, with no manual re-entry required.
- **SC-005**: An operator can hand off one specific memory (plus its attachments, if any) by transferring a small, self-contained set of files, without exporting or granting access to any other agent data.

## Assumptions

- "Open knowledge format" is interpreted as a plain-text document convention with structured metadata (e.g., Markdown with a frontmatter-style metadata block) — the same general style already used for an agent's CORE.md identity document — rather than a binary or proprietary format.
- Reconciliation of manual, out-of-band edits to memory documents (Edge Cases, FR-011) happens lazily, on the next relevant read/query/write, rather than through continuous real-time filesystem watching.
- Enforcement of memory visibility (private / shared-to-specific-agents) is an application-layer concern: it governs access through Vizier's own APIs and tools, not file-permission-level protection of the document once it leaves the app (see FR-007).
- Each memory document is stored as a single named file per memory, scoped under its owning agent, avoiding collisions with other agents' memory by construction (namespacing by agent).
- Existing consumers of the memory data (search/query, filtering, memory graph, WebUI memory views) will be re-pointed to the new document store rather than needing a parallel legacy path once migration completes.
