---
splat_schema: 1
splat_id: 0e2b61b1-d000-5af1-af7c-b265c8b5025f
splat_kind: spell
splat_key: dnd2024:spells/searing_smite
splat_name: Searing Smite
splat_origin:
  key: dnd2024:reference/spell/searing_smite
splat_sources:
- title: SRD 5.2.1
  locator: pp. 160
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: evocation
  classes:
  - paladin
  components:
  - verbal
  concentration: false
  duration: 1 minute
  casting_time: Bonus Action, which you take immediately after hitting a target with a Melee weapon or
    an Unarmed Strike
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/searing_smite
  implementation: dnd2024:spell_searing_smite
  parameters:
    targets:
      identity: target_hit_by_triggering_attack
    extra_attack_damage:
      dice:
        count:
          op: add
          args:
          - 1
          - op: multiply
            args:
            - 1
            - op: subtract
              args:
              - binding: cast_level
              - 1
        sides: 6
        add: 0
      type: fire
    ongoing:
      duration: spell_duration
      event: target_turn_start
      steps:
      - damage:
          dice:
            count:
              op: add
              args:
              - 1
              - op: multiply
                args:
                - 1
                - op: subtract
                  args:
                  - binding: cast_level
                  - 1
            sides: 6
            add: 0
          type: fire
      - save:
          ability: constitution
          dc:
            binding: save_dc
          failure:
            continue_spell: true
          success:
            end_spell: true
splat_contracts:
- id: dnd2024:spell_searing_smite
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
    choices:
      required: false
      type: choice
    trigger:
      required: false
      type: event
  outputs:
  - effects
  - damage
  events:
  - spell.cast_completed
  - turn.start
  - attack.roll
  - attack.hit
  - save.before
  - save.after
  - spell.cast_pending
  capabilities:
  - spellcasting
  - effects
  - dice
  - damage
  - saving_throws
splat_conversion:
  status: specified
  category: spell
  source_sha256: 4083fb8bda0153cd3d8772b57e81bc766e9c374040d554aa180cbd3e7853b46d
  source_path: builtin/srd-5.2.1/spell/searing_smite.md
  pages:
  - 160
  engine_gaps: &id001
  - dnd2024:spell_searing_smite
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 693e4192479cfc58bf1acfc2f334c572ba46add29d51772436cccb5826a4ed93
splat_clauses:
- key: source_rules
  text: '# Searing Smite Level 1 Evocation (Paladin) Casting Time: Bonus Action, which you take immediately
    after hitting a target with a Melee weapon or an Unarmed Strike Range: Self Component: V Duration:
    1 minute As you hit the target, it takes an extra 1d6 Fire damage from the attack. At the start of
    each of its turns until the spell ends, the target takes 1d6 Fire damage and then makes a Constitution
    saving throw. On a failed save, the spell continues. On a successful save, the spell ends. Using a
    Higher-Level Spell Slot. All the damage increases by 1d6 for each spell slot level above 1.'
  sha256: 9d8fa35e58809b10e805ae3efb9c6de013de6eea3c89e8750e81c4a675109e25
  status: contract_specified
splat_requires_mechanics: *id001
---

# Searing Smite

<!-- SRD page 160 -->

Level 1 Evocation (Paladin)\
Casting Time: Bonus Action, which you take immedi-\
ately after hitting a target with a Melee weapon or an\
Unarmed Strike\
Range: Self\
Component: V\
Duration: 1 minute\
As you hit the target, it takes an extra 1d6 Fire dam-\
age from the attack. At the start of each of its turns\
until the spell ends, the target takes 1d6 Fire dam-\
age and then makes a Constitution saving throw.\
On a failed save, the spell continues. On a successful\
save, the spell ends.\
Using a Higher-Level Spell Slot. All the damage\
increases by 1d6 for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/160|page 160]].
