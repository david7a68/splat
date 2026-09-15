---
splat_schema: 1
splat_id: 450aaf9d-fa3a-5c68-b39f-96530f4dd743
splat_kind: item
splat_key: dnd2024:items/potion_of_poison
splat_name: Potion of Poison
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_poison
splat_sources:
- title: SRD 5.2.1
  locator: pp. 236
  url: https://www.dndbeyond.com/srd
splat_item:
  category: potion
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/potion_of_poison
  implementation: dnd2024:item_potion_of_poison
  parameters:
    illusion:
      senses:
      - appearance
      - smell
      - taste
      disguise: beneficial_potion
      identified_by:
        key: dnd2024:spells/identify
    consumption:
      trigger: drink
      consume_item: true
      damage:
        roll:
          dice: 4
          sides: 6
          bonus: 0
        type: poison
      save:
        ability: constitution
        dc: 13
      failure:
        condition: poisoned
        duration:
          value: 1
          unit: hour
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_potion_of_poison
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
  outputs:
  - effects
  - item_state
  events:
  - item.used
  - spell.cast_completed
  capabilities:
  - consumables
  - effects
  - item_spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: fd3271b4656d165b8c7928c51d1c5b57bc7eb68fd340fb83c1338938eb6b5c44
  source_path: builtin/srd-5.2.1/magic_item/potion_of_poison.md
  pages:
  - 236
  engine_gaps: &id001
  - dnd2024:item_potion_of_poison
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 4f906518d5aca49d66fff233dcb1caf98b7e61e76165eb9c30df3e1baef56dbd
splat_clauses:
- key: source_rules
  text: '# Potion of Poison Potion, Uncommon This concoction looks, smells, and tastes like a Potion of
    Healing or another beneficial potion. However, it is actually poison masked by illusion magic. Identify
    reveals its true nature. If you drink this potion, you take 4d6 Poison damage and must succeed on
    a DC 13 Constitution saving throw or have the Poisoned condition for 1 hour.'
  sha256: d13609cd32b854ac8c0ad288fbdd5ac50d05c7479f408c191bef21447371a9d2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Potion of Poison

<!-- SRD page 236 -->

Potion, Uncommon\
This concoction looks, smells, and tastes like a\
Potion of Healing or another beneficial potion. How-\
ever, it is actually poison masked by illusion magic.\
Identify reveals its true nature.\
If you drink this potion, you take 4d6 Poison dam-\
age and must succeed on a DC 13 Constitution sav-\
ing throw or have the Poisoned condition for 1 hour.\

Original layout: [[../../srd-5.2.1/pages/236|page 236]].
