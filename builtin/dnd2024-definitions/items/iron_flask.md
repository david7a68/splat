---
splat_schema: 1
splat_id: 8ea683ac-bc3b-50bb-9d17-b1c4a8f8c7c0
splat_kind: item
splat_key: dnd2024:items/iron_flask
splat_name: Iron Flask
splat_origin:
  key: dnd2024:reference/magic_item/iron_flask
splat_sources:
- title: SRD 5.2.1
  locator: pp. 228
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: legendary
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/iron_flask
  implementation: dnd2024:item_iron_flask
  parameters:
    equipment_state: held
    capture:
      action: magic
      range_ft: 60
      must_see: true
      requires_flask_empty: true
      target_native_plane_must_differ_from_current: true
      save:
        ability: wisdom
        dc: 17
        failure:
          trapped_in_flask: true
        success:
          effect: none
      save_advantage_if: previously_trapped_by_same_flask
    capacity_creatures: 1
    trapped:
      duration: until_released
      age: false
      needs:
      - no_breathing
      - no_food
      - no_drink
    release:
      action: magic
      remove_stopper: true
      obeys_user_commands_duration:
        amount: 1
        unit: hour
      understands_commands_any_language: true
      no_command_or_likely_death_or_imprisonment:
        self_defense: true
        other_actions: false
      after_duration: normal_disposition_and_alignment
    identify:
      reveals_occupied: true
      reveals_creature_type: false
    determine_type: open_flask
    initial_occupant:
      chooser: gm
      optional: true
      type: definition
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_iron_flask
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
  - time.elapsed
  - spell.cast_completed
  - command.issued
  capabilities:
  - inventory
  - effects
  - resources_and_time
  - roll_resolution
  - spatial_state
  - participant_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 54d9f7c2b45891e8d09a4678547988a739ba1a611926ebcfa77f3f2dade1cc6c
  source_path: builtin/srd-5.2.1/magic_item/iron_flask.md
  pages:
  - 228
  engine_gaps: &id001
  - dnd2024:item_iron_flask
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 7ecd7d7b846a9ec84fc8240d7ee86cf9b0cb70f40fd7696c62ffc339b1636dba
splat_clauses:
- key: source_rules
  text: '# Iron Flask Wondrous Item, Legendary While holding this brass-stoppered iron flask, you can
    take a Magic action to target a creature that you can see within 60 feet of yourself. If the flask
    is empty and the target is native to a plane of existence other than the one you’re on, the target
    must succeed on a DC 17 Wisdom saving throw or be trapped in the flask. If the target has been trapped
    by the flask before, it has Advantage on the save. Once trapped, a creature remains in the flask until
    released. The flask can hold only one creature at a time. A creature trapped in the flask doesn’t
    age and doesn’t need to breathe, eat, or drink. You can take a Magic action to remove the flask’s
    stopper and release the creature in the flask. The creature then obeys your commands for 1 hour, understanding
    those commands even if it doesn’t know the language in which the commands are given. If you issue
    no commands or give the creature a command that is likely to result in its death or imprisonment,
    it defends itself but otherwise takes no actions. At the end of the duration, the creature acts in
    accordance with its normal disposition and alignment. An Identify spell reveals if the flask contains
    a creature, but the only way to determine the type of creature is to open the flask. A newly discovered
    Iron Flask might already contain a creature chosen by the GM.'
  sha256: 709969e4a320b3cf3b5e05b73a95d8ccffa355eaeac6b6a60e28b12b7b98bfd7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Iron Flask

<!-- SRD page 228 -->

Wondrous Item, Legendary\
While holding this brass-stoppered iron flask, you\
can take a Magic action to target a creature that\
you can see within 60 feet of yourself. If the flask\
is empty and the target is native to a plane of exis-\
tence other than the one you’re on, the target must\
succeed on a DC 17 Wisdom saving throw or be\
trapped in the flask. If the target has been trapped\
by the flask before, it has Advantage on the save.\
Once trapped, a creature remains in the flask until\
released. The flask can hold only one creature at a\
time. A creature trapped in the flask doesn’t age and\
doesn’t need to breathe, eat, or drink.\
You can take a Magic action to remove the flask’s\
stopper and release the creature in the flask. The\
creature then obeys your commands for 1 hour,\
understanding those commands even if it doesn’t\
know the language in which the commands are\
given. If you issue no commands or give the crea-\
ture a command that is likely to result in its death\
or imprisonment, it defends itself but otherwise\
takes no actions. At the end of the duration, the\
creature acts in accordance with its normal disposi-\
tion and alignment.\
An Identify spell reveals if the flask contains a\
creature, but the only way to determine the type\
of creature is to open the flask. A newly discovered\
Iron Flask might already contain a creature chosen\
by the GM.\

Original layout: [[../../srd-5.2.1/pages/228|page 228]].
