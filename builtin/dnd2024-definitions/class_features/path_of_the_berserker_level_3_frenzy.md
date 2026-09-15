---
splat_schema: 1
splat_id: f2e4240c-f3bf-5302-9f6c-66c6657609e2
splat_kind: rule
splat_key: dnd2024:class_features/path_of_the_berserker_level_3_frenzy
splat_name: 'Level 3: Frenzy'
splat_origin:
  key: dnd2024:reference/pages/030
splat_sources:
- title: SRD 5.2.1
  locator: pp. 30
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/path_of_the_berserker_level_3_frenzy
  parameters:
    when:
      rage_active: true
      reckless_attack_used: true
      turn: self
      attack_ability: strength
      first_target_hit_on_turn: true
    extra_damage:
      roll:
        dice:
          count:
            stat: rage.damage_bonus
          sides: 6
      type: triggering_weapon_or_unarmed_strike_damage_type
  implementation: dnd2024:class_features_path_of_the_berserker_level_3_frenzy
splat_contracts:
- id: dnd2024:class_features_path_of_the_berserker_level_3_frenzy
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
  - attack.hit
  capabilities:
  - attacks
  - damage
  - rage
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 30
  engine_gaps: &id001
  - dnd2024:class_features_path_of_the_berserker_level_3_frenzy
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 99db2bd90d210cca17348dd39dc980acb16fd0c8a7d84a66ca4d11239d8ce7b6
  parent_name: Path of the Berserker
  line_span:
    first: 2459
    count: 8
splat_clauses:
- key: source_rules
  text: '# Level 3: Frenzy If you use Reckless Attack while your Rage is active, you deal extra damage
    to the first target you hit on your turn with a Strength-based attack. To determine the extra damage,
    roll a number of d6s equal to your Rage Damage bonus, and add them together. The damage has the same
    type as the weapon or Unarmed Strike used for the attack.'
  sha256: dfa859c31cee84c5e3a3f9cddebc88d8fe8f6c99b29fc066b4b3278d1d459125
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Frenzy

<!-- SRD page 30 -->

If you use Reckless Attack while your Rage is active,\
you deal extra damage to the first target you hit on\
your turn with a Strength-based attack. To deter-\
mine the extra damage, roll a number of d6s equal\
to your Rage Damage bonus, and add them together.\
The damage has the same type as the weapon or\
Unarmed Strike used for the attack.\
