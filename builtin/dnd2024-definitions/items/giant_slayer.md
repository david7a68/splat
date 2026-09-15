---
splat_schema: 1
splat_id: 963f89d2-fadc-5614-a4b8-15170424c831
splat_kind: item
splat_key: dnd2024:items/giant_slayer
splat_name: Giant Slayer
splat_origin:
  key: dnd2024:reference/magic_item/giant_slayer
splat_sources:
- title: SRD 5.2.1
  locator: pp. 223
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/giant_slayer
  implementation: dnd2024:item_giant_slayer
  parameters:
    attack_roll_bonus: 1
    damage_roll_bonus: 1
    scope: attacks_with_source_item
    on_hit:
      target_creature_type: giant
      extra_damage:
        roll:
          expression: 2d6
          scope: attack
        type: weapon_damage_type
      save:
        ability: strength
        dc: 15
        failure:
          condition: prone
        success:
          effect: none
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_weapons
splat_contracts:
- id: dnd2024:item_giant_slayer
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
  source_sha256: 1a075073e0d827999a833c302aa1df84cddec161b3175d13fffa413a04cafc59
  source_path: builtin/srd-5.2.1/magic_item/giant_slayer.md
  pages:
  - 223
  engine_gaps: &id001
  - dnd2024:item_giant_slayer
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 38c1984eaf23d42824ec86c8ae16a2a59509a1f52eda498ebc618c1fb3148b7c
splat_clauses:
- key: source_rules
  text: '# Giant Slayer Weapon (Any Simple or Martial), Rare You gain a +1 bonus to attack rolls and damage
    rolls made with this magic weapon. When you hit a Giant with this weapon, the Giant takes an extra
    2d6 damage of the weapon’s type and must succeed on a DC 15 Strength saving throw or have the Prone
    condition.'
  sha256: 14fc9986e3d22af93dd5a106790e75cb4a7537b46e4207abce68997d6395aa2c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Slayer

<!-- SRD page 223 -->

Weapon (Any Simple or Martial), Rare\
You gain a +1 bonus to attack rolls and damage rolls\
made with this magic weapon.\
When you hit a Giant with this weapon, the Giant\
takes an extra 2d6 damage of the weapon’s type and\
must succeed on a DC 15 Strength saving throw or\
have the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/223|page 223]].
