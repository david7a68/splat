---
splat_schema: 1
splat_id: aee61ca4-d6e7-5870-9535-bfa3fdbf2dd2
splat_kind: item
splat_key: dnd2024:items/wand_of_fireballs
splat_name: Wand of Fireballs
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_fireballs
splat_sources:
- title: SRD 5.2.1
  locator: pp. 250, 251
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wand
  rarity: rare
  stackable: false
  equipment:
    allowed:
    - carried
    - wielded
    attunement: required
    attunement_requirement: spellcaster
splat_rule:
  mechanic: dnd2024:items/wand_of_fireballs
  implementation: dnd2024:item_wand_of_fireballs
  parameters:
    equipped: held
    attunement_requires: spellcaster
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
        key: dnd2024:spells/fireball
      save_dc: 15
      charge_choice:
        min: 1
        max: 3
      cast_level:
        add:
        - 2
        - charges_spent
    attunement_required: true
splat_contracts:
- id: dnd2024:item_wand_of_fireballs
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
  source_sha256: 6b9828133e84bcf63e3668f7771dd059473117f97116c0f4f14e41084d9450b2
  source_path: builtin/srd-5.2.1/magic_item/wand_of_fireballs.md
  pages:
  - 250
  - 251
  engine_gaps: &id001
  - dnd2024:item_wand_of_fireballs
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: 6dd8a630cb8d6b31ec4fa11b04fdee0bab190ebd7c3ce5592f22fc43054bebfa
splat_clauses:
- key: source_rules
  text: '# Wand of Fireballs Wand, Rare (Requires Attunement by a Spellcaster) This wand has 7 charges.
    While holding it, you can expend no more than 3 charges to cast Fireball (save DC 15) from it. For
    1 charge, you cast the level 3 version of the spell. You can increase the spell’s level by 1 for each
    additional charge you expend. Regaining Charges. The wand regains 1d6 + 1 expended charges daily at
    dawn. If you expend the wand’s last charge, roll 1d20. On a 1, the wand crumbles into ashes and is
    destroyed.'
  sha256: b2ceb6636047765b7c231fc476ef7bed6f3cd16c4c2b19667ee55ece745aaae5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wand of Fireballs

<!-- SRD page 250 -->

Wand, Rare (Requires Attunement by a Spellcaster)\
This wand has 7 charges. While holding it, you can\
expend no more than 3 charges to cast Fireball (save\
DC 15) from it. For 1 charge, you cast the level 3 ver-\
sion of the spell. You can increase the spell’s level by\
1 for each additional charge you expend.\

<!-- SRD page 251 -->

Regaining Charges. The wand regains 1d6 +\
1 expended charges daily at dawn. If you expend\
the wand’s last charge, roll 1d20. On a 1, the wand\
crumbles into ashes and is destroyed.\

Original layout: [[../../srd-5.2.1/pages/250|page 250]], [[../../srd-5.2.1/pages/251|page 251]].
