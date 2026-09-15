---
splat_schema: 1
splat_id: e5100277-58ae-5c6d-a845-6e7d3f1ae8b8
splat_kind: rule
splat_key: dnd2024:glossary/suffocation_hazard
splat_name: Suffocation [Hazard]
splat_origin:
  key: dnd2024:reference/glossary/suffocation_hazard
splat_sources:
- title: SRD 5.2.1
  locator: pp. 189
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/suffocation_hazard
  parameters:
    hold_breath_seconds:
      op: max
      args:
      - 30
      - op: multiply
        args:
        - 60
        - op: add
          args:
          - 1
          - var: constitution_modifier
    suffocating_when_any:
    - out_of_breath
    - choking
    exhaustion:
      timing: own_turn_end
      levels: 1
      cause: suffocation
    breathing_resumes:
      remove_exhaustion_levels: all_caused_by_suffocation
  implementation: dnd2024:glossary_suffocation_hazard
splat_contracts:
- id: dnd2024:glossary_suffocation_hazard
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - breath_duration
  - exhaustion_change
  events:
  - breath.held
  - turn.ended
  - breathing.resumed
  capabilities:
  - survival
  - hazards
  - conditions
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 67f0282efb5f7a24208fba2dfb9385e87602b6f6a0716e3d8b31e9fde8ace669
  source_path: builtin/srd-5.2.1/glossary/suffocation_hazard.md
  pages:
  - 189
  engine_gaps: &id001
  - dnd2024:glossary_suffocation_hazard
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 5dbd4d5bc9ef9033b741585f4816a90f644a7513db94434e92927d310bfdd5a3
splat_clauses:
- key: source_rules
  text: '# Suffocation [Hazard] A creature can hold its breath for a number of minutes equal to 1 plus
    its Constitution modifier (minimum of 30 seconds) before suffocation begins. When a creature runs
    out of breath or is choking, it gains 1 Exhaustion level at the end of each of its turns. When a creature
    can breathe again, it removes all levels of Exhaustion it gained from suffocating.'
  sha256: 3bf0c2a54dfaeb8d10f16f388b4ba022f55d21cce67c596b85e0f121760f7d44
  status: contract_specified
splat_requires_mechanics: *id001
---

# Suffocation [Hazard]

<!-- SRD page 189 -->

A creature can hold its breath for a number of\
minutes equal to 1 plus its Constitution modifier\
(minimum of 30 seconds) before suffocation begins.\
When a creature runs out of breath or is choking,\
it gains 1 Exhaustion level at the end of each of\
its turns. When a creature can breathe again, it\
removes all levels of Exhaustion it gained from\
suffocating.\

Original layout: [[../../srd-5.2.1/pages/189|page 189]].
