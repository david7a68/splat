---
splat_schema: 1
splat_id: bd466de5-38b0-58da-a08b-e07395c573a5
splat_kind: item
splat_key: dnd2024:items/dagger_of_venom
splat_name: Dagger of Venom
splat_origin:
  key: dnd2024:reference/magic_item/dagger_of_venom
splat_sources:
- title: SRD 5.2.1
  locator: pp. 216
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/dagger_of_venom
  implementation: dnd2024:item_dagger_of_venom
  parameters:
    attack_roll_bonus: 1
    damage_roll_bonus: 1
    scope: attacks_with_source_item
    poison_coating:
      action: bonus_action
      uses:
        maximum: 1
        initial: 1
        recovery:
          event: dawn
          amount:
            expression: '1'
            scope: use
          cap: maximum
      duration:
        amount: 1
        unit: minute
      ends_on: first_creature_hit
      on_first_creature_hit:
        save:
          ability: constitution
          dc: 15
          failure:
            damage:
              roll:
                expression: 2d10
                scope: attack
              type: poison
            condition: poisoned
            duration:
              amount: 1
              unit: minute
          success:
            effect: none
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_weapons
splat_contracts:
- id: dnd2024:item_dagger_of_venom
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
  - action.bonus_pending
  - time.dawn
  - time.minute
  - attack.hit
  capabilities:
  - inventory
  - effects
  - resources_and_time
  - roll_resolution
  - participant_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: e20f9616d3288cd7d71c6d558bc8bb03c4941fba3f9d0503f9c5eefc00d99cff
  source_path: builtin/srd-5.2.1/magic_item/dagger_of_venom.md
  pages:
  - 216
  engine_gaps: &id001
  - dnd2024:item_dagger_of_venom
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 9046a9be21cfc9df250446f81219ab459ed40e816fe475beb7bd4d454e6fcfc3
splat_clauses:
- key: source_rules
  text: '# Dagger of Venom Weapon (Dagger), Rare You gain a +1 bonus to attack rolls and damage rolls
    made with this magic weapon. You can take a Bonus Action to magically coat the blade with poison.
    The poison remains for 1 minute or until an attack using this weapon hits a creature. That creature
    must succeed on a DC 15 Constitution saving throw or take 2d10 Poison damage and have the Poisoned
    condition for 1 minute. The weapon can’t be used this way again until the next dawn.'
  sha256: 2dd3888843ed62bedd8a62a86d4a57e1399a24d73d2f6a5a2ef78d679d5b0c22
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dagger of Venom

<!-- SRD page 216 -->

Weapon (Dagger), Rare\
You gain a +1 bonus to attack rolls and damage rolls\
made with this magic weapon.\
You can take a Bonus Action to magically coat the\
blade with poison. The poison remains for 1 minute\
or until an attack using this weapon hits a creature.\
That creature must succeed on a DC 15 Constitution\
saving throw or take 2d10 Poison damage and have\
the Poisoned condition for 1 minute. The weapon\
can’t be used this way again until the next dawn.\

Original layout: [[../../srd-5.2.1/pages/216|page 216]].
