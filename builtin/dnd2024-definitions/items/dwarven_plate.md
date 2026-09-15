---
splat_schema: 1
splat_id: 352fc4da-6796-5ed3-8326-600acc8d4387
splat_kind: item
splat_key: dnd2024:items/dwarven_plate
splat_name: Dwarven Plate
splat_origin:
  key: dnd2024:reference/magic_item/dwarven_plate
splat_sources:
- title: SRD 5.2.1
  locator: pp. 219
  url: https://www.dndbeyond.com/srd
splat_item:
  category: armor
  rarity: very_rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/dwarven_plate
  implementation: dnd2024:item_dwarven_plate
  parameters:
    equipped: worn
    ac_bonus: 2
    reaction:
      trigger: unwilling_movement_along_ground
      distance_reduction_ft:
        choice_min: 0
        choice_max: 10
        not_below_zero: true
    attunement_required: false
splat_contracts:
- id: dnd2024:item_dwarven_plate
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
  - movement.forced_pending
  capabilities:
  - item_effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 1d1eaa448fa0d1ce33362d86dd0c0408e857d8be97e0f96536269696a84a85b5
  source_path: builtin/srd-5.2.1/magic_item/dwarven_plate.md
  pages:
  - 219
  engine_gaps: &id001
  - dnd2024:item_dwarven_plate
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 731372de9ed45eb466ea159851bae53e1ae43212e65b825e240fbe069eba087e
splat_clauses:
- key: source_rules
  text: '# Dwarven Plate Armor (Half Plate Armor or Plate Armor), Very Rare While wearing this armor,
    you gain a +2 bonus to Armor Class. In addition, if an effect moves you against your will along the
    ground, you can take a Reaction to reduce the distance you are moved by up to 10 feet.'
  sha256: a5c39fc4566d3ef7706d18aa380682074b61ba312ec6e7be7736518ea0c84a0c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dwarven Plate

<!-- SRD page 219 -->

Armor (Half Plate Armor or Plate Armor), Very Rare\
While wearing this armor, you gain a +2 bonus to\
Armor Class. In addition, if an effect moves you\
against your will along the ground, you can take a\
Reaction to reduce the distance you are moved by\
up to 10 feet.\

Original layout: [[../../srd-5.2.1/pages/219|page 219]].
