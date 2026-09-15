---
splat_schema: 1
splat_id: ba35052c-b52f-5c7e-8261-63005e16e998
splat_kind: item
splat_key: dnd2024:items/potion_of_vitality
splat_name: Potion of Vitality
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_vitality
splat_sources:
- title: SRD 5.2.1
  locator: pp. 236
  url: https://www.dndbeyond.com/srd
splat_item:
  category: potion
  rarity: very_rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/potion_of_vitality
  implementation: dnd2024:item_potion_of_vitality
  parameters:
    consumption:
      trigger: drink
      consume_item: true
      remove_exhaustion: all_levels
      end_conditions:
      - poisoned
    effect:
      duration:
        value: 24
        unit: hour
      hit_point_dice_spent: maximum_healing
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_potion_of_vitality
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
  outputs:
  - effects
  - item_state
  events:
  - item.used
  capabilities:
  - consumables
  - effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 9125f7c5bbc0b444012ac742ae5970cbdd22f49dce6b520358e5f525551f6a18
  source_path: builtin/srd-5.2.1/magic_item/potion_of_vitality.md
  pages:
  - 236
  engine_gaps: &id001
  - dnd2024:item_potion_of_vitality
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: a1de86552e419f82d550d5327ec85f7efdd2e3e40df03324ed035a2a32800836
splat_clauses:
- key: source_rules
  text: '# Potion of Vitality Potion, Very Rare When you drink this potion, it removes any Exhaustion
    levels you have and ends the Poisoned condition on you. For the next 24 hours, you regain the maximum
    number of Hit Points for any Hit Point Die you spend. This potion’s crimson liquid regularly pulses
    with dull light, calling to mind a heartbeat.'
  sha256: 5c65451fd7933278add599a5831d092691e86d58ffda743dd874302196fa72a6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Potion of Vitality

<!-- SRD page 236 -->

Potion, Very Rare\
When you drink this potion, it removes any\
Exhaustion levels you have and ends the Poisoned\
condition on you. For the next 24 hours, you regain\
the maximum number of Hit Points for any Hit Point\
Die you spend.\
This potion’s crimson liquid regularly pulses with\
dull light, calling to mind a heartbeat.\

Original layout: [[../../srd-5.2.1/pages/236|page 236]].
