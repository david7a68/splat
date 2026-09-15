---
splat_schema: 1
splat_id: 1063d842-0468-5b8a-86de-f8e52750cbac
splat_kind: item
splat_key: dnd2024:items/potions_of_healing
splat_name: Potions of Healing
splat_origin:
  key: dnd2024:reference/magic_item/potions_of_healing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 236
  url: https://www.dndbeyond.com/srd
splat_item:
  category: potion
  rarity: rarity_varies
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/potions_of_healing
  implementation: dnd2024:item_potions_of_healing
  parameters:
    trigger: potion_drunk
    consumed: true
    variants:
    - name: healing
      rarity: common
      heal_hp:
        roll:
          count: 2
          sides: 4
          add: 2
    - name: greater
      rarity: uncommon
      heal_hp:
        roll:
          count: 4
          sides: 4
          add: 4
    - name: superior
      rarity: rare
      heal_hp:
        roll:
          count: 8
          sides: 4
          add: 8
    - name: supreme
      rarity: very_rare
      heal_hp:
        roll:
          count: 10
          sides: 4
          add: 20
    attunement_required: false
splat_contracts:
- id: dnd2024:item_potions_of_healing
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
  - dice
  - item_variants
  - healing
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: c8f6e7c5678be9bcd115b2e49bcd4d16b37c55ff54bc1e73259bd8925fc9f8f4
  source_path: builtin/srd-5.2.1/magic_item/potions_of_healing.md
  pages:
  - 236
  engine_gaps: &id001
  - dnd2024:item_potions_of_healing
  conversion_gaps: []
  execution_profiles:
  - dnd2024:items/potion_of_healing
  - dnd2024:items/potion_of_healing_greater
  - dnd2024:items/potion_of_healing_superior
  - dnd2024:items/potion_of_healing_supreme
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 1d7d56aef669dcbb06712ae9475ebe987fa3e6e336926dd0e4b40f542f6974c3
splat_clauses:
- key: source_rules
  text: '# Potions of Healing Potion, Rarity Varies You regain Hit Points when you drink this potion.
    The number of Hit Points depends on the potion’s rarity, as shown in the table below. Whatever its
    potency, the potion’s red liquid glimmers when agitated. Potion HP Regained Rarity Potion of Healing
    2d4 + 2 Common Potion of Healing 4d4 + 4 Uncommon (greater) Potion of Healing 8d4 + 8 Rare (superior)
    Potion of Healing 10d4 + 20 Very Rare (supreme)'
  sha256: d905881138b42a0c41c68e372f80dcbe9ae0ee16ad940b888ea586353a7f758b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Potions of Healing

<!-- SRD page 236 -->

Potion, Rarity Varies\
You regain Hit Points when you drink this potion.\
The number of Hit Points depends on the potion’s\
rarity, as shown in the table below.\
Whatever its potency, the potion’s red liquid glim-\
mers when agitated.\
Potion HP Regained Rarity\
Potion of Healing 2d4 + 2 Common\
Potion of Healing 4d4 + 4 Uncommon\
(greater)\
Potion of Healing 8d4 + 8 Rare\
(superior)\
Potion of Healing 10d4 + 20 Very Rare\
(supreme)\

Original layout: [[../../srd-5.2.1/pages/236|page 236]].
