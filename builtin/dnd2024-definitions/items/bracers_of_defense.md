---
splat_schema: 1
splat_id: 7fb74bcd-4799-55b3-9662-b3a9c42e6975
splat_kind: item
splat_key: dnd2024:items/bracers_of_defense
splat_name: Bracers of Defense
splat_origin:
  key: dnd2024:reference/magic_item/bracers_of_defense
splat_sources:
- title: SRD 5.2.1
  locator: pp. 214
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/bracers_of_defense
  implementation: dnd2024:item_bracers_of_defense
  parameters:
    equipped: worn
    requires_equipment_state:
      armor_worn: false
      shield_used: false
    ac_bonus: 2
    attunement_required: true
splat_contracts:
- id: dnd2024:item_bracers_of_defense
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
  - item.equipment_changed
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 2a6c9cb1a08ade528e2693f640542c924fbe95c78a9cabeb77374c1cbe7169a9
  source_path: builtin/srd-5.2.1/magic_item/bracers_of_defense.md
  pages:
  - 214
  engine_gaps: &id001
  - dnd2024:item_bracers_of_defense
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 3294db740e0525365341f82d677e08a17021e35152149e30b5547e25b6ec956b
splat_clauses:
- key: source_rules
  text: '# Bracers of Defense Wondrous Item, Rare (Requires Attunement) While wearing these bracers, you
    gain a +2 bonus to Armor Class if you are wearing no armor and using no Shield.'
  sha256: 3d6e5e998173c87132a7919d46479433d8754cc287c15cfea2f2722e4e03d475
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bracers of Defense

<!-- SRD page 214 -->

Wondrous Item, Rare (Requires Attunement)\
While wearing these bracers, you gain a +2 bonus to\
Armor Class if you are wearing no armor and using\
no Shield.\

Original layout: [[../../srd-5.2.1/pages/214|page 214]].
