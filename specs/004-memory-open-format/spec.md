# Feature Specification: Agent Memory as Open Documents

**Feature Branch**: `004-memory-open-format`

**Created**: 2026-08-25

**Status**: Draft

**Input**: User description: "currently memory is saved a regular row in db thus it is not portable and shareable, can we instead saved it as regular documents using open knowledge format"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Agent keeps writing, recalling, and linking its own memory without regression (Priority: P1)

An agent is the primary reader and writer of its own memory: it writes new memories as it learns things, searches its memory to recall relevant knowledge, follows links between related memories, and revisits memories it wrote earlier. Today that memory lives as rows in an internal database, addressed only through a narrow purpose-built API. Moving memory to individual, self-contained documents must not cost the agent anything it can already do — every write, search, link, and lookup the agent relies on has to keep working exactly as before, just backed by a document instead of a row.

**Why this priority**: Everything else this change is meant to unlock (human readability, portability) only matters if the agent's own core memory workflow is preserved. If document storage weakens what the agent can already do with its memory, the feature has failed regardless of any other benefit.

**Independent Test**: Have an agent perform its normal memory lifecycle — write a memory, search for it semantically, link it to a related memory, read it back, have its read count increment — and confirm every step behaves identically to the current row-based implementation, purely with a document as the underlying storage.

**Acceptance Scenarios**:

1. **Given** an agent writing a new memory, **When** it saves title, content, tags, and visibility through its normal memory tool, **Then** that memory is durably stored and immediately retrievable with all fields intact.
2. **Given** an agent with existing memories, **When** it performs a semantic search/query over its memory, **Then** it gets back the same relevant results it would have gotten under the previous storage, with no drop in relevance or coverage.
3. **Given** two memories the agent has linked together, **When** the agent looks up related memories for one of them, **Then** the other is returned via the link, exactly as today's memory graph behaves.

---

### User Story 2 - A developer can understand what an agent remembers by reading the raw file (Priority: P2)

A developer or operator debugging or auditing an agent's behavior wants to see exactly what a specific memory contains — its title, tags, and content — without going through the app's UI or API, by opening the underlying document directly in a plain text or Markdown viewer. This turns memory from an opaque internal detail into something a person can inspect directly when trying to understand why an agent behaved the way it did.

**Why this priority**: This is the direct payoff of storing memory as an open, human-readable document rather than a database row — it depends on Story 1's document representation already existing, and it's the most immediate way that representation earns its value beyond parity with the old system.

**Independent Test**: Open a single memory document outside of Vizier (e.g., in a plain text or Markdown editor) and confirm its title, tags, and content are legible and correctly structured without any Vizier-specific decoding step.

**Acceptance Scenarios**:

1. **Given** a memory document on disk, **When** it is opened in a standard text/Markdown editor, **Then** its title, tags, and content are human-readable without needing Vizier running.
2. **Given** a memory document edited directly on disk (e.g., a developer fixing a factual error the agent recorded), **When** the owning agent next reads or queries that memory, **Then** the corrected content is what the agent sees.

---

### User Story 3 - An agent's memory travels with it across deployments (Priority: P3)

An operator moving an agent between environments — migrating from a laptop to a server, restoring from a backup, or standing up a duplicate deployment — wants the agent's accumulated memory to move along with it as a self-contained set of files, rather than being locked inside a database that has to be exported/imported through special tooling.

**Why this priority**: Portability is a natural consequence of memory already being individual documents (Story 1) in an open format (Story 2), but it's an operational convenience rather than something the agent itself needs day-to-day, so it's addressed after the agent-facing behavior is solid.

**Independent Test**: Copy an agent's memory documents from one Vizier workspace to a fresh one and confirm the agent can query, browse, and traverse its memory graph exactly as before, with all documents present.

**Acceptance Scenarios**:

1. **Given** an agent with an established set of memories, **When** its memory documents are copied to a new Vizier workspace/deployment, **Then** the agent's memory search, memory listing, and related-memory graph all reflect the same content as the original, with no memories missing or corrupted.
2. **Given** an agent's memory has been copied to a new deployment, **When** the agent writes a new memory there, **Then** the new memory coexists correctly alongside the migrated ones (no id/slug collisions, no broken links).

### Edge Cases

