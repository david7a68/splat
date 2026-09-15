---
splat_schema: 1
splat_id: b4589ffa-998a-52df-bd77-d3db52107ad2
splat_kind: spell
splat_key: dnd2024:spells/stinking_cloud
splat_name: Stinking Cloud
splat_origin:
  key: dnd2024:reference/spell/stinking_cloud
splat_sources:
- title: SRD 5.2.1
  locator: pp. 165
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: conjuration
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
    feet: 90
  material:
    description: a rotten egg
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/stinking_cloud
  implementation: dnd2024:spell_stinking_cloud
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 60
    area:
      shape: sphere
      radius_feet: 20
      origin: chosen_point
      range_feet: 90
    properties:
      obscurement: heavy
      medium: gas
    end_early:
      cause: strong_wind_dispersal
      example_spell:
        key: dnd2024:spells/gust_of_wind
    trigger:
      event: turn.start
      subjects: creatures_in_area
    save:
      ability: constitution
      dc: spell_save_dc
      success: unaffected
    failure:
      condition: poisoned
      until:
        event: turn.end
        subject: target
        occurrence: current
      while_poisoned:
        action: forbidden
        bonus_action: forbidden
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_stinking_cloud
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
  events:
  - concentration.ended
  - spell.cast_completed
  - turn.end
  - turn.start
  capabilities:
  - spellcasting
  - effects
  - areas
  - conditions
  - tests
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: 0afef82a092e679b92ee7f9d67244da700b814b41757e7b8b6ecb1cced306506
  source_path: builtin/srd-5.2.1/spell/stinking_cloud.md
  pages:
  - 165
  engine_gaps: &id001
  - dnd2024:spell_stinking_cloud
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 2cbe757485580040953626fd89361cbdc48416b9d69b603457c86c7a2dcdbf68
splat_clauses:
- key: source_rules
  text: '# Stinking Cloud Level 3 Conjuration (Bard, Sorcerer, Wizard) Casting Time: Action Range: 90
    feet Components: V, S, M (a rotten egg) Duration: Concentration, up to 1 minute You create a 20-foot-radius
    Sphere of yellow, nauseating gas centered on a point within range. The cloud is Heavily Obscured.
    The cloud lingers in the air for the duration or until a strong wind (such as the one created by Gust
    of Wind) disperses it. Each creature that starts its turn in the Sphere must succeed on a Constitution
    saving throw or have the Poisoned condition until the end of the current turn. While Poisoned in this
    way, the creature can’t take an action or a Bonus Action.'
  sha256: 00b01371390d5ea2c5fe597a9309101b1eb22ed90bd748a0d80e1f042af829e4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Stinking Cloud

<!-- SRD page 165 -->

Level 3 Conjuration (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 90 feet\
Components: V, S, M (a rotten egg)\
Duration: Concentration, up to 1 minute\
You create a 20-foot-radius Sphere of yellow, nau-\
seating gas centered on a point within range. The\
cloud is Heavily Obscured. The cloud lingers in the\
air for the duration or until a strong wind (such as\
the one created by Gust of Wind) disperses it.\
Each creature that starts its turn in the Sphere\
must succeed on a Constitution saving throw or\
have the Poisoned condition until the end of the cur-\
rent turn. While Poisoned in this way, the creature\
can’t take an action or a Bonus Action.\

Original layout: [[../../srd-5.2.1/pages/165|page 165]].
