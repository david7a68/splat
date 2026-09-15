---
splat_schema: 1
splat_id: dc6c4b33-642c-5453-91e0-379837ef9905
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_advantage_disadvantage
splat_name: Advantage/Disadvantage
splat_origin:
  key: dnd2024:reference/pages/007
splat_sources:
- title: SRD 5.2.1
  locator: pp. 7, 8
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/d20_tests_advantage_disadvantage
  parameters:
    apply_definitions:
    - key: dnd2024:glossary/advantage
    - key: dnd2024:glossary/disadvantage
    - key: dnd2024:glossary/heroic_inspiration
    circumstantial_advantage:
      actor: gm
      choices:
      - advantage
      - disadvantage
      - neither
    heroic_inspiration:
      maximum: 1
      reroll:
        die_count: 1
        timing: immediately_after_roll
        must_use_new_roll: true
        consume: true
      overflow:
        optional: true
        recipient: party_player_character_without_inspiration
      gm_award:
        optional: true
      human_daily_grant: true
  implementation: dnd2024:rule_d20_tests_advantage_disadvantage
splat_contracts:
- id: dnd2024:rule_d20_tests_advantage_disadvantage
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
  - dice
  - heroic_inspiration
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 7
  - 8
  engine_gaps: &id001
  - dnd2024:rule_d20_tests_advantage_disadvantage
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 7e41dadf149ba079b19690fc95fda06129c2f247d9d6d2b25299f8bb7748dd10
  parent_name: D20 Tests
  line_span:
    first: 278
    count: 29
splat_clauses:
- key: source_rules
  text: '# Advantage/Disadvantage Sometimes a D20 Test is modified by Advantage or Disadvantage. Advantage
    reflects the positive circumstances surrounding a d20 roll, while Disadvantage reflects negative circumstances.
    You usually acquire Advantage or Disadvantage through the use of special abilities and actions. The
    HEroic inspiration Sometimes the GM or a rule gives you Heroic Inspiration. If you have Heroic Inspiration,
    you can expend it to reroll any die immediately after rolling it, and you must use the new roll. Only
    One at a Time. You can never have more than one instance of Heroic Inspiration. If something gives
    you Heroic Inspiration and you already have it, you can give it to a player character in your group
    who lacks it. Gaining Heroic Inspiration. Your GM can give you Heroic Inspiration for a variety of
    reasons. Typically, GMs award it when you do something particularly heroic, in character, or entertaining.
    It’s a reward for making the game more fun for everyone playing. Other rules might allow your character
    to gain Heroic Inspiration independent of the GM’s decision. For example, Human characters start each
    day with Heroic Inspiration. GM can also decide that circumstances grant Advantage or impose Disadvantage.'
  sha256: d86f4ea2c85cf82f6025d3fde63c5ddcbde6b1fcd3b1085643321f1496af0825
  status: contract_specified
splat_requires_mechanics: *id001
---

# Advantage/Disadvantage

<!-- SRD page 7 -->

Sometimes a D20 Test is modified by Advantage or\
Disadvantage. Advantage reflects the positive cir-\
cumstances surrounding a d20 roll, while Disadvan-\
tage reflects negative circumstances.\
You usually acquire Advantage or Disadvantage\
through the use of special abilities and actions. The\

<!-- SRD page 8 -->

HEroic inspiration\
Sometimes the GM or a rule gives you Heroic Inspira-\
tion. If you have Heroic Inspiration, you can expend it\
to reroll any die immediately after rolling it, and you\
must use the new roll.\
Only One at a Time. You can never have more\
than one instance of Heroic Inspiration. If something\
gives you Heroic Inspiration and you already have it,\
you can give it to a player character in your group\
who lacks it.\
Gaining Heroic Inspiration. Your GM can\
give you Heroic Inspiration for a variety of reasons.\
Typically, GMs award it when you do something par-\
ticularly heroic, in character, or entertaining. It’s a\
reward for making the game more fun for everyone\
playing.\
Other rules might allow your character to gain\
Heroic Inspiration independent of the GM’s decision.\
For example, Human characters start each day with\
Heroic Inspiration.\
GM can also decide that circumstances grant Ad-\
vantage or impose Disadvantage.\
