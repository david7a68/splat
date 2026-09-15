---
splat_schema: 1
splat_id: c73aa5d8-490d-50b8-b117-d86eccae5c96
splat_kind: spell
splat_key: dnd2024:spells/sunbeam
splat_name: Sunbeam
splat_origin:
  key: dnd2024:reference/spell/sunbeam
splat_sources:
- title: SRD 5.2.1
  locator: pp. 166
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: evocation
  classes:
  - cleric
  - druid
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
    mode: self
  material:
    description: a magnifying glass
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/sunbeam
  implementation: dnd2024:spell_sunbeam
  parameters:
    targeting:
      kind: creatures_in_area
      shape: line
      width_feet: 5
      length_feet: 60
      origin: caster
    save:
      ability: constitution
      failure:
        damage:
          dice: 6d8
          type: radiant
        condition: blinded
        until: caster_next_turn_start
      success:
        damage_fraction:
          numerator: 1
          denominator: 2
    repeat:
      economy: magic_action
      new_line: true
      duration: spell_duration
    mote:
      location: above_caster
      bright_radius_feet: 30
      additional_dim_feet: 30
      sunlight: true
      duration: spell_duration
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: minute
splat_contracts:
- id: dnd2024:spell_sunbeam
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
  outputs:
  - effects
  - damage_results
  events:
  - spell.cast_completed
  - turn.started
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
  source_sha256: ce4d665540dac73d8e8450fdb6d9b6d3f3db4bf5c82a01bcbcbfe210e3321022
  source_path: builtin/srd-5.2.1/spell/sunbeam.md
  pages:
  - 166
  engine_gaps: &id001
  - dnd2024:spell_sunbeam
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: f64ce15df7a0ff870bddfdd17d48353189e8bc44c9e1a453b84444a03722fbe1
splat_clauses:
- key: source_rules
  text: '# Sunbeam Level 6 Evocation (Cleric, Druid, Sorcerer, Wizard) Casting Time: Action Range: Self
    Components: V, S, M (a magnifying glass) Duration: Concentration, up to 1 minute You launch a sunbeam
    in a 5-foot-wide, 60-foot-long Line. Each creature in the Line makes a Constitution saving throw.
    On a failed save, a creature takes 6d8 Radiant damage and has the Blinded condition until the start
    of your next turn. On a successful save, it takes half as much damage only. Until the spell ends,
    you can take a Magic action to create a new Line of radiance. For the duration, a mote of brilliant
    radiance shines above you. It sheds Bright Light in a 30-foot radius and Dim Light for an additional
    30 feet. This light is sunlight.'
  sha256: 8266c26a52af5d69aa8999df7d1ecca9ea1d005cb71f1aa36b26dca8e653d2de
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sunbeam

<!-- SRD page 166 -->

Level 6 Evocation (Cleric, Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a magnifying glass)\
Duration: Concentration, up to 1 minute\
You launch a sunbeam in a 5-foot-wide, 60-foot-long\
Line. Each creature in the Line makes a Constitution\
saving throw. On a failed save, a creature takes 6d8\
Radiant damage and has the Blinded condition until\
the start of your next turn. On a successful save, it\
takes half as much damage only.\
Until the spell ends, you can take a Magic action to\
create a new Line of radiance.\
For the duration, a mote of brilliant radiance\
shines above you. It sheds Bright Light in a 30-foot\
radius and Dim Light for an additional 30 feet. This\
light is sunlight.\

Original layout: [[../../srd-5.2.1/pages/166|page 166]].
