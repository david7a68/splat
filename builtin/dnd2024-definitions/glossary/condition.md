---
splat_schema: 1
splat_id: 77348a61-de0f-5b72-ae1d-3dd0ebee7b6f
splat_kind: rule
splat_key: dnd2024:glossary/condition
splat_name: Condition
splat_origin:
  key: dnd2024:reference/glossary/condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 179
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/condition
  parameters:
    members:
    - key: dnd2024:glossary/blinded_condition
    - key: dnd2024:glossary/charmed_condition
    - key: dnd2024:glossary/deafened_condition
    - key: dnd2024:glossary/exhaustion_condition
    - key: dnd2024:glossary/frightened_condition
    - key: dnd2024:glossary/grappled_condition
    - key: dnd2024:glossary/incapacitated_condition
    - key: dnd2024:glossary/invisible_condition
    - key: dnd2024:glossary/paralyzed_condition
    - key: dnd2024:glossary/petrified_condition
    - key: dnd2024:glossary/poisoned_condition
    - key: dnd2024:glossary/prone_condition
    - key: dnd2024:glossary/restrained_condition
    - key: dnd2024:glossary/stunned_condition
    - key: dnd2024:glossary/unconscious_condition
    default_stacking: false
    stacking_exception: exhaustion
    duration_and_end: source_effect_defined
  implementation: dnd2024:glossary_condition
splat_contracts:
- id: dnd2024:glossary_condition
  inputs:
    subject:
      type: participant
      required: true
    condition:
      type: definition
      required: true
  outputs:
  - condition_state
  events:
  - condition.applied
  - condition.ended
  capabilities:
  - conditions
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 386f0d4fd5463b380a8d9ea4e1ab82d9fea13481d74da30d4c8030ecdfa1526a
  source_path: builtin/srd-5.2.1/glossary/condition.md
  pages:
  - 179
  engine_gaps: &id001
  - dnd2024:glossary_condition
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 614ffbe9e43c80f6a7fc7d5872e32c21095788086a2f391531439a6c6e261e48
splat_clauses:
- key: source_rules
  text: '# Condition A condition is a temporary game state. The definition of a condition says how it
    affects its recipient, and various rules define how to end a condition. This glossary defines these
    conditions: Blinded Grappled Poisoned Charmed Incapacitated Prone Deafened Invisible Restrained Exhaustion
    Paralyzed Stunned Frightened Petrified Unconscious A condition doesn’t stack with itself; a recipient
    either has a condition or doesn’t. The Exhaustion condition is an exception to that rule.'
  sha256: 96dba5ced6276c139fc83ea6d80beacaf6a13be9fcefc0097b9d03aa7366ca78
  status: contract_specified
splat_requires_mechanics: *id001
---

# Condition

<!-- SRD page 179 -->

A condition is a temporary game state. The defini-\
tion of a condition says how it affects its recipient,\
and various rules define how to end a condition.\
This glossary defines these conditions:\
Blinded Grappled Poisoned\
Charmed Incapacitated Prone\
Deafened Invisible Restrained\
Exhaustion Paralyzed Stunned\
Frightened Petrified Unconscious\
A condition doesn’t stack with itself; a recipient\
either has a condition or doesn’t. The Exhaustion\
condition is an exception to that rule.\

Original layout: [[../../srd-5.2.1/pages/179|page 179]].
