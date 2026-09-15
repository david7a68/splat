---
splat_schema: 1
splat_id: 0c7ef23c-412e-528a-8d62-9deb5250fc57
splat_kind: item
splat_key: dnd2024:items/broom_of_flying
splat_name: Broom of Flying
splat_origin:
  key: dnd2024:reference/magic_item/broom_of_flying
splat_sources:
- title: SRD 5.2.1
  locator: pp. 214
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/broom_of_flying
  implementation: dnd2024:item_broom_of_flying
  parameters:
    default_mode: mundane_broom
    activate:
      action: magic
      must_stand_astride: true
      mode: hovering_ridable
    fly_speed_ft: 50
    capacity_lb: 400
    encumbered:
      over_lb: 200
      fly_speed_ft: 30
    hover_ends_on:
    - landing
    - dismounted
    send:
      action: magic
      destination:
        familiar: true
        named: true
        maximum_distance_miles: 1
      travels_alone: true
    recall:
      action: magic
      command_word: true
      maximum_distance_miles: 1
      destination: actor
    attunement_required: true
splat_contracts:
- id: dnd2024:item_broom_of_flying
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
  - item.activation_requested
  - movement.completed
  - item.rider_changed
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: d61c60634a637c90e91e90367bd82810f966bb75c8c89647a3c8c7efbcc63c51
  source_path: builtin/srd-5.2.1/magic_item/broom_of_flying.md
  pages:
  - 214
  engine_gaps: &id001
  - dnd2024:item_broom_of_flying
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 783e953c19acd436f3199bbc29797f5ce622b55b00719e6e2e44cc1d01e921cb
splat_clauses:
- key: source_rules
  text: '# Broom of Flying Wondrous Item, Uncommon (Requires Attunement) This wooden broom functions like
    a mundane broom until you stand astride it and take a Magic action to make it hover beneath you, at
    which time it can be ridden in the air. It has a Fly Speed of 50 feet. It can carry up to 400 pounds,
    but its Fly Speed becomes 30 feet while carrying over 200 pounds. The broom stops hovering when you
    land or when you’re no longer riding it. As a Magic action, you can send the broom to travel alone
    to a destination within 1 mile of you if you name the location and are familiar with it. The broom
    comes back to you when you take a Magic action and use a command word if the broom is still within
    1 mile of you.'
  sha256: 780336fa73fca80936c8818dec3d75859fbdfcbe33af8dee5a4f937e67767000
  status: contract_specified
splat_requires_mechanics: *id001
---

# Broom of Flying

<!-- SRD page 214 -->

Wondrous Item, Uncommon (Requires Attunement)\
This wooden broom functions like a mundane\
broom until you stand astride it and take a Magic\
action to make it hover beneath you, at which time\
it can be ridden in the air. It has a Fly Speed of 50\
feet. It can carry up to 400 pounds, but its Fly Speed\
becomes 30 feet while carrying over 200 pounds.\
The broom stops hovering when you land or when\
you’re no longer riding it.\
As a Magic action, you can send the broom to\
travel alone to a destination within 1 mile of you if\
you name the location and are familiar with it. The\
broom comes back to you when you take a Magic\
action and use a command word if the broom is still\
within 1 mile of you.\

Original layout: [[../../srd-5.2.1/pages/214|page 214]].
