---
splat_schema: 1
splat_id: 4cb693a0-2527-531e-952e-cbbbc19e9261
splat_kind: spell
splat_key: dnd2024:spells/phantasmal_killer
splat_name: Phantasmal Killer
splat_origin:
  key: dnd2024:reference/spell/phantasmal_killer
splat_sources:
- title: SRD 5.2.1
  locator: pp. 152
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: illusion
  classes:
  - bard
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 120
splat_rule:
  mechanic: dnd2024:spells/phantasmal_killer
  implementation: dnd2024:spell_phantasmal_killer
  parameters:
    targets:
      kind: creature
      maximum: 1
      within_spell_range: true
      visible: true
    illusion:
      depiction: target_deepest_fears
      visible_only_to: target
    initial_save:
      ability: wisdom
      dc:
        binding: save_dc
      failure:
        damage:
          dice:
            count:
              op: add
              args:
              - 4
              - op: multiply
                args:
                - 1
                - op: subtract
                  args:
                  - binding: cast_level
                  - 4
            sides: 10
            add: 0
          type: psychic
        disadvantage:
        - ability_checks
        - attack_rolls
        duration: spell_duration
      success:
        damage:
          dice:
            count:
              op: add
              args:
              - 4
              - op: multiply
                args:
                - 1
                - op: subtract
                  args:
                  - binding: cast_level
                  - 4
            sides: 10
            add: 0
          type: psychic
        damage_fraction:
          numerator: 1
          denominator: 2
        end_spell: true
    repeat_save:
      event: target_turn_end
      save:
        ability: wisdom
        dc:
          binding: save_dc
        failure:
          damage:
            dice:
              count:
                op: add
                args:
                - 4
                - op: multiply
                  args:
                  - 1
                  - op: subtract
                    args:
                    - binding: cast_level
                    - 4
              sides: 10
              add: 0
            type: psychic
        success:
          end_spell: true
splat_contracts:
- id: dnd2024:spell_phantasmal_killer
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
  outputs:
  - effects
  - damage
  events:
  - spell.cast_completed
  - turn.end
  - save.before
  - save.after
  capabilities:
  - spellcasting
  - effects
  - dice
  - damage
  - saving_throws
  - roll_modifiers
splat_conversion:
  status: specified
  category: spell
  source_sha256: c973198450941d62f3ed1314592bbd8fce9fb2137712c131daf12273c65595ca
  source_path: builtin/srd-5.2.1/spell/phantasmal_killer.md
  pages:
  - 152
  engine_gaps: &id001
  - dnd2024:spell_phantasmal_killer
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 3c5b1b9d2f2c71229c57929de1bea2112b6b29c37457054bd061d2d680588fcb
splat_clauses:
- key: source_rules
  text: '# Phantasmal Killer Level 4 Illusion (Bard, Wizard) Casting Time: Action Range: 120 feet Components:
    V, S Duration: Concentration, up to 1 minute You tap into the nightmares of a creature you can see
    within range and create an illusion of its deepest fears, visible only to that creature. The target
    makes a Wisdom saving throw. On a failed save, the target takes 4d10 Psychic damage and has Disadvantage
    on ability checks and attack rolls for the duration. On a successful save, the target takes half as
    much damage, and the spell ends. For the duration, the target makes a Wisdom saving throw at the end
    of each of its turns. On a failed save, it takes the Psychic damage again. On a successful save, the
    spell ends. Using a Higher-Level Spell Slot. The damage increases by 1d10 for each spell slot level
    above 4.'
  sha256: fca22be3e3b5abb78932a5ef781f31768394708c43dc939e4513ebeb8a12cd51
  status: contract_specified
splat_requires_mechanics: *id001
---

# Phantasmal Killer

<!-- SRD page 152 -->

Level 4 Illusion (Bard, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Concentration, up to 1 minute\
You tap into the nightmares of a creature you can\
see within range and create an illusion of its deep-\
est fears, visible only to that creature. The target\
makes a Wisdom saving throw. On a failed save, the\
target takes 4d10 Psychic damage and has Disad-\
vantage on ability checks and attack rolls for the\
duration. On a successful save, the target takes half\
as much damage, and the spell ends.\
For the duration, the target makes a Wisdom sav-\
ing throw at the end of each of its turns. On a failed\
save, it takes the Psychic damage again. On a suc-\
cessful save, the spell ends.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d10 for each spell slot level above 4.\

Original layout: [[../../srd-5.2.1/pages/152|page 152]].
