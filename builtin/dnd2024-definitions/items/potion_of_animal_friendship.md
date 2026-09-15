---
splat_schema: 1
splat_id: cb8eacdd-70dc-5c61-a8f3-6c6e12cefb0e
splat_kind: item
splat_key: dnd2024:items/potion_of_animal_friendship
splat_name: Potion of Animal Friendship
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_animal_friendship
splat_sources:
- title: SRD 5.2.1
  locator: pp. 235
  url: https://www.dndbeyond.com/srd
splat_item:
  category: potion
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/potion_of_animal_friendship
  implementation: dnd2024:item_potion_of_animal_friendship
  parameters:
    trigger: potion_drunk
    consumed: true
    cast:
      spell:
        key: dnd2024:spells/animal_friendship
      level: 3
      save_dc: 13
    cast_timing: on_drinking
    optional_cast: true
    attunement_required: false
splat_contracts:
- id: dnd2024:item_potion_of_animal_friendship
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
  - item.consumed
  capabilities:
  - item_effects
  - item_spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 277ef5aa77aec9dd3ee0953efb6f3df1b7ae6d1e269a2c614af100d7fadcc9ed
  source_path: builtin/srd-5.2.1/magic_item/potion_of_animal_friendship.md
  pages:
  - 235
  engine_gaps: &id001
  - dnd2024:item_potion_of_animal_friendship
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 1d0bcd85cd667d627aed97c306a516e7355a8a2faeed1ea4df16e6caf320d0b3
splat_clauses:
- key: source_rules
  text: '# Potion of Animal Friendship Potion, Uncommon When you drink this potion, you can cast the level
    3 version of the Animal Friendship spell (save DC 13). Agitating this potion’s muddy liquid brings
    little bits into view: a fish scale, a hummingbird feather, a cat claw, or a squirrel hair.'
  sha256: 0a23c89f364ed1469110ee48b30a2dbdd2c019ab32d86c83ef0682f055ed2619
  status: contract_specified
splat_requires_mechanics: *id001
---

# Potion of Animal Friendship

<!-- SRD page 235 -->

Potion, Uncommon\
When you drink this potion, you can cast the level\
3 version of the Animal Friendship spell (save DC 13).\
Agitating this potion’s muddy liquid brings little\
bits into view: a fish scale, a hummingbird feather, a\
cat claw, or a squirrel hair.\

Original layout: [[../../srd-5.2.1/pages/235|page 235]].
