---
splat_schema: 1
splat_id: 4251d30c-ae5b-50cc-b428-8cea8d7222a6
splat_kind: rule
splat_key: dnd2024:glossary/immunity
splat_name: Immunity
splat_origin:
  key: dnd2024:reference/glossary/immunity
splat_sources:
- title: SRD 5.2.1
  locator: pp. 183
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/immunity
  parameters:
    scopes:
    - damage_type
    - condition
    matching_damage: 0
    matching_condition_applies: false
    other_matching_effects: false
  implementation: dnd2024:glossary_immunity
splat_contracts:
- id: dnd2024:glossary_immunity
  inputs:
    subject:
      type: object
      required: true
    incoming_effect:
      type: effect
      required: true
    immunity:
      type: definition
      required: true
  outputs:
  - immunity_result
  events:
  - damage.pending
  - condition.applied
  capabilities:
  - damage
  - conditions
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 22fcfe779171e9226510814e74dac31e3f83a9a9c2e3deee7919306499cac7a4
  source_path: builtin/srd-5.2.1/glossary/immunity.md
  pages:
  - 183
  engine_gaps: &id001
  - dnd2024:glossary_immunity
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 92bc7afd9d6447c0b1d6da8df222d8b87a5bd6f3db268b8c6ee741d59d6c2999
splat_clauses:
- key: source_rules
  text: '# Immunity If you have Immunity to a damage type or a condition, it doesn’t affect you in any
    way.'
  sha256: f6c165713b89a0584ca37f6de2b6bcac0364495bb18f2abe0e0c4c5026b95f05
  status: contract_specified
splat_requires_mechanics: *id001
---

# Immunity

<!-- SRD page 183 -->

If you have Immunity to a damage type or a condi-\
tion, it doesn’t affect you in any way.\

Original layout: [[../../srd-5.2.1/pages/183|page 183]].
