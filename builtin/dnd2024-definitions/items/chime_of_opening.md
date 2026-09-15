---
splat_schema: 1
splat_id: 006b7920-7978-513c-a4b5-c2d7b732bd99
splat_kind: item
splat_key: dnd2024:items/chime_of_opening
splat_name: Chime of Opening
splat_origin:
  key: dnd2024:reference/magic_item/chime_of_opening
splat_sources:
- title: SRD 5.2.1
  locator: pp. 215
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/chime_of_opening
  implementation: dnd2024:item_chime_of_opening
  parameters:
    length_ft: 1
    weight_lb: 1
    uses:
      maximum: 10
      initial: 10
    spell_grants:
    - spell:
        key: dnd2024:spells/knock
      charge_cost: 1
      casting_time: magic
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
      activation: strike_chime
      sound_override:
        kind: ringing_tone
        audible_ft: 300
    on_last_use:
      state: cracked_useless
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_chime_of_opening
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
  capabilities:
  - inventory
  - effects
  - spellcasting
  - resources_and_time
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 5f038ff225e3e2c0ba48856fbb48b6932aa2870b2e7c95e797264b5a68cc64ed
  source_path: builtin/srd-5.2.1/magic_item/chime_of_opening.md
  pages:
  - 215
  engine_gaps: &id001
  - dnd2024:item_chime_of_opening
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: a14ebd9de4949beb5d1247f827eaf5debcbbc7f3d119b65a54220a6e143e9da7
splat_clauses:
- key: source_rules
  text: '# Chime of Opening Wondrous Item, Rare This hollow metal tube measures about 1 foot long and
    weighs 1 pound. As a Magic action, you can strike the chime to cast Knock. The spell’s customary knocking
    sound is replaced by the clear, ringing tone of the chime, which is audible out to 300 feet. The chime
    can be used 10 times. After the tenth time, it cracks and becomes useless.'
  sha256: 67c2564ad83da9518a2660c1dca456e4412d87b97060abcfedf287f43e9640b3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Chime of Opening

<!-- SRD page 215 -->

Wondrous Item, Rare\
This hollow metal tube measures about 1 foot long\
and weighs 1 pound. As a Magic action, you can\
strike the chime to cast Knock. The spell’s custom-\
ary knocking sound is replaced by the clear, ringing\
tone of the chime, which is audible out to 300 feet.\
The chime can be used 10 times. After the tenth\
time, it cracks and becomes useless.\

Original layout: [[../../srd-5.2.1/pages/215|page 215]].
