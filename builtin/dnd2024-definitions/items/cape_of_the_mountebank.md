---
splat_schema: 1
splat_id: eac6529b-e8f1-5510-b331-9b08d6893970
splat_kind: item
splat_key: dnd2024:items/cape_of_the_mountebank
splat_name: Cape of the Mountebank
splat_origin:
  key: dnd2024:reference/magic_item/cape_of_the_mountebank
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
  mechanic: dnd2024:items/cape_of_the_mountebank
  implementation: dnd2024:item_cape_of_the_mountebank
  parameters:
    equipment_state: worn
    uses:
      maximum: 1
      initial: 1
      recovery:
        event: dawn
        amount:
          expression: '1'
          scope: use
        cap: maximum
    spell_grants:
    - spell:
        key: dnd2024:spells/dimension_door
      charge_cost: 1
      casting_time: magic
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    on_teleport:
      origin_space:
        obscurity: lightly_obscured
        duration:
          until: end_of_user_next_turn
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_cape_of_the_mountebank
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
  - spell.teleport_completed
  - turn.ended
  - time.dawn
  capabilities:
  - inventory
  - effects
  - spellcasting
  - resources_and_time
  - spatial_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 3901db0180b672c3a92ca2c4f176d2a76b86d973de9e6ee31299fc93f1b7ef62
  source_path: builtin/srd-5.2.1/magic_item/cape_of_the_mountebank.md
  pages:
  - 215
  engine_gaps: &id001
  - dnd2024:item_cape_of_the_mountebank
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 1614b44169f52ff33c250b736eaf444c832c654adbb85e88654780649028f260
splat_clauses:
- key: source_rules
  text: '# Cape of the Mountebank Wondrous Item, Rare This cape smells faintly of brimstone. While wearing
    it, you can use it to cast Dimension Door as a Magic action. This property can’t be used again until
    the next dawn. When you teleport with that spell, you leave behind a cloud of smoke. The space you
    left is Lightly Obscured by that smoke until the end of your next turn.'
  sha256: 874f4f0122e0d8f97eff8c038ef179511018c65fcbaea5e0fbe797f9210745ea
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cape of the Mountebank

<!-- SRD page 215 -->

Wondrous Item, Rare\
This cape smells faintly of brimstone. While wear-\
ing it, you can use it to cast Dimension Door as a\
Magic action. This property can’t be used again\
until the next dawn.\
When you teleport with that spell, you leave be-\
hind a cloud of smoke. The space you left is Lightly\
Obscured by that smoke until the end of your\
next turn.\

Original layout: [[../../srd-5.2.1/pages/215|page 215]].
