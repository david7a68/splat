---
splat_schema: 1
splat_id: 1299b231-a156-55d5-9887-9b1b51002cf4
splat_kind: spell
splat_key: dnd2024:spells/shatter
splat_name: Shatter
splat_origin:
  key: dnd2024:reference/spell/shatter
splat_sources:
- title: SRD 5.2.1
  locator: pp. 161
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: evocation
  classes:
  - bard
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 60
  material:
    description: a chip of mica
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/shatter
  implementation: dnd2024:spell_shatter
  parameters:
    targeting:
      kind: creatures_in_area
      shape: sphere
      radius_feet: 10
      origin: chosen_point_in_range
    save:
      ability: constitution
      disadvantage_types:
      - construct
      failure:
        damage:
          dice: 3d8
          type: thunder
      success:
        damage_fraction:
          numerator: 1
          denominator: 2
    object_damage:
      within_area: true
      magical: false
      worn: false
      carried: false
      damage:
        dice: 3d8
        type: thunder
      save: none
    sound: loud_noise
    slot_scaling:
      above: 2
      damage_per_level: 1d8
      applies_to:
      - creatures
      - objects
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_shatter
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      type: participant_list
      required: true
    locations:
      type: location_list
      required: true
    objects:
      required: false
      type: object_list
    save_dc:
      type: integer
      required: true
    choices:
      type: choice
      required: true
  outputs:
  - effects
  - damage_results
  - locations
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - spatial_effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: f7450ffdafbcbfb9ecdc0c5db6a72cf9c7efaf11c2da20d0c5ebcf0433f074ef
  source_path: builtin/srd-5.2.1/spell/shatter.md
  pages:
  - 161
  engine_gaps: &id001
  - dnd2024:spell_shatter
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 07b917b03854bbbb5a995e68055d6137f37628bc9e6eb5d7d746b6e6bdea91c5
splat_clauses:
- key: source_rules
  text: '# Shatter Level 2 Evocation (Bard, Sorcerer, Wizard) Casting Time: Action Range: 60 feet Components:
    V, S, M (a chip of mica) Duration: Instantaneous A loud noise erupts from a point of your choice within
    range. Each creature in a 10-foot-radius Sphere centered there makes a Constitution saving throw,
    taking 3d8 Thunder damage on a failed save or half as much damage on a successful one. A Construct
    has Disadvantage on the save. A nonmagical object that isn’t being worn or carried also takes the
    damage if it’s in the spell’s area. Using a Higher-Level Spell Slot. The damage increases by 1d8 for
    each spell slot level above 2.'
  sha256: 3f0a5bea1068156b489569a39061073179c5bd385288aa594af6e168f60877aa
  status: contract_specified
splat_requires_mechanics: *id001
---

# Shatter

<!-- SRD page 161 -->

Level 2 Evocation (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a chip of mica)\
Duration: Instantaneous\
A loud noise erupts from a point of your choice\
within range. Each creature in a 10-foot-radius\
Sphere centered there makes a Constitution saving\
throw, taking 3d8 Thunder damage on a failed save\
or half as much damage on a successful one. A Con-\
struct has Disadvantage on the save.\
A nonmagical object that isn’t being worn or car-\
ried also takes the damage if it’s in the spell’s area.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for each spell slot level above 2.\

Original layout: [[../../srd-5.2.1/pages/161|page 161]].
