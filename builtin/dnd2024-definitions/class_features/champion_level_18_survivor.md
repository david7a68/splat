---
splat_schema: 1
splat_id: c8f45d11-5d90-568c-91e8-a9abcb3f39c7
splat_kind: rule
splat_key: dnd2024:class_features/champion_level_18_survivor
splat_name: 'Level 18: Survivor'
splat_origin:
  key: dnd2024:reference/pages/049
splat_sources:
- title: SRD 5.2.1
  locator: pp. 49
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/champion_level_18_survivor
  parameters:
    death_saves:
      advantage: true
      natural_d20_results_count_as_20:
      - 18
      - 19
      - 20
    turn_start_healing:
      requires:
        bloodied: true
        minimum_hit_points: 1
      amount:
        op: add
        args:
        - 5
        - stat: abilities.constitution.modifier
  implementation: dnd2024:class_features_champion_level_18_survivor
splat_contracts:
- id: dnd2024:class_features_champion_level_18_survivor
  inputs:
    actor:
      type: participant
      required: true
    trigger:
      required: false
      type: event
    targets:
      required: false
      type: participant_list
  outputs:
  - feature_state
  - healing
  events:
  - death_save.pending
  - death_save.rolled
  - turn.started
  capabilities:
  - saving_throws
  - healing
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 49
  engine_gaps: &id001
  - dnd2024:class_features_champion_level_18_survivor
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 39f03a019f1fcd388f88f31671144c8fcba6ce9526d413a473198ed67b9cff5c
  parent_name: Champion
  line_span:
    first: 4077
    count: 11
splat_clauses:
- key: source_rules
  text: '# Level 18: Survivor You attain the pinnacle of resilience in battle, giving you these benefits.
    Defy Death. You have Advantage on Death Saving Throws. Moreover, when you roll 18–20 on a Death Saving
    Throw, you gain the benefit of rolling a 20 on it. Heroic Rally. At the start of each of your turns,
    you regain Hit Points equal to 5 plus your Constitution modifier if you are Bloodied and have at least
    1 Hit Point.'
  sha256: c6cdf2ca1f5f9dca70d0cd7045b839a5b1a8ca8cb9a3dd3a599d574e405cd5f3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 18: Survivor

<!-- SRD page 49 -->

You attain the pinnacle of resilience in battle, giving\
you these benefits.\
Defy Death. You have Advantage on Death Saving\
Throws. Moreover, when you roll 18–20 on a Death\
Saving Throw, you gain the benefit of rolling a 20 on\
it.\
Heroic Rally. At the start of each of your turns,\
you regain Hit Points equal to 5 plus your Constitu-\
tion modifier if you are Bloodied and have at least 1\
Hit Point.\
