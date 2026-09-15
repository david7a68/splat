---
splat_schema: 1
splat_id: 5b60481a-9528-52d3-ab33-74a1060eca47
splat_kind: spell
splat_key: dnd2024:spells/ensnaring_strike
splat_name: Ensnaring Strike
splat_origin:
  key: dnd2024:reference/spell/ensnaring_strike
splat_sources:
- title: SRD 5.2.1
  locator: pp. 128
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: conjuration
  classes:
  - ranger
  components:
  - verbal
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Bonus Action, which you take immediately after hitting a creature with a weapon
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/ensnaring_strike
  implementation: dnd2024:spell_ensnaring_strike
  parameters:
    targeting:
      kind: creature_hit_by_triggering_weapon_attack
    save:
      ability: strength
      advantage_if_size_at_least: large
      failure:
        condition: restrained
        until: spell_ends
      success:
        end_spell: true
    restrained_damage:
      event: target.turn_started
      damage:
        dice: 1d6
        type: piercing
    escape:
      actor: target_or_creature_within_reach
      economy: action
      check:
        ability: strength
        skill: athletics
        dc: spell_save_dc
      success: end_spell
    slot_scaling:
      above: 1
      damage_per_level: 1d6
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: minute
splat_contracts:
- id: dnd2024:spell_ensnaring_strike
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
    save_dc:
      type: integer
      required: true
  outputs:
  - effects
  - damage_results
  events:
  - spell.cast_completed
  - turn.started
  - attack.hit
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - conditions
splat_conversion:
  status: specified
  category: spell
  source_sha256: 413a39e46a9f20b4dd9d8bdbf083e847127697d833e47a51e154dac1a5e58f7f
  source_path: builtin/srd-5.2.1/spell/ensnaring_strike.md
  pages:
  - 128
  engine_gaps: &id001
  - dnd2024:spell_ensnaring_strike
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 5527b3d844732d3f596f56b657cc15e42a752045dbc2129e1cacd9411e1bce87
splat_clauses:
- key: source_rules
  text: '# Ensnaring Strike Level 1 Conjuration (Ranger) Casting Time: Bonus Action, which you take immediately
    after hitting a creature with a weapon Range: Self Components: V Duration: Concentration, up to 1
    minute As you hit the target, grasping vines appear on it, and it makes a Strength saving throw. A
    Large or larger creature has Advantage on this save. On a failed save, the target has the Restrained
    condition until the spell ends. On a successful save, the vines shrivel away, and the spell ends.
    While Restrained, the target takes 1d6 Piercing damage at the start of each of its turns. The target
    or a creature within reach of it can take an action to make a Strength (Athletics) check against your
    spell save DC. On a success, the spell ends. Using a Higher-Level Spell Slot. The damage increases
    by 1d6 for each spell slot level above 1.'
  sha256: 7138534775b5b8af89de8e5d6b56e79f0e1d2d7f315fb97730b1d9b976e9745b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ensnaring Strike

<!-- SRD page 128 -->

Level 1 Conjuration (Ranger)\
Casting Time: Bonus Action, which you take immedi-\
ately after hitting a creature with a weapon\
Range: Self\
Components: V\
Duration: Concentration, up to 1 minute\
As you hit the target, grasping vines appear on it,\
and it makes a Strength saving throw. A Large or\
larger creature has Advantage on this save. On a\
failed save, the target has the Restrained condition\
until the spell ends. On a successful save, the vines\
shrivel away, and the spell ends.\
While Restrained, the target takes 1d6 Piercing\
damage at the start of each of its turns. The target\
or a creature within reach of it can take an action to\
make a Strength (Athletics) check against your spell\
save DC. On a success, the spell ends.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d6 for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/128|page 128]].
