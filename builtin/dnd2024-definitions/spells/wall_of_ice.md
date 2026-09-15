---
splat_schema: 1
splat_id: 70ca2817-b63c-53bb-afea-e4dc74389ef2
splat_kind: spell
splat_key: dnd2024:spells/wall_of_ice
splat_name: Wall of Ice
splat_origin:
  key: dnd2024:reference/spell/wall_of_ice
splat_sources:
- title: SRD 5.2.1
  locator: pp. 172, 173
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: evocation
  classes:
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 120
  material:
    description: a piece of quartz
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/wall_of_ice
  implementation: dnd2024:spell_wall_of_ice
  parameters:
    creation:
      kind: object
      surface: solid
      within_spell_range: true
      thickness_feet: 1
      duration: spell_duration
      shape_choice:
        hemispherical_dome:
          radius_feet_maximum: 10
        globe:
          radius_feet_maximum: 10
        flat_surface:
          panels: 10
          panel_side_feet: 10
          each_panel_contiguous_with_another: true
      ac: 12
      hp_per_10_foot_section: 30
      damage_immunities:
      - cold
      - poison
      - psychic
      damage_vulnerabilities:
      - fire
    initial_intersection:
      target: creature_whose_space_wall_cuts
      push_to_side:
        chooser: caster
      save:
        ability: dexterity
        dc:
          binding: save_dc
        failure:
          damage:
            dice:
              count: &id001
                op: add
                args:
                - 10
                - op: multiply
                  args:
                  - 2
                  - op: subtract
                    args:
                    - binding: cast_level
                    - 6
              sides: 6
              add: 0
            type: cold
        success:
          damage:
            dice:
              count: *id001
              sides: 6
              add: 0
            type: cold
          damage_fraction:
            numerator: 1
            denominator: 2
    section_destroyed:
      hp: 0
      replace_with: sheet_of_frigid_air
      location: former_section_space
    frigid_air:
      trigger: creature_moves_through_first_time_per_turn
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
                    - 6
              sides: 6
              add: 0
            type: cold
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
                    - 6
              sides: 6
              add: 0
            type: cold
          damage_fraction:
            numerator: 1
            denominator: 2
splat_contracts:
- id: dnd2024:spell_wall_of_ice
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    objects:
      required: false
      type: object_list
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
  - movement.completed
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
  source_sha256: 87552626a410aa4e2e9260c5599142c0773d5fa3c4ea5a27526abe0be8e9b74b
  source_path: builtin/srd-5.2.1/spell/wall_of_ice.md
  pages:
  - 172
  - 173
  engine_gaps: &id002
  - dnd2024:spell_wall_of_ice
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 18f66839af8d87bcc54c48b0b4c18d7e9324b55de423a90e91d6bff8bd54f622
splat_clauses:
- key: source_rules
  text: '# Wall of Ice Level 6 Evocation (Wizard) Casting Time: Action Range: 120 feet Components: V,
    S, M (a piece of quartz) Duration: Concentration, up to 10 minutes You create a wall of ice on a solid
    surface within range. You can form it into a hemispherical dome or a globe with a radius of up to
    10 feet, or you can shape a flat surface made up of ten 10-foot-square panels. Each panel must be
    contiguous with another panel. In any form, the wall is 1 foot thick and lasts for the duration. If
    the wall cuts through a creature’s space when it appears, the creature is pushed to one side of the
    wall (you choose which side) and makes a Dexterity saving throw, taking 10d6 Cold damage on a failed
    save or half as much damage on a successful one. The wall is an object that can be damaged and thus
    breached. It has AC 12 and 30 Hit Points per 10-foot section, and it has Immunity to Cold, Poison,
    and Psychic damage and Vulnerability to Fire damage. Reducing a 10-foot section of wall to 0 Hit Points
    destroys it and leaves behind a sheet of frigid air in the space the wall occupied. A creature moving
    through the sheet of frigid air for the first time on a turn makes a Constitution saving throw, taking
    5d6 Cold damage on a failed save or half as much damage on a successful one. Using a Higher-Level
    Spell Slot. The damage the wall deals when it appears increases by 2d6 and the damage from passing
    through the sheet of frigid air increases by 1d6 for each spell slot level above 6.'
  sha256: 1192e7031306774edea124b1e206b11424b956e30976243a85dd9bcc444db1d9
  status: contract_specified
splat_requires_mechanics: *id002
---

# Wall of Ice

<!-- SRD page 172 -->

Level 6 Evocation (Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a piece of quartz)\
Duration: Concentration, up to 10 minutes\
You create a wall of ice on a solid surface within\
range. You can form it into a hemispherical dome\
or a globe with a radius of up to 10 feet, or you can\
shape a flat surface made up of ten 10-foot-square\
panels. Each panel must be contiguous with another\
panel. In any form, the wall is 1 foot thick and lasts\
for the duration.\
If the wall cuts through a creature’s space when\
it appears, the creature is pushed to one side of the\
wall (you choose which side) and makes a Dexterity\
saving throw, taking 10d6 Cold damage on a failed\
save or half as much damage on a successful one.\
The wall is an object that can be damaged and\
thus breached. It has AC 12 and 30 Hit Points per\
10-foot section, and it has Immunity to Cold, Poi-\
son, and Psychic damage and Vulnerability to Fire\
damage. Reducing a 10-foot section of wall to 0 Hit\
Points destroys it and leaves behind a sheet of frigid\
air in the space the wall occupied.\
A creature moving through the sheet of frigid air\
for the first time on a turn makes a Constitution\
saving throw, taking 5d6 Cold damage on a failed\
save or half as much damage on a successful one.\

<!-- SRD page 173 -->

Using a Higher-Level Spell Slot. The damage the\
wall deals when it appears increases by 2d6 and the\
damage from passing through the sheet of frigid air\
increases by 1d6 for each spell slot level above 6.\

Original layout: [[../../srd-5.2.1/pages/172|page 172]], [[../../srd-5.2.1/pages/173|page 173]].
