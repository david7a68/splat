---
splat_schema: 1
splat_id: c2faff68-cbd0-58f9-9e07-68dc7c5b0d33
splat_kind: item
splat_key: dnd2024:items/potion_of_resistance
splat_name: Potion of Resistance
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_resistance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 236
  url: https://www.dndbeyond.com/srd
splat_item:
  category: potion
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/potion_of_resistance
  implementation: dnd2024:item_potion_of_resistance
  parameters:
    consume: drink
    duration:
      amount: 1
      unit: hour
    resistance:
      chooser: gm
      method:
      - choose
      - roll
      table:
        roll:
          expression: 1d10
          scope: use
        rows:
        - min: 1
          max: 1
          result:
            damage_type: acid
        - min: 2
          max: 2
          result:
            damage_type: cold
        - min: 3
          max: 3
          result:
            damage_type: fire
        - min: 4
          max: 4
          result:
            damage_type: force
        - min: 5
          max: 5
          result:
            damage_type: lightning
        - min: 6
          max: 6
          result:
            damage_type: necrotic
        - min: 7
          max: 7
          result:
            damage_type: poison
        - min: 8
          max: 8
          result:
            damage_type: psychic
        - min: 9
          max: 9
          result:
            damage_type: radiant
        - min: 10
          max: 10
          result:
            damage_type: thunder
    consume_action: bonus_action
    administer_action: bonus_action
    effect_target: drinker
    consumed_quantity: 1
    effect_starts: immediately_after_consumption
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_potions
splat_contracts:
- id: dnd2024:item_potion_of_resistance
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
  - item.consumed
  - time.elapsed
  - damage.pending
  capabilities:
  - inventory
  - effects
  - resources_and_time
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 38f6e6d3a710edda67c01f9adc243466e6899f8913bba864054712ff90b374c0
  source_path: builtin/srd-5.2.1/magic_item/potion_of_resistance.md
  pages:
  - 236
  engine_gaps: &id001
  - dnd2024:item_potion_of_resistance
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: f83013599a60912f88f9f1d927697f3149420d76b9b9b442283fda6793236ee7
splat_clauses:
- key: source_rules
  text: '# Potion of Resistance Potion, Uncommon When you drink this potion, you have Resistance to one
    type of damage for 1 hour. The GM chooses the type or determines it randomly by rolling on the following
    table. 1d10 Damage Type 1d10 Damage Type 1 Acid 6 Necrotic 2 Cold 7 Poison 3 Fire 8 Psychic 4 Force
    9 Radiant 5 Lightning 10 Thunder'
  sha256: cfe20cfebe162a049491dd01d4046b6702d4edce9baa36a26ba7a496a985611c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Potion of Resistance

<!-- SRD page 236 -->

Potion, Uncommon\
When you drink this potion, you have Resistance to\
one type of damage for 1 hour. The GM chooses the\
type or determines it randomly by rolling on the\
following table.\
1d10 Damage Type 1d10 Damage Type\
1 Acid 6 Necrotic\
2 Cold 7 Poison\
3 Fire 8 Psychic\
4 Force 9 Radiant\
5 Lightning 10 Thunder\

Original layout: [[../../srd-5.2.1/pages/236|page 236]].
