---
splat_schema: 1
splat_id: c82932e3-e0cc-5ed7-8182-ba9da368c0ff
splat_kind: rule
splat_key: dnd2024:class_features/bard_level_2_jack_of_all_trades
splat_name: 'Level 2: Jack of All Trades'
splat_origin:
  key: dnd2024:reference/pages/032
splat_sources:
- title: SRD 5.2.1
  locator: pp. 32
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/bard_level_2_jack_of_all_trades
  parameters:
    roll: ability_check
    requires_skill: true
    skill_proficiency: false
    already_uses_proficiency_bonus: false
    optional_bonus:
      op: floor
      args:
      - op: divide
        args:
        - stat: proficiency_bonus
        - 2
  implementation: dnd2024:class_features_bard_level_2_jack_of_all_trades
splat_contracts:
- id: dnd2024:class_features_bard_level_2_jack_of_all_trades
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - ability_check.pending
  capabilities:
  - character_features
  - effects
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 32
  engine_gaps: &id001
  - dnd2024:class_features_bard_level_2_jack_of_all_trades
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 9cbb3ca91d7d93e46ce4ac51b5028285acdc79b4be68a600327da17ff872c913
  parent_name: Bard
  line_span:
    first: 2654
    count: 8
splat_clauses:
- key: source_rules
  text: '# Level 2: Jack of All Trades You can add half your Proficiency Bonus (round down) to any ability
    check you make that uses a skill proficiency you lack and that doesn’t otherwise use your Proficiency
    Bonus. For example, if you make a Strength (Athletics) check and lack Athletics proficiency, you can
    add half your Proficiency Bonus to the check.'
  sha256: ec315eafedca7bb4c1c6780c9776a62c2989f914169136c965cf37b50fb74898
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 2: Jack of All Trades

<!-- SRD page 32 -->

You can add half your Proficiency Bonus (round\
down) to any ability check you make that uses a\
skill proficiency you lack and that doesn’t otherwise\
use your Proficiency Bonus.\
For example, if you make a Strength (Athletics)\
check and lack Athletics proficiency, you can add\
half your Proficiency Bonus to the check.\
