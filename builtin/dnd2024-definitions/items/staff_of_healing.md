---
splat_schema: 1
splat_id: 7faf89d8-1116-5718-ba6b-098b7b1944e3
splat_kind: item
splat_key: dnd2024:items/staff_of_healing
splat_name: Staff of Healing
splat_origin:
  key: dnd2024:reference/magic_item/staff_of_healing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 245
  url: https://www.dndbeyond.com/srd
splat_item:
  category: staff
  rarity: rare
  equipment:
    attunement: required
    attunement_requirement_text: a Bard, Cleric, or Druid
splat_rule:
  mechanic: dnd2024:items/staff_of_healing
  implementation: dnd2024:item_staff_of_healing
  parameters:
    equipment_state: held
    charges:
      maximum: 10
      initial: 10
      recovery:
        event: dawn
        amount:
          dice: 1
          sides: 6
          bonus: 4
    spells:
    - spell:
        key: dnd2024:spells/cure_wounds
      cost_per_spell_level: 1
      minimum_spell_level: 1
      maximum_spell_level: 4
      ability_modifier: user_spellcasting_ability_modifier
    - spell:
        key: dnd2024:spells/lesser_restoration
      cost: 2
      ability_modifier: user_spellcasting_ability_modifier
    - spell:
        key: dnd2024:spells/mass_cure_wounds
      cost: 5
      ability_modifier: user_spellcasting_ability_modifier
    last_charge:
      roll:
        dice: 1
        sides: 20
        bonus: 0
      on_value: 1
      effect: vanish_permanently
    attunement:
      required: true
      eligibility:
        class_any:
        - bard
        - cleric
        - druid
splat_contracts:
- id: dnd2024:item_staff_of_healing
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
  source_sha256: efb1dc72a2a0847e7052076e0f79036bddbb1c22de92c83002865586f6b15d0f
  source_path: builtin/srd-5.2.1/magic_item/staff_of_healing.md
  pages:
  - 245
  engine_gaps: &id001
  - dnd2024:item_staff_of_healing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: f3d04139c137430316a6ece07d2772ece307f3c04c8ab4b0c5467f796b7ff19c
splat_clauses:
- key: source_rules
  text: '# Staff of Healing Staff, Rare (Requires Attunement by a Bard, Cleric, or Druid) This staff has
    10 charges. While holding the staff, you can cast one of the spells on the following table from it,
    using your spellcasting ability modifier. The table indicates how many charges you must expend to
    cast the spell. Spell Charge Cost Cure Wounds 1 charge per spell level (maximum 4 for a level 4 spell)
    Lesser Restoration 2 Mass Cure Wounds 5 Regaining Charges. The staff regains 1d6 + 4 expended charges
    daily at dawn. If you expend the last charge, roll 1d20. On a 1, the staff vanishes in a flash of
    light, lost forever.'
  sha256: 320a7b8d72e3ff44e04bfb8492ddc47406ce77448cd443907b6c58c8d219ed0a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Staff of Healing

<!-- SRD page 245 -->

Staff, Rare (Requires Attunement by a Bard, Cleric, or\
Druid)\
This staff has 10 charges. While holding the staff,\
you can cast one of the spells on the following table\
from it, using your spellcasting ability modifier. The\
table indicates how many charges you must expend\
to cast the spell.\
Spell Charge Cost\
Cure Wounds 1 charge per spell level\
(maximum 4 for a level 4\
spell)\
Lesser Restoration 2\
Mass Cure Wounds 5\
Regaining Charges. The staff regains 1d6 + 4\
expended charges daily at dawn. If you expend the\
last charge, roll 1d20. On a 1, the staff vanishes in a\
flash of light, lost forever.\

Original layout: [[../../srd-5.2.1/pages/245|page 245]].
