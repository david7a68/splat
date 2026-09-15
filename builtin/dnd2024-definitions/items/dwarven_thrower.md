---
splat_schema: 1
splat_id: acb90637-d08a-534b-a064-87f0cc446e80
splat_kind: item
splat_key: dnd2024:items/dwarven_thrower
splat_name: Dwarven Thrower
splat_origin:
  key: dnd2024:reference/magic_item/dwarven_thrower
splat_sources:
- title: SRD 5.2.1
  locator: pp. 219
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: very_rare
  equipment:
    attunement: required
    attunement_requirement_text: a Dwarf or a Creature Attuned to a Belt of Dwarvenkind
splat_rule:
  mechanic: dnd2024:items/dwarven_thrower
  implementation: dnd2024:item_dwarven_thrower
  parameters:
    weapon_bonus:
      attack: 3
      damage: 3
    attunement_eligibility:
      any_of:
      - species: dwarf
      - attuned_item:
          key: dnd2024:items/belt_of_dwarvenkind
    thrown:
      normal_range_ft: 20
      long_range_ft: 60
    ranged_hit_extra_damage:
      type: force
      normal:
        dice: 1
        sides: 8
        bonus: 0
      giant:
        dice: 2
        sides: 8
        bonus: 0
    return_to_hand:
      event: ranged_attack_hit_or_miss
      timing: immediate
    attunement:
      required: true
      eligibility:
        any_of:
        - species: dwarf
        - attuned_item:
            key: dnd2024:items/belt_of_dwarvenkind
splat_contracts:
- id: dnd2024:item_dwarven_thrower
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
    event:
      required: false
      type: event
  outputs:
  - effects
  - item_state
  events:
  - item.used
  capabilities:
  - equipment
  - item_resources
  - effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: d92cccf915ba601edfbc9afc052067cdb221e58ada710dc18df2ec3538a0825d
  source_path: builtin/srd-5.2.1/magic_item/dwarven_thrower.md
  pages:
  - 219
  engine_gaps: &id001
  - dnd2024:item_dwarven_thrower
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: cea7dbdbc824f43df3cae864f87de0082112d6b16893b163c48a57c1a476c461
splat_clauses:
- key: source_rules
  text: '# Dwarven Thrower Weapon (Warhammer), Very Rare (Requires Attunement by a Dwarf or a Creature
    Attuned to a Belt of Dwarvenkind) You gain a +3 bonus to attack rolls and damage rolls made with this
    magic weapon. It has the Thrown property with a normal range of 20 feet and a long range of 60 feet.
    When you hit with a ranged attack using this weapon, it deals an extra 1d8 Force damage, or an extra
    2d8 Force damage if the target is a Giant. Immediately after hitting or missing, the weapon flies
    back to your hand.'
  sha256: 0fa9a9b30c7c23ba2d3890d253e7407d1651d0e570106ae71c923bb981e6b753
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dwarven Thrower

<!-- SRD page 219 -->

Weapon (Warhammer), Very Rare (Requires\
Attunement by a Dwarf or a Creature Attuned to a Belt\
of Dwarvenkind)\
You gain a +3 bonus to attack rolls and damage rolls\
made with this magic weapon. It has the Thrown\
property with a normal range of 20 feet and a long\
range of 60 feet. When you hit with a ranged attack\
using this weapon, it deals an extra 1d8 Force dam-\
age, or an extra 2d8 Force damage if the target is\
a Giant. Immediately after hitting or missing, the\
weapon flies back to your hand.\

Original layout: [[../../srd-5.2.1/pages/219|page 219]].
