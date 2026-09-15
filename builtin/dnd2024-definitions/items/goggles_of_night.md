---
splat_schema: 1
splat_id: dbda84de-126f-5a7e-bc4d-3220e15d1b56
splat_kind: item
splat_key: dnd2024:items/goggles_of_night
splat_name: Goggles of Night
splat_origin:
  key: dnd2024:reference/magic_item/goggles_of_night
splat_sources:
- title: SRD 5.2.1
  locator: pp. 224
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/goggles_of_night
  implementation: dnd2024:item_goggles_of_night
  parameters:
    equipment_state: worn
    darkvision:
      without_existing_ft: 60
      existing_range_bonus_ft: 60
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_goggles_of_night
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
  outputs:
  - modifiers
  events:
  - equipment.state_changed
  capabilities:
  - equipment
  - derived_statistics
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 7b677a412fc7f4ce37b661f715f4f6509b345a4ba7f6b758c2082cbb191ac123
  source_path: builtin/srd-5.2.1/magic_item/goggles_of_night.md
  pages:
  - 224
  engine_gaps: &id001
  - dnd2024:item_goggles_of_night
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 14252d5f6fe8290547f2043c0580357ce73626bb2ee410fd1e63e9a12a8a31fc
splat_clauses:
- key: source_rules
  text: '# Goggles of Night Wondrous Item, Uncommon While wearing these dark lenses, you have Darkvision
    out to 60 feet. If you already have Darkvision, wearing the goggles increases its range by 60 feet.'
  sha256: b3d4fab8355fb70b6defc8d92387cc6ccb01a74cf0a6ca09645a79b9ee07d9e0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Goggles of Night

<!-- SRD page 224 -->

Wondrous Item, Uncommon\
While wearing these dark lenses, you have Darkvi-\
sion out to 60 feet. If you already have Darkvision,\
wearing the goggles increases its range by 60 feet.\

Original layout: [[../../srd-5.2.1/pages/224|page 224]].
