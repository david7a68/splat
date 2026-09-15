---
splat_schema: 1
splat_id: 5df3db72-49ee-5d1c-aa5e-51e6b7e2496c
splat_kind: spell
splat_key: dnd2024:spells/incendiary_cloud
splat_name: Incendiary Cloud
splat_origin:
  key: dnd2024:reference/spell/incendiary_cloud
splat_sources:
- title: SRD 5.2.1
  locator: pp. 142, 143
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 8
  school: conjuration
  classes:
  - druid
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 150
splat_rule:
  mechanic: dnd2024:spells/incendiary_cloud
  implementation: dnd2024:spell_incendiary_cloud
  parameters:
    targeting:
      kind: creatures_in_area
      shape: sphere
      radius_feet: 20
      origin: chosen_point_in_range
    cloud:
      heavily_obscured: true
      until_any:
      - spell_duration
      - strong_wind_dispersal
    damage_triggers:
      events:
      - cloud_appears
      - cloud_moves_into_creature_space
      - creature_enters_cloud
      - creature_ends_turn_in_cloud
      max_per_creature_per_turn: 1
    save:
      ability: dexterity
      failure:
        damage:
          dice: 10d8
          type: fire
      success:
        damage_fraction:
          numerator: 1
          denominator: 2
    movement:
      event: caster.turn_started
      distance_feet: 10
      direction: caster_choice_away_from_caster
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: minute
splat_contracts:
- id: dnd2024:spell_incendiary_cloud
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
  - turn.started
  - turn.ended
  - area.entered
  - position.changed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - spatial_effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: 9ed9b6d3ed1b2fe1fc63b41dcdf6073af8d6242c929e7f04bc40ee0424e16f2c
  source_path: builtin/srd-5.2.1/spell/incendiary_cloud.md
  pages:
  - 142
  - 143
  engine_gaps: &id001
  - dnd2024:spell_incendiary_cloud
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 9369960b3b78cb2d4dba9d45bce64a67f86c5aa322943e86c619a8492ffb3df7
splat_clauses:
- key: source_rules
  text: '# Incendiary Cloud Level 8 Conjuration (Druid, Sorcerer, Wizard) Casting Time: Action Range:
    150 feet Components: V, S Duration: Concentration, up to 1 minute A swirling cloud of embers and smoke
    fills a 20-foot-radius Sphere centered on a point within range. The cloud’s area is Heavily Obscured.
    It lasts for the duration or until a strong wind (like that created by Gust of Wind) disperses it.
    When the cloud appears, each creature in it makes a Dexterity saving throw, taking 10d8 Fire damage
    on a failed save or half as much damage on a successful one. A creature must also make this save when
    the Sphere moves into its space and when it enters the Sphere or ends its turn there. A creature makes
    this save only once per turn. The cloud moves 10 feet away from you in a direction you choose at the
    start of each of your turns.'
  sha256: df5c3918b8b36cafceab71d9441e35cb1ee7956d7bcafe45dc8794261cda77d5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Incendiary Cloud

<!-- SRD page 142 -->

Level 8 Conjuration (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 150 feet\
Components: V, S\
Duration: Concentration, up to 1 minute\
A swirling cloud of embers and smoke fills a\
20-foot-radius Sphere centered on a point within\
range. The cloud’s area is Heavily Obscured. It lasts\
for the duration or until a strong wind (like that\
created by Gust of Wind) disperses it.\
When the cloud appears, each creature in it makes\
a Dexterity saving throw, taking 10d8 Fire damage\
on a failed save or half as much damage on a suc-\
cessful one. A creature must also make this save\
when the Sphere moves into its space and when it\
enters the Sphere or ends its turn there. A creature\
makes this save only once per turn.\

<!-- SRD page 143 -->

The cloud moves 10 feet away from you in a direc-\
tion you choose at the start of each of your turns.\

Original layout: [[../../srd-5.2.1/pages/142|page 142]], [[../../srd-5.2.1/pages/143|page 143]].
