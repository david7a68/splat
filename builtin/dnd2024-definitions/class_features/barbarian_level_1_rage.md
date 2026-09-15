---
splat_schema: 1
splat_id: 30b96f68-8540-5b47-ad85-b7ff206689d4
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_1_rage
splat_name: 'Level 1: Rage'
splat_origin:
  key: dnd2024:reference/pages/028
splat_sources:
- title: SRD 5.2.1
  locator: pp. 28, 29
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/barbarian_level_1_rage
  parameters:
    action: bonus_action
    requires:
      wearing_heavy_armor: false
    resource:
      id: barbarian.rage
      maximum:
        table:
          key: dnd2024:tables/barbarian_progression
        row:
          var: actor.class_levels.barbarian
        column: rages
      recovery:
      - event: rest.short_completed
        amount: 1
      - event: rest.long_completed
        amount: all
    cost: 1
    duration:
      end: actor.next_turn.end
      maximum:
        minutes: 10
    ends_on:
    - actor.heavy_armor_donned
    - actor.incapacitated
    resistances:
    - bludgeoning
    - piercing
    - slashing
    damage_bonus:
      requires:
        attack_ability: strength
        attack_kind:
        - weapon
        - unarmed_strike
        damage_dealt: true
      amount:
        table:
          key: dnd2024:tables/barbarian_progression
        row:
          var: actor.class_levels.barbarian
        column: rage_damage
    advantage:
    - strength.check
    - strength.save
    prohibits:
    - concentration
    - spellcasting
    extend:
      requires:
        rage_active: true
        turn: actor.turn_after_activation
      any_event:
      - attack.roll_against_enemy
      - enemy.saving_throw_forced
      - bonus_action.rage_extend
      end: actor.next_turn.end
    class_scope:
      class: barbarian
      minimum_level: 1
  implementation: dnd2024:class_features_barbarian_level_1_rage
splat_contracts:
- id: dnd2024:class_features_barbarian_level_1_rage
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - action.bonus
  - turn.start
  - turn.end
  - attack.damage
  - ability.check
  - saving_throw
  - condition.applied
  - equipment.donned
  - rest.short_completed
  - rest.long_completed
  capabilities:
  - character_features
  - resources
  - spellcasting
  - damage
  - d20_tests
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 28
  - 29
  engine_gaps: &id001
  - dnd2024:class_features_barbarian_level_1_rage
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 853922d5f84c27bc61618bc591ef63cd1dd6e0877d7258866295d6ba8a197108
  parent_name: Barbarian
  line_span:
    first: 2262
    count: 58
splat_clauses:
- key: source_rules
  text: '# Level 1: Rage You can imbue yourself with a primal power called Rage, a force that grants you
    extraordinary might and resilience. You can enter it as a Bonus Action if you aren’t wearing Heavy
    armor. You can enter your Rage the number of times shown for your Barbarian level in the Rages column
    of the Barbarian Features table. You regain one expended use when you finish a Short Rest, and you
    regain all expended uses when you finish a Long Rest. Rage Weapon Rages Damage Mastery 2 +2 2 2 +2
    2 3 +2 2 3 +2 3 3 +2 3 4 +2 3 4 +2 3 4 +2 3 4 +3 3 4 +3 4 4 +3 4 5 +3 4 5 +3 4 5 +3 4 5 +3 4 5 +4
    4 6 +4 4 6 +4 4 6 +4 4 6 +4 4 While active, your Rage follows the rules below. Damage Resistance.
    You have Resistance to Bludgeoning, Piercing, and Slashing damage. Rage Damage. When you make an attack
    using Strength—with either a weapon or an Unarmed Strike—and deal damage to the target, you gain a
    bonus to the damage that increases as you gain levels as a Barbarian, as shown in the Rage Damage
    column of the Barbarian Features table. Strength Advantage. You have Advantage on Strength checks
    and Strength saving throws. No Concentration or Spells. You can’t maintain Concentration, and you
    can’t cast spells. Duration. The Rage lasts until the end of your next turn, and it ends early if
    you don Heavy armor or have the Incapacitated condition. If your Rage is still active on your next
    turn, you can extend the Rage for another round by doing one of the following: • Make an attack roll
    against an enemy. • Force an enemy to make a saving throw. • Take a Bonus Action to extend your Rage.
    Each time the Rage is extended, it lasts until the end of your next turn. You can maintain a Rage
    for up to 10 minutes.'
  sha256: 82fd15cd4ff5dcc8cd1879327d956e028fa2938139d83309cb306bbfe3eb1a40
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Rage

<!-- SRD page 28 -->

You can imbue yourself with a primal power called\
Rage, a force that grants you extraordinary might\
and resilience. You can enter it as a Bonus Action if\
you aren’t wearing Heavy armor.\
You can enter your Rage the number of times\
shown for your Barbarian level in the Rages column\
of the Barbarian Features table. You regain one ex-\
pended use when you finish a Short Rest, and you\
regain all expended uses when you finish a Long\
Rest.\
Rage Weapon\
Rages Damage Mastery\
2 +2 2\
2 +2 2\
3 +2 2\
3 +2 3\
3 +2 3\
4 +2 3\
4 +2 3\
4 +2 3\
4 +3 3\
4 +3 4\
4 +3 4\
5 +3 4\
5 +3 4\
5 +3 4\
5 +3 4\
5 +4 4\
6 +4 4\
6 +4 4\
6 +4 4\
6 +4 4\

<!-- SRD page 29 -->

While active, your Rage follows the rules below.\
Damage Resistance. You have Resistance to Blud-\
geoning, Piercing, and Slashing damage.\
Rage Damage. When you make an attack using\
Strength—with either a weapon or an Unarmed\
Strike—and deal damage to the target, you gain\
a bonus to the damage that increases as you gain\
levels as a Barbarian, as shown in the Rage Damage\
column of the Barbarian Features table.\
Strength Advantage. You have Advantage on\
Strength checks and Strength saving throws.\
No Concentration or Spells. You can’t maintain\
Concentration, and you can’t cast spells.\
Duration. The Rage lasts until the end of your\
next turn, and it ends early if you don Heavy armor\
or have the Incapacitated condition. If your Rage\
is still active on your next turn, you can extend\
the Rage for another round by doing one of the\
following:\
•  Make an attack roll against an enemy.\
•  Force an enemy to make a saving throw.\
•  Take a Bonus Action to extend your Rage.\
Each time the Rage is extended, it lasts until the end\
of your next turn. You can maintain a Rage for up to\
10 minutes.\
