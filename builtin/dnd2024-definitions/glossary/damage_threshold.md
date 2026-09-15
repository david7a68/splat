---
splat_schema: 1
splat_id: cd6350c7-7f49-5f31-a86f-2de36a75a108
splat_kind: rule
splat_key: dnd2024:glossary/damage_threshold
splat_name: Damage Threshold
splat_origin:
  key: dnd2024:reference/glossary/damage_threshold
splat_sources:
- title: SRD 5.2.1
  locator: pp. 180
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/damage_threshold
  parameters:
    scope: single_attack_or_effect_instance
    pass_predicate:
      op: greater_or_equal
      args:
      - var: damage_amount
      - var: threshold
    pass_damage:
      var: damage_amount
    fail_damage: 0
    fail_immunity: true
    threshold_is_not_subtracted: true
  implementation: dnd2024:glossary_damage_threshold
splat_contracts:
- id: dnd2024:glossary_damage_threshold
  inputs:
    subject:
      type: object
      required: true
    damage_amount:
      type: integer
      required: true
    threshold:
      type: integer
      required: true
  outputs:
  - damage_amount
  events:
  - damage.pending
  capabilities:
  - damage
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 1742947a9d3b3aa02af52b446bdebb92b7a30948404257f922761e3a74ff0ac1
  source_path: builtin/srd-5.2.1/glossary/damage_threshold.md
  pages:
  - 180
  engine_gaps: &id001
  - dnd2024:glossary_damage_threshold
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 9e23d91c4eedf95f8a6190b31e3043eb72c1d8c2a63c829b85f517cc547b216f
splat_clauses:
- key: source_rules
  text: '# Damage Threshold A creature or an object that has a damage threshold has Immunity to all damage
    unless it takes an amount of damage from a single attack or effect equal to or greater than its damage
    threshold, in which case it takes that entire instance of damage. Any damage that fails to meet or
    exceed the damage threshold is superficial and doesn’t reduce Hit Points. For example, if an object
    has a damage threshold of 10, the object takes no damage if 9 damage is dealt to it, since that damage
    fails to exceed the threshold. If the same object is dealt 11 damage, it takes all of that damage.'
  sha256: c68b4004dbbf601c81199595464ed7a0b2dba7b4f5d8c284b276bf2db1377ef3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Damage Threshold

<!-- SRD page 180 -->

A creature or an object that has a damage thresh-\
old has Immunity to all damage unless it takes an\
amount of damage from a single attack or effect\
equal to or greater than its damage threshold, in\
which case it takes that entire instance of dam-\
age. Any damage that fails to meet or exceed the\
damage threshold is superficial and doesn’t reduce\
Hit Points. For example, if an object has a damage\
threshold of 10, the object takes no damage if 9\
damage is dealt to it, since that damage fails to\
exceed the threshold. If the same object is dealt 11\
damage, it takes all of that damage.\

Original layout: [[../../srd-5.2.1/pages/180|page 180]].
