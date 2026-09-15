---
splat_schema: 1
splat_id: d9f396fd-3080-5390-b321-289e8c9082fc
splat_kind: item
splat_key: dnd2024:items/cloak_of_displacement
splat_name: Cloak of Displacement
splat_origin:
  key: dnd2024:reference/magic_item/cloak_of_displacement
splat_sources:
- title: SRD 5.2.1
  locator: pp. 215
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/cloak_of_displacement
  implementation: dnd2024:item_cloak_of_displacement
  parameters:
    equipment_state: worn
    attacks_against_wearer: disadvantage
    damage_taken:
      suppress_until: start_of_wearer_next_turn
    suppress_when:
      speed_ft: 0
    effect_type: illusion
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_cloak_of_displacement
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
  - damage.applied
  - turn.started
  - speed.changed
  - attack.roll_pending
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 2541ed777da66fa5e32686805710ac02b982b082aeb2c335e41a897747ab1f21
  source_path: builtin/srd-5.2.1/magic_item/cloak_of_displacement.md
  pages:
  - 215
  engine_gaps: &id001
  - dnd2024:item_cloak_of_displacement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 551491533de9f297afbe085ad2d8fdb267a833167d92d1ad138e679dd77a8c80
splat_clauses:
- key: source_rules
  text: '# Cloak of Displacement Wondrous Item, Rare (Requires Attunement) While you wear this cloak,
    it magically projects an illusion that makes you appear to be standing in a place near your actual
    location, causing any creature to have Disadvantage on attack rolls against you. If you take damage,
    the property ceases to function until the start of your next turn. This property is suppressed while
    your Speed is 0.'
  sha256: d90ab7f52f91202caf2b7140964d10b86cd87e9f8f596f269851e66b788fb665
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cloak of Displacement

<!-- SRD page 215 -->

Wondrous Item, Rare (Requires Attunement)\
While you wear this cloak, it magically projects an\
illusion that makes you appear to be standing in a\
place near your actual location, causing any crea-\
ture to have Disadvantage on attack rolls against\
you. If you take damage, the property ceases to\
function until the start of your next turn. This prop-\
erty is suppressed while your Speed is 0.\

Original layout: [[../../srd-5.2.1/pages/215|page 215]].
