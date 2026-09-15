---
splat_schema: 1
splat_id: d96beda7-1874-5f6c-861d-3e4f2d84ed2b
splat_kind: item
splat_key: dnd2024:items/sword_of_life_stealing
splat_name: Sword of Life Stealing
splat_origin:
  key: dnd2024:reference/magic_item/sword_of_life_stealing
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
  mechanic: dnd2024:items/sword_of_life_stealing
  implementation: dnd2024:item_sword_of_life_stealing
  parameters:
    trigger:
      event: attack_roll
      natural_d20: 20
      target:
        kind: creature
        excluded_types:
        - construct
        - undead
    extra_damage:
      amount: 15
      type: necrotic
    temporary_hit_points:
      equal_to: necrotic_damage_taken_by_target
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_sword_of_life_stealing
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
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 4b36869130ce2f109f70bea9cc00bbdc8be77bb81d38d9cff67f7670d2e67032
  source_path: builtin/srd-5.2.1/magic_item/sword_of_life_stealing.md
  pages:
  - 248
  engine_gaps: &id001
  - dnd2024:item_sword_of_life_stealing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 15f8c20337b8bbc707a8649941f78b5a502e5d41c206b86d80aa7a6035b665af
splat_clauses:
- key: source_rules
  text: '# Sword of Life Stealing Weapon (Glaive, Greatsword, Longsword, Rapier, Scimitar, or Shortsword),
    Rare (Requires Attunement) When you attack a creature with this magic weapon and roll a 20 on the
    d20 for the attack roll, that target takes an extra 15 Necrotic damage if it isn’t a Construct or
    an Undead, and you gain Temporary Hit Points equal to the amount of Necrotic damage taken.'
  sha256: da125b55059bf705c3370c41405d14f2570fe08b8df103ea804c267b725445cd
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sword of Life Stealing

<!-- SRD page 248 -->

Weapon (Glaive, Greatsword, Longsword, Rapier,\
Scimitar, or Shortsword), Rare (Requires Attunement)\
When you attack a creature with this magic weapon\
and roll a 20 on the d20 for the attack roll, that\
target takes an extra 15 Necrotic damage if it isn’t\
a Construct or an Undead, and you gain Temporary\
Hit Points equal to the amount of Necrotic damage\
taken.\

Original layout: [[../../srd-5.2.1/pages/248|page 248]].
