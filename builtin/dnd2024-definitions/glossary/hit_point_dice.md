---
splat_schema: 1
splat_id: 146adcd4-4b17-5d9c-99cd-e91d38094b0b
splat_kind: rule
splat_key: dnd2024:glossary/hit_point_dice
splat_name: Hit Point Dice
splat_origin:
  key: dnd2024:reference/glossary/hit_point_dice
splat_sources:
- title: SRD 5.2.1
  locator: pp. 183
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/hit_point_dice
  parameters:
    player_character_hp_maximum_uses_hit_dice: true
    short_rest_spending_allowed: true
    spending_rule:
      key: dnd2024:glossary/short_rest
  implementation: dnd2024:glossary_hit_point_dice
splat_contracts:
- id: dnd2024:glossary_hit_point_dice
  inputs:
    subject:
      type: participant
      required: true
    hit_dice:
      type: choice
      required: true
  outputs:
  - hit_dice_resource
  events: []
  capabilities:
  - hit_points
  - rests
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 4717a37030a799b23e9953d2166b25f2e6755a03161da93c4a98dccc2220ee7b
  source_path: builtin/srd-5.2.1/glossary/hit_point_dice.md
  pages:
  - 183
  engine_gaps: &id001
  - dnd2024:glossary_hit_point_dice
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: bc68bd1f48c0cf8b29daf6c62640966efb84920430110dede068ad4fae9ca485
splat_clauses:
- key: source_rules
  text: '# Hit Point Dice Hit Point Dice, or Hit Dice for short, help determine a player character’s Hit
    Point maximum, as explained in “Character Creation.” Most monsters also have Hit Dice. A creature
    can spend Hit Dice during a Short Rest to regain Hit Points. See also “Short Rest.”'
  sha256: 9f7bf1a65add5bdb239aabbcc9cc58065e99fac433a4ed1dfc499d39fba4d17d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hit Point Dice

<!-- SRD page 183 -->

Hit Point Dice, or Hit Dice for short, help determine\
a player character’s Hit Point maximum, as ex-\
plained in “Character Creation.” Most monsters also\
have Hit Dice. A creature can spend Hit Dice during\
a Short Rest to regain Hit Points. See also “Short\
Rest.”\

Original layout: [[../../srd-5.2.1/pages/183|page 183]].
