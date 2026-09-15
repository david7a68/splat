---
splat_schema: 1
splat_id: 3172a067-3138-5c24-9631-0ace78c13094
splat_kind: rule
splat_key: dnd2024:glossary/initiative
splat_name: Initiative
splat_origin:
  key: dnd2024:reference/glossary/initiative
splat_sources:
- title: SRD 5.2.1
  locator: pp. 184
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/initiative
  parameters:
    purpose: combat_turn_order
    mode_choice:
      chooser: gm
      options:
      - roll
      - score
      default: roll
    score:
      op: add
      args:
      - 10
      - var: dexterity_modifier
    score_roll_mode_adjustment:
      advantage: 5
      disadvantage: -5
      normal: 0
  implementation: dnd2024:glossary_initiative
splat_contracts:
- id: dnd2024:glossary_initiative
  inputs:
    subject:
      type: participant
      required: true
    mode:
      type: choice
      required: true
    roll_mode:
      type: choice
      required: true
  outputs:
  - initiative_score
  events:
  - initiative.requested
  capabilities:
  - initiative
  - checks
  - gm_choices
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 914ea9fd83ced18ed8060c1e8f6ac71e88caf141f97cab5163e8580928a2713f
  source_path: builtin/srd-5.2.1/glossary/initiative.md
  pages:
  - 184
  engine_gaps: &id001
  - dnd2024:glossary_initiative
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: baab9d3395f99ee2063a46342a0ee3343fccc87e2576e0e2f5d779cdf399f19b
splat_clauses:
- key: source_rules
  text: '# Initiative Initiative determines the order of turns during combat. The combat rules in “Playing
    the Game” explain how to roll Initiative. Sometimes a GM might have combatants use their Initiative
    scores instead of rolling Initiative. Your Initiative score equals 10 plus your Dexterity modifier.
    If you have Advantage on Initiative rolls, increase your Initiative score by 5. If you have Disadvantage
    on those rolls, decrease that score by 5. See also “Playing the Game” (“Combat”).'
  sha256: 6c018b1bc5502c3768f195ea367d85b71abd35da29284113b1858b85599f9638
  status: contract_specified
splat_requires_mechanics: *id001
---

# Initiative

<!-- SRD page 184 -->

Initiative determines the order of turns during com-\
bat. The combat rules in “Playing the Game” explain\
how to roll Initiative.\
Sometimes a GM might have combatants use\
their Initiative scores instead of rolling Initiative.\
Your Initiative score equals 10 plus your Dexterity\
modifier. If you have Advantage on Initiative rolls,\
increase your Initiative score by 5. If you have Dis-\
advantage on those rolls, decrease that score by 5.\
See also “Playing the Game” (“Combat”).\

Original layout: [[../../srd-5.2.1/pages/184|page 184]].
