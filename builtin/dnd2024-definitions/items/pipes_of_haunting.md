---
splat_schema: 1
splat_id: 82fa3bfa-6986-51ce-b6cf-60eef480f825
splat_kind: item
splat_key: dnd2024:items/pipes_of_haunting
splat_name: Pipes of Haunting
splat_origin:
  key: dnd2024:reference/magic_item/pipes_of_haunting
splat_sources:
- title: SRD 5.2.1
  locator: pp. 234
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/pipes_of_haunting
  implementation: dnd2024:item_pipes_of_haunting
  parameters:
    charges:
      maximum: 3
      initial: 3
      recovery:
        event: dawn
        amount:
          expression: 1d3
          scope: use
        cap: maximum
    activate:
      action: magic
      play_pipes: true
      charge_cost: 1
      targets:
        chooser: user
        range_ft: 30
      save:
        ability: wisdom
        dc: 15
        failure:
          condition: frightened
          duration:
            amount: 1
            unit: minute
        success:
          immune_to_same_pipes_effect:
            amount: 24
            unit: hour
    repeat_save:
      event: target_turn_end
      ability: wisdom
      dc: 15
      success:
        end_effect: true
        immune_to_same_pipes_effect:
          amount: 24
          unit: hour
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_pipes_of_haunting
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
  - action.magic_pending
  - turn.ended
  - time.dawn
  - time.elapsed
  capabilities:
  - inventory
  - effects
  - resources_and_time
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 1460c32d8f7bfcb1eae127f245926a0f65f5972149919b2ecbf243386de611d5
  source_path: builtin/srd-5.2.1/magic_item/pipes_of_haunting.md
  pages:
  - 234
  engine_gaps: &id001
  - dnd2024:item_pipes_of_haunting
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: d4de896e322c7a8680208e3ca6ee2709e2fd3b4b824be7e00b9669515548b5cd
splat_clauses:
- key: source_rules
  text: '# Pipes of Haunting Wondrous Item, Uncommon These pipes have 3 charges and regain 1d3 expended
    charges daily at dawn. You can take a Magic action to play them and expend 1 charge to create an eerie,
    spellbinding tune. Each creature of your choice within 30 feet of you must succeed on a DC 15 Wisdom
    saving throw or have the Frightened condition for 1 minute. A creature that fails the save repeats
    it at the end of each of its turns, ending the effect on itself on a success. A creature that succeeds
    on its save is immune to the effect of these pipes for 24 hours.'
  sha256: 93a1b1aff54e372b1c75c06de52f92b1529c63ebd8f8494a4011158f58a4bc47
  status: contract_specified
splat_requires_mechanics: *id001
---

# Pipes of Haunting

<!-- SRD page 234 -->

Wondrous Item, Uncommon\
These pipes have 3 charges and regain 1d3 ex-\
pended charges daily at dawn. You can take a Magic\
action to play them and expend 1 charge to create\
an eerie, spellbinding tune. Each creature of your\
choice within 30 feet of you must succeed on a DC\
15 Wisdom saving throw or have the Frightened\
condition for 1 minute. A creature that fails the save\
repeats it at the end of each of its turns, ending the\
effect on itself on a success. A creature that suc-\
ceeds on its save is immune to the effect of these\
pipes for 24 hours.\

Original layout: [[../../srd-5.2.1/pages/234|page 234]].
