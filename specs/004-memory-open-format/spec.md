# Feature Specification: Agent Memory as Open Documents

**Feature Branch**: `004-memory-open-format`

**Created**: 2026-08-25

**Status**: Draft

**Input**: User description: "currently memory is saved a regular row in db thus it is not portable and shareable, can we instead saved it as regular documents using open knowledge format"

## Clarifications

### Session 2026-08-25

- Q: What does "bundled form" mean for an open-knowledge-format memory store — what do the "multiple documents inside one knowledge" represent? → A: Per the referenced Open Knowledge Format spec (https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md), a bundle is a hierarchical directory of markdown documents with YAML frontmatter. An agent's memory is organized into concept documents (one markdown file per memory, individually addressable by its file path, mapping to the existing slug), alongside two reserved documents per bundle — an **index document** (a directory listing enabling progressive disclosure of available memories) and a **log document** (chronological update history). Attachments remain non-document files inside the same bundle, referenced from their owning concept document.
- Q: On a memory slug/path collision, what should happen? → A: Reject the write and return an error to the agent, requiring it to pick a different slug/title — no silent auto-renaming.
- Q: Are the reserved `index` and `log` documents in scope for this feature? → A: Yes — the system auto-maintains both an index document (listing of current memories) and a log document (chronological update history) as part of each bundle.
- Q: Should the existing private/global/shared memory visibility model (and `shared_to`) carry forward into the bundle model? → A: No — drop it entirely for this feature. Every memory becomes private to its owning agent; there is no cross-agent visibility or access of any kind. Memories previously marked `global` or `shared` become private on migration.
- Q: Can an agent organize its memory into more than one bundle, and can a concept in one bundle link to another bundle (same agent only)? → A: Yes — an agent may own multiple named bundles (e.g., one about a particular person, project, or subject). A concept document can link either to a specific concept in a different bundle owned by the same agent, or to another bundle as a whole (its index); both forms must resolve the same way intra-bundle links do.
- Q: How does a new bundle come into existence? → A: Implicitly — the agent names a bundle when writing a memory, and if it doesn't exist yet, it's created automatically. A write that doesn't name a bundle goes to that agent's default bundle, preserving today's single-collection experience for agents that don't need multiple bundles.
- Q: How should the WebUI memory graph represent an agent's multiple bundles and bundle-level links? → A: Two-level view — a top-level graph shows an agent's bundles as nodes with cross-bundle links as edges between them; opening a bundle switches to a concept-level graph showing that bundle's concept documents and their links, matching today's existing single-collection graph.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Agent keeps writing, recalling, and linking its own memory without regression (Priority: P1)

An agent is the primary reader and writer of its own memory: it writes new memories as it learns things, searches its memory to recall relevant knowledge, follows links between related memories, and revisits memories it wrote earlier. Today that memory lives as rows in an internal database, addressed only through a narrow purpose-built API. Moving memory to individual, self-contained documents — organized into one or more named bundles by topic — must not cost the agent anything it can already do — every write, search, link, and lookup the agent relies on has to keep working exactly as before, just backed by documents instead of rows.

**Why this priority**: Everything else this change is meant to unlock (human readability, portability) only matters if the agent's own core memory workflow is preserved. If document storage weakens what the agent can already do with its memory, the feature has failed regardless of any other benefit.

**Independent Test**: Have an agent perform its normal memory lifecycle — write a memory, search for it semantically, link it to a related memory (in the same bundle and in a different bundle it owns), read it back, have its read count increment — and confirm every step behaves identically to (or a strict superset of) the current row-based implementation, purely with documents as the underlying storage.

**Acceptance Scenarios**:

1. **Given** an agent writing a new memory without naming a bundle, **When** it saves title, content, and tags through its normal memory tool, **Then** that memory is durably stored as a concept document in its default bundle and immediately retrievable with all fields intact.
2. **Given** an agent with existing memories across multiple bundles, **When** it performs a semantic search/query over its memory, **Then** it gets back the same relevant results it would have gotten under the previous storage, with no drop in relevance or coverage.
3. **Given** two memories the agent has linked together within the same bundle, **When** the agent looks up related memories for one of them, **Then** the other is returned via the link, exactly as today's memory graph behaves.
4. **Given** a concept in one bundle that links to a specific concept in a different bundle owned by the same agent, **When** the agent looks up related memories for it, **Then** the cross-bundle concept is returned just as an intra-bundle link would be.
5. **Given** a concept that references another bundle as a whole rather than a specific concept, **When** the agent follows that reference, **Then** it resolves to that bundle's index, listing everything available there.
6. **Given** an agent writes a memory naming a bundle that doesn't exist yet, **When** the write completes, **Then** the bundle is created automatically with no separate creation step required.

