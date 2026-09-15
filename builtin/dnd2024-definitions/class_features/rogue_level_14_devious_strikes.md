---
splat_schema: 1
splat_id: e102040a-fbb9-5eb3-9f47-0dae6fe81a30
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_14_devious_strikes
splat_name: 'Level 14: Devious Strikes'
splat_origin:
  key: dnd2024:reference/pages/063
splat_sources:
- title: SRD 5.2.1
  locator: pp. 63
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/rogue_level_14_devious_strikes
  parameters:
    extends: rogue.cunning_strike
    save_dc:
      op: add
      args:
      - 8
      - var: actor.abilities.dexterity.modifier
      - var: actor.proficiency_bonus
    cost_resource: sneak_attack.d6_dice
    options:
      daze:
        cost: 2
        save: constitution
        failure:
          during: target.next_turn
          activity_limit:
            choose_only_one:
            - move
            - action
            - bonus_action
      knock_out:
        cost: 6
        save: constitution
        failure:
          condition: unconscious
          duration:
            minutes: 1
          ends_on:
          - target.damage_taken
          repeat_save:
            event: target.turn.end
            success: end_effect
      obscure:
        cost: 3
        save: dexterity
        failure:
          condition: blinded
          end: target.next_turn.end
    class_scope:
      class: rogue
      minimum_level: 14
  implementation: dnd2024:class_features_rogue_level_14_devious_strikes
splat_contracts:
- id: dnd2024:class_features_rogue_level_14_devious_strikes
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - sneak_attack.damage_pending
  - turn.end
  - damage.taken
  capabilities:
  - character_features
  - resources
  - damage
  - conditions
  - proficiencies
  - d20_tests
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 63
  engine_gaps: &id001
  - dnd2024:class_features_rogue_level_14_devious_strikes
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 34069d4566e46751e4868efb8aa52189747caa781c7050559e0f592b3a576557
  parent_name: Rogue
  line_span:
    first: 5315
    count: 17
splat_clauses:
- key: source_rules
  text: '# Level 14: Devious Strikes You’ve practiced new ways to use your Sneak Attack deviously. The
    following effects are now among your Cunning Strike options. Daze (Cost: 2d6). The target must succeed
    on a Constitution saving throw, or on its next turn, it can do only one of the following: move or
    take an action or a Bonus Action. Knock Out (Cost: 6d6). The target must succeed on a Constitution
    saving throw, or it has the Unconscious condition for 1 minute or until it takes any damage. The Unconscious
    target repeats the save at the end of each of its turns, ending the effect on itself on a success.
    Obscure (Cost: 3d6). The target must succeed on a Dexterity saving throw, or it has the Blinded condition
    until the end of its next turn.'
  sha256: b5b84926952f8bd11c3d72bb83f49720f2ca70b586e07a5cb230fc77d2ecf65c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 14: Devious Strikes

<!-- SRD page 63 -->

You’ve practiced new ways to use your Sneak Attack\
deviously. The following effects are now among\
your Cunning Strike options.\
Daze (Cost: 2d6). The target must succeed on a\
Constitution saving throw, or on its next turn, it can\
do only one of the following: move or take an action\
or a Bonus Action.\
Knock Out (Cost: 6d6). The target must succeed\
on a Constitution saving throw, or it has the Uncon-\
scious condition for 1 minute or until it takes any\
damage. The Unconscious target repeats the save\
at the end of each of its turns, ending the effect on\
itself on a success.\
Obscure (Cost: 3d6). The target must succeed on\
a Dexterity saving throw, or it has the Blinded con-\
dition until the end of its next turn.\
