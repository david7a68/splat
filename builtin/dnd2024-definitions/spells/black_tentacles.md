---
splat_schema: 1
splat_id: 47dc5eea-7370-5391-a655-11e86303c94d
splat_kind: spell
splat_key: dnd2024:spells/black_tentacles
splat_name: Black Tentacles
splat_origin:
  key: dnd2024:reference/spell/black_tentacles
splat_sources:
- title: SRD 5.2.1
  locator: pp. 113
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: conjuration
  classes:
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
    feet: 90
  material:
    description: a tentacle
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/black_tentacles
  implementation: dnd2024:spell_black_tentacles
  parameters:
    area:
      shape: square
      side_feet: 20
      ground: true
      visible: true
      difficult_terrain: true
    triggers:
    - spell.cast_completed
    - area.enter
    - turn.end
    limit:
      saves_per_creature_per_turn: 1
    save:
      ability: strength
      dc:
        binding: save_dc
      failure:
        damage:
          amount:
            dice:
              count: 3
              sides: 6
              add: 0
          type: bludgeoning
        condition: restrained
        condition_duration: spell
      success:
        effect: none
    escape:
      action: action
      ability: strength
      skill: athletics
      dc:
        binding: save_dc
      success: end_restrained_on_self
splat_contracts:
- id: dnd2024:spell_black_tentacles
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    save_dc:
      type: integer
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - area_effect
  - saves
  - damage
  - restrained
  - escape_check
  events:
  - spell.cast_completed
  - turn.end
  - area.enter
  capabilities:
  - spellcasting
  - effects
  - damage
  - saving_throws
  - conditions
  - spatial_areas
  - time
  - checks
splat_conversion:
  status: specified
  category: spell
  source_sha256: b7b1ddb4c5712a2cc84197f2627b7959b75206d5e08d62feb4b888a89c8fbe1f
  source_path: builtin/srd-5.2.1/spell/black_tentacles.md
  pages:
  - 113
  engine_gaps: &id001
  - dnd2024:spell_black_tentacles
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: c7250f4e668a12d458c9e3a3a368681d865dc7ad51e913e4a5dc2628f6e1c769
splat_clauses:
- key: source_rules
  text: '# Black Tentacles Level 4 Conjuration (Wizard) Casting Time: Action Range: 90 feet Components:
    V, S, M (a tentacle) Duration: Concentration, up to 1 minute Squirming, ebony tentacles fill a 20-foot
    square on ground that you can see within range. For the duration, these tentacles turn the ground
    in that area into Difficult Terrain. Each creature in that area makes a Strength saving throw. On
    a failed save, it takes 3d6 Bludgeoning damage, and it has the Restrained condition until the spell
    ends. A creature also makes that save if it enters the area or ends it turn there. A creature makes
    that save only once per turn. A Restrained creature can take an action to make a Strength (Athletics)
    check against your spell save DC, ending the condition on itself on a success.'
  sha256: e164b6af28be2e9fda2f5aca4da8226bc8f3879edbf9f5cc179ed90a934b26ef
  status: contract_specified
splat_requires_mechanics: *id001
---

# Black Tentacles

<!-- SRD page 113 -->

Level 4 Conjuration (Wizard)\
Casting Time: Action\
Range: 90 feet\
Components: V, S, M (a tentacle)\
Duration: Concentration, up to 1 minute\
Squirming, ebony tentacles fill a 20-foot square on\
ground that you can see within range. For the du-\
ration, these tentacles turn the ground in that area\
into Difficult Terrain.\
Each creature in that area makes a Strength sav-\
ing throw. On a failed save, it takes 3d6 Bludgeoning\
damage, and it has the Restrained condition until\
the spell ends. A creature also makes that save if\
it enters the area or ends it turn there. A creature\
makes that save only once per turn.\
A Restrained creature can take an action to make\
a Strength (Athletics) check against your spell save\
DC, ending the condition on itself on a success.\

Original layout: [[../../srd-5.2.1/pages/113|page 113]].
