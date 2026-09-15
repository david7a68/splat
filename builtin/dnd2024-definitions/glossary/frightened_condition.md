---
splat_schema: 1
splat_id: 648d48b0-27a5-58ed-bd03-9a3d9b258460
splat_kind: rule
splat_key: dnd2024:glossary/frightened_condition
splat_name: Frightened [Condition]
splat_origin:
  key: dnd2024:reference/glossary/frightened_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 182
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/frightened_condition
  parameters:
    condition: frightened
    source_in_line_of_sight:
      ability_checks: disadvantage
      attack_rolls: disadvantage
    willing_movement_closer_to_source_allowed: false
  implementation: dnd2024:glossary_frightened_condition
splat_contracts:
- id: dnd2024:glossary_frightened_condition
  inputs:
    subject:
      type: participant
      required: true
    fear_source:
      type: object
      required: true
    geometry:
      type: resolution
      required: true
  outputs:
  - condition_effects
  events:
  - ability_check.pending
  - attack.pending
  - movement.pending
  capabilities:
  - conditions
  - checks
  - movement
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 58c220f10642a0117d6f951b39c75f80f0d3c7a5fda48d653e0a14c118e3a132
  source_path: builtin/srd-5.2.1/glossary/frightened_condition.md
  pages:
  - 182
  engine_gaps: &id001
  - dnd2024:glossary_frightened_condition
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 21d9d1ed28bb2340176a9909f733976853cfc4b4dc245d9cca806fe58ebdf54a
splat_clauses:
- key: source_rules
  text: '# Frightened [Condition] While you have the Frightened condition, you experience the following
    effects. Ability Checks and Attacks Affected. You have Disadvantage on ability checks and attack rolls
    while the source of fear is within line of sight. Can’t Approach. You can’t willingly move closer
    to the source of fear.'
  sha256: 51d506ea2b9064f89c8a6abcce1c7adceaeef6445398c77ed6281c1ea61dad12
  status: contract_specified
splat_requires_mechanics: *id001
---

# Frightened [Condition]

<!-- SRD page 182 -->

While you have the Frightened condition, you expe-\
rience the following effects.\
Ability Checks and Attacks Affected. You have\
Disadvantage on ability checks and attack rolls\
while the source of fear is within line of sight.\
Can’t Approach. You can’t willingly move closer to\
the source of fear.\

Original layout: [[../../srd-5.2.1/pages/182|page 182]].
