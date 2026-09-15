---
splat_schema: 1
splat_id: 2a99d2e1-62c2-52c0-95f6-16a7715c9305
splat_kind: rule
splat_key: dnd2024:rules/weapons_ammunition
splat_name: Ammunition
splat_origin:
  key: dnd2024:reference/pages/089
splat_sources:
- title: SRD 5.2.1
  locator: pp. 89
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/weapons_ammunition
  parameters:
    ranged_attack_requires: compatible_ammunition
    ammunition_consumed_per_attack: 1
    draw_ammunition_cost: part_of_attack
    load_one_handed_weapon_requires_free_hand: true
    recover_after_combat:
      time_minutes: 1
      quantity:
        op: floor
        args:
        - op: divide
          args:
          - variable: ammunition_used
          - 2
      remaining_ammunition: lost
  implementation: dnd2024:rule_weapons_ammunition
splat_contracts:
- id: dnd2024:rule_weapons_ammunition
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    ammunition_used:
      type: number
      required: true
  outputs:
  - item_effects
  events: []
  capabilities:
  - equipment
  - attacks
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 89
  engine_gaps: &id001
  - dnd2024:rule_weapons_ammunition
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 62605678740544f8e409985d54927e03be06ef097078160c081dda9394e89ba5
  parent_name: Weapons
  line_span:
    first: 7695
    count: 11
splat_clauses:
- key: source_rules
  text: '# Ammunition You can use a weapon that has the Ammunition property to make a ranged attack only
    if you have ammunition to fire from it. The type of ammunition required is specified with the weapon’s
    range. Each attack expends one piece of ammunition. Drawing the ammunition is part of the attack (you
    need a free hand to load a one-handed weapon). After a fight, you can spend 1 minute to recover half
    the ammunition (round down) you used in the fight; the rest is lost.'
  sha256: d32a1d02076b7d52d65c620228525dd8d86872282c6d155f9b5dd046ba19dc38
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ammunition

<!-- SRD page 89 -->

You can use a weapon that has the Ammunition\
property to make a ranged attack only if you have\
ammunition to fire from it. The type of ammunition\
required is specified with the weapon’s range. Each\
attack expends one piece of ammunition. Drawing\
the ammunition is part of the attack (you need a\
free hand to load a one-handed weapon). After a\
fight, you can spend 1 minute to recover half the\
ammunition (round down) you used in the fight; the\
rest is lost.\
