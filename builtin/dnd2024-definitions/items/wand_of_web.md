---
splat_schema: 1
splat_id: 4dcf10d2-e55b-560d-8d33-1a89b6f7fd52
splat_kind: item
splat_key: dnd2024:items/wand_of_web
splat_name: Wand of Web
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_web
splat_sources:
- title: SRD 5.2.1
  locator: pp. 251
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wand
  rarity: uncommon
  equipment:
    attunement: required
    attunement_requirement_text: a Spellcaster
splat_rule:
  mechanic: dnd2024:items/wand_of_web
  implementation: dnd2024:item_wand_of_web
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
        key: dnd2024:spells/web
      charge_cost: 1
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
      save_dc: 13
    last_charge:
      roll:
        expression: 1d20
        scope: use
      'on': 1
      effect: destroyed
    attunement:
      required: true
      requires_spellcaster: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wands
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_wand_of_web
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
  source_sha256: 472f8587c34276d645cb1387fbb56a4c670e0ffd922e99768f77a2398496654e
  source_path: builtin/srd-5.2.1/magic_item/wand_of_web.md
  pages:
  - 251
  engine_gaps: &id001
  - dnd2024:item_wand_of_web
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: c6fa06ad163ee62719a76d9926b550ddf76e0d7be083507fc085e921e087f78e
splat_clauses:
- key: source_rules
  text: '# Wand of Web Wand, Uncommon (Requires Attunement by a Spellcaster) This wand has 7 charges.
    While holding it, you can expend 1 charge to cast Web (save DC 13) from it. Regaining Charges. The
    wand regains 1d6 + 1 expended charges daily at dawn. If you expend the wand’s last charge, roll 1d20.
    On a 1, the wand crumbles into ashes and is destroyed.'
  sha256: b98624b3143fa030163c6a700a95fe56f9a25a2034272848a995e9083822c84a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wand of Web

<!-- SRD page 251 -->

Wand, Uncommon (Requires Attunement by a\
Spellcaster)\
This wand has 7 charges. While holding it, you can\
expend 1 charge to cast Web (save DC 13) from it.\
Regaining Charges. The wand regains 1d6 +\
1 expended charges daily at dawn. If you expend\
the wand’s last charge, roll 1d20. On a 1, the wand\
crumbles into ashes and is destroyed.\

Original layout: [[../../srd-5.2.1/pages/251|page 251]].
