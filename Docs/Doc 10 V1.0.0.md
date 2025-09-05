* **Version:** 1.0 Final Blueprint
* **Date:** September 5, 2025
* **Author:** Christophe Duy Quang Nguyen
* **Vibe Coding Engine:** Gemini 2.5, Google

# **The `cdqn` Ecosystem: The Content Management Layer**

## **Introduction: The "Living Document"**

For centuries, the final form of a creative or scientific work—a book, a paper, a blueprint—has been a static, flat artifact. It is the end of a process. Once published, its contents are frozen, its connections to its source data are severed, and its knowledge is locked away in a non-interactive format.

The `cdqn` Content Management System (CMS) is designed to change this. It introduces the concept of the **"Living Document."** In `cdqn`, a publication is not the end of a process; it is a **new beginning**. It is a rich, queryable, and forkable universe of knowledge, deeply and verifiably linked to the facts, proofs, and ideas that brought it into being.

This layer provides the workflows for long-term, complex, and goal-oriented work, from artistic creation to scientific discovery, all managed by a dedicated `ContentAgent` that acts as the user's creative and scientific partner.

---

## **1. The Core Architecture: A Universe of Knowledge**

*   **What it is:** A complex document is not a single `cdu`. It is a high-level **Projection** of a complex, interconnected `cdu` lineage. This architecture is built on three new `cdu` subtypes:
    1.  **`cduWorld`:** The "world bible" or "simulation parameter set." This is a foundational `cdu` that defines the core rules, facts, characters, or physical laws of a specific knowledge domain.
    2.  **`cduChapter`:** A `cdu` that represents a single, self-contained part of a larger work (a chapter, an article, a scene). Each `chapter` is verifiably linked to its parent `world`.
    3.  **`cduPublication`:** The final, compiled "book." It is a master `cdu` that acts as a verifiable table of contents, linking all its `chapter`s and the foundational `cdu`s (proofs, facts, etc.) that form its intellectual backbone.
*   **Why we do this (Best Practice):** This **decoupled, component-based** approach to content is incredibly robust and flexible. It allows for non-linear creation, easy reuse of components (a `cduFact` about a character can be used in multiple stories), and perfect, auditable version control. It treats knowledge creation like a modern software development project, not a linear word-processing task.
*   **A Practical Use Case (Fiction):** A novelist creates a `cduWorld` for their fantasy series. They can then write multiple `cduPublication`s (novels) that all share and are consistent with the same `world`. If they later decide to change a core rule of magic in the `world`, their `ContentAgent` can automatically flag all the chapters in all their books that are now inconsistent with the new lore.

---

## **2. The Creative & Scientific Workflow**

This is the end-to-end process of creating a living document, managed by the user's `ContentAgent`.

| Component | Role | Entity Type | Core Function |
| :--- | :--- | :--- | :--- |
| **`ContentAgent`** | The Creative Partner | `Agent` | The user's primary partner for all long-form content creation, from worldbuilding to drafting and simulation. |
| **`consistency-oracle.wasi`**| The Lorekeeper / Lab Assistant | `Worker` | A specialized worker that checks a piece of draft text for consistency against the rules of its parent `cduWorld`. |
| **`publication-compiler.wasi`**| The Bookbinder | `Automata` | Manages the process of taking a `world` and its `chapter`s and compiling them into a single, final `cduPublication`, while also acting as a **License Auditor**. |

*   **`cdqnLang` Example (The Consistency Check during drafting):**
    ```cdqnlang
    agent ContentAgent {
      state {
        cid: current_world_cid,
      }

      // The agent observes when the user saves a new version of a chapter.
      on cdu where cdu.type = chapter {
        // As a background task, check for consistency with the world's rules.
        emit cdu {
          cdu_type: task,
          subject: "consistency-oracle::check",
          content: {
            world_cid: self.state.current_world_cid,
            draft_content: cdu.content.content_body,
          },
        };
      /on

      // It then handles the result from the oracle.
      on cdu where cdu.subject = "consistency-oracle::Result"
        if (cdu.content.is_consistent = false)
          ↦
          // If there's a conflict, alert the user with a helpful suggestion.
          self.notify_user(f"Consistency warning: {cdu.content.explanation}");
        /if
      /on
    }
    ```

---

## **3. The Interactive Experience: The Queryable Document**

*   **What it is:** A published `cduPublication` is not a static block of text. It is an interactive knowledge base. A user can perform deep, semantic queries on the document using the **`document-querent.wasi`** `worker`.
*   **Why we do this (Best Practice):** This is the ultimate expression of **structured, semantic content**. By breaking a document down into its constituent, causally-linked parts, we unlock its knowledge. This transforms reading from a passive act of consumption into an active process of discovery.
*   **`cdqnLang` Example (Deriving a new work):**
    ```cdqnlang
    agent MyResearchAssistant {
      on cdu where cdu.subject = "Create Character Guide"
        cid: source_novel_cid ← cdu.content.source_cid

        // 1. Task the querent to extract all character descriptions.
        emit cdu {
          cdu_type: task,
          subject: "document-querent::extract_characters",
          content: source_novel_cid,
        };
      /on

      // 2. When the character data returns, create a new publication.
      on cdu where cdu.subject = "document-querent::Result"
        list<character>: characters ← cdu.content

        // 3. Create a new "world" and "chapters" for the guide.
        // ... (workflow to create the new cdu's) ...

        // 4. Finally, call the compiler to create the new "Character Guide" publication.
        emit cdu {
          cdu_type: task,
          subject: "publication-compiler::compile",
          content: { ... }, // The manifest for the new guide
        };
      /on
    }
    ```

---

## **4. The Integrated Licensing Framework**

*   **What it is:** The CMS is deeply integrated with the ecosystem's sovereign licensing system. The `publication-compiler.wasi` acts as a mandatory **License Auditor** before any work can be finalized.
*   **Why we do this (Best Practice):** This provides an automated **intellectual property compliance** workflow. It protects creators by ensuring their licenses are respected, and it protects users by preventing accidental copyright infringement. It makes the complex world of software and content licensing transparent and manageable.
*   **The Agile Negotiation Workflow:** If the `publication-compiler` detects a license conflict, it does not simply fail. It pauses and emits a `LicenseConflictDetected` event. This allows the user's `ContentAgent` to initiate a formal **License Negotiation** with the original author of the conflicting component. This workflow, managed via a `cduContract`, allows for an agile, market-based resolution to IP challenges, turning a hard blocker into a solvable business negotiation.

---

## **5. Formal Specification: `workflows.wit` (Relevant Sections)**
```wit
world workflow-world {
    // ...
    // --- Structures for CMS ---
    record world-profile { description: string, foundational-cids: list<cid> }
    record chapter-document { world-cid: cid, publication-cid: cid, chapter-number: u32, content-body: string }
    record publication-manifest { world-cid: cid, chapters: list<cid>, foundational-proofs: list<cid> }

    // --- Structures for Licensing ---
    record license-negotiation-contract { requesting-node: entity-id, authoring-node: entity-id, target-cdu-cid: cid, scope-publication-cid: cid, proposed-terms: string, offer: barter-contract }
    record granted-license { grantee-node: entity-id, licensed-cdu-cid: cid, usage-scope-cid: cid, terms: string, negotiation-contract-cid: cid }
}
```
