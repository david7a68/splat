---
splat_schema: 1
splat_id: b22461fd-0a88-51ea-9cd9-05f24fdce28a
splat_kind: rule
splat_key: dnd2024:rules/warlock_investment_of_the_chain_master
splat_name: Investment of the Chain Master
splat_origin:
  key: dnd2024:reference/pages/073
splat_sources:
- title: SRD 5.2.1
  locator: pp. 73
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_investment_of_the_chain_master
  parameters:
    prerequisites:
      warlock_level_minimum: 5
      invocation: pact_of_the_chain
    trigger: find_familiar.cast
    familiar_benefits:
      movement_choice:
        options:
        - fly
        - swim
        speed_feet: 40
      command_attack:
        economy: bonus_action
        familiar_action: attack
      damage_type_choice:
        replace_types:
        - bludgeoning
        - piercing
        - slashing
        options:
        - necrotic
        - radiant
        optional: true
        timing: each_damage_instance
      saving_throw_dc: caster_spell_save_dc
      damage_resistance:
        trigger: familiar.damage_pending
        economy: reaction
        scope: triggering_damage
  implementation: dnd2024:rules_warlock_investment_of_the_chain_master
splat_contracts:
- id: dnd2024:rules_warlock_investment_of_the_chain_master
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    choice:
      type: choice
      required: true
    event:
      required: false
      type: event
  outputs:
  - effects
  - resource_updates
  events:
  - spell.cast_completed
  - damage.pending
  capabilities:
  - class_features
  - effects
  - resources
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 73
  engine_gaps: &id001
  - dnd2024:rules_warlock_investment_of_the_chain_master
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 9565da0b99a876c3f0ddca96dc018dd45e99659d3a0986d2f22ae13f3883f10f
  parent_name: Warlock
  line_span:
    first: 6236
    count: 19
splat_clauses:
- key: source_rules
  text: '# Investment of the Chain Master Prerequisite: Level 5+ Warlock, Pact of the Chain Invocation
    When you cast Find Familiar, you infuse the summoned familiar with a measure of your eldritch power,
    granting the creature the following benefits. Aerial or Aquatic. The familiar gains either a Fly Speed
    or a Swim Speed (your choice) of 40 feet. Quick Attack. As a Bonus Action, you can command the familiar
    to take the Attack action. Necrotic or Radiant Damage. Whenever the familiar deals Bludgeoning, Piercing,
    or Slashing damage, you can make it deal Necrotic or Radiant damage instead. Your Save DC. If the
    familiar forces a creature to make a saving throw, it uses your spell save DC. Resistance. When the
    familiar takes damage, you can take a Reaction to grant it Resistance against that damage.'
  sha256: 17759c0e3909449f3e29d9f455737e531381ab63505166ac6650b9f71e5210ab
  status: contract_specified
splat_requires_mechanics: *id001
---

# Investment of the Chain Master

<!-- SRD page 73 -->

Prerequisite: Level 5+ Warlock, Pact of the Chain\
Invocation\
When you cast Find Familiar, you infuse the sum-\
moned familiar with a measure of your eldritch\
power, granting the creature the following benefits.\
Aerial or Aquatic. The familiar gains either a Fly\
Speed or a Swim Speed (your choice) of 40 feet.\
Quick Attack. As a Bonus Action, you can com-\
mand the familiar to take the Attack action.\
Necrotic or Radiant Damage. Whenever the\
familiar deals Bludgeoning, Piercing, or Slashing\
damage, you can make it deal Necrotic or Radiant\
damage instead.\
Your Save DC. If the familiar forces a creature to\
make a saving throw, it uses your spell save DC.\
Resistance. When the familiar takes damage, you\
can take a Reaction to grant it Resistance against\
that damage.\
