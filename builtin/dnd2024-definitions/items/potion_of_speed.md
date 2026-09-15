---
splat_schema: 1
splat_id: dccf5ec1-52af-5ec6-8992-8da6062e975b
splat_kind: item
splat_key: dnd2024:items/potion_of_speed
splat_name: Potion of Speed
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_speed
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
  mechanic: dnd2024:items/potion_of_speed
  implementation: dnd2024:item_potion_of_speed
  parameters:
    trigger: potion_drunk
    consumed: true
    effect:
      spell:
        key: dnd2024:spells/haste
      duration:
        amount: 1
        unit: minutes
      concentration: false
      end_lethargy: false
    attunement_required: false
splat_contracts:
- id: dnd2024:item_potion_of_speed
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
  - time.elapsed
  capabilities:
  - item_effects
  - item_spellcasting
  - effect_duration
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: fadd3e7756a7755e7b43546018321a66389aac4a3bc386882a2adacc804e6b04
  source_path: builtin/srd-5.2.1/magic_item/potion_of_speed.md
  pages:
  - 236
  engine_gaps: &id001
  - dnd2024:item_potion_of_speed
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: a38ba100e9b28c7ca81746c9aea7060789ae220a75ddf98c88faf2412433efd6
splat_clauses:
- key: source_rules
  text: '# Potion of Speed Potion, Very Rare When you drink this potion, you gain the effect of the Haste
    spell for 1 minute (no Concentration required) without suffering the wave of lethargy that typically
    occurs when the effect ends. This potion’s yellow fluid is streaked with black and swirls on its own.'
  sha256: 713947215c2eaab1a16fc10c1b5ae0375e18c19d9c78c241335a59c03728bacb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Potion of Speed

<!-- SRD page 236 -->

Potion, Very Rare\
When you drink this potion, you gain the effect of\
the Haste spell for 1 minute (no Concentration re-\
quired) without suffering the wave of lethargy that\
typically occurs when the effect ends.\
This potion’s yellow fluid is streaked with black\
and swirls on its own.\

Original layout: [[../../srd-5.2.1/pages/236|page 236]].
