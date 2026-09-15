---
splat_schema: 1
splat_id: db471f07-b63f-599f-8670-4fa50832dc6f
splat_kind: item
splat_key: dnd2024:items/potion_of_heroism
splat_name: Potion of Heroism
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_heroism
splat_sources:
- title: SRD 5.2.1
  locator: pp. 236
  url: https://www.dndbeyond.com/srd
splat_item:
  category: potion
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/potion_of_heroism
  implementation: dnd2024:item_potion_of_heroism
  parameters:
    consume: drink
    temporary_hp: 10
    duration:
      amount: 1
      unit: hour
    spell_effect:
      key: dnd2024:spells/bless
    concentration: false
    consume_action: bonus_action
    administer_action: bonus_action
    effect_target: drinker
    consumed_quantity: 1
    effect_starts: immediately_after_consumption
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_potions
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_potion_of_heroism
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
  - item.consumed
  - time.elapsed
  capabilities:
  - inventory
  - effects
  - spellcasting
  - resources_and_time
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 7ec50f0f647527774b5539a5a72cabe1f67d5d1af2012f1d3110b4cced1f539a
  source_path: builtin/srd-5.2.1/magic_item/potion_of_heroism.md
  pages:
  - 236
  engine_gaps: &id001
  - dnd2024:item_potion_of_heroism
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 3910db2459541ce85d99a356f12845b51d717821dfbd00052161ed3feea2bbac
splat_clauses:
- key: source_rules
  text: '# Potion of Heroism Potion, Rare When you drink this potion, you gain 10 Temporary Hit Points
    that last for 1 hour. For the same duration, you are under the effect of the Bless spell (no Concentration
    required). This potion’s blue liquid bubbles and steams as if boiling.'
  sha256: 5b962365b755db040cb6a6915907a1355eb7dad8764fcbc614962a4c1c387fa9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Potion of Heroism

<!-- SRD page 236 -->

Potion, Rare\
When you drink this potion, you gain 10 Temporary\
Hit Points that last for 1 hour. For the same dura-\
tion, you are under the effect of the Bless spell (no\
Concentration required).\
This potion’s blue liquid bubbles and steams as if\
boiling.\

Original layout: [[../../srd-5.2.1/pages/236|page 236]].
