---
splat_schema: 1
splat_id: 7ae08674-eedb-525a-8926-276aeb4843fd
splat_kind: rule
splat_key: dnd2024:glossary/help_action
splat_name: Help [Action]
splat_origin:
  key: dnd2024:reference/glossary/help_action
splat_sources:
- title: SRD 5.2.1
  locator: pp. 182, 183
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/help_action
  parameters:
    cost: action
    select_one:
      ability_check:
        choose_own_proficiency_kind:
        - skill
        - tool
        choose_one_ally: true
        proximity:
          verbal_or_physical_assistance_possible: true
          final_decision: gm
        advantage: next_ally_check_using_chosen_proficiency
        expires: own_next_turn_start
        consumed_after_matching_check: true
      attack_roll:
        enemy_distance_feet_max: 5
        advantage: next_attack_by_any_one_ally_against_enemy
        expires: own_next_turn_start
        consumed_after_matching_attack: true
  implementation: dnd2024:glossary_help_action
splat_contracts:
- id: dnd2024:glossary_help_action
  inputs:
    subject:
      type: participant
      required: true
    mode:
      type: choice
      required: true
    target:
      type: participant
      required: true
    proficiency:
      required: false
      type: definition
    assistance_possible:
      required: false
      type: boolean
  outputs:
  - help_effect
  events:
  - action.requested
  - ability_check.pending
  - attack.pending
  - turn.started
  capabilities:
  - action_economy
  - checks
  - attacks
  - gm_choices
splat_conversion:
  status: specified
  category: glossary
  source_sha256: ef7353097d167fd6278997f206f4b529d2c8ae680851be21448640464dc66fe2
  source_path: builtin/srd-5.2.1/glossary/help_action.md
  pages:
  - 182
  - 183
  engine_gaps: &id001
  - dnd2024:glossary_help_action
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 23637b2e70f1bc486f4ff77b3ec2db3936a7891d83872e5ab11c1a5a6dd025d7
splat_clauses:
- key: source_rules
  text: '# Help [Action] When you take the Help action, you do one of the following. Assist an Ability
    Check. Choose one of your skill or tool proficiencies and one ally who is near enough for you to assist
    verbally or physically when they make an ability check. That ally has Advantage on the next ability
    check they make with the chosen skill or tool. This benefit expires if the ally doesn’t use it before
    the start of your next turn. The GM has final say on whether your assistance is possible. Assist an
    Attack Roll. You momentarily distract an enemy within 5 feet of you, giving Advantage to the next
    attack roll by one of your allies against that enemy. This benefit expires at the start of your next
    turn.'
  sha256: db3ef3f67837831370327ba0f8e2da1a646cbbea840317b5d88b1833a758ffd7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Help [Action]

<!-- SRD page 182 -->

When you take the Help action, you do one of the\
following.\
Assist an Ability Check. Choose one of your\
skill or tool proficiencies and one ally who is near\

<!-- SRD page 183 -->

enough for you to assist verbally or physically when\
they make an ability check. That ally has Advantage\
on the next ability check they make with the chosen\
skill or tool. This benefit expires if the ally doesn’t\
use it before the start of your next turn. The GM has\
final say on whether your assistance is possible.\
Assist an Attack Roll. You momentarily distract\
an enemy within 5 feet of you, giving Advantage to\
the next attack roll by one of your allies against that\
enemy. This benefit expires at the start of your next\
turn.\

Original layout: [[../../srd-5.2.1/pages/182|page 182]], [[../../srd-5.2.1/pages/183|page 183]].
