---
splat_schema: 1
splat_id: f823b2bb-c413-5344-9ed4-c22b3b4cc961
splat_kind: item
splat_key: dnd2024:items/wand_of_fear
splat_name: Wand of Fear
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_fear
splat_sources:
- title: SRD 5.2.1
  locator: pp. 250
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wand
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/wand_of_fear
  implementation: dnd2024:item_wand_of_fear
  parameters:
    equipment_state: held
    charges:
      maximum: 7
      initial: 7
      recovery:
        event: dawn
        amount:
          expression: 1d6+1
          scope: use
        cap: maximum
    spell_grants:
    - spell:
        key: dnd2024:spells/command
      charge_cost: 1
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
      save_dc: 15
      allowed_commands:
      - flee
      - grovel
    - spell:
        key: dnd2024:spells/fear
      charge_cost: 3
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
      save_dc: 15
      area_override:
        shape: cone
        length_ft: 60
    last_charge:
      roll:
        expression: 1d20
        scope: use
      'on': 1
      effect: destroyed
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wands
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_wand_of_fear
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
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - spellcasting
  - resources_and_time
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 6cdf3b4cf2e80ac44801e7bcaa52c1df1322ebf49a7657ec1ce9dbc31c65d1bf
  source_path: builtin/srd-5.2.1/magic_item/wand_of_fear.md
  pages:
  - 250
  engine_gaps: &id001
  - dnd2024:item_wand_of_fear
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 7c90b9333b027378e5cc914bbd376f5050602938c152e9aeb169e914acec174b
splat_clauses:
- key: source_rules
  text: '# Wand of Fear Wand, Rare (Requires Attunement) This wand has 7 charges. Spells. While holding
    the wand, you can cast one of the spells (save DC 15) on the following table from it. The table indicates
    how many charges you must expend to cast the spell. Charge Spell Cost Command (“flee” or “grovel”
    only) 1 Fear (60-foot Cone) 3 Regaining Charges. The wand regains 1d6 + 1 expended charges daily at
    dawn. If you expend the wand’s last charge, roll 1d20. On a 1, the wand crumbles into ashes and is
    destroyed.'
  sha256: 8afa667da7f7c4f687fa8f19f09641051330a00dd211911b92b86d74011a560e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wand of Fear

<!-- SRD page 250 -->

Wand, Rare (Requires Attunement)\
This wand has 7 charges.\
Spells. While holding the wand, you can cast one\
of the spells (save DC 15) on the following table\
from it. The table indicates how many charges you\
must expend to cast the spell.\
Charge\
Spell Cost\
Command (“flee” or “grovel” only) 1\
Fear (60-foot Cone) 3\
Regaining Charges. The wand regains 1d6 +\
1 expended charges daily at dawn. If you expend\
the wand’s last charge, roll 1d20. On a 1, the wand\
crumbles into ashes and is destroyed.\

Original layout: [[../../srd-5.2.1/pages/250|page 250]].
