---
splat_schema: 1
splat_id: ca7e9da1-ce27-5267-8c28-e409ea166f46
splat_kind: rule
splat_key: dnd2024:feats/epic_boon_feats_boon_of_irresistible_offense
splat_name: Boon of Irresistible Offense
splat_origin:
  key: dnd2024:reference/pages/088
splat_sources:
- title: SRD 5.2.1
  locator: pp. 88
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:feats/epic_boon_feats_boon_of_irresistible_offense
  parameters:
    feat_category: epic_boon
    minimum_character_level: 19
    ability_score_increase:
      choice:
      - strength
      - dexterity
      amount: 1
      maximum: 30
    ignore_damage_resistance:
    - bludgeoning
    - piercing
    - slashing
    overwhelming_strike:
      trigger:
        attack_d20_natural_result: 20
      optional_extra_damage:
        amount:
          stat: ability_chosen_for_this_feat.score
        type: triggering_attack_damage_type
  implementation: dnd2024:feats_epic_boon_feats_boon_of_irresistible_offense
splat_contracts:
- id: dnd2024:feats_epic_boon_feats_boon_of_irresistible_offense
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
  - damage
  events:
  - character.feat_selected
  - damage.pending
  - attack.rolled
  capabilities:
  - feats
  - ability_scores
  - damage
splat_conversion:
  status: specified
  category: feat
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 88
  engine_gaps: &id001
  - dnd2024:feats_epic_boon_feats_boon_of_irresistible_offense
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 2340c641be261f34289413b18e0293747df0d815678dfa2ac5beb438999be45e
  parent_name: Epic Boon Feats
  line_span:
    first: 7597
    count: 13
splat_clauses:
- key: source_rules
  text: '# Boon of Irresistible Offense Epic Boon Feat (Prerequisite: Level 19+) You gain the following
    benefits. Ability Score Increase. Increase your Strength or Dexterity score by 1, to a maximum of
    30. Overcome Defenses. The Bludgeoning, Piercing, and Slashing damage you deal always ignores Resistance.
    Overwhelming Strike. When you roll a 20 on the d20 for an attack roll, you can deal extra damage to
    the target equal to the ability score increased by this feat. The extra damage’s type is the same
    as the attack’s type.'
  sha256: 065afaf6acee62422680e4392f9e78bddba0fabf2402003846abf060cb53a402
  status: contract_specified
splat_requires_mechanics: *id001
---

# Boon of Irresistible Offense

<!-- SRD page 88 -->

Epic Boon Feat (Prerequisite: Level 19+)\
You gain the following benefits.\
Ability Score Increase. Increase your Strength or\
Dexterity score by 1, to a maximum of 30.\
Overcome Defenses. The Bludgeoning, Pierc-\
ing, and Slashing damage you deal always ignores\
Resistance.\
Overwhelming Strike. When you roll a 20 on the\
d20 for an attack roll, you can deal extra damage\
to the target equal to the ability score increased by\
this feat. The extra damage’s type is the same as the\
attack’s type.\
