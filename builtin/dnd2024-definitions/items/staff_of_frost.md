---
splat_schema: 1
splat_id: e9833cc4-c7a6-5188-a882-956c6797a70a
splat_kind: item
splat_key: dnd2024:items/staff_of_frost
splat_name: Staff of Frost
splat_origin:
  key: dnd2024:reference/magic_item/staff_of_frost
splat_sources:
- title: SRD 5.2.1
  locator: pp. 245
  url: https://www.dndbeyond.com/srd
splat_item:
  category: staff
  rarity: very_rare
  equipment:
    attunement: required
    attunement_requirement_text: a Druid, Sorcerer, Warlock, or Wizard
splat_rule:
  mechanic: dnd2024:items/staff_of_frost
  implementation: dnd2024:item_staff_of_frost
  parameters:
    equipped: held
    attunement_classes:
    - druid
    - sorcerer
    - warlock
    - wizard
    resistance:
    - cold
    charges:
      initial: 10
      maximum: 10
      recovery:
        event: time.dawn
        amount:
          roll:
            count: 1
            sides: 6
            add: 4
        cap_at_maximum: true
      last_charge:
        roll:
          roll:
            count: 1
            sides: 20
        'on': 1
        result: turn_to_water_and_destroy
    spells:
    - spell:
        key: dnd2024:spells/cone_of_cold
      charge_cost: 5
      save_dc: user.spell_save_dc
    - spell:
        key: dnd2024:spells/ice_storm
      charge_cost: 4
      save_dc: user.spell_save_dc
    - spell:
        key: dnd2024:spells/fog_cloud
      charge_cost: 1
      save_dc: user.spell_save_dc
    - spell:
        key: dnd2024:spells/wall_of_ice
      charge_cost: 4
      save_dc: user.spell_save_dc
    attunement_required: true
splat_contracts:
- id: dnd2024:item_staff_of_frost
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
  - damage.pending
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
  source_sha256: cb6fd106b4c49588627c8e765b8885a54aa79df20c3a10c4130305cf7ac6117d
  source_path: builtin/srd-5.2.1/magic_item/staff_of_frost.md
  pages:
  - 245
  engine_gaps: &id001
  - dnd2024:item_staff_of_frost
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: b9851be5a3fef13adfc443a81d5b3829472ba88baf2b606c20d513f457feae07
splat_clauses:
- key: source_rules
  text: '# Staff of Frost Staff, Very Rare (Requires Attunement by a Druid, Sorcerer, Warlock, or Wizard)
    You have Resistance to Cold damage while you hold this staff. Spells. The staff has 10 charges. While
    holding the staff, you can cast one of the spells on the following table from it, using your spell
    save DC. The table indicates how many charges you must expend to cast the spell. Charge Charge Spell
    Cost Spell Cost Cone of Cold 5 Ice Storm 4 Fog Cloud 1 Wall of Ice 4 Regaining Charges. The staff
    regains 1d6 + 4 expended charges daily at dawn. If you expend the last charge, roll 1d20. On a 1,
    the staff turns to water and is destroyed.'
  sha256: fb33844a7734c4e2969bd90601a2d98f306059cc91175f4f2bd29ed0a7a092ba
  status: contract_specified
splat_requires_mechanics: *id001
---

# Staff of Frost

<!-- SRD page 245 -->

Staff, Very Rare (Requires Attunement by a Druid,\
Sorcerer, Warlock, or Wizard)\
You have Resistance to Cold damage while you hold\
this staff.\
Spells. The staff has 10 charges. While holding the\
staff, you can cast one of the spells on the following\
table from it, using your spell save DC. The table in-\
dicates how many charges you must expend to cast\
the spell.\
Charge Charge\
Spell Cost Spell Cost\
Cone of Cold 5 Ice Storm 4\
Fog Cloud 1 Wall of Ice 4\
Regaining Charges. The staff regains 1d6 + 4 ex-\
pended charges daily at dawn. If you expend the last\
charge, roll 1d20. On a 1, the staff turns to water\
and is destroyed.\

Original layout: [[../../srd-5.2.1/pages/245|page 245]].
