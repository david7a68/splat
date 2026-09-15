---
splat_schema: 1
splat_id: e41a0eb6-bbbe-527a-a388-db5af40fa96d
splat_kind: spell
splat_key: dnd2024:spells/entangle
splat_name: Entangle
splat_origin:
  key: dnd2024:reference/spell/entangle
splat_sources:
- title: SRD 5.2.1
  locator: pp. 128
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: conjuration
  classes:
  - druid
  - ranger
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 90
splat_rule:
  mechanic: dnd2024:spells/entangle
  implementation: dnd2024:spell_entangle
  parameters:
    area:
      shape: square
      side_feet: 20
      origin: ground_within_spell_range
      terrain: difficult
      duration: spell_duration
      plants_disappear_on_end: true
    targets:
      kind: creature
      in_area_at_cast: true
      exclude_caster: true
    save:
      ability: strength
      dc:
        binding: save_dc
      failure:
        condition: restrained
        until: spell_ends
      success:
        effect: none
    escape:
      cost: action
      check:
        ability: strength
        skill: athletics
        dc:
          binding: save_dc
      success: end_restrained_from_spell
splat_contracts:
- id: dnd2024:spell_entangle
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
  events:
  - spell.cast_completed
  - turn.end
  - effect.ended
  - movement.completed
  - save.before
  - save.after
  capabilities:
  - spellcasting
  - effects
  - dice
  - spatial_relationships
  - conditions
  - saving_throws
splat_conversion:
  status: specified
  category: spell
  source_sha256: c15b00312429fb26eca412b0579e05df2400caa4a4ece5e2003d81080f0f2710
  source_path: builtin/srd-5.2.1/spell/entangle.md
  pages:
  - 128
  engine_gaps: &id001
  - dnd2024:spell_entangle
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 0892ff3845df5328433c1c084642dfc5076a87091c7eda4470f61581247559f0
splat_clauses:
- key: source_rules
  text: '# Entangle Level 1 Conjuration (Druid, Ranger) Casting Time: Action Range: 90 feet Components:
    V, S Duration: Concentration, up to 1 minute Grasping plants sprout from the ground in a 20-foot square
    within range. For the duration, these plants turn the ground in the area into Difficult Terrain. They
    disappear when the spell ends. Each creature (other than you) in the area when you cast the spell
    must succeed on a Strength saving throw or have the Restrained condition until the spell ends. A Restrained
    creature can take an action to make a Strength (Athletics) check against your spell save DC. On a
    success, it frees itself from the grasping plants and is no longer Restrained by them.'
  sha256: e4c8326ea85a6cf68220ca0c773097d244f43d61e5c2127ab0d4a62654eac187
  status: contract_specified
splat_requires_mechanics: *id001
---

# Entangle

<!-- SRD page 128 -->

Level 1 Conjuration (Druid, Ranger)\
Casting Time: Action\
Range: 90 feet\
Components: V, S\
Duration: Concentration, up to 1 minute\
Grasping plants sprout from the ground in a 20-foot\
square within range. For the duration, these plants\
turn the ground in the area into Difficult Terrain.\
They disappear when the spell ends.\
Each creature (other than you) in the area when\
you cast the spell must succeed on a Strength\
saving throw or have the Restrained condition until\
the spell ends. A Restrained creature can take an\
action to make a Strength (Athletics) check against\
your spell save DC. On a success, it frees itself from\
the grasping plants and is no longer Restrained by\
them.\

Original layout: [[../../srd-5.2.1/pages/128|page 128]].
