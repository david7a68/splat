---
splat_schema: 1
splat_id: cca9130c-2589-51f6-ac49-80e43703d267
splat_kind: rule
splat_key: dnd2024:rules/rhythm_of_play
splat_name: Rhythm of Play
splat_origin:
  key: dnd2024:reference/pages/005
splat_sources:
- title: SRD 5.2.1
  locator: pp. 5
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/rhythm_of_play
  parameters:
    resolution_precedence:
    - explicit_exception
    - general_rule
    scene_cycle:
    - gm_describes_scene
    - players_declare_actions
    - gm_resolves_and_narrates
    outside_combat:
      actor: gm
      choices:
      - activity_order
      - resolution_method
      all_characters_receive_opportunity: true
    combat_turns: true
  implementation: dnd2024:rule_rhythm_of_play
splat_contracts:
- id: dnd2024:rule_rhythm_of_play
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - resolution
  events: []
  capabilities:
  - rules
  - turns
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 5
  engine_gaps: &id001
  - dnd2024:rule_rhythm_of_play
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 469b47e854db9996fcb76d65059f81614b0491f1ea88bf47d7c5a025df37ea82
  parent_name: Rhythm of Play
  line_span:
    first: 1
    count: 53
splat_clauses:
- key: source_rules
  text: '# Rhythm of Play The three main pillars of D&D play are social interaction, exploration, and
    combat. Whichever one you’re experiencing, the game unfolds according to this basic pattern: 1: The
    Game Master Describes a Scene. The GM tells the players where their adventurers are and what’s around
    them (how many doors lead out of a room, what’s on a table, and so on). 2: The Players Describe What
    Their Characters Do. Typically, the characters stick together as they travel through a dungeon or
    another environment. Sometimes different adventurers do different things: one adventurer might search
    a treasure chest while a second examines a mysterious symbol engraved on a wall and a third keeps
    watch for monsters. Outside combat, the GM ensures that every character has a chance to act and decides
    how to resolve their activity. In combat, the characters take turns. 3: The GM Narrates the Results
    of the Adventurers’ Actions. Sometimes resolving a task is easy. If an adventurer walks across a room
    and tries to open a door, the GM might say the door opens and describe what lies beyond. But the door
    might be locked, the floor might hide a trap, or some other circumstance might make it challenging
    for an adventurer to complete a task. In those cases, the GM might ask the player to roll a die to
    help determine what happens. Describing the results often leads to another decision point, which brings
    the game back to step 1. This pattern holds during every game session (each time you sit down to play
    D&D), whether the ExcEptions supErsEdE GEnEral rulEs General rules govern each part of the game. For
    example, the combat rules tell you that melee attacks use Strength and ranged attacks use Dexterity.
    That’s a general rule, and a general rule is in effect as long as something in the game doesn’t explicitly
    say otherwise. The game also includes elements—class features, feats, weapon properties, spells, magic
    items, monster abilities, and the like—that sometimes contradict a general rule. When an exception
    and a general rule disagree, the exception wins. For example, if a feature says you can make melee
    attacks using your Charisma, you can do so, even though that statement disagrees with the general
    rule. adventurers are talking to a noble, exploring a ruin, or fighting a dragon. In certain situations—particularly
    combat—the action is more structured, and everyone takes turns.'
  sha256: 9954af6ee206c65d445478ba7e649c6d6bca12b63bd26a642b8a522d6d6edab9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rhythm of Play

<!-- SRD page 5 -->

The three main pillars of D&D play are social in-\
teraction, exploration, and combat. Whichever one\
you’re experiencing, the game unfolds according to\
this basic pattern:\
1: The Game Master Describes a Scene. The GM\
tells the players where their adventurers are and\
what’s around them (how many doors lead out of\
a room, what’s on a table, and so on).\
2: The Players Describe What Their Characters\
Do. Typically, the characters stick together as\
they travel through a dungeon or another envi-\
ronment. Sometimes different adventurers do\
different things: one adventurer might search a\
treasure chest while a second examines a mys-\
terious symbol engraved on a wall and a third\
keeps watch for monsters. Outside combat, the\
GM ensures that every character has a chance to\
act and decides how to resolve their activity. In\
combat, the characters take turns.\
3: The GM Narrates the Results of the Adventur-\
ers’ Actions. Sometimes resolving a task is easy.\
If an adventurer walks across a room and tries to\
open a door, the GM might say the door opens and\
describe what lies beyond. But the door might be\
locked, the floor might hide a trap, or some other\
circumstance might make it challenging for an\
adventurer to complete a task. In those cases, the\
GM might ask the player to roll a die to help de-\
termine what happens. Describing the results of-\
ten leads to another decision point, which brings\
the game back to step 1.\
This pattern holds during every game session\
(each time you sit down to play D&D), whether the\
ExcEptions supErsEdE GEnEral rulEs\
General rules govern each part of the game. For\
example, the combat rules tell you that melee at-\
tacks use Strength and ranged attacks use Dexterity.\
That’s a general rule, and a general rule is in effect as\
long as something in the game doesn’t explicitly say\
otherwise.\
The game also includes elements—class features,\
feats, weapon properties, spells, magic items, mon-\
ster abilities, and the like—that sometimes contra-\
dict a general rule. When an exception and a general\
rule disagree, the exception wins. For example, if a\
feature says you can make melee attacks using your\
Charisma, you can do so, even though that statement\
disagrees with the general rule.\
adventurers are talking to a noble, exploring a ruin,\
or fighting a dragon. In certain situations—partic-\
ularly combat—the action is more structured, and\
everyone takes turns.\
