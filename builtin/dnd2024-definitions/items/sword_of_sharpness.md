---
splat_schema: 1
splat_id: 7eb7ac23-9716-5f16-9f5c-54f75ddef069
splat_kind: item
splat_key: dnd2024:items/sword_of_sharpness
splat_name: Sword of Sharpness
splat_origin:
  key: dnd2024:reference/magic_item/sword_of_sharpness
splat_sources:
- title: SRD 5.2.1
  locator: pp. 248
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/sword_of_sharpness
  implementation: dnd2024:item_sword_of_sharpness
  parameters:
    on_hit_object:
      maximize: weapon_damage_dice
    on_natural_20_against_creature:
      extra_damage:
        amount: 14
        type: slashing
      exhaustion_increase: 1
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_weapons
splat_contracts:
- id: dnd2024:item_sword_of_sharpness
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
  - attack.hit
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - roll_resolution
  - participant_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: e74fd875199ee67039450b3b136c2d217064075813ceeabf95b774806d6327df
  source_path: builtin/srd-5.2.1/magic_item/sword_of_sharpness.md
  pages:
  - 248
  engine_gaps: &id001
  - dnd2024:item_sword_of_sharpness
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: dad7917560a0c5fa6b406a22f04feb0a736223abbacf8fcb460aa0b8df25916c
splat_clauses:
- key: source_rules
  text: '# Sword of Sharpness Weapon (Glaive, Greatsword, Longsword, or Scimitar), Very Rare (Requires
    Attunement) When you attack an object with this magic weapon and hit, maximize your weapon damage
    dice against the target. When you attack a creature with this weapon and roll a 20 on the d20 for
    the attack roll, that target takes an extra 14 Slashing damage and gains 1 Exhaustion level.'
  sha256: 493c6af0a22cb4bfafd17dec82b327f603a676e32ffa832dbaf2f434b101169d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sword of Sharpness

<!-- SRD page 248 -->

Weapon (Glaive, Greatsword, Longsword, or Scimitar),\
Very Rare (Requires Attunement)\
When you attack an object with this magic weapon\
and hit, maximize your weapon damage dice\
against the target.\
When you attack a creature with this weapon\
and roll a 20 on the d20 for the attack roll, that tar-\
get takes an extra 14 Slashing damage and gains\
1 Exhaustion level.\

Original layout: [[../../srd-5.2.1/pages/248|page 248]].
