---
splat_schema: 1
splat_id: c8f436ab-e351-503b-a543-fafe5e134c0a
splat_kind: spell
splat_key: dnd2024:spells/fly
splat_name: Fly
splat_origin:
  key: dnd2024:reference/spell/fly
splat_sources:
- title: SRD 5.2.1
  locator: pp. 133
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: transmutation
  classes:
  - sorcerer
  - warlock
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
    mode: touch
  material:
    description: a feather
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/fly
  implementation: dnd2024:spell_fly
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 600
    targets:
      kind: creature
      willing: true
      touch: true
      count:
        base: 1
        increment_per_slot_above: 1
        base_slot: 3
    grant:
      fly_speed_feet: 60
      hover: true
    on_end:
      if: target_aloft
      effect: fall
      exception: target_can_stop_fall
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_fly
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
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  events:
  - concentration.ended
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - movement
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: b2e91226632083570b155f07f4a142b00d20e607a80596d5d026e1e614fdb908
  source_path: builtin/srd-5.2.1/spell/fly.md
  pages:
  - 133
  engine_gaps: &id001
  - dnd2024:spell_fly
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 25add5d0114935d1376cd07c1d9525f4ca1544020756b23b064df77ba946c9fe
splat_clauses:
- key: source_rules
  text: '# Fly Level 3 Transmutation (Sorcerer, Warlock, Wizard) Casting Time: Action Range: Touch Components:
    V, S, M (a feather) Duration: Concentration, up to 10 minutes You touch a willing creature. For the
    duration, the target gains a Fly Speed of 60 feet and can hover. When the spell ends, the target falls
    if it is still aloft unless it can stop the fall. Using a Higher-Level Spell Slot. You can target
    one additional creature for each spell slot level above 3.'
  sha256: 51adef45d75ccdfaa62e0982aea61ad0edfee10bbc82250d4ddf9e1c01ecae4b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Fly

<!-- SRD page 133 -->

Level 3 Transmutation (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a feather)\
Duration: Concentration, up to 10 minutes\
You touch a willing creature. For the duration, the\
target gains a Fly Speed of 60 feet and can hover.\
When the spell ends, the target falls if it is still aloft\
unless it can stop the fall.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 3.\

Original layout: [[../../srd-5.2.1/pages/133|page 133]].
