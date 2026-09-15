---
splat_schema: 1
splat_id: d6a17aa0-2350-5d92-a070-6a74eb76bf95
splat_kind: item
splat_key: dnd2024:items/cloak_of_elvenkind
splat_name: Cloak of Elvenkind
splat_origin:
  key: dnd2024:reference/magic_item/cloak_of_elvenkind
splat_sources:
- title: SRD 5.2.1
  locator: pp. 215
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/cloak_of_elvenkind
  implementation: dnd2024:item_cloak_of_elvenkind
  parameters:
    equipped: worn
    checks:
    - ability: wisdom
      skill: perception
      actor: other
      target: wearer
      mode: disadvantage
    - ability: dexterity
      skill: stealth
      actor: wearer
      mode: advantage
    attunement_required: true
splat_contracts:
- id: dnd2024:item_cloak_of_elvenkind
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    selection:
      required: false
      type: choice
    context:
      required: false
      type: resolution
  outputs:
  - effects
  - item_state_changes
  events:
  - ability_check.pending
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 8940dc568cc746831597d4392eb06ae41634d7caab4151a5af4f28956c8c5149
  source_path: builtin/srd-5.2.1/magic_item/cloak_of_elvenkind.md
  pages:
  - 215
  engine_gaps: &id001
  - dnd2024:item_cloak_of_elvenkind
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 140df2cba2d5f2bc2296e9bbf64866259b70c671b9c44c4619b44fc25dd84f59
splat_clauses:
- key: source_rules
  text: '# Cloak of Elvenkind Wondrous Item, Uncommon (Requires Attunement) While you wear this cloak,
    Wisdom (Perception) checks made to perceive you have Disadvantage, and you have Advantage on Dexterity
    (Stealth) checks.'
  sha256: 4be94e58968af6d9a1481d1dc8fc1f64c75fffe4f90a86a5b0fba9439dba5cf4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cloak of Elvenkind

<!-- SRD page 215 -->

Wondrous Item, Uncommon (Requires Attunement)\
While you wear this cloak, Wisdom (Perception)\
checks made to perceive you have Disadvantage,\
and you have Advantage on Dexterity (Stealth)\
checks.\

Original layout: [[../../srd-5.2.1/pages/215|page 215]].
