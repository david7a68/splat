---
splat_schema: 1
splat_id: 6cae251e-c8df-5f4c-b400-af77014d3027
splat_kind: rule
splat_key: dnd2024:glossary/blinded_condition
splat_name: Blinded [Condition]
splat_origin:
  key: dnd2024:reference/glossary/blinded_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 177
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/blinded_condition
  parameters:
    condition: blinded
    can_see: false
    automatic_failure:
      check_kind: ability_check
      requires_sight: true
    attack_rolls_against: advantage
    own_attack_rolls: disadvantage
  implementation: dnd2024:glossary_blinded_condition
splat_contracts:
- id: dnd2024:glossary_blinded_condition
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - condition_effects
  events:
  - condition.changed
  - ability_check.pending
  - attack.pending
  capabilities:
  - conditions
  - visibility
  - checks
  - attacks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 1ec8d2d97fad96cf33b87dc910bc0c3d9f8f8375376c4e464a2975561bf0b4e9
  source_path: builtin/srd-5.2.1/glossary/blinded_condition.md
  pages:
  - 177
  engine_gaps: &id001
  - dnd2024:glossary_blinded_condition
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 189369ac3d5dd01e6120929eccbce722b54e86d80243db9dfa9ef66dc724b242
splat_clauses:
- key: source_rules
  text: '# Blinded [Condition] While you have the Blinded condition, you experience the following effects.
    Can’t See. You can’t see and automatically fail any ability check that requires sight. Attacks Affected.
    Attack rolls against you have Advantage, and your attack rolls have Disadvantage.'
  sha256: 233ae1dff73b6d65ff2dbc3c9069fad55c29b79e10edf312ebdec5bda9ae12f8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Blinded [Condition]

<!-- SRD page 177 -->

While you have the Blinded condition, you experi-\
ence the following effects.\
Can’t See. You can’t see and automatically fail any\
ability check that requires sight.\
Attacks Affected. Attack rolls against you have\
Advantage, and your attack rolls have Disadvantage.\

Original layout: [[../../srd-5.2.1/pages/177|page 177]].
