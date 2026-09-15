---
splat_schema: 1
splat_id: bdbdaf92-8dfd-5bc6-8d11-10d4946d054d
splat_kind: spell
splat_key: dnd2024:spells/storm_of_vengeance
splat_name: Storm of Vengeance
splat_origin:
  key: dnd2024:reference/spell/storm_of_vengeance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 165, 166
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 9
  school: conjuration
  classes:
  - druid
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    mode: 1 mile
splat_rule:
  mechanic: dnd2024:spells/storm_of_vengeance
  implementation: dnd2024:spell_storm_of_vengeance
  parameters:
    cloud:
      radius_feet: 300
      center: chosen_point_within_range
      duration: spell_duration
    initial:
      targets: creatures_under_cloud_at_appearance
      save:
        ability: constitution
        dc:
          binding: save_dc
        failure:
          damage:
            dice:
              count: 2
              sides: 6
              add: 0
            type: thunder
          condition: deafened
          duration: spell_duration
        success:
          effect: none
    later_effects:
      event: caster_turn_start
      turns:
      - number: 2
        targets: each_creature_and_object_under_cloud
        damage:
          dice:
            count: 4
            sides: 6
            add: 0
          type: acid
      - number: 3
        targets:
          count: 6
          different: true
          kinds:
          - creature
          - object
          under_cloud: true
          chooser: caster
        save:
          ability: dexterity
          dc:
            binding: save_dc
          failure:
            damage:
              dice:
                count: 10
                sides: 6
                add: 0
              type: lightning
          success:
            damage:
              dice:
                count: 10
                sides: 6
                add: 0
              type: lightning
            damage_fraction:
              numerator: 1
              denominator: 2
      - number: 4
        targets: each_creature_under_cloud
        damage:
          dice:
            count: 2
            sides: 6
            add: 0
          type: bludgeoning
      - numbers:
        - 5
        - 6
        - 7
        - 8
        - 9
        - 10
        targets: each_creature_under_cloud
        damage:
          dice:
            count: 1
            sides: 6
            add: 0
          type: cold
        area_until_spell_ends:
          terrain: difficult
          obscurement: heavy
          ranged_weapon_attacks: impossible
          wind: strong
splat_contracts:
- id: dnd2024:spell_storm_of_vengeance
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
  - turn.start
  - movement.completed
  - save.before
  - save.after
  capabilities:
  - spellcasting
  - effects
  - dice
  - damage
  - spatial_relationships
  - conditions
  - saving_throws
splat_conversion:
  status: specified
  category: spell
  source_sha256: c1ad40bb6092c8df4caeedced610adbb2ec30c9fc1cccbe4e8c98af11194bb92
  source_path: builtin/srd-5.2.1/spell/storm_of_vengeance.md
  pages:
  - 165
  - 166
  engine_gaps: &id001
  - dnd2024:spell_storm_of_vengeance
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 7c8191846cccd1336e0418ee3507782ea4df242f7cffc31f7653a1be33176ea6
splat_clauses:
- key: source_rules
  text: '# Storm of Vengeance Level 9 Conjuration (Druid) Casting Time: Action Range: 1 mile Components:
    V, S Duration: Concentration, up to 1 minute A churning storm cloud forms for the duration, centered
    on a point within range and spreading to a radius of 300 feet. Each creature under the cloud when
    it appears must succeed on a Constitution saving throw or take 2d6 Thunder damage and have the Deafened
    condition for the duration. At the start of each of your later turns, the storm produces different
    effects, as detailed below. Turn 2. Acidic rain falls. Each creature and object under the cloud takes
    4d6 Acid damage. Turn 3. You call six bolts of lightning from the cloud to strike six different creatures
    or objects beneath it. Each target makes a Dexterity saving throw, taking 10d6 Lightning damage on
    a failed save or half as much damage on a successful one. Turn 4. Hailstones rain down. Each creature
    under the cloud takes 2d6 Bludgeoning damage. Turns 5–10. Gusts and freezing rain assail the area
    under the cloud. Each creature there takes 1d6 Cold damage. Until the spell ends, the area is Difficult
    Terrain and Heavily Obscured, ranged attacks with weapons are impossible there, and strong wind blows
    through the area.'
  sha256: bd9f467ea06e98e624d744a880b673e39cfd02a96c657a08a6532563df9d7c12
  status: contract_specified
splat_requires_mechanics: *id001
---

# Storm of Vengeance

<!-- SRD page 165 -->

Level 9 Conjuration (Druid)\
Casting Time: Action\
Range: 1 mile\
Components: V, S\
Duration: Concentration, up to 1 minute\
A churning storm cloud forms for the duration,\
centered on a point within range and spreading to\
a radius of 300 feet. Each creature under the cloud\
when it appears must succeed on a Constitution\
saving throw or take 2d6 Thunder damage and have\
the Deafened condition for the duration.\
At the start of each of your later turns, the storm\
produces different effects, as detailed below.\
Turn 2. Acidic rain falls. Each creature and object\
under the cloud takes 4d6 Acid damage.\
Turn 3. You call six bolts of lightning from the\
cloud to strike six different creatures or objects\
beneath it. Each target makes a Dexterity saving\
throw, taking 10d6 Lightning damage on a failed\
save or half as much damage on a successful one.\

<!-- SRD page 166 -->

Turn 4. Hailstones rain down. Each creature un-\
der the cloud takes 2d6 Bludgeoning damage.\
Turns 5–10. Gusts and freezing rain assail the\
area under the cloud. Each creature there takes 1d6\
Cold damage. Until the spell ends, the area is Diffi-\
cult Terrain and Heavily Obscured, ranged attacks\
with weapons are impossible there, and strong\
wind blows through the area.\

Original layout: [[../../srd-5.2.1/pages/165|page 165]], [[../../srd-5.2.1/pages/166|page 166]].
