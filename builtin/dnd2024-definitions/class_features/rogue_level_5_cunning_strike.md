---
splat_schema: 1
splat_id: 5ef5564c-4227-52a8-b5ae-9e2113ffe8f1
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_5_cunning_strike
splat_name: 'Level 5: Cunning Strike'
splat_origin:
  key: dnd2024:reference/pages/063
splat_sources:
- title: SRD 5.2.1
  locator: pp. 63
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/rogue_level_5_cunning_strike
  parameters:
    trigger: sneak_attack.damage_pending
    maximum_effect_choices: 1
    cost:
      resource: sneak_attack_damage_dice
      remove_before_roll: true
    effect_timing: immediately_after_attack_damage
    save_dc:
      op: add
      args:
      - 8
      - stat: abilities.dexterity.modifier
      - stat: proficiency_bonus
    options:
    - id: poison
      cost_d6: 1
      requires_carried_item: poisoners_kit
      save:
        ability: constitution
        failure_condition: poisoned
        duration:
          minutes: 1
        repeat:
          event: target.turn_ended
          success_ends: true
    - id: trip
      cost_d6: 1
      maximum_target_size: large
      save:
        ability: dexterity
        failure_condition: prone
    - id: withdraw
      cost_d6: 1
      move:
        maximum_feet:
          op: divide
          args:
          - stat: speed
          - 2
        provokes_opportunity_attacks: false
  implementation: dnd2024:class_features_rogue_level_5_cunning_strike
splat_contracts:
- id: dnd2024:class_features_rogue_level_5_cunning_strike
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
    destination:
      required: false
      type: location
  outputs:
  - feature_state
  - movement
  - conditions
  events:
  - sneak_attack.damage_pending
  - attack.damage_completed
  - turn.ended
  capabilities:
  - damage_dice_costs
  - conditions
  - movement
  - saving_throws
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 63
  engine_gaps: &id001
  - dnd2024:class_features_rogue_level_5_cunning_strike
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: adfe641194d71cdaefd9599a9abed7ae06e29a2e8cdb629f29428f4eaf5629b5
  parent_name: Rogue
  line_span:
    first: 5267
    count: 28
splat_clauses:
- key: source_rules
  text: '# Level 5: Cunning Strike You’ve developed cunning ways to use your Sneak Attack. When you deal
    Sneak Attack damage, you can add one of the following Cunning Strike effects. Each effect has a die
    cost, which is the number of Sneak Attack damage dice you must forgo to add the effect. You remove
    the die before rolling, and the effect occurs immediately after the attack’s damage is dealt. For
    example, if you add the Poison effect, remove 1d6 from the Sneak Attack’s damage before rolling. If
    a Cunning Strike effect requires a saving throw, the DC equals 8 plus your Dexterity modifier and
    Proficiency Bonus. Poison (Cost: 1d6). You add a toxin to your strike, forcing the target to make
    a Constitution saving throw. On a failed save, the target has the Poisoned condition for 1 minute.
    At the end of each of its turns, the Poisoned target repeats the save, ending the effect on itself
    on a success. To use this effect, you must have a Poisoner’s Kit on your person. Trip (Cost: 1d6).
    If the target is Large or smaller, it must succeed on a Dexterity saving throw or have the Prone condition.
    Withdraw (Cost: 1d6). Immediately after the attack, you move up to half your Speed without provoking
    Opportunity Attacks.'
  sha256: f88090b2c1094e56e1f08b0999c71432200f2af7a1273153482ff4280e982a8d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 5: Cunning Strike

<!-- SRD page 63 -->

You’ve developed cunning ways to use your Sneak\
Attack. When you deal Sneak Attack damage, you\
can add one of the following Cunning Strike effects.\
Each effect has a die cost, which is the number of\
Sneak Attack damage dice you must forgo to add the\
effect. You remove the die before rolling, and the\
effect occurs immediately after the attack’s damage\
is dealt. For example, if you add the Poison effect,\
remove 1d6 from the Sneak Attack’s damage before\
rolling.\
If a Cunning Strike effect requires a saving throw,\
the DC equals 8 plus your Dexterity modifier and\
Proficiency Bonus.\
Poison (Cost: 1d6). You add a toxin to your strike,\
forcing the target to make a Constitution saving\
throw. On a failed save, the target has the Poisoned\
condition for 1 minute. At the end of each of its\
turns, the Poisoned target repeats the save, ending\
the effect on itself on a success.\
To use this effect, you must have a Poisoner’s Kit\
on your person.\
Trip (Cost: 1d6). If the target is Large or smaller,\
it must succeed on a Dexterity saving throw or have\
the Prone condition.\
Withdraw (Cost: 1d6). Immediately after the\
attack, you move up to half your Speed without pro-\
voking Opportunity Attacks.\
