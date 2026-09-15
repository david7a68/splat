---
splat_schema: 1
splat_id: 44dc08e2-2b14-5ce7-8f42-ead52582e3d2
splat_kind: item
splat_key: dnd2024:items/scimitar_of_speed
splat_name: Scimitar of Speed
splat_origin:
  key: dnd2024:reference/magic_item/scimitar_of_speed
splat_sources:
- title: SRD 5.2.1
  locator: pp. 243
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/scimitar_of_speed
  implementation: dnd2024:item_scimitar_of_speed
  parameters:
    attack_roll_bonus: 2
    damage_roll_bonus: 2
    scope: attacks_with_source_item
    bonus_action_attack:
      count: 1
      weapon: source_item
      on_own_turn: true
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_weapons
splat_contracts:
- id: dnd2024:item_scimitar_of_speed
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
  - action.bonus_pending
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 46445efd98549b7783b2fee4f309a79fd53dc185155e159ccc91839cb3513d25
  source_path: builtin/srd-5.2.1/magic_item/scimitar_of_speed.md
  pages:
  - 243
  engine_gaps: &id001
  - dnd2024:item_scimitar_of_speed
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: a6add75b0ec17eb766ae7aee52f686d2c2f9d9e236f0a293ad3e1d0d4dde722a
splat_clauses:
- key: source_rules
  text: '# Scimitar of Speed Weapon (Scimitar), Very Rare (Requires Attunement) You gain a +2 bonus to
    attack rolls and damage rolls made with this magic weapon. In addition, you can make one attack with
    it as a Bonus Action on each of your turns.'
  sha256: 0f147117d6175468a2f7fd822448ff4b73e077387f99c3295c88de88d4f11844
  status: contract_specified
splat_requires_mechanics: *id001
---

# Scimitar of Speed

<!-- SRD page 243 -->

Weapon (Scimitar), Very Rare (Requires Attunement)\
You gain a +2 bonus to attack rolls and damage rolls\
made with this magic weapon. In addition, you can\
make one attack with it as a Bonus Action on each of\
your turns.\

Original layout: [[../../srd-5.2.1/pages/243|page 243]].
