---
splat_schema: 1
splat_id: 7ab5c16e-4f9b-58c2-a437-4ea7fb3cbc2f
splat_kind: item
splat_key: dnd2024:items/wand_of_binding
splat_name: Wand of Binding
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_binding
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
  mechanic: dnd2024:items/wand_of_binding
  implementation: dnd2024:item_wand_of_binding
  parameters:
    equipped: held
    charges:
      initial: 7
      maximum: 7
      recovery:
        event: time.dawn
        amount:
          roll:
            count: 1
            sides: 6
            add: 1
        cap_at_maximum: true
      last_charge:
        roll:
          roll:
            count: 1
            sides: 20
        'on': 1
        result: destroy_to_ashes
    spells:
    - spell:
        key: dnd2024:spells/hold_monster
      charge_cost: 5
      save_dc: 17
    - spell:
        key: dnd2024:spells/hold_person
      charge_cost: 2
      save_dc: 17
    attunement_required: true
splat_contracts:
- id: dnd2024:item_wand_of_binding
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
  - spell.cast_pending
  - time.dawn
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - dice
  - item_spellcasting
  - item_charges
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 8578626fbfd68c0f051346234a62d7ef9c32d5c9bc601767640e5d95532b02a9
  source_path: builtin/srd-5.2.1/magic_item/wand_of_binding.md
  pages:
  - 250
  engine_gaps: &id001
  - dnd2024:item_wand_of_binding
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 0a6a609d95c6df86f303aea1988a9ea9954533784c198df40df0420b6b083d48
splat_clauses:
- key: source_rules
  text: '# Wand of Binding Wand, Rare (Requires Attunement) This wand has 7 charges. Spells. While holding
    the wand, you can cast one of the spells (save DC 17) on the following table from it. The table indicates
    how many charges you must expend to cast the spell. Spell Charge Cost Hold Monster 5 Hold Person 2
    Regaining Charges. The wand regains 1d6 + 1 expended charges daily at dawn. If you expend the wand’s
    last charge, roll 1d20. On a 1, the wand crumbles into ashes and is destroyed.'
  sha256: 012c3d01a95019ded19c63feaa306339324f791dd05a9fbc6e039486a0a1da6a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wand of Binding

<!-- SRD page 250 -->

Wand, Rare (Requires Attunement)\
This wand has 7 charges.\
Spells. While holding the wand, you can cast one\
of the spells (save DC 17) on the following table\
from it. The table indicates how many charges you\
must expend to cast the spell.\
Spell Charge\
Cost\
Hold Monster 5\
Hold Person 2\
Regaining Charges. The wand regains 1d6 +\
1 expended charges daily at dawn. If you expend\
the wand’s last charge, roll 1d20. On a 1, the wand\
crumbles into ashes and is destroyed.\

Original layout: [[../../srd-5.2.1/pages/250|page 250]].
