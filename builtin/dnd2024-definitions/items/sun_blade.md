---
splat_schema: 1
splat_id: 5afb35f3-dc56-5f6d-8f4c-a92cecbc4719
splat_kind: item
splat_key: dnd2024:items/sun_blade
splat_name: Sun Blade
splat_origin:
  key: dnd2024:reference/magic_item/sun_blade
splat_sources:
- title: SRD 5.2.1
  locator: pp. 248
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/sun_blade
  implementation: dnd2024:item_sun_blade
  parameters:
    default_form: hilt
    blade_toggle:
      action: bonus_action
      requires: grasping_hilt
    while_blade_exists:
      weapon:
        key: dnd2024:equipment/longsword
      properties_add:
      - finesse
      proficient_if_any:
      - longsword_proficiency
      - shortsword_proficiency
      attack_bonus: 2
      damage_bonus: 2
      replace_damage_type:
        from: slashing
        to: radiant
      on_hit:
        target_type: undead
        damage:
          amount:
            roll:
              count: 1
              sides: 8
          type: radiant
      light:
        bright_radius_ft: 15
        additional_dim_ft: 15
        sunlight: true
      adjust_light:
        action: magic
        increment_ft: 5
        minimum_bright_and_additional_dim_ft: 10
        maximum_bright_and_additional_dim_ft: 30
        adjust_both_together: true
    attunement_required: true
splat_contracts:
- id: dnd2024:item_sun_blade
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    selection:
      required: false
      type: choice
    context:
      required: false
      type: resolution
  outputs:
  - effects
  - item_state_changes
  events:
  - item.activation_requested
  - attack.hit
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - dice
  - damage
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 091da2b42e4757fe2fff45cedba134bb50e3f1f97fe11029d2219ca019c15a9f
  source_path: builtin/srd-5.2.1/magic_item/sun_blade.md
  pages:
  - 248
  engine_gaps: &id001
  - dnd2024:item_sun_blade
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: f82ba62480c23dda0ca9b47f9c4dd4c04185f5a822dee79d62ab5ed024f6a1ec
splat_clauses:
- key: source_rules
  text: '# Sun Blade Weapon (Longsword), Rare (Requires Attunement) This item appears to be a sword hilt.
    Blade of Radiance. While grasping the hilt, you can take a Bonus Action to cause a blade of pure radiance
    to spring into existence or make the blade disappear. While the blade exists, this magic weapon functions
    as a Longsword with the Finesse property. If you are proficient with Longswords or Shortswords, you
    are proficient with the Sun Blade. You gain a +2 bonus to attack rolls and damage rolls made with
    this weapon, which deals Radiant damage instead of Slashing damage. When you hit an Undead with it,
    that target takes an extra 1d8 Radiant damage. Sunlight. The sword’s luminous blade emits Bright Light
    in a 15-foot radius and Dim Light for an additional 15 feet. The light is sunlight. While the blade
    persists, you can take a Magic action to expand or reduce its radius of Bright Light and Dim Light
    by 5 feet each, to a maximum of 30 feet each or a minimum of 10 feet each.'
  sha256: c46cbd509c3bd0e7db4170e55d5a2a51c5ca9750ff9ba2c640c4ebaf01534e4f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sun Blade

<!-- SRD page 248 -->

Weapon (Longsword), Rare (Requires Attunement)\
This item appears to be a sword hilt.\
Blade of Radiance. While grasping the hilt, you\
can take a Bonus Action to cause a blade of pure\
radiance to spring into existence or make the\
blade disappear. While the blade exists, this magic\
weapon functions as a Longsword with the Finesse\
property. If you are proficient with Longswords or\
Shortswords, you are proficient with the Sun Blade.\
You gain a +2 bonus to attack rolls and damage\
rolls made with this weapon, which deals Radiant\
damage instead of Slashing damage. When you hit\
an Undead with it, that target takes an extra 1d8\
Radiant damage.\
Sunlight. The sword’s luminous blade emits\
Bright Light in a 15-foot radius and Dim Light for\
an additional 15 feet. The light is sunlight. While\
the blade persists, you can take a Magic action to\
expand or reduce its radius of Bright Light and Dim\
Light by 5 feet each, to a maximum of 30 feet each\
or a minimum of 10 feet each.\

Original layout: [[../../srd-5.2.1/pages/248|page 248]].
