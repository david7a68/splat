---
splat_schema: 1
splat_id: 3d1a30f2-af5e-562a-9759-c5b10bf4d467
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_initiative
splat_name: Initiative
splat_origin:
  key: dnd2024:reference/pages/254
splat_sources:
- title: SRD 5.2.1
  locator: pp. 254
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_initiative
  parameters:
    initiative_entry:
      first: modifier
      parenthetical: score
    resolution_choice:
    - roll_with_modifier
    - use_fixed_score
    default_modifier: dexterity_modifier
    additional_modifiers: stat_block
  implementation: dnd2024:rules_parts_of_a_stat_block_initiative
splat_contracts:
- id: dnd2024:rules_parts_of_a_stat_block_initiative
  inputs:
    monster:
      type: participant
      required: true
    mode:
      type: choice
      required: true
  outputs:
  - initiative
  events: []
  capabilities:
  - initiative
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 254
  engine_gaps: &id001
  - dnd2024:rules_parts_of_a_stat_block_initiative
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 929cd452e0ab7ac291f365f5a75f6ba165d96d7243c878ed176f08384822764b
  parent_name: Parts of a Stat Block
  line_span:
    first: 10950
    count: 13
splat_clauses:
- key: source_rules
  text: '# Initiative The Initiative entry specifies the monster’s Initiative modifier followed by the
    monster’s Initiative score in parentheses. Use the modifier when you roll to determine a monster’s
    Initiative. A monster’s Initiative modifier is typically equal to its Dexterity modifier, but some
    monsters have additional modifiers, such as Proficiency Bonus, applied to that number. If you don’t
    want to roll a monster’s Initiative, use the Initiative score as the monster’s Initiative in combat.
    Initiative is further detailed in “Playing the Game.”'
  sha256: fd7d0a70fdec2bf42ad76c951f0664dc0ac0e407c5bbd4b038bef5a5d6ff9fd1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Initiative

<!-- SRD page 254 -->

The Initiative entry specifies the monster’s Initia-\
tive modifier followed by the monster’s Initiative\
score in parentheses. Use the modifier when you\
roll to determine a monster’s Initiative. A monster’s\
Initiative modifier is typically equal to its Dexterity\
modifier, but some monsters have additional mod-\
ifiers, such as Proficiency Bonus, applied to that\
number.\
If you don’t want to roll a monster’s Initiative, use\
the Initiative score as the monster’s Initiative in\
combat. Initiative is further detailed in “Playing the\
Game.”\
