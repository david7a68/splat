---
splat_schema: 1
splat_id: 2453ff76-7894-510c-a00b-a82913d32f92
splat_kind: spell
splat_key: dnd2024:spells/fire_shield
splat_name: Fire Shield
splat_origin:
  key: dnd2024:reference/spell/fire_shield
splat_sources:
- title: SRD 5.2.1
  locator: pp. 132
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: evocation
  classes:
  - druid
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 10 minutes
  casting_time: Action
  ritual: false
  range:
    mode: self
  material:
    description: a bit of phosphorus or a firefly
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/fire_shield
  implementation: dnd2024:spell_fire_shield
  parameters:
    spell_lifetime:
      concentration: false
      maximum_seconds: 600
    target: caster
    light:
      bright_radius_feet: 10
      additional_dim_feet: 10
    choice:
      chooser: caster
      options:
      - warm
      - chill
    warm:
      resistance: cold
      retaliatory_damage:
        dice: 2d8
        type: fire
    chill:
      resistance: fire
      retaliatory_damage:
        dice: 2d8
        type: cold
    retaliation:
      event: attack.hit
      attack_kind: melee_attack_roll
      target: caster
      attacker_kind: creature
      attacker_within_feet: 5
      damage_recipient: attacker
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_fire_shield
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    shield_choice:
      type: choice
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  - damage
  events:
  - attack.hit
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - areas
  - attacks
  - damage
splat_conversion:
  status: specified
  category: spell
  source_sha256: d0f1310ff40b8a675968b8f4b161c6c48dc115817249864366abdf4bdb46f2b3
  source_path: builtin/srd-5.2.1/spell/fire_shield.md
  pages:
  - 132
  engine_gaps: &id001
  - dnd2024:spell_fire_shield
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: fc0de145790e037fbdbab3310ac154376e9c13fc16961f65d7c7e13477cf9c50
splat_clauses:
- key: source_rules
  text: '# Fire Shield Level 4 Evocation (Druid, Sorcerer, Wizard) Casting Time: Action Range: Self Components:
    V, S, M (a bit of phosphorus or a firefly) Duration: 10 minutes Wispy flames wreathe your body for
    the duration, shedding Bright Light in a 10-foot radius and Dim Light for an additional 10 feet. The
    flames provide you with a warm shield or a chill shield, as you choose. The warm shield grants you
    Resistance to Cold damage, and the chill shield grants you Resistance to Fire damage. In addition,
    whenever a creature within 5 feet of you hits you with a melee attack roll, the shield erupts with
    flame. The attacker takes 2d8 Fire damage from a warm shield or 2d8 Cold damage from a chill shield.'
  sha256: 771f303e13a405508dccd8c6b171d32bbe9f88a7cb7bd4bb2860679963a72990
  status: contract_specified
splat_requires_mechanics: *id001
---

# Fire Shield

<!-- SRD page 132 -->

Level 4 Evocation (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a bit of phosphorus or a firefly)\
Duration: 10 minutes\
Wispy flames wreathe your body for the duration,\
shedding Bright Light in a 10-foot radius and Dim\
Light for an additional 10 feet.\
The flames provide you with a warm shield or a\
chill shield, as you choose. The warm shield grants\
you Resistance to Cold damage, and the chill shield\
grants you Resistance to Fire damage.\
In addition, whenever a creature within 5 feet\
of you hits you with a melee attack roll, the shield\
erupts with flame. The attacker takes 2d8 Fire dam-\
age from a warm shield or 2d8 Cold damage from a\
chill shield.\

Original layout: [[../../srd-5.2.1/pages/132|page 132]].