- What happens when two memories for the same agent end up with the same title/slug (e.g., after a copy or a manual edit)? The system must not silently overwrite one with the other.
- What happens when a memory document referenced by another memory's link is missing, moved, or renamed (e.g., after a partial copy)? The agent's related-memory lookups must degrade gracefully rather than erroring out.
- What happens when a memory document is deleted directly on disk while Vizier is running? The next query/listing the agent performs must not surface a stale entry or crash.
- What happens when an agent accumulates a very large number of memory documents? Search, listing, and graph traversal must remain responsive enough not to slow down the agent's own reasoning loop.
- What happens when a memory document's metadata (frontmatter) is malformed after a manual external edit? The system must handle it without crashing the agent's memory operations, and should surface a clear error rather than silently losing the memory.
- What happens when a memory marked private, or shared only to specific agents, is read directly as a file outside of Vizier? The file itself carries no access-control enforcement once read outside the app — this is a documented, expected limitation of file-based storage, not a silent security gap.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST let an agent continue to create, update, semantically search, filter, and delete its own memories through its existing memory operations, with no regression in behavior versus the current row-based storage.
- **FR-002**: Each memory document MUST preserve everything the agent wrote to it — title, content, tags, visibility, shared-to list — with the same fidelity on read-back as on write.
- **FR-003**: System MUST continue to support the memory graph and related-memory links an agent relies on to recall connected knowledge, resolving them the same way regardless of the underlying storage representation.
- **FR-004**: System MUST continue tracking per-memory metadata the memory system depends on (read count, creation/update timestamps) as part of each document.
- **FR-005**: System MUST persist each memory entry as an individual, self-contained document (metadata plus content) rather than as an opaque row in an internal database.
- **FR-006**: The document format MUST be plain-text and human-readable, using a widely recognized, openly documented convention (not a proprietary or Vizier-specific binary encoding), so both the agent's own tools and standard external editors can read and understand it.
- **FR-007**: A developer or operator MUST be able to open any memory document directly, outside of Vizier's UI or API, and read its title, tags, and content, in order to understand what a given agent remembers.
- **FR-008**: System MUST detect and reject (or safely rename) a new memory document whose title/slug would collide with an existing one for the same agent, protecting the agent from silently overwriting its own memory.
- **FR-009**: System MUST tolerate a memory document being edited or removed directly on disk between Vizier operations, reflecting the change (or its absence) the next time the agent accesses that memory, rather than serving stale or corrupted data.
- **FR-010**: System MUST continue enforcing an agent's configured memory visibility rules (private vs. shared-to-specific-agents) for all access performed through Vizier itself, while documenting that a document read outside the app is no longer protected by that enforcement.
- **FR-011**: System MUST allow an agent's full set of memory documents to be copied to a different Vizier workspace/deployment and load there with no data loss and no id/slug collisions with memories already present.
- **FR-012**: System MUST provide a migration path so memories already stored under the previous (row-based) mechanism are carried forward and remain accessible after the system adopts document-based storage — no existing memory should become unreadable or vanish as a result of this change.
- **FR-013**: System MUST keep each memory's attachments associated with their owning document so that copying an agent's memory (e.g., for migration) does not silently drop attachments.

### Key Entities

- **Memory Document**: A single unit of an agent's long-term memory, stored as one self-contained, human-readable file combining metadata (title, slug, tags, visibility, shared-to list, timestamps, read count) and free-form content. Replaces the current database-row representation. The agent itself is the primary author and reader of these documents.
- **Memory Link**: A reference from one memory document to another (used for related-memory lookups and the memory graph), expressed in a way that survives being read outside of Vizier.
- **Attachment**: A supplementary file associated with a specific memory document (e.g., an image or file the memory refers to), which must stay identifiable as belonging to that document.
- **Agent**: The owner and primary reader/writer of a scoped collection of memory documents; memory remains partitioned per agent.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of an agent's existing memory operations (write, search/query, link, list, delete) succeed with document storage, with no functional regression versus current behavior.
- **SC-002**: Memory search and related-memory lookups continue to return results in under 1 second for an agent with up to 5,000 memory documents, matching current responsiveness.
- **SC-003**: A developer can open any single memory document in a standard text/Markdown viewer and read its full title, tags, and content without running Vizier.
- **SC-004**: 100% of memories that existed before the change to document-based storage remain queryable and readable by their owning agent after upgrade, with no manual re-entry required.
- **SC-005**: An agent's full memory set can be copied to a new deployment with 100% of documents, tags, and links intact.

## Assumptions

- The primary consumer of memory documents is the owning agent itself, through its existing memory tools; human readability and portability are valuable properties that fall out of that representation, not independent goals pursued at the expense of the agent's own workflow.
- Sharing an individual memory with another person or team outside of Vizier (e.g., handing off a single file to a colleague) is out of scope for this spec — it may be revisited separately once agent-facing document storage and human-readability are in place.
- "Open knowledge format" is interpreted as a plain-text document convention with structured metadata (e.g., Markdown with a frontmatter-style metadata block) — the same general style already used for an agent's CORE.md identity document — rather than a binary or proprietary format.
- Reconciliation of manual, out-of-band edits to memory documents (Edge Cases, FR-009) happens lazily, on the next relevant read/query/write, rather than through continuous real-time filesystem watching.
- Enforcement of memory visibility (private / shared-to-specific-agents) is an application-layer concern: it governs access through Vizier's own APIs and tools, not file-permission-level protection of the document once it leaves the app (see FR-010).
- Each memory document is stored as a single named file per memory, scoped under its owning agent, avoiding collisions with other agents' memory by construction (namespacing by agent).
- Existing consumers of the memory data (search/query, filtering, memory graph, WebUI memory views) will be re-pointed to the new document store rather than needing a parallel legacy path once migration completes.
