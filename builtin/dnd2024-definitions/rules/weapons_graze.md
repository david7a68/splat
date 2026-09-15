---
splat_schema: 1
splat_id: 49f600ee-d551-5574-975d-3a01920ffa48
splat_kind: rule
splat_key: dnd2024:rules/weapons_graze
splat_name: Graze
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/weapons_graze
  parameters:
    trigger: weapon_attack_misses_creature
    optional_damage:
      amount: attack_ability_modifier
      type: weapon_damage_type
      increase_sources_allowed:
      - ability_modifier_increase
  implementation: dnd2024:rule_weapons_graze
splat_contracts:
- id: dnd2024:rule_weapons_graze
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
  - effects
  events:
  - attack.missed
  capabilities:
  - weapon_mastery
  - damage
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 90
  engine_gaps: &id001
  - dnd2024:rule_weapons_graze
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: a612f6ede8f980d28c6305c612f04d1fab452b962f3e9f5297472da6733cff87
  parent_name: Weapons
  line_span:
    first: 7781
    count: 7
splat_clauses:
- key: source_rules
  text: '# Graze If your attack roll with this weapon misses a creature, you can deal damage to that creature
    equal to the ability modifier you used to make the attack roll. This damage is the same type dealt
    by the weapon, and the damage can be increased only by increasing the ability modifier.'
  sha256: 7a462d31b6874d8038ce4587250b825d1f9ddd8d2a8a717f61545a294f1774a9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Graze

<!-- SRD page 90 -->

If your attack roll with this weapon misses a crea-\
ture, you can deal damage to that creature equal\
to the ability modifier you used to make the attack\
roll. This damage is the same type dealt by the\
weapon, and the damage can be increased only by\
increasing the ability modifier.\
