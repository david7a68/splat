---
splat_schema: 1
splat_id: f9b2dd51-e21e-5589-ba8f-6e6fb4a551f4
splat_kind: rule
splat_key: dnd2024:rules/warlock_fiendish_vigor
splat_name: Fiendish Vigor
splat_origin:
  key: dnd2024:reference/pages/073
splat_sources:
- title: SRD 5.2.1
  locator: pp. 73
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_fiendish_vigor
  parameters:
    warlock_level_minimum: 2
    cast:
      spell:
        key: dnd2024:spells/false_life
      spell_slot_cost: 0
      target: self
      uses: unlimited
      temporary_hit_points_dice_result: maximum
  implementation: dnd2024:rule_warlock_fiendish_vigor
splat_contracts:
- id: dnd2024:rule_warlock_fiendish_vigor
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - feature_effects
  events: []
  capabilities:
  - invocations
  - spellcasting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 73
  engine_gaps: &id001
  - dnd2024:rule_warlock_fiendish_vigor
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 2e7a6f32e735c0832651a0226c5af98b3be46433550dc2b9aec301e8d5bb1ae9
  parent_name: Warlock
  line_span:
    first: 6192
    count: 7
splat_clauses:
- key: source_rules
  text: '# Fiendish Vigor Prerequisite: Level 2+ Warlock You can cast False Life on yourself without expending
    a spell slot. When you cast the spell with this feature, you don’t roll the die for the Temporary
    Hit Points; you automatically get the highest number on the die.'
  sha256: 002715a604b82a753a52fa1539d4366d0ebacb89b6d5dcf13ad532f1795a84b3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Fiendish Vigor

<!-- SRD page 73 -->

Prerequisite: Level 2+ Warlock\
You can cast False Life on yourself without expend-\
ing a spell slot. When you cast the spell with this\
feature, you don’t roll the die for the Temporary Hit\
Points; you automatically get the highest number on\
the die.\
