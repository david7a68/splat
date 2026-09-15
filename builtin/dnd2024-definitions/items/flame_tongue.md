---
splat_schema: 1
splat_id: 03f997cb-7c4e-58c3-997a-c948ba0ef9ab
splat_kind: item
splat_key: dnd2024:items/flame_tongue
splat_name: Flame Tongue
splat_origin:
  key: dnd2024:reference/magic_item/flame_tongue
splat_sources:
- title: SRD 5.2.1
  locator: pp. 223
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/flame_tongue
  implementation: dnd2024:item_flame_tongue
  parameters:
    equipment_state: held
    ignite:
      action: bonus_action
      command_word: true
    ablaze:
      bright_radius_ft: 40
      dim_additional_ft: 40
      on_hit:
        damage:
          roll:
            expression: 2d6
            scope: attack
          type: fire
    extinguish:
      action: bonus_action
      command_word: true
    ends_on:
    - drop
    - stow
    - sheathe
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_weapons
splat_contracts:
- id: dnd2024:item_flame_tongue
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
  - equipment.changed
  - attack.hit
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - roll_resolution
  - spatial_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: df1462a3a00322dbc6df7cf1547381734db71ddf055073db115e16d296974cde
  source_path: builtin/srd-5.2.1/magic_item/flame_tongue.md
  pages:
  - 223
  engine_gaps: &id001
  - dnd2024:item_flame_tongue
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: c073d25a03374a08b534633f15bb9a6c605e2fd4b9d12023536bab04260c4812
splat_clauses:
- key: source_rules
  text: '# Flame Tongue Weapon (Any Melee Weapon), Rare (Requires Attunement) While holding this magic
    weapon, you can take a Bonus Action and use a command word to cause flames to engulf the damage-dealing
    part of the weapon. These flames shed Bright Light in a 40- foot radius and Dim Light for an additional
    40 feet. While the weapon is ablaze, it deals an extra 2d6 Fire damage on a hit. The flames last until
    you take a Bonus Action to issue the command again or until you drop, stow, or sheathe the weapon.'
  sha256: ab5406ebe7aca9153a16213b57b13addc8ff67b78aba683159f1ab611c1851ca
  status: contract_specified
splat_requires_mechanics: *id001
---

# Flame Tongue

<!-- SRD page 223 -->

Weapon (Any Melee Weapon), Rare (Requires\
Attunement)\
While holding this magic weapon, you can take a\
Bonus Action and use a command word to cause\
flames to engulf the damage-dealing part of the\
weapon. These flames shed Bright Light in a 40-\
foot radius and Dim Light for an additional 40 feet.\
While the weapon is ablaze, it deals an extra 2d6\
Fire damage on a hit. The flames last until you take\
a Bonus Action to issue the command again or until\
you drop, stow, or sheathe the weapon.\

Original layout: [[../../srd-5.2.1/pages/223|page 223]].
