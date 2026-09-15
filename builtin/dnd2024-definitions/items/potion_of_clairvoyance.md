---
splat_schema: 1
splat_id: c56f82f6-583d-559f-a482-c9b586cd3638
splat_kind: item
splat_key: dnd2024:items/potion_of_clairvoyance
splat_name: Potion of Clairvoyance
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_clairvoyance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 235
  url: https://www.dndbeyond.com/srd
splat_item:
  category: potion
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/potion_of_clairvoyance
  implementation: dnd2024:item_potion_of_clairvoyance
  parameters:
    consumption:
      trigger: drink
      consume_item: true
      gain_spell_effect:
        spell:
          key: dnd2024:spells/clairvoyance
        concentration: false
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_potion_of_clairvoyance
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
  - spell.cast_completed
  capabilities:
  - consumables
  - effects
  - item_spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 8559d010d130844a80940842f929ef0afad61edb7bca46ee41a5a9074424ad2b
  source_path: builtin/srd-5.2.1/magic_item/potion_of_clairvoyance.md
  pages:
  - 235
  engine_gaps: &id001
  - dnd2024:item_potion_of_clairvoyance
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: af0898a10c91ffce03e53cf604ba8e4d55ed8ee75752ed7891d9e2a54a527f11
splat_clauses:
- key: source_rules
  text: '# Potion of Clairvoyance Potion, Rare When you drink this potion, you gain the effect of the
    Clairvoyance spell (no Concentration required). An eyeball bobs in this potion’s yellowish liquid
    but vanishes when the potion is opened.'
  sha256: e28e7719463a89a3ac411e4114a357e7dd74945e93d410c205381b781382b6e8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Potion of Clairvoyance

<!-- SRD page 235 -->

Potion, Rare\
When you drink this potion, you gain the effect of\
the Clairvoyance spell (no Concentration required).\
An eyeball bobs in this potion’s yellowish liquid\
but vanishes when the potion is opened.\

Original layout: [[../../srd-5.2.1/pages/235|page 235]].
