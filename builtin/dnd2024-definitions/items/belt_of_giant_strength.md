---
splat_schema: 1
splat_id: ad24d43b-f077-5484-a9d1-04d040dd4afd
splat_kind: item
splat_key: dnd2024:items/belt_of_giant_strength
splat_name: Belt of Giant Strength
splat_origin:
  key: dnd2024:reference/magic_item/belt_of_giant_strength
splat_sources:
- title: SRD 5.2.1
  locator: pp. 213
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rarity_varies
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/belt_of_giant_strength
  implementation: dnd2024:item_belt_of_giant_strength
  parameters:
    equipped: worn
    ability_floor:
      ability: strength
      when: unmodified_score_below_variant
    variants:
    - giant: hill
      score: 21
      rarity: rare
    - giant: frost
      score: 23
      rarity: very_rare
    - giant: stone
      score: 23
      rarity: very_rare
    - giant: fire
      score: 25
      rarity: very_rare
    - giant: cloud
      score: 27
      rarity: legendary
    - giant: storm
      score: 29
      rarity: legendary
    attunement_required: true
splat_contracts:
- id: dnd2024:item_belt_of_giant_strength
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
  - item.equipment_changed
  - ability_score.changed
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - item_variants
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: f90d26b9151d19449b3ba68f9d1f167675890e5acee51a739a4193f0d4ce4179
  source_path: builtin/srd-5.2.1/magic_item/belt_of_giant_strength.md
  pages:
  - 213
  engine_gaps: &id001
  - dnd2024:item_belt_of_giant_strength
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: a7d032e0b7b620410e64107898a5260ff4c1f0820943e036c528498b9a9ee0a3
splat_clauses:
- key: source_rules
  text: '# Belt of Giant Strength Wondrous Item, Rarity Varies (Requires Attunement) While wearing this
    belt, your Strength changes to a score granted by the belt. The type of giant determines the score
    (see the table below). The item has no effect on you if your Strength without the belt is equal to
    or greater than the belt’s score. Belt Str. Rarity Belt of Giant Strength (hill) 21 Rare Belt of Giant
    Strength (frost or 23 Very Rare stone) Belt of Giant Strength (fire) 25 Very Rare Belt of Giant Strength
    (cloud) 27 Legendary Belt of Giant Strength (storm) 29 Legendary'
  sha256: d836c250aadc65fd087b085afb07b02a384d538ed6a4793d29acc04ddfc950a2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Belt of Giant Strength

<!-- SRD page 213 -->

Wondrous Item, Rarity Varies (Requires Attunement)\
While wearing this belt, your Strength changes to\
a score granted by the belt. The type of giant deter-\
mines the score (see the table below). The item has\
no effect on you if your Strength without the belt is\
equal to or greater than the belt’s score.\
Belt Str. Rarity\
Belt of Giant Strength (hill) 21 Rare\
Belt of Giant Strength (frost or 23 Very Rare\
stone)\
Belt of Giant Strength (fire) 25 Very Rare\
Belt of Giant Strength (cloud) 27 Legendary\
Belt of Giant Strength (storm) 29 Legendary\

Original layout: [[../../srd-5.2.1/pages/213|page 213]].
