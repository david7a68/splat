---
splat_schema: 1
splat_id: c76ed8ed-0c74-5d58-8cfa-24e9a38c1094
splat_kind: item
splat_key: dnd2024:items/potion_of_climbing
splat_name: Potion of Climbing
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_climbing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 235
  url: https://www.dndbeyond.com/srd
splat_item:
  category: potion
  rarity: common
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/potion_of_climbing
  implementation: dnd2024:item_potion_of_climbing
  parameters:
    consume: drink
    duration:
      amount: 1
      unit: hour
    climb_speed: speed
    check_advantage:
      ability: strength
      skill: athletics
      task: climb
    consume_action: bonus_action
    administer_action: bonus_action
    effect_target: drinker
    consumed_quantity: 1
    effect_starts: immediately_after_consumption
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_potions
splat_contracts:
- id: dnd2024:item_potion_of_climbing
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
  - item.consumed
  - time.elapsed
  - check.pending
  - speed.changed
  capabilities:
  - inventory
  - effects
  - resources_and_time
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: e1fdfdf3e1f69ee5c3ef529e027eeff3332c4fba37c1497330d49fbc21194340
  source_path: builtin/srd-5.2.1/magic_item/potion_of_climbing.md
  pages:
  - 235
  engine_gaps: &id001
  - dnd2024:item_potion_of_climbing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 88996d0282d30b57b88d1ba205dae97fa85004b782f09d85561355304a43b4a9
splat_clauses:
- key: source_rules
  text: '# Potion of Climbing Potion, Common When you drink this potion, you gain a Climb Speed equal
    to your Speed for 1 hour. During this time, you have Advantage on Strength (Athletics) checks to climb.
    This potion is separated into brown, silver, and gray layers resembling bands of stone. Shaking the
    bottle fails to mix the colors.'
  sha256: d02eadd9a2d3a5c6e595c6b0d5b16efccde80975d5d38fba5439fd84c57a3ba7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Potion of Climbing

<!-- SRD page 235 -->

Potion, Common\
When you drink this potion, you gain a Climb Speed\
equal to your Speed for 1 hour. During this time, you\
have Advantage on Strength (Athletics) checks to\
climb.\
This potion is separated into brown, silver, and\
gray layers resembling bands of stone. Shaking the\
bottle fails to mix the colors.\

Original layout: [[../../srd-5.2.1/pages/235|page 235]].
