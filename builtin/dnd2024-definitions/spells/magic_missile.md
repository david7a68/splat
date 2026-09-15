---
splat_schema: 1
splat_id: eba460f4-6bf8-5344-8a11-bff968ccfecc
splat_kind: spell
splat_key: dnd2024:spells/magic_missile
splat_name: Magic Missile
splat_origin:
  key: dnd2024:reference/spell/magic_missile
splat_sources:
- title: SRD 5.2.1
  locator: pp. 146
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: evocation
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 120
splat_rule:
  mechanic: dnd2024:spells/magic_missile
  implementation: dnd2024:spell_magic_missile
  parameters:
    darts:
      count:
        op: add
        args:
        - 3
        - op: multiply
          args:
          - 1
          - op: subtract
            args:
            - binding: cast_level
            - 1
      target_per_dart:
        kind: creature
        maximum: 1
        within_spell_range: true
        visible: true
      allow_same_target_multiple_darts: true
      automatic_strike: true
      simultaneous: true
      damage_per_dart:
        dice:
          count: 1
          sides: 4
          add: 1
        type: force
splat_contracts:
- id: dnd2024:spell_magic_missile
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
    choices:
      required: false
      type: choice
  outputs:
  - effects
  - damage
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - dice
  - damage
splat_conversion:
  status: specified
  category: spell
  source_sha256: e1ecf4abb76e978ddf2d61acdf70bcf22993505a80d9400e835866ca4c30aa73
  source_path: builtin/srd-5.2.1/spell/magic_missile.md
  pages:
  - 146
  engine_gaps: &id001
  - dnd2024:spell_magic_missile
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 952debb5d883af8fa7b12ef57ba80e471c41c18ac75746c11701e3ce26db7c9d
splat_clauses:
- key: source_rules
  text: '# Magic Missile Level 1 Evocation (Sorcerer, Wizard) Casting Time: Action Range: 120 feet Components:
    V, S Duration: Instantaneous You create three glowing darts of magical force. Each dart strikes a
    creature of your choice that you can see within range. A dart deals 1d4 + 1 Force damage to its target.
    The darts all strike simultaneously, and you can direct them to hit one creature or several. Using
    a Higher-Level Spell Slot. The spell creates one more dart for each spell slot level above 1.'
  sha256: b02b50562dee8b1ff105bb21d6e5e32cbcc45a51bebb1529bd9ee5956b4b0a49
  status: contract_specified
splat_requires_mechanics: *id001
---

# Magic Missile

<!-- SRD page 146 -->

Level 1 Evocation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Instantaneous\
You create three glowing darts of magical force.\
Each dart strikes a creature of your choice that you\
can see within range. A dart deals 1d4 + 1 Force\
damage to its target. The darts all strike simultane-\
ously, and you can direct them to hit one creature or\
several.\
Using a Higher-Level Spell Slot. The spell creates\
one more dart for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/146|page 146]].
