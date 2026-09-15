---
splat_schema: 1
splat_id: d562d78e-a442-533a-af52-5725303f1251
splat_kind: spell
splat_key: dnd2024:spells/jump
splat_name: Jump
splat_origin:
  key: dnd2024:reference/spell/jump
splat_sources:
- title: SRD 5.2.1
  locator: pp. 143
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: transmutation
  classes:
  - druid
  - ranger
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 minute
  casting_time: Bonus Action
  ritual: false
  range:
    mode: touch
  material:
    description: a grasshopper’s hind leg
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/jump
  implementation: dnd2024:spell_jump
  parameters:
    spell_lifetime:
      concentration: false
      maximum_seconds: 60
    targets:
      kind: creature
      willing: true
      touch: true
      count:
        base: 1
        increment_per_slot_above: 1
        base_slot: 1
    jump_grant:
      timing: target_turn
      count_per_turn: 1
      maximum_distance_feet: 30
      movement_cost_feet: 10
      optional: true
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_jump
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
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - movement
splat_conversion:
  status: specified
  category: spell
  source_sha256: 4f2ae176e2c225e47a5ab9d1868cf7b7d7b48af3ecfa5a0200a20642b6462052
  source_path: builtin/srd-5.2.1/spell/jump.md
  pages:
  - 143
  engine_gaps: &id001
  - dnd2024:spell_jump
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: aa0d339dae82ce0d9d579ca1813ae3a4dba98cd3efc80136ee28c8eda229e70a
splat_clauses:
- key: source_rules
  text: '# Jump Level 1 Transmutation (Druid, Ranger, Sorcerer, Wizard) Casting Time: Bonus Action Range:
    Touch Component: V, S, M (a grasshopper’s hind leg) Duration: 1 minute You touch a willing creature.
    Once on each of its turns until the spell ends, that creature can jump up to 30 feet by spending 10
    feet of movement. Using a Higher-Level Spell Slot. You can target one additional creature for each
    spell slot level above 1.'
  sha256: f6f40a3b5c56fc323e1d9bed8a0fc6d3cd482477199b05c37e6351dcdabce191
  status: contract_specified
splat_requires_mechanics: *id001
---

# Jump

<!-- SRD page 143 -->

Level 1 Transmutation (Druid, Ranger, Sorcerer,\
Wizard)\
Casting Time: Bonus Action\
Range: Touch\
Component: V, S, M (a grasshopper’s hind leg)\
Duration: 1 minute\
You touch a willing creature. Once on each of its\
turns until the spell ends, that creature can jump up\
to 30 feet by spending 10 feet of movement.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 1.\

Original layout: [[../../srd-5.2.1/pages/143|page 143]].
