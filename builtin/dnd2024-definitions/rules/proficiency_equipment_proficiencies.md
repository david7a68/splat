---
splat_schema: 1
splat_id: 1352e174-a8f3-5417-a28c-5aa8a211ab0d
splat_kind: rule
splat_key: dnd2024:rules/proficiency_equipment_proficiencies
splat_name: Equipment Proficiencies
splat_origin:
  key: dnd2024:reference/pages/009
splat_sources:
- title: SRD 5.2.1
  locator: pp. 9
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/proficiency_equipment_proficiencies
  parameters:
    weapon_proficiency:
      required_to_wield: false
      attack_bonus: proficiency_bonus
    tool_proficiency:
      ability_check_bonus: proficiency_bonus
      matching_skill_proficiency: advantage
      proficiency_bonus_times: 1
  implementation: dnd2024:rules_proficiency_equipment_proficiencies
splat_contracts:
- id: dnd2024:rules_proficiency_equipment_proficiencies
  inputs:
    actor:
      type: participant
      required: true
    test:
      type: resolution
      required: true
  outputs:
  - test_modifiers
  - test_result
  events: []
  capabilities:
  - d20_tests
  - proficiency
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 9
  engine_gaps: &id001
  - dnd2024:rules_proficiency_equipment_proficiencies
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: d23ea0d42b14e8db6f6ddf68044f997367986102c9a7f4d4b98a4c3cb0870df6
  parent_name: Proficiency
  line_span:
    first: 435
    count: 15
splat_clauses:
- key: source_rules
  text: '# Equipment Proficiencies A character gains proficiency with various weapons and tools from their
    class and background. There are two categories of equipment proficiency: Weapons. Anyone can wield
    a weapon, but proficiency makes you better at wielding it. If you have proficiency with a weapon,
    you add your Proficiency Bonus to attack rolls you make with it. Tools. If you have proficiency with
    a tool, you can add your Proficiency Bonus to any ability check you make that uses the tool. If you
    have proficiency in the skill that’s also used with that check, you have Advantage on the check too.
    This means you can benefit from both skill proficiency and tool proficiency on the same ability check.'
  sha256: 242206ef78d765846b76108ca382d66b9d11cc6ce32d8747d04668d6a26861fa
  status: contract_specified
splat_requires_mechanics: *id001
---

# Equipment Proficiencies

<!-- SRD page 9 -->

A character gains proficiency with various weapons\
and tools from their class and background. There\
are two categories of equipment proficiency:\
Weapons. Anyone can wield a weapon, but profi-\
ciency makes you better at wielding it. If you have\
proficiency with a weapon, you add your Profi-\
ciency Bonus to attack rolls you make with it.\
Tools. If you have proficiency with a tool, you can\
add your Proficiency Bonus to any ability check\
you make that uses the tool. If you have profi-\
ciency in the skill that’s also used with that check,\
you have Advantage on the check too. This means\
you can benefit from both skill proficiency and\
tool proficiency on the same ability check.\