---

### User Story 2 - A developer can understand what an agent remembers by reading the raw file (Priority: P2)

A developer or operator debugging or auditing an agent's behavior wants to see exactly what a specific memory contains — its title, tags, and content — without going through the app's UI or API, by opening the underlying document directly in a plain text or Markdown viewer. This turns memory from an opaque internal detail into something a person can inspect directly when trying to understand why an agent behaved the way it did.

**Why this priority**: This is the direct payoff of storing memory as an open, human-readable document rather than a database row — it depends on Story 1's document representation already existing, and it's the most immediate way that representation earns its value beyond parity with the old system.

**Independent Test**: Open a single memory document outside of Vizier (e.g., in a plain text or Markdown editor) and confirm its title, tags, and content are legible and correctly structured without any Vizier-specific decoding step.

**Acceptance Scenarios**:

1. **Given** a memory document on disk, **When** it is opened in a standard text/Markdown editor, **Then** its title, tags, and content are human-readable without needing Vizier running.
2. **Given** a memory document edited directly on disk (e.g., a developer fixing a factual error the agent recorded), **When** the owning agent next reads or queries that memory, **Then** the corrected content is what the agent sees.
3. **Given** one of an agent's memory bundles, **When** a developer opens that bundle's index document, **Then** they see a listing of all the memories in that bundle without needing to open each concept document individually.
4. **Given** an agent with multiple bundles, **When** a developer opens the WebUI memory graph, **Then** they first see a bundle-level graph (bundles as nodes, cross-bundle links as edges), and opening a bundle switches to a concept-level graph showing that bundle's memories and their links, matching today's single-collection graph experience.

---

### User Story 3 - An agent's memory travels with it across deployments (Priority: P3)

An operator moving an agent between environments — migrating from a laptop to a server, restoring from a backup, or standing up a duplicate deployment — wants the agent's accumulated memory to move along with it as a self-contained set of files, rather than being locked inside a database that has to be exported/imported through special tooling.

**Why this priority**: Portability is a natural consequence of memory already being individual documents (Story 1) in an open format (Story 2), but it's an operational convenience rather than something the agent itself needs day-to-day, so it's addressed after the agent-facing behavior is solid.

**Independent Test**: Copy an agent's memory bundles from one Vizier workspace to a fresh one and confirm the agent can query, browse, and traverse its memory graph exactly as before, with all concept documents present.

**Acceptance Scenarios**:

1. **Given** an agent with an established set of memories across one or more bundles, **When** those bundles are copied to a new Vizier workspace/deployment, **Then** the agent's memory search, memory listing, and related-memory graph all reflect the same content as the original, with no memories missing or corrupted.
2. **Given** an agent's memory has been copied to a new deployment, **When** the agent writes a new memory there, **Then** the new memory coexists correctly alongside the migrated ones (no id/slug collisions within a bundle, no broken links).

### Edge Cases

