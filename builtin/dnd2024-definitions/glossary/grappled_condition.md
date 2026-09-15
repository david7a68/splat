---
splat_schema: 1
splat_id: 5a4d8879-58d0-523b-8bcc-ea6ca8f3490f
splat_kind: rule
splat_key: dnd2024:glossary/grappled_condition
splat_name: Grappled [Condition]
splat_origin:
  key: dnd2024:reference/glossary/grappled_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 182
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/grappled_condition
  parameters:
    condition: grappled
    speed: 0
    speed_can_increase: false
    attack_disadvantage_unless_target_is_grappler: true
    grappler_moves_subject:
      allowed_modes:
      - drag
      - carry
      extra_cost_per_foot: 1
      extra_cost_exceptions:
      - target_size: tiny
      - target_size_at_least_categories_smaller: 2
  implementation: dnd2024:glossary_grappled_condition
splat_contracts:
- id: dnd2024:glossary_grappled_condition
  inputs:
    subject:
      type: participant
      required: true
    grappler:
      type: participant
      required: true
  outputs:
  - condition_effects
  - grappler_movement_cost
  events:
  - attack.pending
  - movement.pending
  - speed.calculated
  capabilities:
  - conditions
  - movement
  - attacks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 958756dbff5760b7920c895034a654ef7e5ae7f8b5f6274982a0aa50bbdb9314
  source_path: builtin/srd-5.2.1/glossary/grappled_condition.md
  pages:
  - 182
  engine_gaps: &id001
  - dnd2024:glossary_grappled_condition
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 5e0e2bf95f8011bc7999bfadc442bda833e1898fe6d641a86f741532c525d3c7
splat_clauses:
- key: source_rules
  text: '# Grappled [Condition] While you have the Grappled condition, you experience the following effects.
    Speed 0. Your Speed is 0 and can’t increase. Attacks Affected. You have Disadvantage on attack rolls
    against any target other than the grappler. Movable. The grappler can drag or carry you when it moves,
    but every foot of movement costs it 1 extra foot unless you are Tiny or two or more sizes smaller
    than it.'
  sha256: 31426f50416b770d3f9401d88babd8bf420ded203a822e2e73010a681db6c9ec
  status: contract_specified
splat_requires_mechanics: *id001
---

# Grappled [Condition]

<!-- SRD page 182 -->

While you have the Grappled condition, you experi-\
ence the following effects.\
Speed 0. Your Speed is 0 and can’t increase.\
Attacks Affected. You have Disadvantage on at-\
tack rolls against any target other than the grappler.\
Movable. The grappler can drag or carry you\
when it moves, but every foot of movement costs it 1\
extra foot unless you are Tiny or two or more sizes\
smaller than it.\

Original layout: [[../../srd-5.2.1/pages/182|page 182]].
