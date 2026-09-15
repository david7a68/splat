---
splat_schema: 1
splat_id: 20d2b5d7-9345-53e5-8d54-8a688f803bc6
splat_kind: rule
splat_key: dnd2024:rules/weapons_light
splat_name: Light
splat_origin:
  key: dnd2024:reference/pages/089
splat_sources:
- title: SRD 5.2.1
  locator: pp. 89
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/weapons_light
  parameters:
    trigger:
      action: attack
      own_turn: true
      weapon_property: light
    extra_attack:
      count: 1
      cost: bonus_action
      timing: later_same_turn
      different_weapon: true
      weapon_property: light
      damage_ability_modifier:
        op: min
        args:
        - 0
        - variable: attack_ability_modifier
  implementation: dnd2024:rule_weapons_light
splat_contracts:
- id: dnd2024:rule_weapons_light
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    attack_ability_modifier:
      type: number
      required: true
  outputs:
  - item_effects
  events:
  - attack.completed
  capabilities:
  - equipment
  - attacks
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 89
  engine_gaps: &id001
  - dnd2024:rule_weapons_light
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 63df3168c71ad5ef18106967300584858099be2bdc19079000a8c037173f3880
  parent_name: Weapons
  line_span:
    first: 7716
    count: 13
splat_clauses:
- key: source_rules
  text: '# Light When you take the Attack action on your turn and attack with a Light weapon, you can
    make one extra attack as a Bonus Action later on the same turn. That extra attack must be made with
    a different Light weapon, and you don’t add your ability modifier to the extra attack’s damage unless
    that modifier is negative. For example, you can attack with a Shortsword in one hand and a Dagger
    in the other using the Attack action and a Bonus Action, but you don’t add your Strength or Dexterity
    modifier to the damage roll of the Bonus Action unless that modifier is negative.'
  sha256: 5fcb94874d08fa8e51e6b74d04f795bc08d49db727652a4cbfeb695368814561
  status: contract_specified
splat_requires_mechanics: *id001
---

# Light

<!-- SRD page 89 -->

When you take the Attack action on your turn and\
attack with a Light weapon, you can make one extra\
attack as a Bonus Action later on the same turn.\
That extra attack must be made with a different\
Light weapon, and you don’t add your ability modi-\
fier to the extra attack’s damage unless that modi-\
fier is negative. For example, you can attack with a\
Shortsword in one hand and a Dagger in the other\
using the Attack action and a Bonus Action, but you\
don’t add your Strength or Dexterity modifier to the\
damage roll of the Bonus Action unless that modi-\
fier is negative.\
