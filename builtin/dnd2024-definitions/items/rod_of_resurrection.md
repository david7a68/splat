---
splat_schema: 1
splat_id: c40bfae4-5482-555c-949d-f937c57c1b35
splat_kind: item
splat_key: dnd2024:items/rod_of_resurrection
splat_name: Rod of Resurrection
splat_origin:
  key: dnd2024:reference/magic_item/rod_of_resurrection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 242
  url: https://www.dndbeyond.com/srd
splat_item:
  category: rod
  rarity: legendary
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/rod_of_resurrection
  implementation: dnd2024:item_rod_of_resurrection
  parameters:
    equipment_state: held
    charges:
      maximum: 5
      initial: 5
      recovery:
        event: dawn
        amount:
          expression: '1'
          scope: use
        cap: maximum
    spell_grants:
    - spell:
        key: dnd2024:spells/heal
      charge_cost: 1
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    - spell:
        key: dnd2024:spells/resurrection
      charge_cost: 5
      casting_time: spell_default
      method: item
      slot_cost: 0
      components: []
      level: lowest_possible
      range: spell_default
      duration: spell_default
      concentration: spell_default
    last_charge:
      roll:
        expression: 1d20
        scope: use
      'on': 1
      effect: item_disappears_harmlessly
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_rods
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_rod_of_resurrection
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
  source_sha256: 8c297d343747dfe8e5a5ae7c05370a3435b9ef3fdc669396d2ffb23cc8924946
  source_path: builtin/srd-5.2.1/magic_item/rod_of_resurrection.md
  pages:
  - 242
  engine_gaps: &id001
  - dnd2024:item_rod_of_resurrection
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 35baec6f7b241e95bf6bd2caa409f69f49c15f2acfc8cec3d9f2002a5fa56830
splat_clauses:
- key: source_rules
  text: '# Rod of Resurrection Rod, Legendary (Requires Attunement) The rod has 5 charges. While you hold
    it, you can cast one of the following spells from it: Heal (expends 1 charge) or Resurrection (expends
    5 charges). The rod regains 1 expended charge daily at dawn. If you expend the last charge, roll 1d20.
    On a 1, the rod disappears in a harmless burst of radiance.'
  sha256: d6d71285849249a2420736bd157447cef52125b6584ad941e284c1821af5e661
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rod of Resurrection

<!-- SRD page 242 -->

Rod, Legendary (Requires Attunement)\
The rod has 5 charges. While you hold it, you can\
cast one of the following spells from it: Heal (ex-\
pends 1 charge) or Resurrection (expends 5 charges).\
The rod regains 1 expended charge daily at dawn.\
If you expend the last charge, roll 1d20. On a 1, the\
rod disappears in a harmless burst of radiance.\

Original layout: [[../../srd-5.2.1/pages/242|page 242]].
