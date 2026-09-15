---
splat_schema: 1
splat_id: 281f1de3-6295-5241-b07d-3d5645a13612
splat_kind: item
splat_key: dnd2024:items/cloak_of_arachnida
splat_name: Cloak of Arachnida
splat_origin:
  key: dnd2024:reference/magic_item/cloak_of_arachnida
splat_sources:
- title: SRD 5.2.1
  locator: pp. 215
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/cloak_of_arachnida
  implementation: dnd2024:item_cloak_of_arachnida
  parameters:
    equipment_state: worn
    resistance:
    - poison
    climb:
      speed: walking_speed
      vertical_surfaces: true
      ceilings: true
      hands_free: true
    webs:
      cannot_be_caught: true
      movement: difficult_terrain
    spells:
    - spell:
        key: dnd2024:spells/web
      save_dc: 13
      area_multiplier: 2
      resource:
        uses: 1
        recovery: dawn
      area_multiplier_scope: web_area
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_cloak_of_arachnida
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
  - equipment
  - item_resources
  - effects
  - item_spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: d3dd91ae762dd1aec4fad8f06eea75f0631149c5cf52e7997db7fd5deec8fc3b
  source_path: builtin/srd-5.2.1/magic_item/cloak_of_arachnida.md
  pages:
  - 215
  engine_gaps: &id001
  - dnd2024:item_cloak_of_arachnida
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 0d5e921efaa25be2b0cfd11d56c453e160cc8f6ad8dfcb415257c1b6139ca51a
splat_clauses:
- key: source_rules
  text: '# Cloak of Arachnida Wondrous Item, Very Rare (Requires Attunement) This fine garment is made
    of black silk interwoven with faint, silvery threads. While wearing it, you gain the following benefits.
    Poison Resistance. You have Resistance to Poison damage. Spider Climb. You have a Climb Speed equal
    to your Speed and can move up, down, and across vertical surfaces and along ceilings, while leaving
    your hands free. Spider Walk. You can’t be caught in webs of any sort and can move through webs as
    if they were Difficult Terrain. Web. You can cast Web (save DC 13). The web created by the spell fills
    twice its normal area. Once used, this property can’t be used again until the next dawn.'
  sha256: 687397d28a318f7e0c3e28ec31e01ea469a8980340bb4810b561a7fdc1757b06
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cloak of Arachnida

<!-- SRD page 215 -->

Wondrous Item, Very Rare (Requires Attunement)\
This fine garment is made of black silk interwoven\
with faint, silvery threads. While wearing it, you\
gain the following benefits.\
Poison Resistance. You have Resistance to Poison\
damage.\
Spider Climb. You have a Climb Speed equal to\
your Speed and can move up, down, and across ver-\
tical surfaces and along ceilings, while leaving your\
hands free.\
Spider Walk. You can’t be caught in webs of any\
sort and can move through webs as if they were Dif-\
ficult Terrain.\
Web. You can cast Web (save DC 13). The web\
created by the spell fills twice its normal area. Once\
used, this property can’t be used again until the\
next dawn.\

Original layout: [[../../srd-5.2.1/pages/215|page 215]].
