# Feature Specification: Agent Memory as Open Documents

**Feature Branch**: `004-memory-open-format`

**Created**: 2026-08-25

**Status**: Draft

**Input**: User description: "currently memory is saved a regular row in db thus it is not portable and shareable, can we instead saved it as regular documents using open knowledge format"

## Clarifications

### Session 2026-08-25

- Q: What does "bundled form" mean for an open-knowledge-format memory store — what do the "multiple documents inside one knowledge" represent? → A: Per the referenced Open Knowledge Format spec (https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md), a bundle is a hierarchical directory of markdown documents with YAML frontmatter. An agent's memory store is one such bundle: each existing memory becomes one **concept document** (a single markdown file, individually addressable by its file path, mapping to the existing slug) inside the bundle, alongside two reserved documents — an **index document** (a directory listing enabling progressive disclosure of available memories) and a **log document** (chronological update history). Attachments remain non-document files inside the same bundle, referenced from their owning concept document.
- Q: On a memory slug/path collision, what should happen? → A: Reject the write and return an error to the agent, requiring it to pick a different slug/title — no silent auto-renaming.
- Q: Are the reserved `index` and `log` documents in scope for this feature? → A: Yes — the system auto-maintains both an index document (listing of current memories) and a log document (chronological update history) as part of an agent's bundle.

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
3. **Given** an agent's memory bundle, **When** a developer opens the bundle's index document, **Then** they see a listing of all of that agent's current memories without needing to open each concept document individually.

---

### User Story 3 - An agent's memory travels with it across deployments (Priority: P3)

An operator moving an agent between environments — migrating from a laptop to a server, restoring from a backup, or standing up a duplicate deployment — wants the agent's accumulated memory to move along with it as a self-contained set of files, rather than being locked inside a database that has to be exported/imported through special tooling.

**Why this priority**: Portability is a natural consequence of memory already being individual documents (Story 1) in an open format (Story 2), but it's an operational convenience rather than something the agent itself needs day-to-day, so it's addressed after the agent-facing behavior is solid.

**Independent Test**: Copy an agent's memory bundle from one Vizier workspace to a fresh one and confirm the agent can query, browse, and traverse its memory graph exactly as before, with all concept documents present.

**Acceptance Scenarios**:

1. **Given** an agent with an established set of memories, **When** its memory bundle is copied to a new Vizier workspace/deployment, **Then** the agent's memory search, memory listing, and related-memory graph all reflect the same content as the original, with no memories missing or corrupted.
2. **Given** an agent's memory has been copied to a new deployment, **When** the agent writes a new memory there, **Then** the new memory coexists correctly alongside the migrated ones (no id/slug collisions, no broken links).

### Edge Cases

- What happens when two memories for the same agent end up with the same title/slug (e.g., after a copy or a manual edit)? The system must not silently overwrite one with the other.
- What happens when a memory document referenced by another memory's link is missing, moved, or renamed (e.g., after a partial copy)? The agent's related-memory lookups must degrade gracefully rather than erroring out.
- What happens when a memory document is deleted directly on disk while Vizier is running? The next query/listing the agent performs must not surface a stale entry or crash.
- What happens when an agent accumulates a very large number of memory documents? Search, listing, and graph traversal must remain responsive enough not to slow down the agent's own reasoning loop.
- What happens when a memory document's metadata (frontmatter) is malformed after a manual external edit? The system must handle it without crashing the agent's memory operations, and should surface a clear error rather than silently losing the memory.
- What happens when a memory marked private, or shared only to specific agents, is read directly as a file outside of Vizier? The file itself carries no access-control enforcement once read outside the app — this is a documented, expected limitation of file-based storage, not a silent security gap.
- What happens when the index or log document falls out of sync with the concept documents actually present (e.g., after a manual edit or deletion outside Vizier)? They must be reconciled/regenerated on next access rather than showing a stale listing or history indefinitely.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST let an agent continue to create, update, semantically search, filter, and delete its own memories through its existing memory operations, with no regression in behavior versus the current row-based storage.
- **FR-002**: Each memory document MUST preserve everything the agent wrote to it — title, content, tags, visibility, shared-to list — with the same fidelity on read-back as on write.
- **FR-003**: System MUST continue to support the memory graph and related-memory links an agent relies on to recall connected knowledge, resolving them the same way regardless of the underlying storage representation.
- **FR-004**: System MUST continue tracking per-memory metadata the memory system depends on (read count, creation/update timestamps) as part of each document.
- **FR-005**: System MUST organize an agent's memory as a single hierarchical bundle (directory) containing one concept document per memory, per the Open Knowledge Format bundle convention, rather than as unrelated rows in an internal database.
- **FR-006**: Each concept document's format MUST be plain-text and human-readable, using a widely recognized, openly documented convention (markdown with structured frontmatter metadata, not a proprietary or Vizier-specific binary encoding), so both the agent's own tools and standard external editors can read and understand it.
- **FR-007**: A developer or operator MUST be able to open any memory document directly, outside of Vizier's UI or API, and read its title, tags, and content, in order to understand what a given agent remembers.
- **FR-008**: System MUST detect a new memory whose title/slug would collide with an existing concept document's path for the same agent and reject the write with a clear error, rather than overwriting the existing document or silently auto-renaming it.
- **FR-009**: System MUST tolerate a memory document being edited or removed directly on disk between Vizier operations, reflecting the change (or its absence) the next time the agent accesses that memory, rather than serving stale or corrupted data.
- **FR-010**: System MUST continue enforcing an agent's configured memory visibility rules (private vs. shared-to-specific-agents) for all access performed through Vizier itself, while documenting that a document read outside the app is no longer protected by that enforcement.
- **FR-011**: System MUST allow an agent's full memory bundle to be copied to a different Vizier workspace/deployment and load there with no data loss and no id/slug collisions with memories already present.
- **FR-012**: System MUST provide a migration path so memories already stored under the previous (row-based) mechanism are carried forward into the bundle and remain accessible after the system adopts document-based storage — no existing memory should become unreadable or vanish as a result of this change.
- **FR-013**: System MUST keep each memory's attachments as files within the same bundle, associated with their owning concept document, so that copying an agent's bundle (e.g., for migration) does not silently drop attachments.
- **FR-014**: System MUST automatically maintain the bundle's index document to reflect the current set of concept documents, kept up to date as memories are written, updated, or deleted, so a developer or agent can see what's available without opening every concept document individually.
- **FR-015**: System MUST automatically maintain the bundle's log document as a chronological history of memory writes and updates.

