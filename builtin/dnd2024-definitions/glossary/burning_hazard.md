---
splat_schema: 1
splat_id: 9cfc86c9-7bfb-5579-a4f0-30ab90c2b223
splat_kind: rule
splat_key: dnd2024:glossary/burning_hazard
splat_name: Burning [Hazard]
splat_origin:
  key: dnd2024:reference/glossary/burning_hazard
splat_sources:
- title: SRD 5.2.1
  locator: pp. 178
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/burning_hazard
  parameters:
    eligible_targets:
    - creature
    - object
    damage:
      dice: 1d4
      type: fire
      timing: subject_turn_start
    self_extinguish:
      cost: action
      apply_condition: prone
      required_activity: roll_on_ground
      ends_burning: true
    also_ends_when:
    - doused
    - submerged
    - suffocated
  implementation: dnd2024:glossary_burning_hazard
splat_contracts:
- id: dnd2024:glossary_burning_hazard
  inputs:
    subject:
      type: object
      required: true
    extinguish_method:
      required: false
      type: choice
  outputs:
  - damage
  - condition_changes
  - burning_ended
  events:
  - turn.started
  - action.requested
  - environment.changed
  capabilities:
  - hazards
  - damage
  - conditions
  - action_economy
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 827ef6cd5e089e62fb9302b012a475c4b3c92ee8aebea89d00768f2fcd41143b
  source_path: builtin/srd-5.2.1/glossary/burning_hazard.md
  pages:
  - 178
  engine_gaps: &id001
  - dnd2024:glossary_burning_hazard
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 8a7b99169a6ce036698143308b281fa9211c53b7ce218fffe2de6903808b5e0f
splat_clauses:
- key: source_rules
  text: '# Burning [Hazard] A burning creature or object takes 1d4 Fire damage at the start of each of
    its turns. As an action, you can extinguish fire on yourself by giving yourself the Prone condition
    and rolling on the ground. The fire also goes out if it is doused, submerged, or suffocated.'
  sha256: cc3d407d9b9cab405198d945ad1dcfba55158b61a822e41a10a579f1dccb4e9a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Burning [Hazard]

<!-- SRD page 178 -->

A burning creature or object takes 1d4 Fire damage\
at the start of each of its turns. As an action, you\
can extinguish fire on yourself by giving yourself\
the Prone condition and rolling on the ground.\
The fire also goes out if it is doused, submerged, or\
suffocated.\

Original layout: [[../../srd-5.2.1/pages/178|page 178]].
