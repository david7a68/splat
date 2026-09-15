---
splat_schema: 1
splat_id: 43c6436f-6610-5460-925c-0479374081d0
splat_kind: spell
splat_key: dnd2024:spells/acid_arrow
splat_name: Acid Arrow
splat_origin:
  key: dnd2024:reference/spell/acid_arrow
splat_sources:
- title: SRD 5.2.1
  locator: pp. 107
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: evocation
  classes:
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 90
  material:
    description: powdered rhubarb leaf
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/acid_arrow
  implementation: dnd2024:spell_acid_arrow
  parameters:
    target:
      kind: creature_or_object
      count: 1
      visible_to_caster: false
    attack:
      kind: ranged_spell
      hit:
        initial:
          amount:
            dice:
              count: 4
              sides: 4
              add: 0
          type: acid
        later:
          event: turn.end
          subject: target
          turn: next
          damage:
            amount:
              dice:
                count: 2
                sides: 4
                add: 0
            type: acid
      miss:
        damage:
          amount:
            dice:
              count: 4
              sides: 4
              add: 0
          type: acid
        scale:
          numerator: 1
          denominator: 2
        round: down
        later_damage: false
    scaling:
      per_slot_above: 2
      initial_dice: 1
      later_dice: 1
      sides: 4
splat_contracts:
- id: dnd2024:spell_acid_arrow
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    spell_attack_modifier:
      type: integer
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - attack_result
  - damage
  - delayed_damage_effect
  events:
  - spell.cast_completed
  - turn.end
  capabilities:
  - spellcasting
  - effects
  - damage
  - attacks
  - inventory
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: 55f5106415a010d6c50477acdbf7cdf6d76edb326bb4f226fcd831485a24be35
  source_path: builtin/srd-5.2.1/spell/acid_arrow.md
  pages:
  - 107
  engine_gaps: &id001
  - dnd2024:spell_acid_arrow
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 2a2f0cdc3f82287a5d0b5f5511e5b737d47fc0f9f2f191089af45e42cfea2919
splat_clauses:
- key: source_rules
  text: '# Acid Arrow Level 2 Evocation (Wizard) Casting Time: Action Range: 90 feet Components: V, S,
    M (powdered rhubarb leaf) Duration: Instantaneous A shimmering green arrow streaks toward a target
    within range and bursts in a spray of acid. Make a ranged spell attack against the target. On a hit,
    the target takes 4d4 Acid damage and 2d4 Acid damage at the end of its next turn. On a miss, the arrow
    splashes the target with acid for half as much of the initial damage only. Using a Higher-Level Spell
    Slot. The damage (both initial and later) increases by 1d4 for each spell slot level above 2.'
  sha256: 9bee13ce6969d9ef62bfa27fdb3eb0d69702317d5e7c08329f82b169fd99be83
  status: contract_specified
splat_requires_mechanics: *id001
---

# Acid Arrow

<!-- SRD page 107 -->

Level 2 Evocation (Wizard)\
Casting Time: Action\
Range: 90 feet\
Components: V, S, M (powdered rhubarb leaf)\
Duration: Instantaneous\
A shimmering green arrow streaks toward a target\
within range and bursts in a spray of acid. Make a\
ranged spell attack against the target. On a hit, the\
target takes 4d4 Acid damage and 2d4 Acid dam-\
age at the end of its next turn. On a miss, the arrow\
splashes the target with acid for half as much of the\
initial damage only.\
Using a Higher-Level Spell Slot. The damage\
(both initial and later) increases by 1d4 for each\
spell slot level above 2.\

Original layout: [[../../srd-5.2.1/pages/107|page 107]].
