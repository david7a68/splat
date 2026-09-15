---
splat_schema: 1
splat_id: 97691d2b-03ba-59cb-a949-ec1ff72defd0
splat_kind: spell
splat_key: dnd2024:spells/blade_barrier
splat_name: Blade Barrier
splat_origin:
  key: dnd2024:reference/spell/blade_barrier
splat_sources:
- title: SRD 5.2.1
  locator: pp. 113
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: evocation
  classes:
  - cleric
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 90
splat_rule:
  mechanic: dnd2024:spells/blade_barrier
  implementation: dnd2024:spell_blade_barrier
  parameters:
    targeting:
      kind: area
      within_range: true
    wall:
      choices:
      - shape: straight
        max_length_feet: 100
      - shape: ring
        max_diameter_feet: 60
      height_feet: 20
      thickness_feet: 5
      cover: three_quarters
      difficult_terrain: true
      duration: spell_duration
    damage_triggers:
      events:
      - spell.cast_completed
      - area.entered
      - turn.ended
      scope: creature_in_wall
      max_saves_per_turn: 1
    save:
      ability: dexterity
      failure:
        damage:
          dice: 6d10
          type: force
      success:
        damage_fraction:
          numerator: 1
          denominator: 2
    spell_lifetime:
      concentration: true
      maximum:
        value: 10
        unit: minute
splat_contracts:
- id: dnd2024:spell_blade_barrier
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
  - turn.ended
  - area.entered
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - spatial_effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: 257aa4a00d7dc66432372faab6100af3a32aaabda84b25c2b76b3cc897d0868a
  source_path: builtin/srd-5.2.1/spell/blade_barrier.md
  pages:
  - 113
  engine_gaps: &id001
  - dnd2024:spell_blade_barrier
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 47e96dfcdff9ffdc63297bc05396e942d543976fa40e9fda7af29bc4aac6ea13
splat_clauses:
- key: source_rules
  text: '# Blade Barrier Level 6 Evocation (Cleric) Casting Time: Action Range: 90 feet Components: V,
    S Duration: Concentration, up to 10 minutes You create a wall of whirling blades made of magical energy.
    The wall appears within range and lasts for the duration. You make a straight wall up to 100 feet
    long, 20 feet high, and 5 feet thick, or a ringed wall up to 60 feet in diameter, 20 feet high, and
    5 feet thick. The wall provides Three-Quarters Cover, and its space is Difficult Terrain. Any creature
    in the wall’s space makes a Dexterity saving throw, taking 6d10 Force damage on a failed save or half
    as much damage on a successful one. A creature also makes that save if it enters the wall’s space
    or ends it turn there. A creature makes that save only once per turn.'
  sha256: b707b7c4a2fbbe1b2ce9149ee6323a65c1795c75b3bec5bee7b55b42067aaf12
  status: contract_specified
splat_requires_mechanics: *id001
---

# Blade Barrier

<!-- SRD page 113 -->

Level 6 Evocation (Cleric)\
Casting Time: Action\
Range: 90 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes\
You create a wall of whirling blades made of magical\
energy. The wall appears within range and lasts for\
the duration. You make a straight wall up to 100\
feet long, 20 feet high, and 5 feet thick, or a ringed\
wall up to 60 feet in diameter, 20 feet high, and 5\
feet thick. The wall provides Three-Quarters Cover,\
and its space is Difficult Terrain.\
Any creature in the wall’s space makes a Dexter-\
ity saving throw, taking 6d10 Force damage on a\
failed save or half as much damage on a successful\
one. A creature also makes that save if it enters the\
wall’s space or ends it turn there. A creature makes\
that save only once per turn.\

Original layout: [[../../srd-5.2.1/pages/113|page 113]].
