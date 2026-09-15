---
splat_schema: 1
splat_id: 00889495-d8d4-52f0-b527-78b0157399c2
splat_kind: rule
splat_key: dnd2024:rules/d20_tests
splat_name: D20 Tests
splat_origin:
  key: dnd2024:reference/pages/006
splat_sources:
- title: SRD 5.2.1
  locator: pp. 6
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/d20_tests
  parameters:
    test_kinds:
    - ability_check
    - saving_throw
    - attack_roll
    die:
      count: 1
      sides: 20
      add: 0
    advantage:
      dice:
        count: 2
        sides: 20
        add: 0
      keep: highest
    disadvantage:
      dice:
        count: 2
        sides: 20
        add: 0
      keep: lowest
    total:
      op: sum
      terms:
      - selected_die
      - relevant_ability_modifier
      - applicable_proficiency_bonus
      - circumstantial_bonuses_and_penalties
    success:
      op: gte
      left: total
      right: target_number
    targets:
      ability_check: difficulty_class
      saving_throw: difficulty_class
      attack_roll: armor_class
    target_number_authority: gm
  implementation: dnd2024:rules_d20_tests
splat_contracts:
- id: dnd2024:rules_d20_tests
  inputs:
    actor:
      type: participant
      required: true
    test:
      type: resolution
      required: true
  outputs:
  - test_modifiers
  - test_result
  events: []
  capabilities:
  - d20_tests
  - proficiency
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 6
  engine_gaps: &id001
  - dnd2024:rules_d20_tests
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 532dd631433696a82bd15c4ebe540b3421e97081df1f6873dd14a71d68a2a587
  parent_name: D20 Tests
  line_span:
    first: 103
    count: 37
splat_clauses:
- key: source_rules
  text: '# D20 Tests When the outcome of an action is uncertain, the game uses a d20 roll to determine
    success or failure. These rolls are called D20 Tests, and they come in three kinds: ability checks,
    saving throws, and attack rolls. They follow these steps: 4: Roll 1d20. You always want to roll high.
    If the roll has Advantage or Disadvantage (described later in “Playing the Game”), you roll two d20s,
    but you use the number from only one of them— the higher one if you have Advantage or the lower one
    if you have Disadvantage. 5: Add Modifiers. Add these modifiers to the number rolled on the d20: •
    The Relevant Ability Modifier. “Playing the Game” and “Rules Glossary” explain which ability modifiers
    to use for various D20 Tests. • Your Proficiency Bonus If Relevant. Each creature has a Proficiency
    Bonus, a number added when making a D20 Test that uses something, such as a skill, in which the creature
    has proficiency. See “Proficiency” later in “Playing the Game.” • Circumstantial Bonuses and Penalties.
    A class feature, a spell, or another rule might give a bonus or penalty to the die roll. 6: Compare
    the Total to a Target Number. If the total of the d20 and its modifiers equals or exceeds the target
    number, the D20 Test succeeds. Otherwise, it fails. The Game Master determines target numbers and
    tells players whether their rolls are successful. The target number for an ability check or a saving
    throw is called a Difficulty Class (DC). The target number for an attack roll is called an Armor Class
    (AC), which appears on a character sheet or in a stat block (see “Rules Glossary”).'
  sha256: 6841a1d01c9325677d4ad5dfe0b05316d33e745fe3bc20798db2e0aaadf19a17
  status: contract_specified
splat_requires_mechanics: *id001
---

# D20 Tests

<!-- SRD page 6 -->

When the outcome of an action is uncertain, the\
game uses a d20 roll to determine success or fail-\
ure. These rolls are called D20 Tests, and they come\
in three kinds: ability checks, saving throws, and\
attack rolls. They follow these steps:\
4: Roll 1d20. You always want to roll high. If the\
roll has Advantage or Disadvantage (described\
later in “Playing the Game”), you roll two d20s,\
but you use the number from only one of them—\
the higher one if you have Advantage or the lower\
one if you have Disadvantage.\
5: Add Modifiers. Add these modifiers to the num-\
ber rolled on the d20:\
•  The Relevant Ability Modifier. “Playing the\
Game” and “Rules Glossary” explain which ability\
modifiers to use for various D20 Tests.\
•  Your Proficiency Bonus If Relevant. Each crea-\
ture has a Proficiency Bonus, a number added\
when making a D20 Test that uses something,\
such as a skill, in which the creature has pro-\
ficiency. See “Proficiency” later in “Playing the\
Game.”\
•  Circumstantial Bonuses and Penalties. A class\
feature, a spell, or another rule might give a bonus\
or penalty to the die roll.\
6: Compare the Total to a Target Number. If the\
total of the d20 and its modifiers equals or ex-\
ceeds the target number, the D20 Test succeeds.\
Otherwise, it fails. The Game Master determines\
target numbers and tells players whether their\
rolls are successful. The target number for an\
ability check or a saving throw is called a Diffi-\
culty Class (DC). The target number for an attack\
roll is called an Armor Class (AC), which appears\
on a character sheet or in a stat block (see “Rules\
Glossary”).\