- What happens when two memories within the *same* bundle for the same agent end up with the same title/slug (e.g., after a copy or a manual edit)? The system must reject the write rather than silently overwriting one with the other.
- Using the same title/slug in two *different* bundles owned by the same agent is expected and must not be treated as a collision — each concept is addressed by its bundle plus path.
- What happens when a memory link — to a specific concept or to a whole bundle — points to something missing, moved, renamed, or to a bundle that no longer exists? The agent's related-memory lookups must degrade gracefully (a broken link) rather than erroring out.
- What happens when a memory document is deleted directly on disk while Vizier is running? The next query/listing the agent performs must not surface a stale entry or crash.
- What happens when an agent accumulates a very large number of bundles or memory documents? Search, listing, and graph traversal must remain responsive enough not to slow down the agent's own reasoning loop.
- What happens when a memory document's metadata (frontmatter) is malformed after a manual external edit? The system must handle it without crashing the agent's memory operations, and should surface a clear error rather than silently losing the memory.
- What happens when a bundle's index or log document falls out of sync with the concept documents actually present (e.g., after a manual edit or deletion outside Vizier)? They must be reconciled/regenerated on next access rather than showing a stale listing or history indefinitely.
- What happens to a memory that was previously marked `global` or `shared` when it's migrated into the new, all-private bundle model? It becomes private to its owning agent like every other memory — any cross-agent access it previously allowed is intentionally not preserved (see FR-013).
- What happens when a concept in the currently open bundle's concept-level graph has a link pointing outside that bundle? The concept-level graph must indicate the outward link (e.g., a boundary indicator pointing back to the bundle-level view) rather than silently dropping it or crashing.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST let an agent continue to create, update, semantically search, filter, and delete its own memories through its existing memory operations, with no regression in behavior versus the current row-based storage.
- **FR-002**: Each concept document MUST preserve everything the agent wrote to it — title, content, and tags — with the same fidelity on read-back as on write.
- **FR-003**: System MUST continue to support the memory graph and related-memory links an agent relies on to recall connected knowledge, including links that cross bundle boundaries (see FR-011), resolving them the same way regardless of the underlying storage representation.
- **FR-004**: System MUST continue tracking per-memory metadata the memory system depends on (read count, creation/update timestamps) as part of each concept document.
- **FR-005**: System MUST let an agent organize its memory into one or more named bundles — each a hierarchical directory of concept documents about a cohesive topic — per the Open Knowledge Format bundle convention, rather than a single flat table of unrelated database rows.
- **FR-006**: System MUST create a new bundle automatically the first time an agent's memory write names a bundle that doesn't already exist; a write that doesn't specify a bundle MUST go to that agent's default bundle, preserving today's single-collection experience for agents that don't need multiple bundles.
- **FR-007**: Each concept document's format MUST be plain-text and human-readable, using a widely recognized, openly documented convention (markdown with structured frontmatter metadata, not a proprietary or Vizier-specific binary encoding), so both the agent's own tools and standard external editors can read and understand it.
- **FR-008**: A developer or operator MUST be able to open any memory document directly, outside of Vizier's UI or API, and read its title, tags, and content, in order to understand what a given agent remembers.
- **FR-009**: System MUST detect a new memory whose title/slug would collide with an existing concept document's path *within the same bundle* and reject the write with a clear error, rather than overwriting the existing document or silently auto-renaming it; the same slug used in a different bundle owned by the same agent is not a collision.
- **FR-010**: System MUST tolerate a memory document being edited or removed directly on disk between Vizier operations, reflecting the change (or its absence) the next time the agent accesses that memory, rather than serving stale or corrupted data.
- **FR-011**: System MUST allow a concept document to link either to a specific concept in a different bundle owned by the same agent, or to another bundle as a whole (its index), and MUST resolve both forms through the same related-memory/graph lookups used for intra-bundle links.
- **FR-012**: System MUST allow an agent's full set of memory bundles to be copied to a different Vizier workspace/deployment and load there with no data loss and no id/slug collisions with memories already present.
- **FR-013**: System MUST provide a migration path so memories already stored under the previous (row-based) mechanism are carried forward into the new bundle structure and remain accessible after the system adopts document-based storage; migrated memories become private to their owning agent (their previous `global`/`shared` visibility is not preserved) and land in that agent's default bundle unless already organized otherwise — no existing memory should become unreadable or vanish as a result of this change.
- **FR-014**: System MUST keep each memory's attachments as files within the same bundle, associated with their owning concept document, so that copying an agent's bundles (e.g., for migration) does not silently drop attachments.
- **FR-015**: System MUST automatically maintain each bundle's index document to reflect the current set of concept documents in that bundle, kept up to date as memories are written, updated, or deleted, so a developer or agent can see what's available without opening every concept document individually.
- **FR-016**: System MUST automatically maintain each bundle's log document as a chronological history of that bundle's memory writes and updates.
- **FR-017**: The WebUI memory graph MUST offer two levels of view: a bundle-level graph showing an agent's bundles as nodes and cross-bundle links as edges between them, and a concept-level graph — entered by opening a bundle — showing that bundle's concept documents as nodes and their links as edges, matching today's existing single-collection graph interaction.

### Key Entities

