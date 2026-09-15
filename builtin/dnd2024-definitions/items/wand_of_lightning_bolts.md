---
splat_schema: 1
splat_id: 14d7eb55-c00b-5868-8a8b-f9d3df0830bd
splat_kind: item
splat_key: dnd2024:items/wand_of_lightning_bolts
splat_name: Wand of Lightning Bolts
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_lightning_bolts
splat_sources:
- title: SRD 5.2.1
  locator: pp. 251
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wand
  rarity: rare
  equipment:
    attunement: required
    attunement_requirement_text: a Spellcaster
splat_rule:
  mechanic: dnd2024:items/wand_of_lightning_bolts
  implementation: dnd2024:item_wand_of_lightning_bolts
  parameters:
    equipment_state: held
    charges:
      maximum: 7
      initial: 7
      recovery:
        event: dawn
        amount:
          dice: 1
          sides: 6
          bonus: 1
    spells:
    - spell:
        key: dnd2024:spells/lightning_bolt
      save_dc: 15
      charge_choice:
        minimum: 1
        maximum: 3
      spell_level:
        sum:
        - 2
        - value_from: charges_spent
    last_charge:
      roll:
        dice: 1
        sides: 20
        bonus: 0
      on_value: 1
      effect: destroy_item
    attunement:
      required: true
      eligibility:
        spellcaster: true
splat_contracts:
- id: dnd2024:item_wand_of_lightning_bolts
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
  outputs:
  - effects
  - item_state
  events:
  - item.used
  - spell.cast_completed
  capabilities:
  - equipment
  - item_resources
  - effects
  - item_spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 9e106773ccf017a850f5717bf43564f9f1465ee2de4889a732eb2cd963d2214c
  source_path: builtin/srd-5.2.1/magic_item/wand_of_lightning_bolts.md
  pages:
  - 251
  engine_gaps: &id001
  - dnd2024:item_wand_of_lightning_bolts
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 9e5eec9a6bbdad369ea4bf1da38a5c7c55c0a6048cf0e2dca9e1e01d637a8b0a
splat_clauses:
- key: source_rules
  text: '# Wand of Lightning Bolts Wand, Rare (Requires Attunement by a Spellcaster) This wand has 7 charges.
    While holding it, you can expend no more than 3 charges to cast Lightning Bolt (save DC 15) from it.
    For 1 charge, you cast the level 3 version of the spell. You can increase the spell’s level by 1 for
    each additional charge you expend. Regaining Charges. The wand regains 1d6 + 1 expended charges daily
    at dawn. If you expend the wand’s last charge, roll 1d20. On a 1, the wand crumbles into ashes and
    is destroyed.'
  sha256: 996082fdc1815da3073a239c8a521170245d179422b762e481ee8b432cee0f8a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wand of Lightning Bolts

<!-- SRD page 251 -->

Wand, Rare (Requires Attunement by a Spellcaster)\
This wand has 7 charges. While holding it, you can\
expend no more than 3 charges to cast Lightning\
Bolt (save DC 15) from it. For 1 charge, you cast\
the level 3 version of the spell. You can increase\
the spell’s level by 1 for each additional charge you\
expend.\
Regaining Charges. The wand regains 1d6 +\
1 expended charges daily at dawn. If you expend\
the wand’s last charge, roll 1d20. On a 1, the wand\
crumbles into ashes and is destroyed.\

Original layout: [[../../srd-5.2.1/pages/251|page 251]].
