---
splat_schema: 1
splat_id: e5de36e0-984b-5e2f-a757-f53366ad3ebc
splat_kind: item
splat_key: dnd2024:items/cubic_gate
splat_name: Cubic Gate
splat_origin:
  key: dnd2024:reference/magic_item/cubic_gate
splat_sources:
- title: SRD 5.2.1
  locator: pp. 216
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: legendary
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/cubic_gate
  implementation: dnd2024:item_cubic_gate
  parameters:
    dimensions:
      side_inches: 3
    planes:
      side_count: 6
      all_distinct: true
      fixed:
      - material
      gm_choices: 5
    charges:
      maximum: 3
      initial: 3
      recovery:
        event: dawn
        amount:
          dice: 1
          sides: 3
          bonus: 0
    activation:
      action: magic
      cost: 1
      options:
      - spell:
          key: dnd2024:spells/gate
        presses: 1
        destination: selected_side_plane
      - spell:
          key: dnd2024:spells/plane_shift
        presses: 2
        destination: selected_side_plane
    attunement:
      required: false
splat_contracts:
- id: dnd2024:item_cubic_gate
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
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
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
  - spatial_queries
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 2b4aa3f4e99e74fafc1224a33ab8300d64bc2fa3461da4a64c15a725f6f1b03c
  source_path: builtin/srd-5.2.1/magic_item/cubic_gate.md
  pages:
  - 216
  engine_gaps: &id001
  - dnd2024:item_cubic_gate
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: e5ae310811f96b3ecab807c99437d812355a0bc6f0ed6d72731c6ccd7f9af0fd
splat_clauses:
- key: source_rules
  text: '# Cubic Gate Wondrous Item, Legendary This cube is 3 inches across and radiates palpable magical
    energy. The six sides of the cube are each keyed to a different plane of existence, one of which is
    the Material Plane. The other sides are linked to planes determined by the GM. The cube has 3 charges
    and regains 1d3 expended charges daily at dawn. As a Magic action, you can expend 1 of the cube’s
    charges to cast one of the following spells using the cube. Gate. Pressing one side of the cube, you
    cast Gate, opening a portal to the plane of existence keyed to that side. Plane Shift. Pressing one
    side of the cube twice, you cast Plane Shift, transporting the targets to the plane of existence keyed
    to that side.'
  sha256: d2bc79f251a37ccf2629368bbb0e63f40d2f5cd60a9e5254dc7068396b7ab019
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cubic Gate

<!-- SRD page 216 -->

Wondrous Item, Legendary\
This cube is 3 inches across and radiates palpable\
magical energy. The six sides of the cube are each\
keyed to a different plane of existence, one of which\
is the Material Plane. The other sides are linked to\
planes determined by the GM.\
The cube has 3 charges and regains 1d3 expended\
charges daily at dawn. As a Magic action, you can\
expend 1 of the cube’s charges to cast one of the fol-\
lowing spells using the cube.\
Gate. Pressing one side of the cube, you cast Gate,\
opening a portal to the plane of existence keyed to\
that side.\
Plane Shift. Pressing one side of the cube twice,\
you cast Plane Shift, transporting the targets to the\
plane of existence keyed to that side.\

Original layout: [[../../srd-5.2.1/pages/216|page 216]].
