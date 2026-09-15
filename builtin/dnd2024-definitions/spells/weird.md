---
splat_schema: 1
splat_id: 9ce28dee-20bd-5fa6-b585-1a86c8c37f08
splat_kind: spell
splat_key: dnd2024:spells/weird
splat_name: Weird
splat_origin:
  key: dnd2024:reference/spell/weird
splat_sources:
- title: SRD 5.2.1
  locator: pp. 174
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 9
  school: illusion
  classes:
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 120
splat_rule:
  mechanic: dnd2024:spells/weird
  implementation: dnd2024:spell_weird
  parameters:
    targeting:
      kind: chosen_creatures_in_area
      shape: sphere
      radius_feet: 30
      origin: chosen_point_in_range
    initial_save:
      ability: wisdom
      failure:
        damage:
          dice: 10d10
          type: psychic
        condition: frightened
        duration: spell_duration
      success:
        damage_fraction:
          numerator: 1
          denominator: 2
    repeat:
      event: frightened_target.turn_ended
      save:
        ability: wisdom
        failure:
          damage:
            dice: 5d10
            type: psychic
        success:
          end_spell_on_target: true
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: minute
splat_contracts:
- id: dnd2024:spell_weird
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
    save_dc:
      type: integer
      required: true
    choices:
      type: choice
      required: true
  outputs:
  - effects
  - damage_results
  events:
  - spell.cast_completed
  - turn.ended
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - conditions
  - spatial_effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: 7cada96a5180aba9458df73b6d23701ba7075d963a61ab3ea425f6531ea6edff
  source_path: builtin/srd-5.2.1/spell/weird.md
  pages:
  - 174
  engine_gaps: &id001
  - dnd2024:spell_weird
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 8d824bd53f9f15e20a1fb3bd7e5cd883e528f0d8e9549cc7876953505181bd18
splat_clauses:
- key: source_rules
  text: '# Weird Level 9 Illusion (Warlock, Wizard) Casting Time: Action Range: 120 feet Components: V,
    S Duration: Concentration, up to 1 minute You try to create illusory terrors in others’ minds. Each
    creature of your choice in a 30-foot-radius Sphere centered on a point within range makes a Wisdom
    saving throw. On a failed save, a target takes 10d10 Psychic damage and has the Frightened condition
    for the duration. On a successful save, a target takes half as much damage only. A Frightened target
    makes a Wisdom saving throw at the end of each of its turns. On a failed save, it takes 5d10 Psychic
    damage. On a successful save, the spell ends on that target.'
  sha256: 7fb2a82e75e0ca715d4ed6e69334925353e0f91c696747077998e95e3c743107
  status: contract_specified
splat_requires_mechanics: *id001
---

# Weird

<!-- SRD page 174 -->

Level 9 Illusion (Warlock, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Concentration, up to 1 minute\
You try to create illusory terrors in others’ minds.\
Each creature of your choice in a 30-foot-radius\
Sphere centered on a point within range makes a\
Wisdom saving throw. On a failed save, a target\
takes 10d10 Psychic damage and has the Frightened\
condition for the duration. On a successful save, a\
target takes half as much damage only.\
A Frightened target makes a Wisdom saving\
throw at the end of each of its turns. On a failed\
save, it takes 5d10 Psychic damage. On a successful\
save, the spell ends on that target.\

Original layout: [[../../srd-5.2.1/pages/174|page 174]].