### Key Entities

- **Memory Bundle**: The hierarchical directory that holds one agent's entire memory store — a collection of concept documents plus the bundle's reserved index and log documents, per the Open Knowledge Format. Replaces the current database as the top-level unit of memory storage; one bundle per agent.
- **Memory Concept Document** *(formerly referred to as "Memory Document")*: A single markdown file within the bundle representing one memory — title, slug, tags, visibility, shared-to list, timestamps, and read count as frontmatter metadata, with free-form content as the body. Individually addressable by its file path within the bundle (maps to the existing slug). The agent itself is the primary author and reader of these documents.
- **Index Document**: The bundle's reserved listing document (e.g., `index.md`), giving a directory-level view of available memories so an agent or a human browsing the bundle can see what's there before opening individual concept documents.
- **Log Document**: The bundle's reserved chronological history document (e.g., `log.md`), recording memory updates over time.
- **Memory Link**: A reference from one concept document to another (used for related-memory lookups and the memory graph), expressed in a way that survives being read outside of Vizier.
- **Attachment**: A supplementary, non-document file stored within the bundle and associated with a specific concept document (e.g., an image or file the memory refers to), which must stay identifiable as belonging to that document.
- **Agent**: The owner and primary reader/writer of its own Memory Bundle; memory remains partitioned per agent.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of an agent's existing memory operations (write, search/query, link, list, delete) succeed with document storage, with no functional regression versus current behavior.
- **SC-002**: Memory search and related-memory lookups continue to return results in under 1 second for an agent with up to 5,000 memory documents, matching current responsiveness.
- **SC-003**: A developer can open any single memory document in a standard text/Markdown viewer and read its full title, tags, and content without running Vizier.
- **SC-004**: 100% of memories that existed before the change to document-based storage remain queryable and readable by their owning agent after upgrade, with no manual re-entry required.
- **SC-005**: An agent's full memory set can be copied to a new deployment with 100% of documents, tags, and links intact.
- **SC-006**: A bundle's index document reflects 100% of an agent's current memories after any write, update, or delete, verified by comparing the index listing to the concept documents actually present.

## Assumptions

- The primary consumer of memory documents is the owning agent itself, through its existing memory tools; human readability and portability are valuable properties that fall out of that representation, not independent goals pursued at the expense of the agent's own workflow.
- Sharing an individual memory with another person or team outside of Vizier (e.g., handing off a single file to a colleague) is out of scope for this spec — it may be revisited separately once agent-facing document storage and human-readability are in place.
- "Open knowledge format" refers specifically to the bundle convention described in the referenced Open Knowledge Format spec — a hierarchical directory of markdown documents with structured frontmatter metadata — the same general style already used for an agent's CORE.md identity document, rather than a binary or proprietary format.
- Reconciliation of manual, out-of-band edits to concept documents (Edge Cases, FR-009) happens lazily, on the next relevant read/query/write, rather than through continuous real-time filesystem watching.
- Enforcement of memory visibility (private / shared-to-specific-agents) is an application-layer concern: it governs access through Vizier's own APIs and tools, not file-permission-level protection of the document once it leaves the app (see FR-010).
- Each agent owns exactly one memory bundle (one concept document per memory within it), scoped under that agent, avoiding collisions with other agents' memory by construction (namespacing by agent/bundle).
- Existing consumers of the memory data (search/query, filtering, memory graph, WebUI memory views) will be re-pointed to the new bundle-backed store rather than needing a parallel legacy path once migration completes.
