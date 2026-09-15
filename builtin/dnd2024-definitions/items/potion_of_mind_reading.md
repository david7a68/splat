---
splat_schema: 1
splat_id: 7db614ca-c115-5745-880e-f780e9772a4d
splat_kind: item
splat_key: dnd2024:items/potion_of_mind_reading
splat_name: Potion of Mind Reading
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_mind_reading
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
  mechanic: dnd2024:items/potion_of_mind_reading
  implementation: dnd2024:item_potion_of_mind_reading
  parameters:
    trigger: potion_drunk
    consumed: true
    effect:
      spell:
        key: dnd2024:spells/detect_thoughts
      save_dc: 13
      duration:
        amount: 10
        unit: minutes
      concentration: false
    attunement_required: false
splat_contracts:
- id: dnd2024:item_potion_of_mind_reading
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
  source_sha256: cf568e69593b559c33d48275f3cd7774cf719a712931c1ef6b9653e4a4facc17
  source_path: builtin/srd-5.2.1/magic_item/potion_of_mind_reading.md
  pages:
  - 236
  engine_gaps: &id001
  - dnd2024:item_potion_of_mind_reading
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 4f558b1a2741c139450b2440ba2e3419b6bc33b4b175921af69b645a8577eff9
splat_clauses:
- key: source_rules
  text: '# Potion of Mind Reading Potion, Rare When you drink this potion, you gain the effect of the
    Detect Thoughts spell (save DC 13) for 10 minutes (no Concentration required). This potion’s dense,
    purple liquid has an ovoid cloud of pink floating in it.'
  sha256: c32f552a0fb447a1d1aa4d127452ee34903bdf7142d89e9229a8e0d9995998bf
  status: contract_specified
splat_requires_mechanics: *id001
---

# Potion of Mind Reading

<!-- SRD page 236 -->

Potion, Rare\
When you drink this potion, you gain the effect of\
the Detect Thoughts spell (save DC 13) for 10 min-\
utes (no Concentration required).\
This potion’s dense, purple liquid has an ovoid\
cloud of pink floating in it.\

Original layout: [[../../srd-5.2.1/pages/236|page 236]].
