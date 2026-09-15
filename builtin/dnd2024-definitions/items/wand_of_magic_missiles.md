---
splat_schema: 1
splat_id: 131ad305-433e-5c2b-b948-986c734f888c
splat_kind: item
splat_key: dnd2024:items/wand_of_magic_missiles
splat_name: Wand of Magic Missiles
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_magic_missiles
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
  mechanic: dnd2024:items/wand_of_magic_missiles
  implementation: dnd2024:item_wand_of_magic_missiles
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
        key: dnd2024:spells/magic_missile
      charge_choice:
        min: 1
        max: 3
      cast_level: charges_spent
    attunement_required: false
splat_contracts:
- id: dnd2024:item_wand_of_magic_missiles
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
  capabilities:
  - item_effects
  - dice
  - item_spellcasting
  - item_charges
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: d3f3c5ab0b44b8de4a5dd40799894a269c5c1325805dacdbd4e3ae1816afa027
  source_path: builtin/srd-5.2.1/magic_item/wand_of_magic_missiles.md
  pages:
  - 251
  engine_gaps: &id001
  - dnd2024:item_wand_of_magic_missiles
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 0cfef011cc9c96de3de2c4a1dd56f588b0610df6a371ef4b3782b052340c20d1
splat_clauses:
- key: source_rules
  text: '# Wand of Magic Missiles Wand, Uncommon This wand has 7 charges. While holding it, you can expend
    no more than 3 charges to cast Magic Missile from it. For 1 charge, you cast the level 1 version of
    the spell. You can increase the spell’s level by 1 for each additional charge you expend. Regaining
    Charges. The wand regains 1d6 + 1 expended charges daily at dawn. If you expend the wand’s last charge,
    roll 1d20. On a 1, the wand crumbles into ashes and is destroyed.'
  sha256: 5af4fe933d4c31a5862f5a428841ca33bf37bfb24c6a68788adb8c376da24fa4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wand of Magic Missiles

<!-- SRD page 251 -->

Wand, Uncommon\
This wand has 7 charges. While holding it, you can\
expend no more than 3 charges to cast Magic Missile\
from it. For 1 charge, you cast the level 1 version of\
the spell. You can increase the spell’s level by 1 for\
each additional charge you expend.\
Regaining Charges. The wand regains 1d6 +\
1 expended charges daily at dawn. If you expend\
the wand’s last charge, roll 1d20. On a 1, the wand\
crumbles into ashes and is destroyed.\

Original layout: [[../../srd-5.2.1/pages/251|page 251]].
