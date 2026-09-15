---
splat_schema: 1
splat_id: d8b984a1-d4d8-527a-a4d3-583135d21cc8
splat_kind: item
splat_key: dnd2024:items/scarab_of_protection
splat_name: Scarab of Protection
splat_origin:
  key: dnd2024:reference/magic_item/scarab_of_protection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 243
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: legendary
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/scarab_of_protection
  implementation: dnd2024:item_scarab_of_protection
  parameters:
    equipment_state: on_person
    armor_class_bonus: 1
    saving_throws_against_spells: advantage
    charges:
      initial: 12
      maximum: 12
      recovery: null
      on_zero: destroy_item
    preservation:
      action: reaction
      trigger: failed_saving_throw
      eligible_effect:
        any_of:
        - spell_school: necromancy
        - origin_type: undead
          harmful: true
      cost: 1
      replace_result: success
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_scarab_of_protection
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
  - modifiers
  - effects
  - damage
  - item_state
  events:
  - equipment.state_changed
  - attack.resolved
  - saving_throw.resolved
  capabilities:
  - equipment
  - combat_reactions
  - effects
  - damage
  - item_resources
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 3cd075a51fb2088d03759187e3d85506a9eaac5121167eb78c4b951a47e02a29
  source_path: builtin/srd-5.2.1/magic_item/scarab_of_protection.md
  pages:
  - 243
  engine_gaps: &id001
  - dnd2024:item_scarab_of_protection
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: fdeaa42c015e053f3351d601c978142665d8230cb064188e0cbeeed66144dba4
splat_clauses:
- key: source_rules
  text: '# Scarab of Protection Wondrous Item, Legendary (Requires Attunement) This beetle-shaped medallion
    provides three benefits while it is on your person. Defense. You gain a +1 bonus to Armor Class. Preservation.
    The scarab has 12 charges. If you fail a saving throw against a Necromancy spell or a harmful effect
    originating from an Undead, you can take a Reaction to expend 1 charge and turn the failed save into
    a successful one. The scarab crumbles into powder and is destroyed when its last charge is expended.
    Spell Resistance. You have Advantage on saving throws against spells.'
  sha256: 610b0d3555db370d8851a8866a071140a24d18fc12cb1551659ff059854150c9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Scarab of Protection

<!-- SRD page 243 -->

Wondrous Item, Legendary (Requires Attunement)\
This beetle-shaped medallion provides three bene-\
fits while it is on your person.\
Defense. You gain a +1 bonus to Armor Class.\
Preservation. The scarab has 12 charges. If you\
fail a saving throw against a Necromancy spell or\
a harmful effect originating from an Undead, you\
can take a Reaction to expend 1 charge and turn the\
failed save into a successful one. The scarab crum-\
bles into powder and is destroyed when its last\
charge is expended.\
Spell Resistance. You have Advantage on saving\
throws against spells.\

Original layout: [[../../srd-5.2.1/pages/243|page 243]].
