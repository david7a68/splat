---
splat_schema: 1
splat_id: 5542e450-360d-50dd-a176-b461a8cf4654
splat_kind: item
splat_key: dnd2024:items/eyes_of_the_eagle
splat_name: Eyes of the Eagle
splat_origin:
  key: dnd2024:reference/magic_item/eyes_of_the_eagle
splat_sources:
- title: SRD 5.2.1
  locator: pp. 221
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/eyes_of_the_eagle
  implementation: dnd2024:item_eyes_of_the_eagle
  parameters:
    equipment_state: worn_over_eyes
    check_advantage:
      ability: wisdom
      skill: perception
      sense: sight
    clear_visibility:
      minimum_detail_object_width_ft: 2
      distance_limit: extremely_distant
      distance_adjudication:
        chooser: gm
        type: number
        scope: visibility_distance
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_eyes_of_the_eagle
  inputs:
    actor:
      type: participant
      required: true
    source_item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choice:
      required: false
      type: choice
    event:
      required: false
      type: event
    campaign:
      required: false
      type: campaign
  outputs:
  - item_state
  - effects
  - resolutions
  events:
  - equipment.changed
  - check.pending
  - perception.pending
  capabilities:
  - inventory
  - effects
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 946918b1b3336a335979689b7c3f9789371f46b3faa8b9d27efb867dc01c8da3
  source_path: builtin/srd-5.2.1/magic_item/eyes_of_the_eagle.md
  pages:
  - 221
  engine_gaps: &id001
  - dnd2024:item_eyes_of_the_eagle
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 660c54da9cccfa7d6bb044e0fae79bf1dd7f18a1b8d80f0d6e863f61fcb217d9
splat_clauses:
- key: source_rules
  text: '# Eyes of the Eagle Wondrous Item, Uncommon These crystal lenses fit over the eyes. While wearing
    them, you have Advantage on Wisdom (Perception) checks that rely on sight. In conditions of clear
    visibility, you can make out details of even extremely distant creatures and objects as small as 2
    feet across.'
  sha256: ab6f69bbc83c788390ba4554e8dce29224cd4473701c0e5ba633fa820f3bd87d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Eyes of the Eagle

<!-- SRD page 221 -->

Wondrous Item, Uncommon\
These crystal lenses fit over the eyes. While wear-\
ing them, you have Advantage on Wisdom (Per-\
ception) checks that rely on sight. In conditions of\
clear visibility, you can make out details of even\
extremely distant creatures and objects as small as\
2 feet across.\

Original layout: [[../../srd-5.2.1/pages/221|page 221]].
