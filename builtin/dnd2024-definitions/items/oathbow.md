---
splat_schema: 1
splat_id: fcc1fd54-e8f6-57b6-be5f-c4128d587fe8
splat_kind: item
splat_key: dnd2024:items/oathbow
splat_name: Oathbow
splat_origin:
  key: dnd2024:reference/magic_item/oathbow
splat_sources:
- title: SRD 5.2.1
  locator: pp. 233
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/oathbow
  implementation: dnd2024:item_oathbow
  parameters:
    declare_enemy:
      trigger: ranged_attack_with_bow
      command:
        modes:
        - utter
        - sign
      target: attack_target
      maximum_active: 1
      duration:
        until_any:
        - enemy_death
        - dawn_seven_days_later
      if_enemy_dies_next_selection_after: next_dawn
    against_sworn_enemy:
      attack_advantage: true
      ignore_cover:
      - half
      - three_quarters
      ignore_long_range_disadvantage: true
      on_hit:
        damage:
          roll:
            expression: 3d6
            scope: attack
          type: piercing
    while_sworn_enemy_alive:
      other_weapons_attack_disadvantage: true
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_weapons
splat_contracts:
- id: dnd2024:item_oathbow
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
  - participant.died
  - time.dawn
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - resources_and_time
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: aa071c65c397630ad458dba7642f0ce0b2c9750c487091add23488ce91ead33b
  source_path: builtin/srd-5.2.1/magic_item/oathbow.md
  pages:
  - 233
  engine_gaps: &id001
  - dnd2024:item_oathbow
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 58ff322540589ce4d8a66014233b7fb72033521849b1103ad295c8cde3740f3b
splat_clauses:
- key: source_rules
  text: '# Oathbow Weapon (Longbow or Shortbow), Very Rare (Requires Attunement) When you nock an arrow
    on this bow, it whispers in Elvish, “Swift defeat to my enemies.” When you use this weapon to make
    a ranged attack, you can utter or sign the following command words: “Swift death to you who have wronged
    me.” The target of your attack becomes your sworn enemy until it dies or until dawn 7 days later.
    You can have only one such sworn enemy at a time. When your sworn enemy dies, you can choose a new
    one after the next dawn. When you make a ranged attack roll with this weapon against your sworn enemy,
    you have Advantage on the roll. In addition, your target gains no benefit from Half Cover or Three-Quarters
    Cover, and you suffer no Disadvantage due to long range. If the attack hits, your sworn enemy takes
    an extra 3d6 Piercing damage. While your sworn enemy lives, you have Disadvantage on attack rolls
    with all other weapons.'
  sha256: 0348ee472479dfec1037e78fb194b119e542d3d9be6473310b1569f5992b890c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Oathbow

<!-- SRD page 233 -->

Weapon (Longbow or Shortbow), Very Rare (Requires\
Attunement)\
When you nock an arrow on this bow, it whispers in\
Elvish, “Swift defeat to my enemies.” When you use\
this weapon to make a ranged attack, you can utter\
or sign the following command words: “Swift death\
to you who have wronged me.” The target of your\
attack becomes your sworn enemy until it dies or\
until dawn 7 days later. You can have only one such\
sworn enemy at a time. When your sworn enemy\
dies, you can choose a new one after the next dawn.\
When you make a ranged attack roll with this\
weapon against your sworn enemy, you have Ad-\
vantage on the roll. In addition, your target gains no\
benefit from Half Cover or Three-Quarters Cover,\
and you suffer no Disadvantage due to long range.\
If the attack hits, your sworn enemy takes an extra\
3d6 Piercing damage.\
While your sworn enemy lives, you have Disad-\
vantage on attack rolls with all other weapons.\

Original layout: [[../../srd-5.2.1/pages/233|page 233]].
