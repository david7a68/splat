---
splat_schema: 1
splat_id: b10f57ec-1516-5549-8079-74b97a695911
splat_kind: spell
splat_key: dnd2024:spells/insect_plague
splat_name: Insect Plague
splat_origin:
  key: dnd2024:reference/spell/insect_plague
splat_sources:
- title: SRD 5.2.1
  locator: pp. 143
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: conjuration
  classes:
  - cleric
  - druid
  - sorcerer
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 300
  material:
    description: a locust
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/insect_plague
  implementation: dnd2024:spell_insect_plague
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 600
    area:
      shape: sphere
      radius_feet: 20
      origin: chosen_point
      range_feet: 300
      stationary: true
    properties:
      obscurement: light
      difficult_terrain: true
    triggers:
    - event: spell.cast_completed
      subjects: creatures_in_area
    - event: creature.entered_area
      occurrence_per_turn: first
    - event: turn.end
      subjects: creatures_in_area
    save_limit:
      count: 1
      per: turn
      scope: target
    save:
      ability: constitution
      dc: spell_save_dc
    damage:
      dice:
        base_count: 4
        sides: 10
        extra_per_slot: 1
        base_slot: 5
      type: piercing
    success_damage_scale:
      numerator: 1
      denominator: 2
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_insect_plague
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
  - concentration.ended
  - creature.entered_area
  - spell.cast_completed
  - turn.end
  capabilities:
  - spellcasting
  - effects
  - areas
  - damage
  - tests
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: ef9baf0bad9d27467d40edaebfe4ad8623cc3febbd54452279d70f015324050b
  source_path: builtin/srd-5.2.1/spell/insect_plague.md
  pages:
  - 143
  engine_gaps: &id001
  - dnd2024:spell_insect_plague
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 126070318ea40c687874081a74ca80fb2fd5fe9b71104489f084b5c2741f897a
splat_clauses:
- key: source_rules
  text: '# Insect Plague Level 5 Conjuration (Cleric, Druid, Sorcerer) Casting Time: Action Range: 300
    feet Components: V, S, M (a locust) Duration: Concentration, up to 10 minutes Swarming locusts fill
    a 20-foot-radius Sphere centered on a point you choose within range. The Sphere remains for the duration,
    and its area is Lightly Obscured and Difficult Terrain. When the swarm appears, each creature in it
    makes a Constitution saving throw, taking 4d10 Piercing damage on a failed save or half as much damage
    on a successful one. A creature also makes this save when it enters the spell’s area for the first
    time on a turn or ends its turn there. A creature makes this save only once per turn. Using a Higher-Level
    Spell Slot. The damage increases by 1d10 for each spell slot level above 5.'
  sha256: ec996d6da3408e1909a19ca559c1330a834b6b987383848f486e0c874869236f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Insect Plague

<!-- SRD page 143 -->

Level 5 Conjuration (Cleric, Druid, Sorcerer)\
Casting Time: Action\
Range: 300 feet\
Components: V, S, M (a locust)\
Duration: Concentration, up to 10 minutes\
Swarming locusts fill a 20-foot-radius Sphere\
centered on a point you choose within range. The\
Sphere remains for the duration, and its area is\
Lightly Obscured and Difficult Terrain.\
When the swarm appears, each creature in it\
makes a Constitution saving throw, taking 4d10\
Piercing damage on a failed save or half as much\
damage on a successful one. A creature also makes\
this save when it enters the spell’s area for the first\
time on a turn or ends its turn there. A creature\
makes this save only once per turn.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d10 for each spell slot level above 5.\

Original layout: [[../../srd-5.2.1/pages/143|page 143]].
