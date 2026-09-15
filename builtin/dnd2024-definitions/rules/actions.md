---
splat_schema: 1
splat_id: 84a14b5e-aa13-542c-ac4c-cda3ccef4675
splat_kind: rule
splat_key: dnd2024:rules/actions
splat_name: Actions
splat_origin:
  key: dnd2024:reference/pages/009
splat_sources:
- title: SRD 5.2.1
  locator: pp. 9, 10
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/actions
  parameters:
    main_actions:
    - key: dnd2024:glossary/attack_action
    - key: dnd2024:glossary/dash_action
    - key: dnd2024:glossary/disengage_action
    - key: dnd2024:glossary/dodge_action
    - key: dnd2024:glossary/help_action
    - key: dnd2024:glossary/hide_action
    - key: dnd2024:glossary/influence_action
    - key: dnd2024:glossary/magic_action
    - key: dnd2024:glossary/ready_action
    - key: dnd2024:glossary/search_action
    - key: dnd2024:glossary/study_action
    - key: dnd2024:glossary/utilize_action
    default_action_required_except:
    - movement
    - communication
    additional_actions_source:
    - class_features
    - other_abilities
    improvised_action:
      authority: gm
      choices:
        possible: boolean
        test: optional_d20_test
  implementation: dnd2024:rules_actions
splat_contracts:
- id: dnd2024:rules_actions
  inputs:
    subject:
      type: participant
      required: true
    proposed_action:
      type: resolution
      required: true
  outputs:
  - action_options
  - action_adjudication
  events: []
  capabilities:
  - actions
  - checks
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 9
  - 10
  engine_gaps: &id001
  - dnd2024:rules_actions
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 40d0acfe1e89293780c3436a28f268806ea06e6fbca02e4c200f1fd13c5fcb7d
  parent_name: Actions
  line_span:
    first: 450
    count: 44
splat_clauses:
- key: source_rules
  text: '# Actions When you do something other than moving or communicating, you typically take an action.
    The Action table lists the game’s main actions, which are defined in more detail in “Rules Glossary.”
    Actions Action Summary Attack Attack with a weapon or an Unarmed Strike. Dash For the rest of the
    turn, give yourself extra movement equal to your Speed. Action Summary Disengage Your movement doesn’t
    provoke Opportunity Attacks for the rest of the turn. Dodge Until the start of your next turn, attack
    rolls against you have Disadvantage, and you make Dexterity saving throws with Advantage. You lose
    this benefit if you have the Incapacitated condition or if your Speed is 0. Help Help another creature’s
    ability check or attack roll, or administer first aid. Hide Make a Dexterity (Stealth) check. Influence
    Make a Charisma (Deception, Intimidation, Performance, or Persuasion) or Wisdom (Animal Handling)
    check to alter a creature’s attitude. Magic Cast a spell, use a magic item, or use a magical feature.
    Ready Prepare to take an action in response to a trigger you define. Search Make a Wisdom (Insight,
    Medicine, Perception, or Survival) check. Study Make an Intelligence (Arcana, History, Investigation,
    Nature, or Religion) check. Utilize Use a nonmagical object. Player characters and monsters can also
    do things not covered by these actions. Many class features and other abilities provide additional
    action options, and you can improvise other actions. When you describe an action not detailed elsewhere
    in the rules, the Game Master tells you whether that action is possible and what kind of D20 Test
    you need to make, if any.'
  sha256: 82bbb6b5bf47c1ee7df067c039923709d0962aebb5479bd0f5dc43a15c643594
  status: contract_specified
splat_requires_mechanics: *id001
---

# Actions

<!-- SRD page 9 -->

When you do something other than moving or com-\
municating, you typically take an action. The Action\
table lists the game’s main actions, which are de-\
fined in more detail in “Rules Glossary.”\
Actions\
Action Summary\
Attack Attack with a weapon or an Unarmed\
Strike.\
Dash For the rest of the turn, give yourself extra\
movement equal to your Speed.\

<!-- SRD page 10 -->

Action Summary\
Disengage Your movement doesn’t provoke Oppor-\
tunity Attacks for the rest of the turn.\
Dodge Until the start of your next turn, attack\
rolls against you have Disadvantage, and\
you make Dexterity saving throws with\
Advantage. You lose this benefit if you\
have the Incapacitated condition or if your\
Speed is 0.\
Help Help another creature’s ability check or\
attack roll, or administer first aid.\
Hide Make a Dexterity (Stealth) check.\
Influence Make a Charisma (Deception, Intimidation,\
Performance, or Persuasion) or Wisdom\
(Animal Handling) check to alter a crea-\
ture’s attitude.\
Magic Cast a spell, use a magic item, or use a\
magical feature.\
Ready Prepare to take an action in response to a\
trigger you define.\
Search Make a Wisdom (Insight, Medicine,\
Perception, or Survival) check.\
Study Make an Intelligence (Arcana, History, In-\
vestigation, Nature, or Religion) check.\
Utilize Use a nonmagical object.\
Player characters and monsters can also do things\
not covered by these actions. Many class features\
and other abilities provide additional action op-\
tions, and you can improvise other actions. When\
you describe an action not detailed elsewhere in the\
rules, the Game Master tells you whether that action\
is possible and what kind of D20 Test you need to\
make, if any.\
