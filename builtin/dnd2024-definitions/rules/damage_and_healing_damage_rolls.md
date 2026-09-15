---
splat_schema: 1
splat_id: 59db096b-3ba9-530b-9514-f7084d964f31
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_damage_rolls
splat_name: Damage Rolls
splat_origin:
  key: dnd2024:reference/pages/016
splat_sources:
- title: SRD 5.2.1
  locator: pp. 16
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/damage_and_healing_damage_rolls
  parameters:
    damage:
      op: max
      terms:
      - 0
      - op: sum
        terms:
        - damage_dice
        - modifiers
    weapon_roll_ability_modifier: same_as_attack
    spell_modifiers: specified_by_spell
    fixed_damage_ability_modifier_default: false
  implementation: dnd2024:rules_damage_and_healing_damage_rolls
splat_contracts:
- id: dnd2024:rules_damage_and_healing_damage_rolls
  inputs:
    target:
      type: participant
      required: true
    damage:
      required: false
      type: resolution
    event:
      type: event
      required: true
  outputs:
  - hit_point_updates
  - condition_updates
  - death_save_updates
  events: []
  capabilities:
  - damage
  - healing
  - death_saves
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 16
  engine_gaps: &id001
  - dnd2024:rules_damage_and_healing_damage_rolls
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 1418b5e081c9f09f9fd0b5db61cb75b4150c5bae60ce146b572473914e660539
  parent_name: Damage and Healing
  line_span:
    first: 1127
    count: 15
splat_clauses:
- key: source_rules
  text: '# Damage Rolls Each weapon, spell, and damaging monster ability specifies the damage it deals.
    You roll the damage dice, add any modifiers, and deal the damage to your target. If there’s a penalty
    to the damage, it’s possible to deal 0 damage but not negative damage. When attacking with a weapon,
    you add your ability modifier—the same modifier used for the attack roll—to the damage roll. A spell
    tells you which dice to roll for damage and whether to add any modifiers. Unless a rule says otherwise,
    you don’t add your ability modifier to a fixed damage amount that doesn’t use a roll, such as the
    damage of a Blowgun. See “Equipment” for weapons’ damage dice and “Spells” for spells’ damage dice.'
  sha256: 2c083b9b5be9e45c44438b7135753b27c74f6494e7c09e556bb1596e4f15b05c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Damage Rolls

<!-- SRD page 16 -->

Each weapon, spell, and damaging monster ability\
specifies the damage it deals. You roll the damage\
dice, add any modifiers, and deal the damage to\
your target. If there’s a penalty to the damage, it’s\
possible to deal 0 damage but not negative damage.\
When attacking with a weapon, you add your abil-\
ity modifier—the same modifier used for the attack\
roll—to the damage roll. A spell tells you which dice\
to roll for damage and whether to add any modi-\
fiers. Unless a rule says otherwise, you don’t add\
your ability modifier to a fixed damage amount that\
doesn’t use a roll, such as the damage of a Blowgun.\
See “Equipment” for weapons’ damage dice and\
“Spells” for spells’ damage dice.\
