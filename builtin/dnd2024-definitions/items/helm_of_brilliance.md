---
splat_schema: 1
splat_id: d77716f3-9bd9-5fa1-9d18-848fd77c494d
splat_kind: item
splat_key: dnd2024:items/helm_of_brilliance
splat_name: Helm of Brilliance
splat_origin:
  key: dnd2024:reference/magic_item/helm_of_brilliance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 225, 226
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/helm_of_brilliance
  implementation: dnd2024:item_helm_of_brilliance
  parameters:
    equipment_state: worn
    gems:
      initial:
        diamond:
          dice: 1
          sides: 10
          bonus: 0
        ruby:
          dice: 2
          sides: 10
          bonus: 0
        fire_opal:
          dice: 3
          sides: 10
          bonus: 0
        opal:
          dice: 4
          sides: 10
          bonus: 0
      pried_out: destroy_gem
      none_remaining: item_loses_magic
    diamond_light:
      requires_gem: diamond
      shape: emanation
      radius_ft: 30
      light_if_undead_present: dim
      undead_turn_start_damage:
        roll:
          dice: 1
          sides: 6
          bonus: 0
        type: radiant
    fire_opal_flames:
      requires_gem: fire_opal
      action: magic
      target: one_held_weapon
      light:
        bright_radius_ft: 10
        additional_dim_ft: 10
      harmless_to:
      - user
      - weapon
      on_hit_extra_damage:
        roll:
          dice: 1
          sides: 6
          bonus: 0
        type: fire
      end_on:
      - bonus_action_extinguish
      - weapon_dropped
      - weapon_stowed
    ruby_resistance:
      requires_gem: ruby
      resistance: fire
    spells:
    - spell:
        key: dnd2024:spells/daylight
      save_dc: 18
      component:
        gem: opal
        quantity: 1
        consumed: true
        remove_from_helm: true
    - spell:
        key: dnd2024:spells/fireball
      save_dc: 18
      component:
        gem: fire_opal
        quantity: 1
        consumed: true
        remove_from_helm: true
    - spell:
        key: dnd2024:spells/prismatic_spray
      save_dc: 18
      component:
        gem: diamond
        quantity: 1
        consumed: true
        remove_from_helm: true
    - spell:
        key: dnd2024:spells/wall_of_fire
      save_dc: 18
      component:
        gem: ruby
        quantity: 1
        consumed: true
        remove_from_helm: true
    fire_damage_mishap:
      trigger:
        event: fire_damage_received
        cause: failed_save_against_spell
      roll:
        dice: 1
        sides: 20
        bonus: 0
      on_value: 1
      effect:
        shape: emanation
        radius_ft: 60
        targets: all_creatures
        save:
          ability: dexterity
          dc: 17
        failure_damage:
          value_from: remaining_gem_count
          type: radiant
        success_damage: 0
        then: destroy_helm
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_helm_of_brilliance
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
  - spell.cast_completed
  capabilities:
  - equipment
  - item_resources
  - effects
  - item_spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 1d05f10c0e1d435c4675ad1cf5f21e8b0acf88ce23dfc8b0d3b640bbfda9b4b8
  source_path: builtin/srd-5.2.1/magic_item/helm_of_brilliance.md
  pages:
  - 225
  - 226
  engine_gaps: &id001
  - dnd2024:item_helm_of_brilliance
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: a8cb377a0e7508d197f383875834a5346e8bac100b30dc4e9bcb56a32226e4ea
splat_clauses:
- key: source_rules
  text: '# Helm of Brilliance Wondrous Item, Very Rare (Requires Attunement) This helm is set with 1d10
    diamonds, 2d10 rubies, 3d10 fire opals, and 4d10 opals. Any gem pried from the helm crumbles to dust.
    When all the gems are removed or destroyed, the helm loses its magic. You gain the following benefits
    while wearing the helm. Diamond Light. As long as it has at least one diamond, the helm emits a 30-foot
    Emanation. When at least one Undead is within that area, the Emanation is filled with Dim Light. Any
    Undead that starts its turn in that area takes 1d6 Radiant damage. Fire Opal Flames. As long as the
    helm has at least one fire opal, you can take a Magic action to cause one weapon you are holding to
    burst into flames. The flames emit Bright Light in a 10-foot radius and Dim Light for an additional
    10 feet. The flames are harmless to you and the weapon. When you hit with an attack using the blazing
    weapon, the target takes an extra 1d6 Fire damage. The flames last until you take a Bonus Action to
    extinguish them or until you drop or stow the weapon. Ruby Resistance. As long as the helm has at
    least one ruby, you have Resistance to Fire damage. Spells. You can cast one of the following spells
    (save DC 18), using one of the helm’s gems of the specified type as a component: Daylight (opal),
    Fireball (fire opal), Prismatic Spray (diamond), or Wall of Fire (ruby). The gem is destroyed when
    the spell is cast and disappears from the helm. Taking Fire Damage. Roll 1d20 if you are wearing the
    helm and take Fire damage as a result of failing a saving throw against a spell. On a roll of 1, the
    helm emits beams of light from its remaining gems and is then destroyed. Each creature within a 60-
    foot Emanation originating from you must succeed on a DC 17 Dexterity saving throw or be struck by
    a beam, taking Radiant damage equal to the number of gems in the helm.'
  sha256: ece47e903259a96e443e57b249d4691e25aadd0099235adad486cb3bf3f31d01
  status: contract_specified
splat_requires_mechanics: *id001
---

# Helm of Brilliance

<!-- SRD page 225 -->

Wondrous Item, Very Rare (Requires Attunement)\
This helm is set with 1d10 diamonds, 2d10 rubies,\
3d10 fire opals, and 4d10 opals. Any gem pried from\
the helm crumbles to dust. When all the gems are\
removed or destroyed, the helm loses its magic.\
You gain the following benefits while wearing the\
helm.\
Diamond Light. As long as it has at least one\
diamond, the helm emits a 30-foot Emanation.\
When at least one Undead is within that area, the\
Emanation is filled with Dim Light. Any Undead\
that starts its turn in that area takes 1d6 Radiant\
damage.\
Fire Opal Flames. As long as the helm has at least\
one fire opal, you can take a Magic action to cause\
one weapon you are holding to burst into flames.\
The flames emit Bright Light in a 10-foot radius and\
Dim Light for an additional 10 feet. The flames are\
harmless to you and the weapon. When you hit with\
an attack using the blazing weapon, the target takes\
an extra 1d6 Fire damage. The flames last until you\
take a Bonus Action to extinguish them or until you\
drop or stow the weapon.\
Ruby Resistance. As long as the helm has at least\
one ruby, you have Resistance to Fire damage.\
Spells. You can cast one of the following spells\
(save DC 18), using one of the helm’s gems of the\
specified type as a component: Daylight (opal), Fire-\
ball (fire opal), Prismatic Spray (diamond), or Wall of\
Fire (ruby). The gem is destroyed when the spell is\
cast and disappears from the helm.\
Taking Fire Damage. Roll 1d20 if you are wearing\
the helm and take Fire damage as a result of failing\

<!-- SRD page 226 -->

a saving throw against a spell. On a roll of 1, the\
helm emits beams of light from its remaining gems\
and is then destroyed. Each creature within a 60-\
foot Emanation originating from you must succeed\
on a DC 17 Dexterity saving throw or be struck by a\
beam, taking Radiant damage equal to the number\
of gems in the helm.\

Original layout: [[../../srd-5.2.1/pages/225|page 225]], [[../../srd-5.2.1/pages/226|page 226]].
