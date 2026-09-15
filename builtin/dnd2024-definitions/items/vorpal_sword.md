---
splat_schema: 1
splat_id: 793209c7-3287-5bbd-a8e3-4e99c5897553
splat_kind: item
splat_key: dnd2024:items/vorpal_sword
splat_name: Vorpal Sword
splat_origin:
  key: dnd2024:reference/magic_item/vorpal_sword
splat_sources:
- title: SRD 5.2.1
  locator: pp. 250
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: legendary
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/vorpal_sword
  implementation: dnd2024:item_vorpal_sword
  parameters:
    attack_roll_bonus: 3
    damage_roll_bonus: 3
    scope: attacks_with_source_item
    ignore_damage_resistance:
    - slashing
    on_natural_20:
      target: creature
      decapitation:
        heads_removed: 1
        requires_at_least_one_head: true
        dies_if_cannot_survive_head_loss: true
        immune_if:
        - slashing_damage_immunity
        - no_head
        - head_not_needed
        gm_choice:
          type: boolean
          scope: head_too_large_to_sever_with_weapon
        legendary_resistance:
          optional_use: true
          daily_uses_cost: 1
          effect: replace_decapitation_with_extra_damage
      instead_if_decapitation_immune_or_resisted:
        extra_damage:
          amount: 30
          type: slashing
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_weapons
splat_contracts:
- id: dnd2024:item_vorpal_sword
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
  - damage.pending
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - resources_and_time
  - roll_resolution
  - participant_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: b374b33f337b2f4296f4ea7a9c2e9ab3b15cae2441968ed312c876dfc4212847
  source_path: builtin/srd-5.2.1/magic_item/vorpal_sword.md
  pages:
  - 250
  engine_gaps: &id001
  - dnd2024:item_vorpal_sword
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: e9c5881f6934657902625dcd440e110eb6d1e433ae88e615b82b77b7bbaf7f4c
splat_clauses:
- key: source_rules
  text: '# Vorpal Sword Weapon (Glaive, Greatsword, Longsword, or Scimitar), Legendary (Requires Attunement)
    You gain a +3 bonus to attack rolls and damage rolls made with this magic weapon. In addition, the
    weapon ignores Resistance to Slashing damage. When you use this weapon to attack a creature that has
    at least one head and roll a 20 on the d20 for the attack roll, you cut off one of the creature’s
    heads. The creature dies if it can’t survive without the lost head. A creature is immune to this effect
    if it has Immunity to Slashing damage, if it doesn’t have or need a head, or if the GM decides that
    the creature is too big for its head to be cut off with this weapon. Such a creature instead takes
    an extra 30 Slashing damage from the hit. If the creature has Legendary Resistance, it can expend
    one daily use of that trait to avoid losing its head, taking the extra damage instead.'
  sha256: 7535317893cec6e80790528d33ce9aa548398e2f8eea04f9be0ee644b73b720c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Vorpal Sword

<!-- SRD page 250 -->

Weapon (Glaive, Greatsword, Longsword, or Scimitar),\
Legendary (Requires Attunement)\
You gain a +3 bonus to attack rolls and damage\
rolls made with this magic weapon. In addition, the\
weapon ignores Resistance to Slashing damage.\
When you use this weapon to attack a creature\
that has at least one head and roll a 20 on the d20\
for the attack roll, you cut off one of the creature’s\
heads. The creature dies if it can’t survive without\
the lost head. A creature is immune to this effect\
if it has Immunity to Slashing damage, if it doesn’t\
have or need a head, or if the GM decides that the\
creature is too big for its head to be cut off with this\
weapon. Such a creature instead takes an extra 30\
Slashing damage from the hit. If the creature has\
Legendary Resistance, it can expend one daily use\
of that trait to avoid losing its head, taking the extra\
damage instead.\

Original layout: [[../../srd-5.2.1/pages/250|page 250]].
