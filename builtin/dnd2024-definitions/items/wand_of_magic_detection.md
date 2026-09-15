---
splat_schema: 1
splat_id: af994011-d082-588c-8cf3-ef3b1e04b0d7
splat_kind: item
splat_key: dnd2024:items/wand_of_magic_detection
splat_name: Wand of Magic Detection
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_magic_detection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 251
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wand
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/wand_of_magic_detection
  implementation: dnd2024:item_wand_of_magic_detection
  parameters:
    equipment_state: held
    charges:
      maximum: 3
      initial: 3
      recovery:
        event: dawn
        amount:
          expression: 1d3
          scope: use
        cap: maximum
    spell_grants:
    - spell:
        key: dnd2024:spells/detect_magic
      charge_cost: 1
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wands
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_wand_of_magic_detection
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
  - spell.cast_pending
  - time.dawn
  capabilities:
  - inventory
  - effects
  - spellcasting
  - resources_and_time
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 171c9142bdaa816f7e3a5a9dc05f5382a3bed248b2b746c01a8b1d34842f9374
  source_path: builtin/srd-5.2.1/magic_item/wand_of_magic_detection.md
  pages:
  - 251
  engine_gaps: &id001
  - dnd2024:item_wand_of_magic_detection
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 71780f1edbeee399e195e3c5a759952f98670d5ee7c9dcb8793c37c166b81df3
splat_clauses:
- key: source_rules
  text: '# Wand of Magic Detection Wand, Uncommon This wand has 3 charges. While holding it, you can expend
    1 charge to cast Detect Magic from it. The wand regains 1d3 expended charges daily at dawn.'
  sha256: e671248257fd4b7237749227e51a52d29f57064cd3dc22a36308b85d472bc634
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wand of Magic Detection

<!-- SRD page 251 -->

Wand, Uncommon\
This wand has 3 charges. While holding it, you can\
expend 1 charge to cast Detect Magic from it. The\
wand regains 1d3 expended charges daily at dawn.\

Original layout: [[../../srd-5.2.1/pages/251|page 251]].
