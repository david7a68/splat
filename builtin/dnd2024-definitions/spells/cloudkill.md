---
splat_schema: 1
splat_id: d8636a54-0b4a-51bf-b387-2389a6f0741b
splat_kind: spell
splat_key: dnd2024:spells/cloudkill
splat_name: Cloudkill
splat_origin:
  key: dnd2024:reference/spell/cloudkill
splat_sources:
- title: SRD 5.2.1
  locator: pp. 116
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: conjuration
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 120
splat_rule:
  mechanic: dnd2024:spells/cloudkill
  implementation: dnd2024:spell_cloudkill
  parameters:
    area:
      shape: sphere
      radius_feet: 20
      origin: chosen_point_within_range
      obscurement: heavy
      duration: spell_duration
    save:
      ability: constitution
      dc:
        binding: save_dc
      failure:
        damage:
          dice:
            count:
              op: add
              args:
              - 5
              - op: multiply
                args:
                - 1
                - op: subtract
                  args:
                  - binding: cast_level
                  - 5
            sides: 8
            add: 0
          type: poison
      success:
        damage:
          dice:
            count:
              op: add
              args:
              - 5
              - op: multiply
                args:
                - 1
                - op: subtract
                  args:
                  - binding: cast_level
                  - 5
            sides: 8
            add: 0
          type: poison
        damage_fraction:
          numerator: 1
          denominator: 2
    save_triggers:
    - initial_in_area
    - sphere_moves_into_creature_space
    - creature_enters_area
    - creature_turn_ends_in_area
    save_limit:
      per_creature_per_turn: 1
    movement:
      event: caster_turn_start
      feet: 10
      direction: away_from_caster
    end_when:
      strong_wind_disperses_fog: true
splat_contracts:
- id: dnd2024:spell_cloudkill
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
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    save_dc:
      type: integer
      required: true
    choices:
      required: false
      type: choice
  outputs:
  - effects
  - damage
  events:
  - spell.cast_completed
  - turn.start
  - effect.ended
  - movement.completed
  - turn.end
  - save.before
  - save.after
  capabilities:
  - spellcasting
  - effects
  - dice
  - damage
  - spatial_relationships
  - saving_throws
splat_conversion:
  status: specified
  category: spell
  source_sha256: 4b87567ee3c32121d5d0f69f8c28b963cd3204728adc8800f37377046be83a04
  source_path: builtin/srd-5.2.1/spell/cloudkill.md
  pages:
  - 116
  engine_gaps: &id001
  - dnd2024:spell_cloudkill
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 4a34909c42b613c6e9027c4b15faf4c06fe25443b71e099f3be24e98be4ce37f
splat_clauses:
- key: source_rules
  text: '# Cloudkill Level 5 Conjuration (Sorcerer, Wizard) Casting Time: Action Range: 120 feet Components:
    V, S Duration: Concentration, up to 10 minutes You create a 20-foot-radius Sphere of yellow-green
    fog centered on a point within range. The fog lasts for the duration or until strong wind (such as
    the one created by Gust of Wind) disperses it, ending the spell. Its area is Heavily Obscured. Each
    creature in the Sphere makes a Constitution saving throw, taking 5d8 Poison damage on a failed save
    or half as much damage on a successful one. A creature must also make this save when the Sphere moves
    into its space and when it enters the Sphere or ends its turn there. A creature makes this save only
    once per turn. The Sphere moves 10 feet away from you at the start of each of your turns. Using a
    Higher-Level Spell Slot. The damage increases by 1d8 for each spell slot level above 5.'
  sha256: 24cc4d9f8bb38ce9d648b8a2d3c318c1d5ca5ce97720f0ffa676270b1d261ac7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cloudkill

<!-- SRD page 116 -->

Level 5 Conjuration (Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes\
You create a 20-foot-radius Sphere of yellow-green\
fog centered on a point within range. The fog lasts\
for the duration or until strong wind (such as the\
one created by Gust of Wind) disperses it, ending the\
spell. Its area is Heavily Obscured.\
Each creature in the Sphere makes a Constitution\
saving throw, taking 5d8 Poison damage on a failed\
save or half as much damage on a successful one. A\
creature must also make this save when the Sphere\
moves into its space and when it enters the Sphere\
or ends its turn there. A creature makes this save\
only once per turn.\
The Sphere moves 10 feet away from you at the\
start of each of your turns.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for each spell slot level above 5.\

Original layout: [[../../srd-5.2.1/pages/116|page 116]].
