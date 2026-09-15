---
splat_schema: 1
splat_id: 96634ed3-2452-5736-a380-1fde627182b0
splat_kind: item
splat_key: dnd2024:items/potion_of_diminution
splat_name: Potion of Diminution
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_diminution
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
  mechanic: dnd2024:items/potion_of_diminution
  implementation: dnd2024:item_potion_of_diminution
  parameters:
    trigger: potion_drunk
    consumed: true
    effect:
      spell:
        key: dnd2024:spells/enlarge_reduce
      mode: reduce
      duration:
        amount:
          roll:
            count: 1
            sides: 4
        unit: hours
      concentration: false
    attunement_required: false
splat_contracts:
- id: dnd2024:item_potion_of_diminution
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
  - dice
  - item_spellcasting
  - effect_duration
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 0efc5db23c663034b1464ec9bcfef7cee38abfe3217852ca749f6fa84c0a7ccc
  source_path: builtin/srd-5.2.1/magic_item/potion_of_diminution.md
  pages:
  - 235
  engine_gaps: &id001
  - dnd2024:item_potion_of_diminution
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 6510eb022433c5540a4ca54ea47c7a5efe6031f59e7d92b580980b4e635fca97
splat_clauses:
- key: source_rules
  text: '# Potion of Diminution Potion, Rare When you drink this potion, you gain the “reduce” effect
    of the Enlarge/Reduce spell for 1d4 hours (no Concentration required). The red in the potion’s liquid
    continuously contracts to a tiny bead and then expands to color the clear liquid around it. Shaking
    the bottle fails to interrupt this process.'
  sha256: bdcb1199d4e9e76e89bac2d3869b30999b0b93e7911244be040144d78e08254d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Potion of Diminution

<!-- SRD page 235 -->

Potion, Rare\
When you drink this potion, you gain the “reduce”\
effect of the Enlarge/Reduce spell for 1d4 hours (no\
Concentration required).\
The red in the potion’s liquid continuously con-\
tracts to a tiny bead and then expands to color the\
clear liquid around it. Shaking the bottle fails to in-\
terrupt this process.\

Original layout: [[../../srd-5.2.1/pages/235|page 235]].
