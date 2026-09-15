---
splat_schema: 1
splat_id: 9a565af0-d3f5-5b2b-ae60-c1ef925a3dd3
splat_kind: item
splat_key: dnd2024:items/gloves_of_missile_snaring
splat_name: Gloves of Missile Snaring
splat_origin:
  key: dnd2024:reference/magic_item/gloves_of_missile_snaring
splat_sources:
- title: SRD 5.2.1
  locator: pp. 224
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/gloves_of_missile_snaring
  implementation: dnd2024:item_gloves_of_missile_snaring
  parameters:
    equipment_state: worn
    reaction:
      trigger:
        hit_by: attack_roll
        weapon_category:
        - ranged
        - thrown
      requires: free_hand
      damage_reduction:
        sum:
        - dice: 1
          sides: 10
          bonus: 0
        - value_from: dexterity_modifier
      if_damage_reduced_to_zero:
        may_catch:
        - ammunition
        - weapon
        requires: fits_free_hand
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_gloves_of_missile_snaring
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    event:
      required: false
      type: event
  outputs:
  - modifiers
  - effects
  - damage
  - item_state
  events:
  - equipment.state_changed
  - attack.resolved
  - saving_throw.resolved
  capabilities:
  - equipment
  - combat_reactions
  - effects
  - damage
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: d4d15520e86acc8949c9663eaf0af8c781fee767c64184f0ae22220eeb038390
  source_path: builtin/srd-5.2.1/magic_item/gloves_of_missile_snaring.md
  pages:
  - 224
  engine_gaps: &id001
  - dnd2024:item_gloves_of_missile_snaring
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 97425a80f53f2d650bd1c4410953fb0e07947b08d3ffb3dbe10c0b5b6c8371f6
splat_clauses:
- key: source_rules
  text: '# Gloves of Missile Snaring Wondrous Item, Uncommon (Requires Attunement) If you’re hit by an
    attack roll made with a Ranged or Thrown weapon while wearing these gloves, you can take a Reaction
    to reduce the damage by 1d10 plus your Dexterity modifier if you have a free hand. If you reduce the
    damage to 0, you can catch the ammunition or weapon if it is small enough for you to hold in that
    hand.'
  sha256: 2ea56d32bb56fabe21b3f402d30d7e2ed8523050faab5b947c4d9b2c271a72d6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gloves of Missile Snaring

<!-- SRD page 224 -->

Wondrous Item, Uncommon (Requires Attunement)\
If you’re hit by an attack roll made with a Ranged\
or Thrown weapon while wearing these gloves, you\
can take a Reaction to reduce the damage by 1d10\
plus your Dexterity modifier if you have a free hand.\
If you reduce the damage to 0, you can catch the am-\
munition or weapon if it is small enough for you to\
hold in that hand.\

Original layout: [[../../srd-5.2.1/pages/224|page 224]].
