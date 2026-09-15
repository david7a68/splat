---
splat_schema: 1
splat_id: a60ce9cd-15e0-5c4f-8ef5-3bff5d68721f
splat_kind: rule
splat_key: dnd2024:rules/magic_item_resilience
splat_name: Magic Item Resilience
splat_origin:
  key: dnd2024:reference/pages/206
splat_sources:
- title: SRD 5.2.1
  locator: pp. 206
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magic_item_resilience
  parameters:
    durability_minimum: nonmagical_equivalent
    default_resistance: all_damage
    default_resistance_exceptions:
    - potions
    - scrolls
    artifact:
      damage_immunity: true
      destruction_requires: artifact_specific_method
  implementation: dnd2024:rules_magic_item_resilience
splat_contracts:
- id: dnd2024:rules_magic_item_resilience
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - item_attributes
  - actor_modifiers
  events: []
  capabilities:
  - equipment
  - armor_class
  - crafting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 206
  engine_gaps: &id001
  - dnd2024:rules_magic_item_resilience
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: af3721ca0eab0c4056be25d0fef7eaedb65ca605a33ca6099d90c5565bdb0376
  parent_name: Magic Item Resilience
  line_span:
    first: 10650
    count: 8
splat_clauses:
- key: source_rules
  text: '# Magic Item Resilience A magic item is at least as durable as a nonmagical item of its kind.
    Most magic items, other than Potions and Scrolls, have Resistance to all damage. An Artifact can be
    destroyed only in some special way. Otherwise, it is impervious to damage. Learning how to destroy
    an Artifact usually requires research or the completion of a quest.'
  sha256: a89609548de113dbf96e9bc478333441e3ebbfc6ce8e8c46fd8ea01efbba7d96
  status: contract_specified
splat_requires_mechanics: *id001
---

# Magic Item Resilience

<!-- SRD page 206 -->

A magic item is at least as durable as a nonmagical\
item of its kind. Most magic items, other than Po-\
tions and Scrolls, have Resistance to all damage.\
An Artifact can be destroyed only in some special\
way. Otherwise, it is impervious to damage. Learn-\
ing how to destroy an Artifact usually requires\
research or the completion of a quest.\
