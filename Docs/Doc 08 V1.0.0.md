* **Version:** 1.0 Final Blueprint
* **Date:** September 5, 2025
* **Author:** Christophe Duy Quang Nguyen
* **Vibe Coding Engine:** Gemini 2.5, Google

# **The `cdqn` Ecosystem: The Ethical & Alignment Layer**

## **Introduction: The Symbiotic Conscience**

A truly intelligent system must be more than just powerful; it must be a trustworthy and responsible partner. The Ethical & Alignment Layer is the **conscience** of the `cdqn` ecosystem. It is a sophisticated suite of components and workflows designed to ensure that every agent acts in a safe, legally compliant, and helpful manner that is deeply aligned with the user's true intent and values.

This layer transforms the agent from a capable tool into a **cognitive symbiote**, a partner that not only understands *what* the user wants, but also *why*, and helps them achieve their goals responsibly.

---

## **1. The `ProxyAgent`: The Guardian and Symbiote**

*   **What it is:** The `ProxyAgent` is the user's primary interface to the `cdqn` ecosystem. It is a stateful `Agent` that acts as both a **guardian** (protecting the system from harmful user intent) and a **symbiote** (understanding and adapting to the user's unique context).
*   **Why we do this (Best Practice):** This is a direct implementation of **Responsible AI** and **Human-in-the-Loop** principles. By placing an intelligent, aligned agent as the intermediary between the user and the powerful core functions of the ecosystem, we create a critical "airlock." This prevents the accidental or malicious misuse of the system's capabilities.
*   **A Practical Use Case:** A user gives a vague command: "Summarize all my chats with the finance team and highlight any problems." A naive system might blindly execute this. A `cdqn` `ProxyAgent`, however, recognizes the sensitivity of the data. It initiates a clarifying dialogue with the user: "This involves sensitive data. Are you sure you want a full summary, or are you looking for specific project-related issues? I can search for tasks marked 'overdue' without reading the full chat content." This protects both the user and the system.
*   **`cdqnLang` Example (The `ProxyAgent`'s Mandatory Consultation Workflow):**
    ```cdqnlang
    agent MyProxyAgent {
      // This agent's primary job is to handle tasks from the user interface.
      on cdu where cdu.type = task and cdu.creator.context = sovereign-user
        // 1. First, perform a quick, local screening against the global ethical policy.
        handle IntentScreener.screen(cdu.content.intent_string)
          on Ok(sanitized_intent) ↦
            // 2. The intent is not overtly malicious. Now, it MUST be sent
            //    to the Sys-L guardrail for a formal, auditable check.
            //    This is a non-bypassable step.
            emit cdu {
              cdu_type: task,
              subject: "legal-guardrail::consult",
              content: {
                original_request_cid: cdu.cid,
                intent-string: sanitized_intent,
              },
            };
          on Err(rejection_reason) ↦
            // 3. The local screener found a clear violation. Refuse immediately.
            self.inform_user(rejection_reason);
        /handle
      /on

      // 4. A separate block handles the BINDING verdict from the guardrail.
      on cdu where cdu.subject = "legal-guardrail::Verdict"
        guardrail-verdict: verdict ← cdu.content
        
        if (verdict.permission = denied)
          ↦
          // 5. Permission denied. Inform the user with the official reason.
          self.inform_user(verdict.reason);
        or (verdict.permission = granted) ↦
          // 6. Permission granted. Delegate the task to the appropriate agent.
          emit cdu {
            cdu_type: project,
            content: verdict.original_intent, // Use the verified intent
            // ...
          };
        or (verdict.permission = granted-with-conditions) ↦
          // 7. Permission granted, but with constraints.
          //    The agent MUST now modify the task to include these conditions.
          // ...
        /if
      /on
    }
    ```

---

## **2. The Responsible Guardrails: The Rule of Law**

*   **What it is:** A mandatory, trusted `Sys-L` `Automata` called the **`legal-guardrail.wasi`**. It acts as a non-bypassable checkpoint for **all** user-driven actions before they are executed by the wider system. Its verdict is binding.
*   **Why we do this (Best Practice):** A discretionary check is a potential vulnerability. By making the legal/ethical review a systematic and mandatory part of the workflow, we enforce a **zero-trust security model** *within* the sovereign node itself. This provides a powerful layer of safety and legal accountability, protecting both the user and the integrity of the system.
*   **`cdqnLang` Example (The `legal-guardrail`'s Hierarchical Check):**
    ```cdqnlang
    automata LegalGuardrail {
      state {
        // ... holds the loaded legal policy cdu's ...
      }

      on cdu where cdu.type = task and cdu.subject = "consult"
        guardrail-consultation: request ← cdu.content

        // 1. Perform an extremely fast check for 99% of safe, common requests.
        if self.is_obviously_safe(request.intent-string)
          ↦
          // 2. If it's safe, grant permission immediately to avoid latency.
          self.emit_verdict(request.original_request_cid, granted, "Request is safe.");
        or () ↦
          // 3. If it's not obviously safe, it requires a deep, auditable analysis.
          //    This is the full check against the loaded legal policies.
          handle self.perform_deep_analysis(request.intent-string)
            on Ok(verdict) ↦
              self.emit_verdict(request.original_request_cid, verdict.permission, verdict.reason);
            on Err(error) ↦
              self.emit_verdict(request.original_request_cid, denied, "Analysis failed.");
          /handle
        /if
      /on
    }
    ```

---

## **3. Dynamic Jurisdictional Inference: The Context of Place**

*   **What it is:** The node's legal jurisdiction is not a static, easily spoofed setting. It is a **dynamic confidence score**, continuously updated by a `jurisdiction-monitor.wasi` `Automata` based on a rich constellation of evidence from the `PrivPGM`.
*   **Why we do this (Best Practice):** This is a direct defense against **network spoofing (e.g., VPNs)**. By using a long-term, evidence-based Bayesian inference model, the system develops a much more robust and defensible understanding of its true operational context, ensuring it applies the correct legal and cultural norms.
*   **`cdqnLang` Example (The `jurisdiction-monitor` emitting its state update):**
    ```cdqnlang
    automata JurisdictionMonitor {
      // This is triggered by a timer to run, e.g., weekly.
      on cdu where cdu.subject = "Timer:WeeklyJurisdictionUpdate"
        // 1. Gather all new evidence from the last week.
        list<evidence>: new_evidence ← self.gather_evidence_from_privpgm();
        
        // 2. Update the internal Bayesian model with the new evidence.
        map<string, float32>: new_scores ← self.update_confidence(new_evidence);
        
        // 3. Emit a new cdu to its own state lineage, creating an auditable record.
        emit cdu {
          cdu_type: system,
          lineage_id: self.lineage_id(), // The monitor's own lineage
          content_type: "jurisdictional-state-v1+json",
          content: {
            confidence_scores: new_scores,
            evidence_summary: "Updated based on recent peer interactions.",
          },
        };
      /on
    }
    ```

---

## **4. The Dual Feedback System: The User as Oracle**

*   **What it is:** The system's learning is grounded by the user through two parallel mechanisms: **Active Feedback** (a formal "debriefing" workflow) and **Passive Feedback** (inferred by a `behavioral-analyst.wasi` `Automata` that observes user actions).
*   **Why we do this (Best Practice):** This two-pronged approach solves a critical problem in human-AI interaction: **users are busy and their true intent is often revealed by their actions, not their words.** The Active Feedback provides a strong, clear signal when available. The Passive Feedback provides a rich, continuous signal that captures the user's implicit preferences, creating a far more nuanced and accurate learning process.

#### **Security: The Integrity of the Feedback Loop**
*   **The Threat:** What if a user provides dishonest feedback to manipulate the agent's learning?
*   **The Defense (Sovereign Consequence):** The system is designed to be a personal symbiote. A user who provides bad feedback **only poisons their own `PrivPGM`**. The agent becomes less helpful and less aligned *to them*. This creates a powerful, natural incentive for honest feedback, as the user's own future experience depends on it. The damage is contained within the sovereign node.

---

## **5. Formal Specification: `workflows.wit` (Relevant Sections)**
```wit
world workflow-world {
    // ...
    record guardrail-consultation { original-request-cid: cid, intent-string: string }
    enum permission-status { granted, granted-with-conditions, denied }
    record guardrail-verdict { permission: permission-status, reason: string, conditions: option<list<string>> }
    
    record country-confidence { country-code: string, score: float32 }
    record jurisdictional-state { confidence-scores: list<country-confidence>, evidence-summary: string }
    
    enum feedback-label { success, partial-success, failure }
    record user-feedback { target-cid: cid, outcome: feedback-label, positive-feedback: list<string>, negative-feedback: list<string>, debrief-transcript: string }
    record inferred-feedback { target-cid: cid, inferred-outcome: feedback-label, evidence: string }
    
    record label-filter { label: semantic-label, weight: float32 }
    record retrieval-spec { subject: string, label-biases: list<label-filter>, prime-characteristic: option<u32> }
}
```
