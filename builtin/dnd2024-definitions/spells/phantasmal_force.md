---
splat_schema: 1
splat_id: 0331a325-1bf8-55b5-8810-62b0fed75e95
splat_kind: spell
splat_key: dnd2024:spells/phantasmal_force
splat_name: Phantasmal Force
splat_origin:
  key: dnd2024:reference/spell/phantasmal_force
splat_sources:
- title: SRD 5.2.1
  locator: pp. 151, 152
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: illusion
  classes:
  - bard
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 60
  material:
    description: a bit of fleece
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/phantasmal_force
  implementation: dnd2024:spell_phantasmal_force
  parameters:
    targeting:
      kind: creature
      count: 1
      visible: true
      within_range: true
    save:
      ability: intelligence
      failure: create_phantasm
      success: no_effect
    phantasm:
      choices:
      - object
      - creature
      - phenomenon
      max_cube_side_feet: 10
      perceived_by: target_only
      duration: spell_duration
      stimuli:
      - visual
      - sound
      - temperature
      - other_sensory
      target_believes_real: true
      illogical_interaction: rationalized
    discern:
      actor: target
      economy: study_action
      check:
        ability: intelligence
        skill: investigation
        dc: spell_save_dc
      success:
        realizes_illusion: true
        end_spell: true
    damage:
      requires: phantasm_depicts_dangerous_creature_or_hazard
      timing: each_caster_turn
      optional: true
      target_location: inside_or_within_5_feet_of_phantasm
      dice: 2d8
      type: psychic
      perceived_type: appropriate_to_illusion
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: minute
splat_contracts:
- id: dnd2024:spell_phantasmal_force
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
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - perception
splat_conversion:
  status: specified
  category: spell
  source_sha256: bf057b1127896fcf5ce519ad5054869020f6513d3c6c0d5fc46218c437937992
  source_path: builtin/srd-5.2.1/spell/phantasmal_force.md
  pages:
  - 151
  - 152
  engine_gaps: &id001
  - dnd2024:spell_phantasmal_force
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: dc3dc0727e83172163b28f657573af98e2b6e3c3bbb0cda155f37ed43d4c9196
splat_clauses:
- key: source_rules
  text: '# Phantasmal Force Level 2 Illusion (Bard, Sorcerer, Wizard) Casting Time: Action Range: 60 feet
    Components: V, S, M (a bit of fleece) Duration: Concentration, up to 1 minute You attempt to craft
    an illusion in the mind of a creature you can see within range. The target makes an Intelligence saving
    throw. On a failed save, you create a phantasmal object, creature, or other phenomenon that is no
    larger than a 10-foot Cube and that is perceivable only to the target for the duration. The phantasm
    includes sound, temperature, and other stimuli. The target can take a Study action to examine the
    phantasm with an Intelligence (Investigation) check against your spell save DC. If the check succeeds,
    the target realizes that the phantasm is an illusion, and the spell ends. While affected by the spell,
    the target treats the phantasm as if it were real and rationalizes any illogical outcomes from interacting
    with it. For example, if the target steps through a phantasmal bridge and survives the fall, it believes
    the bridge exists and something else caused it to fall. An affected target can even take damage from
    the illusion if the phantasm represents a dangerous creature or hazard. On each of your turns, such
    a phantasm can deal 2d8 Psychic damage to the target if it is in the phantasm’s area or within 5 feet
    of the phantasm. The target perceives the damage as a type appropriate to the illusion.'
  sha256: e3fd041d4ad884dbf64713e0d6d322d9d579d3dd542dacc0b604073765ee5745
  status: contract_specified
splat_requires_mechanics: *id001
---

# Phantasmal Force

<!-- SRD page 151 -->

Level 2 Illusion (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a bit of fleece)\
Duration: Concentration, up to 1 minute\
You attempt to craft an illusion in the mind of a\
creature you can see within range. The target\
makes an Intelligence saving throw. On a failed\
save, you create a phantasmal object, creature, or\
other phenomenon that is no larger than a 10-foot\
Cube and that is perceivable only to the target for\
the duration. The phantasm includes sound, tem-\
perature, and other stimuli.\
The target can take a Study action to examine the\
phantasm with an Intelligence (Investigation) check\
against your spell save DC. If the check succeeds,\

<!-- SRD page 152 -->

the target realizes that the phantasm is an illusion,\
and the spell ends.\
While affected by the spell, the target treats the\
phantasm as if it were real and rationalizes any il-\
logical outcomes from interacting with it. For exam-\
ple, if the target steps through a phantasmal bridge\
and survives the fall, it believes the bridge exists\
and something else caused it to fall.\
An affected target can even take damage from\
the illusion if the phantasm represents a dangerous\
creature or hazard. On each of your turns, such a\
phantasm can deal 2d8 Psychic damage to the tar-\
get if it is in the phantasm’s area or within 5 feet of\
the phantasm. The target perceives the damage as a\
type appropriate to the illusion.\

Original layout: [[../../srd-5.2.1/pages/151|page 151]], [[../../srd-5.2.1/pages/152|page 152]].
