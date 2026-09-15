---
splat_schema: 1
splat_id: 064624ef-1abe-55f7-83bb-1b014c33d5b9
splat_kind: item
splat_key: dnd2024:items/dragon_slayer
splat_name: Dragon Slayer
splat_origin:
  key: dnd2024:reference/magic_item/dragon_slayer
splat_sources:
- title: SRD 5.2.1
  locator: pp. 219
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/dragon_slayer
  implementation: dnd2024:item_dragon_slayer
  parameters:
    attack_roll_bonus: 1
    damage_roll_bonus: 1
    scope: attacks_with_source_item
    on_hit:
      target_creature_type: dragon
      extra_damage:
        roll:
          expression: 3d6
          scope: attack
        type: weapon_damage_type
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_weapons
splat_contracts:
- id: dnd2024:item_dragon_slayer
  inputs:
    actor:
      type: participant
      required: true
    source_item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choice:
      required: false
      type: choice
    event:
      required: false
      type: event
    campaign:
      required: false
      type: campaign
  outputs:
  - item_state
  - effects
  - resolutions
  events:
  - attack.roll_pending
  - attack.hit
  capabilities:
  - inventory
  - effects
  - roll_resolution
  - participant_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 1e85779239040e6d86186335fad98070f7fd7d682e5f600a80667f53b0ff84b9
  source_path: builtin/srd-5.2.1/magic_item/dragon_slayer.md
  pages:
  - 219
  engine_gaps: &id001
  - dnd2024:item_dragon_slayer
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 72808f93996643a44749516b9f7000b9e7f3f247fd600d5e9c69a0b0f6358f4d
splat_clauses:
- key: source_rules
  text: '# Dragon Slayer Weapon (Any Simple or Martial), Rare You gain a +1 bonus to attack rolls and
    damage rolls made with this magic weapon. The weapon deals an extra 3d6 damage of the weapon’s type
    if the target is a Dragon.'
  sha256: 94765b08672ea88b0d3ccd2b7e4679ff54753535252da7f469c6d18f4fcf3ce7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dragon Slayer

<!-- SRD page 219 -->

Weapon (Any Simple or Martial), Rare\
You gain a +1 bonus to attack rolls and damage rolls\
made with this magic weapon.\
The weapon deals an extra 3d6 damage of the\
weapon’s type if the target is a Dragon.\

Original layout: [[../../srd-5.2.1/pages/219|page 219]].
