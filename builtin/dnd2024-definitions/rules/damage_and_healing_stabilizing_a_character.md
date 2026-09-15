---
splat_schema: 1
splat_id: e82ba8ae-7528-581a-8985-ece751b33ea3
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_stabilizing_a_character
splat_name: Stabilizing a Character
splat_origin:
  key: dnd2024:reference/pages/018
splat_sources:
- title: SRD 5.2.1
  locator: pp. 18
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/damage_and_healing_stabilizing_a_character
  parameters:
    action: help
    requires_target_hp: 0
    check:
      ability: wisdom
      skill: medicine
      dc: 10
    success:
      stable: true
      death_saves: false
      unconscious: true
      hp: 0
    stable_damage:
      stable: false
      resume_death_saves: true
    stable_without_healing:
      after:
        dice:
          count: 1
          sides: 4
        unit: hour
      regain_hp: 1
  implementation: dnd2024:rules_damage_and_healing_stabilizing_a_character
splat_contracts:
- id: dnd2024:rules_damage_and_healing_stabilizing_a_character
  inputs:
    subject:
      type: participant
      required: true
    target:
      type: participant
      required: true
  outputs:
  - stability
  - hit_points
  - conditions
  events:
  - action.help
  - damage.applied
  - duration.expired
  capabilities:
  - death_saves
  - healing
  - conditions
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 18
  engine_gaps: &id001
  - dnd2024:rules_damage_and_healing_stabilizing_a_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: acacbce3587714f6061bf15df9e758bdd4a6a8d79612ae36463760aff4161473
  parent_name: Damage and Healing
  line_span:
    first: 1278
    count: 10
splat_clauses:
- key: source_rules
  text: '# Stabilizing a Character You can take the Help action to try to stabilize a creature with 0
    Hit Points, which requires a successful DC 10 Wisdom (Medicine) check. A Stable creature doesn’t make
    Death Saving Throws even though it has 0 Hit Points, but it still has the Unconscious condition. If
    the creature takes damage, it stops being Stable and starts making Death Saving Throws again. A Stable
    creature that isn’t healed regains 1 Hit Point after 1d4 hours.'
  sha256: 319f83338a501d9c5573dab2ff9476c68b87fad1658b9d7e27adeb6f7e71f944
  status: contract_specified
splat_requires_mechanics: *id001
---

# Stabilizing a Character

<!-- SRD page 18 -->

You can take the Help action to try to stabilize a\
creature with 0 Hit Points, which requires a suc-\
cessful DC 10 Wisdom (Medicine) check.\
A Stable creature doesn’t make Death Saving\
Throws even though it has 0 Hit Points, but it still\
has the Unconscious condition. If the creature takes\
damage, it stops being Stable and starts making\
Death Saving Throws again. A Stable creature that\
isn’t healed regains 1 Hit Point after 1d4 hours.\
