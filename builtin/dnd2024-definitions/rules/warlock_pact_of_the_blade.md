---
splat_schema: 1
splat_id: e483c744-8b02-5b76-b952-e1203c8eb034
splat_kind: rule
splat_key: dnd2024:rules/warlock_pact_of_the_blade
splat_name: Pact of the Blade
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_pact_of_the_blade
  parameters:
    activation: bonus_action
    options:
      conjure:
        location: hand
        weapon_categories:
        - simple
        - martial
        weapon_kind: melee
        choice: warlock
      bond_existing:
        weapon_magic: true
        touch_required: true
        prohibited_if:
        - attuned_to_someone_else
        - bonded_to_another_warlock
    bond_benefits:
      weapon_proficiency: true
      spellcasting_focus: true
      optional_attack_and_damage_ability: charisma
      damage_type_choice:
      - normal
      - necrotic
      - psychic
      - radiant
    bond_ends_when:
    - activate_feature_again
    - warlock_dies
    - distance_feet_greater_than: 5
      continuous_minutes: 1
    conjured_weapon_disappears_when_bond_ends: true
  implementation: dnd2024:rule_warlock_pact_of_the_blade
splat_contracts:
- id: dnd2024:rule_warlock_pact_of_the_blade
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
  - feature_effects
  events:
  - bonus_action.used
  - participant.died
  capabilities:
  - invocations
  - equipment
  - attacks
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 74
  engine_gaps: &id001
  - dnd2024:rule_warlock_pact_of_the_blade
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 9ae5e7dac781fc242ebefac2e1cb015aaff54ee7a1967863cfc74544b7aff608
  parent_name: Warlock
  line_span:
    first: 6294
    count: 20
splat_clauses:
- key: source_rules
  text: '# Pact of the Blade As a Bonus Action, you can conjure a pact weapon in your hand—a Simple or
    Martial Melee weapon of your choice with which you bond—or create a bond with a magic weapon you touch;
    you can’t bond with a magic weapon if someone else is attuned to it or another Warlock is bonded with
    it. Until the bond ends, you have proficiency with the weapon, and you can use it as a Spellcasting
    Focus. Whenever you attack with the bonded weapon, you can use your Charisma modifier for the attack
    and damage rolls instead of using Strength or Dexterity; and you can cause the weapon to deal Necrotic,
    Psychic, or Radiant damage or its normal damage type. Your bond with the weapon ends if you use this
    feature’s Bonus Action again, if the weapon is more than 5 feet away from you for 1 minute or more,
    or if you die. A conjured weapon disappears when the bond ends.'
  sha256: a2ac2e21726739b85ea362fb7d11a9c09e67fb7010221231be66a7ee86b49bfc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Pact of the Blade

<!-- SRD page 74 -->

As a Bonus Action, you can conjure a pact weapon\
in your hand—a Simple or Martial Melee weapon of\
your choice with which you bond—or create a bond\
with a magic weapon you touch; you can’t bond with\
a magic weapon if someone else is attuned to it or\
another Warlock is bonded with it. Until the bond\
ends, you have proficiency with the weapon, and\
you can use it as a Spellcasting Focus.\
Whenever you attack with the bonded weapon,\
you can use your Charisma modifier for the attack\
and damage rolls instead of using Strength or\
Dexterity; and you can cause the weapon to deal\
Necrotic, Psychic, or Radiant damage or its normal\
damage type.\
Your bond with the weapon ends if you use this\
feature’s Bonus Action again, if the weapon is more\
than 5 feet away from you for 1 minute or more, or\
if you die. A conjured weapon disappears when the\
bond ends.\
