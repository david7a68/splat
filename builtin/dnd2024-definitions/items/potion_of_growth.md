---
splat_schema: 1
splat_id: a91e7a5c-29f3-5ae1-bd9f-fcc4b5fbc863
splat_kind: item
splat_key: dnd2024:items/potion_of_growth
splat_name: Potion of Growth
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_growth
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
  mechanic: dnd2024:items/potion_of_growth
  implementation: dnd2024:item_potion_of_growth
  parameters:
    consumption:
      trigger: drink
      consume_item: true
      gain_spell_effect:
        spell:
          key: dnd2024:spells/enlarge_reduce
        mode: enlarge
        duration:
          value: 10
          unit: minute
        concentration: false
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_potion_of_growth
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
  source_sha256: 4122d829640e0086e55bd15ab12f5a89c3b99b211b6b826068eed3c154f3609f
  source_path: builtin/srd-5.2.1/magic_item/potion_of_growth.md
  pages:
  - 235
  engine_gaps: &id001
  - dnd2024:item_potion_of_growth
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: cf98e9150686fa11eef18eed8c798249388ce80098e090987a5c9f094b7698e7
splat_clauses:
- key: source_rules
  text: '# Potion of Growth Potion, Uncommon When you drink this potion, you gain the “enlarge” effect
    of the Enlarge/Reduce spell for 10 minutes (no Concentration required). The red in the potion’s liquid
    continuously expands from a tiny bead to color the clear liquid around it and then contracts. Shaking
    the bottle fails to interrupt this process.'
  sha256: 52a96b6a241552e7bfefcf12bb22f249f8f672265d89b9c1269cf202a068ae49
  status: contract_specified
splat_requires_mechanics: *id001
---

# Potion of Growth

<!-- SRD page 235 -->

Potion, Uncommon\
When you drink this potion, you gain the “enlarge”\
effect of the Enlarge/Reduce spell for 10 minutes (no\
Concentration required).\
The red in the potion’s liquid continuously ex-\
pands from a tiny bead to color the clear liquid\
around it and then contracts. Shaking the bottle\
fails to interrupt this process.\

Original layout: [[../../srd-5.2.1/pages/235|page 235]].