- **Memory Bundle**: A named, hierarchical directory of concept documents about a cohesive topic (e.g., a particular person, project, or subject), plus that bundle's reserved index and log documents, per the Open Knowledge Format. An agent may own multiple bundles; a bundle is created automatically the first time a memory names it. Replaces the current flat, single-collection database representation.
- **Memory Concept Document** *(formerly referred to as "Memory Document")*: A single markdown file within a bundle representing one memory — title, slug, tags, timestamps, and read count as frontmatter metadata, with free-form content as the body. Addressable by its bundle plus file path (its concept ID). The agent itself is the primary author and reader of these documents.
- **Index Document**: Each bundle's reserved listing document (e.g., `index.md`), giving a directory-level view of the concepts available in that bundle so an agent or a human browsing it can see what's there before opening individual concept documents.
- **Log Document**: Each bundle's reserved chronological history document (e.g., `log.md`), recording that bundle's memory updates over time.
- **Memory Link**: A reference from one concept document to another concept, or to another bundle as a whole, used for related-memory lookups and the memory graph. May cross bundle boundaries as long as both bundles are owned by the same agent; expressed in a way that survives being read outside of Vizier.
- **Attachment**: A supplementary, non-document file stored within a bundle and associated with a specific concept document (e.g., an image or file the memory refers to), which must stay identifiable as belonging to that document.
- **Agent**: The owner of one or more Memory Bundles, and the primary reader/writer of every concept document within them. All memory is private to its owning agent — there is no cross-agent visibility or access.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of an agent's existing memory operations (write, search/query, link, list, delete) succeed with document storage, with no functional regression versus current behavior.
- **SC-002**: Memory search and related-memory lookups continue to return results in under 1 second for an agent with up to 5,000 memory documents, matching current responsiveness.
- **SC-003**: A developer can open any single memory document in a standard text/Markdown viewer and read its full title, tags, and content without running Vizier.
- **SC-004**: 100% of memories that existed before the change to document-based storage remain queryable and readable by their owning agent after upgrade, with no manual re-entry required.
- **SC-005**: An agent's full set of memory bundles can be copied to a new deployment with 100% of documents, tags, and links intact.
- **SC-006**: A bundle's index document reflects 100% of that bundle's current memories after any write, update, or delete, verified by comparing the index listing to the concept documents actually present.
- **SC-007**: An agent can start using a brand-new bundle simply by naming it in a memory write, with 100% success and no separate creation step.
- **SC-008**: A concept in one bundle can link to a specific concept, or to the bundle as a whole, in a different bundle owned by the same agent, and that link resolves correctly through the same related-memory/graph lookups used for intra-bundle links.
- **SC-009**: A developer can go from the bundle-level graph to any individual bundle's concept-level graph in a single action (opening the bundle node), and back, without needing to know the bundle's structure ahead of time.

## Assumptions

- The primary consumer of memory documents is the owning agent itself, through its existing memory tools; human readability and portability are valuable properties that fall out of that representation, not independent goals pursued at the expense of the agent's own workflow.
- Sharing an individual memory with another person or team outside of Vizier (e.g., handing off a single file to a colleague) is out of scope for this spec — it may be revisited separately once agent-facing document storage and human-readability are in place.
- "Open knowledge format" refers specifically to the bundle convention described in the referenced Open Knowledge Format spec — a hierarchical directory of markdown documents with structured frontmatter metadata — the same general style already used for an agent's CORE.md identity document, rather than a binary or proprietary format.
- Reconciliation of manual, out-of-band edits to concept documents (Edge Cases, FR-010) happens lazily, on the next relevant read/query/write, rather than through continuous real-time filesystem watching.
- This feature drops the previous private/global/shared visibility distinction and the `shared_to` list entirely: every memory is private to its owning agent, with no cross-agent access of any kind. Existing memories marked `global` or `shared` are treated as private on migration (see FR-013) — any cross-agent access they previously allowed no longer applies after this change. Reintroducing cross-agent visibility, if ever needed, is a separate future feature.
- An agent may own multiple memory bundles, typically organized by topic (e.g., one bundle per person, project, or subject it has learned about); bundles are created implicitly by naming them in a memory write, and memories written without naming a bundle go to that agent's default bundle.
- Cross-bundle links (FR-011) are scoped to bundles owned by the same agent; there is no mechanism in this feature for one agent's bundle to reference another agent's bundle.
- An empty bundle (all of its concept documents deleted) is not automatically removed; it persists with just its index/log documents until an operator or future feature cleans it up.
- Existing consumers of the memory data (search/query, filtering, memory graph, WebUI memory views) will be re-pointed to the new bundle-backed store rather than needing a parallel legacy path once migration completes.
