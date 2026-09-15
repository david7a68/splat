---
splat_schema: 1
splat_id: 85c3e405-aefe-523e-acdf-0c2ac7c8ec52
splat_kind: spell
splat_key: dnd2024:spells/vitriolic_sphere
splat_name: Vitriolic Sphere
splat_origin:
  key: dnd2024:reference/spell/vitriolic_sphere
splat_sources:
- title: SRD 5.2.1
  locator: pp. 172
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: evocation
  classes:
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
    feet: 150
  material:
    description: a drop of bile
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/vitriolic_sphere
  implementation: dnd2024:spell_vitriolic_sphere
  parameters:
    spell_lifetime:
      concentration: false
      instantaneous: true
    area:
      shape: sphere
      radius_feet: 20
      origin: chosen_location
      range_feet: 150
      projectile_diameter_feet: 1
    targets:
      kind: creature
      selection: all_in_area
    save:
      ability: dexterity
      dc: spell_save_dc
    failure:
      initial_damage:
        dice:
          base_count: 10
          sides: 4
          extra_per_slot: 2
          base_slot: 4
        type: acid
      delayed_damage:
        event: turn.end
        subject: target
        occurrence: next
        dice: 5d4
        type: acid
    success:
      initial_damage_scale:
        numerator: 1
        denominator: 2
      delayed_damage: false
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_vitriolic_sphere
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    origin:
      type: location
      required: true
    targets:
      type: participant_list
      required: true
    save_dc:
      type: integer
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  - damage
  events:
  - spell.cast_completed
  - turn.end
  capabilities:
  - spellcasting
  - effects
  - areas
  - damage
  - tests
splat_conversion:
  status: specified
  category: spell
  source_sha256: 429aebb2e8cb388e424ea25b5d07dc2966ab32cc0a64ee4e71e975139b7b16ea
  source_path: builtin/srd-5.2.1/spell/vitriolic_sphere.md
  pages:
  - 172
  engine_gaps: &id001
  - dnd2024:spell_vitriolic_sphere
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: f8f10ab26b6f2b437038daad328e74be7579266a6f71f5a58d21192b0788b774
splat_clauses:
- key: source_rules
  text: '# Vitriolic Sphere Level 4 Evocation (Sorcerer, Wizard) Casting Time: Action Range: 150 feet
    Components: V, S, M (a drop of bile) Duration: Instantaneous You point at a location within range,
    and a glowing, 1-foot-diameter ball of acid streaks there and explodes in a 20-foot-radius Sphere.
    Each creature in that area makes a Dexterity saving throw. On a failed save, a creature takes 10d4
    Acid damage and another 5d4 Acid damage at the end of its next turn. On a successful save, a creature
    takes half the initial damage only. Using a Higher-Level Spell Slot. The initial damage increases
    by 2d4 for each spell slot level above 4.'
  sha256: 3378e67d9b79b5667b475cebf97436c8f109a1904798a0791e762eb2d302c11b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Vitriolic Sphere

<!-- SRD page 172 -->

Level 4 Evocation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 150 feet\
Components: V, S, M (a drop of bile)\
Duration: Instantaneous\
You point at a location within range, and a glow-\
ing, 1-foot-diameter ball of acid streaks there and\
explodes in a 20-foot-radius Sphere. Each creature\
in that area makes a Dexterity saving throw. On a\
failed save, a creature takes 10d4 Acid damage and\
another 5d4 Acid damage at the end of its next turn.\
On a successful save, a creature takes half the initial\
damage only.\
Using a Higher-Level Spell Slot. The initial dam-\
age increases by 2d4 for each spell slot level above\
4.\

Original layout: [[../../srd-5.2.1/pages/172|page 172]].
