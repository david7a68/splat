---
format: 1
source: SRD 5.2.1
definitions:
  dnd2024:class_features/barbarian_level_1_rage:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/barbarian_level_1_rage
        implementation: dnd2024:class_features_barbarian_level_1_rage
        parameters:
          action: bonus_action
          requires:
            wearing_heavy_armor: false
          resource:
            id: barbarian.rage
            maximum:
              table:
                key: dnd2024:tables/barbarian_progression
              row:
                var: actor.class_levels.barbarian
              column: rages
            recovery:
            - event: rest.short_completed
              amount: 1
            - event: rest.long_completed
              amount: all
          cost: 1
          duration:
            end: actor.next_turn.end
            maximum:
              minutes: 10
          ends_on:
          - actor.heavy_armor_donned
          - actor.incapacitated
          resistances:
          - bludgeoning
          - piercing
          - slashing
          damage_bonus:
            requires:
              attack_ability: strength
              attack_kind:
              - weapon
              - unarmed_strike
              damage_dealt: true
            amount:
              table:
                key: dnd2024:tables/barbarian_progression
              row:
                var: actor.class_levels.barbarian
              column: rage_damage
          advantage:
          - strength.check
          - strength.save
          prohibits:
          - concentration
          - spellcasting
          extend:
            requires:
              rage_active: true
              turn: actor.turn_after_activation
            any_event:
            - attack.roll_against_enemy
            - enemy.saving_throw_forced
            - bonus_action.rage_extend
            end: actor.next_turn.end
          class_scope:
            class: barbarian
            minimum_level: 1
    contract:
      id: dnd2024:class_features_barbarian_level_1_rage
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - action.bonus
      - turn.start
      - turn.end
      - attack.damage
      - ability.check
      - saving_throw
      - condition.applied
      - equipment.donned
      - rest.short_completed
      - rest.long_completed
      capabilities:
      - character_features
      - resources
      - spellcasting
      - damage
      - d20_tests
    requires:
    - dnd2024:class_features_barbarian_level_1_rage
    citations:
    - SRD 5.2.1 pp. 28, 29; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/barbarian_level_1_rage.md
  dnd2024:class_features/barbarian_level_1_weapon_mastery:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/barbarian_level_1_weapon_mastery
        implementation: dnd2024:class_features_barbarian_level_1_weapon_mastery
        parameters:
          mastery_choices:
            count:
              table:
                key: dnd2024:tables/barbarian_progression
              row:
                var: actor.class_levels.barbarian
              column: weapon_mastery
            eligible:
              weapon_category:
              - simple
              - martial
              weapon_range: melee
            replacement:
              event: rest.long_completed
              maximum: 1
          class_scope:
            class: barbarian
            minimum_level: 1
    contract:
      id: dnd2024:class_features_barbarian_level_1_weapon_mastery
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.level_gained
      - rest.long_completed
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_barbarian_level_1_weapon_mastery
    citations:
    - SRD 5.2.1 pp. 29; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/barbarian_level_1_weapon_mastery.md
  dnd2024:class_features/barbarian_level_2_reckless_attack:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/barbarian_level_2_reckless_attack
        implementation: dnd2024:class_features_barbarian_level_2_reckless_attack
        parameters:
          optional: true
          trigger: actor.turn.first_attack_roll
          duration_end: actor.next_turn.start
          advantage:
            actor_attacks:
              ability: strength
            attacks_against_actor: true
          class_scope:
            class: barbarian
            minimum_level: 2
    contract:
      id: dnd2024:class_features_barbarian_level_2_reckless_attack
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - attack.roll_pending
      - turn.start
      capabilities:
      - character_features
      - d20_tests
    requires:
    - dnd2024:class_features_barbarian_level_2_reckless_attack
    citations:
    - SRD 5.2.1 pp. 29; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/barbarian_level_2_reckless_attack.md
  dnd2024:class_features/barbarian_level_3_primal_knowledge:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/barbarian_level_3_primal_knowledge
        implementation: dnd2024:class_features_barbarian_level_3_primal_knowledge
        parameters:
          skill_proficiency:
            count: 1
            choices:
            - animal_handling
            - athletics
            - intimidation
            - nature
            - perception
            - survival
          ability_substitution:
            requires:
              rage_active: true
            skills:
            - acrobatics
            - intimidation
            - perception
            - stealth
            - survival
            ability: strength
            optional: true
          class_scope:
            class: barbarian
            minimum_level: 3
    contract:
      id: dnd2024:class_features_barbarian_level_3_primal_knowledge
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.feature_acquired
      - ability.check_pending
      capabilities:
      - character_features
      - proficiencies
    requires:
    - dnd2024:class_features_barbarian_level_3_primal_knowledge
    citations:
    - SRD 5.2.1 pp. 29; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/barbarian_level_3_primal_knowledge.md
  dnd2024:class_features/barbarian_level_5_extra_attack:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/barbarian_level_5_extra_attack
        implementation: dnd2024:class_features_barbarian_level_5_extra_attack
        parameters:
          action: attack
          turn: actor
          attack_count: 2
          class_scope:
            class: barbarian
            minimum_level: 5
    contract:
      id: dnd2024:class_features_barbarian_level_5_extra_attack
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - action.attack
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_barbarian_level_5_extra_attack
    citations:
    - SRD 5.2.1 pp. 29; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/barbarian_level_5_extra_attack.md
  dnd2024:class_features/barbarian_level_7_feral_instinct:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/barbarian_level_7_feral_instinct
        implementation: dnd2024:class_features_barbarian_level_7_feral_instinct
        parameters:
          advantage:
          - initiative
          class_scope:
            class: barbarian
            minimum_level: 7
    contract:
      id: dnd2024:class_features_barbarian_level_7_feral_instinct
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - initiative.roll
      capabilities:
      - character_features
      - d20_tests
      - initiative
    requires:
    - dnd2024:class_features_barbarian_level_7_feral_instinct
    citations:
    - SRD 5.2.1 pp. 29; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/barbarian_level_7_feral_instinct.md
  dnd2024:class_features/barbarian_level_9_brutal_strike:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/barbarian_level_9_brutal_strike
        implementation: dnd2024:class_features_barbarian_level_9_brutal_strike
        parameters:
          requires:
            reckless_attack_used: true
            attack_ability: strength
            attack_disadvantage: false
            turn: actor
          limit:
            count: 1
            period: actor.turn
          cost:
            forgo_all_advantage: true
          on_hit:
            extra_damage:
              dice:
                count: 1
                sides: 10
              type: attack.weapon_or_unarmed_damage_type
            choose_effects: 1
            options:
              forceful_blow:
                push:
                  feet: 15
                  direction: straight_away_from_actor
                follow_movement:
                  maximum_feet:
                    op: multiply
                    args:
                    - var: actor.speed
                    - 0.5
                  direction: straight_toward_target
                  opportunity_attacks: false
              hamstring_blow:
                speed_bonus_feet: -15
                end: actor.next_turn.start
                stacking: latest_only
          class_scope:
            class: barbarian
            minimum_level: 9
    contract:
      id: dnd2024:class_features_barbarian_level_9_brutal_strike
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - attack.roll_pending
      - attack.hit
      - turn.start
      capabilities:
      - character_features
      - damage
      - d20_tests
      - movement
    requires:
    - dnd2024:class_features_barbarian_level_9_brutal_strike
    citations:
    - SRD 5.2.1 pp. 29, 30; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/barbarian_level_9_brutal_strike.md
  dnd2024:class_features/barbarian_level_13_improved_brutal_strike:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/barbarian_level_13_improved_brutal_strike
        implementation: dnd2024:class_features_barbarian_level_13_improved_brutal_strike
        parameters:
          extends:
            key: dnd2024:class_features/barbarian_level_9_brutal_strike
          options:
            staggering_blow:
              next_saving_throw: disadvantage
              opportunity_attacks: false
              opportunity_attack_prohibition_end: actor.next_turn.start
            sundering_blow:
              next_attack:
                attacker_not_actor: true
                target: struck_creature
                bonus: 5
                before: actor.next_turn.start
                maximum_stacking_instances: 1
          class_scope:
            class: barbarian
            minimum_level: 13
    contract:
      id: dnd2024:class_features_barbarian_level_13_improved_brutal_strike
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - attack.hit
      - saving_throw
      - attack.roll_pending
      - turn.start
      capabilities:
      - character_features
      - d20_tests
    requires:
    - dnd2024:class_features_barbarian_level_13_improved_brutal_strike
    citations:
    - SRD 5.2.1 pp. 30; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/barbarian_level_13_improved_brutal_strike.md
  dnd2024:class_features/barbarian_level_17_improved_brutal_strike:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/barbarian_level_17_improved_brutal_strike
        implementation: dnd2024:class_features_barbarian_level_17_improved_brutal_strike
        parameters:
          modifies:
            key: dnd2024:class_features/barbarian_level_9_brutal_strike
          extra_damage_dice:
            count: 2
            sides: 10
          effects_chosen: 2
          effects_distinct: true
          class_scope:
            class: barbarian
            minimum_level: 17
    contract:
      id: dnd2024:class_features_barbarian_level_17_improved_brutal_strike
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - attack.hit
      capabilities:
      - character_features
      - damage
    requires:
    - dnd2024:class_features_barbarian_level_17_improved_brutal_strike
    citations:
    - SRD 5.2.1 pp. 30; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/barbarian_level_17_improved_brutal_strike.md
  dnd2024:class_features/barbarian_level_19_epic_boon:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/barbarian_level_19_epic_boon
        implementation: dnd2024:class_features_barbarian_level_19_epic_boon
        parameters:
          feat_choice:
            count: 1
            category_preference: epic_boon
            allowed_categories: any
            must_meet_prerequisites: true
          class_scope:
            class: barbarian
            minimum_level: 19
    contract:
      id: dnd2024:class_features_barbarian_level_19_epic_boon
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.feature_acquired
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_barbarian_level_19_epic_boon
    citations:
    - SRD 5.2.1 pp. 30; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/barbarian_level_19_epic_boon.md
  dnd2024:subclasses/barbarian_path_of_the_berserker:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:subclasses/barbarian_path_of_the_berserker
        implementation: dnd2024:subclasses_barbarian_path_of_the_berserker
        parameters:
          class: barbarian
          minimum_class_level: 3
          feature_grants:
          - class_level: 10
            definition:
              key: dnd2024:class_features/path_of_the_berserker_level_10_retaliation
          - class_level: 14
            definition:
              key: dnd2024:class_features/path_of_the_berserker_level_14_intimidating_presence
          - class_level: 3
            definition:
              key: dnd2024:class_features/path_of_the_berserker_level_3_frenzy
          - class_level: 6
            definition:
              key: dnd2024:class_features/path_of_the_berserker_level_6_mindless_rage
          grant_eligible_features:
            'on':
            - subclass.selected
            - character.level_gained
            comparison: class_level_at_least_threshold
            persistent: true
    contract:
      id: dnd2024:subclasses_barbarian_path_of_the_berserker
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - feature_grants
      events:
      - subclass.selected
      - character.level_gained
      capabilities:
      - character_features
    requires:
    - dnd2024:subclasses_barbarian_path_of_the_berserker
    citations:
    - SRD 5.2.1 pp. 30; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/subclasses/barbarian_path_of_the_berserker.md
  dnd2024:class_features/path_of_the_berserker_level_6_mindless_rage:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/path_of_the_berserker_level_6_mindless_rage
        implementation: dnd2024:class_features_path_of_the_berserker_level_6_mindless_rage
        parameters:
          while:
            rage_active: true
          condition_immunities:
          - charmed
          - frightened
          rage_activation_ends_conditions:
          - charmed
          - frightened
          class_scope:
            class: barbarian
            minimum_level: 6
    contract:
      id: dnd2024:class_features_path_of_the_berserker_level_6_mindless_rage
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - rage.started
      - condition.applied
      - rage.ended
      capabilities:
      - character_features
      - conditions
    requires:
    - dnd2024:class_features_path_of_the_berserker_level_6_mindless_rage
    citations:
    - SRD 5.2.1 pp. 30; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/path_of_the_berserker_level_6_mindless_rage.md
  dnd2024:class_features/path_of_the_berserker_level_14_intimidating_presence:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/path_of_the_berserker_level_14_intimidating_presence
        implementation: dnd2024:class_features_path_of_the_berserker_level_14_intimidating_presence
        parameters:
          action: bonus_action
          resource:
            id: berserker.intimidating_presence
            maximum: 1
            recovery:
            - event: rest.long_completed
              amount: all
          cost: 1
          recharge_alternative:
            action: none
            cost:
              resource: barbarian.rage
              amount: 1
            restore_uses: 1
          area:
            shape: emanation
            radius_feet: 30
            origin: actor
          targets:
            type: creature
            selection: actor_choice
          save:
            ability: wisdom
            dc:
              op: add
              args:
              - 8
              - var: actor.abilities.strength.modifier
              - var: actor.proficiency_bonus
            failure:
              condition: frightened
              source: actor
              duration:
                minutes: 1
            repeat:
              event: target.turn.end
              success: end_effect
          class_scope:
            class: barbarian
            minimum_level: 14
    contract:
      id: dnd2024:class_features_path_of_the_berserker_level_14_intimidating_presence
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - action.bonus
      - turn.end
      - rest.long_completed
      capabilities:
      - character_features
      - resources
      - conditions
      - proficiencies
      - d20_tests
    requires:
    - dnd2024:class_features_path_of_the_berserker_level_14_intimidating_presence
    citations:
    - SRD 5.2.1 pp. 30; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/path_of_the_berserker_level_14_intimidating_presence.md
  dnd2024:class_features/bard_level_1_bardic_inspiration:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/bard_level_1_bardic_inspiration
        implementation: dnd2024:class_features_bard_level_1_bardic_inspiration
        parameters:
          action: bonus_action
          resource:
            id: bard.bardic_inspiration
            maximum:
              op: max
              args:
              - 1
              - var: actor.abilities.charisma.modifier
            recovery:
            - event: rest.long_completed
              amount: all
          cost: 1
          target:
            creature: true
            self: false
            range_feet: 60
            perceives_actor_any:
            - sight
            - hearing
          die:
            sides_by_bard_level:
              1: 6
              5: 8
              10: 10
              15: 12
            count: 1
          duration:
            hours: 1
          maximum_dice_per_creature: 1
          use:
            trigger: target.d20_test.failed
            optional: true
            roll_die: true
            add_to: d20
            recompute_success: true
            consume: on_roll
            maximum_uses: 1
          class_scope:
            class: bard
            minimum_level: 1
    contract:
      id: dnd2024:class_features_bard_level_1_bardic_inspiration
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - action.bonus
      - d20_test.failed
      - rest.long_completed
      capabilities:
      - character_features
      - resources
    requires:
    - dnd2024:class_features_bard_level_1_bardic_inspiration
    citations:
    - SRD 5.2.1 pp. 31, 32; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/bard_level_1_bardic_inspiration.md
  dnd2024:class_features/bard_level_2_expertise:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/bard_level_2_expertise
        implementation: dnd2024:class_features_bard_level_2_expertise
        parameters:
          expertise_grants:
          - bard_level: 2
            count: 2
            eligible:
              proficient: true
              expertise: false
            grant: expertise
          - bard_level: 9
            count: 2
            eligible:
              proficient: true
              expertise: false
            grant: expertise
          class_scope:
            class: bard
            minimum_level: 2
    contract:
      id: dnd2024:class_features_bard_level_2_expertise
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.level_gained
      capabilities:
      - character_features
      - proficiencies
    requires:
    - dnd2024:class_features_bard_level_2_expertise
    citations:
    - SRD 5.2.1 pp. 32; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/bard_level_2_expertise.md
  dnd2024:class_features/bard_level_3_bard_subclass:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/bard_level_3_bard_subclass
        implementation: dnd2024:class_features_bard_level_3_bard_subclass
        parameters:
          subclass_choice:
            count: 1
            class: bard
          feature_grant_policy:
            class_level_at_least_feature_level: true
            events:
            - subclass.selected
            - character.level_gained
            persistent: true
          class_scope:
            class: bard
            minimum_level: 3
    contract:
      id: dnd2024:class_features_bard_level_3_bard_subclass
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - subclass
      - feature_grants
      events:
      - subclass.selected
      - character.level_gained
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_bard_level_3_bard_subclass
    citations:
    - SRD 5.2.1 pp. 32; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/bard_level_3_bard_subclass.md
  dnd2024:class_features/bard_level_5_font_of_inspiration:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/bard_level_5_font_of_inspiration
        implementation: dnd2024:class_features_bard_level_5_font_of_inspiration
        parameters:
          resource_recovery_override:
            id: bard.bardic_inspiration
            maximum:
              op: max
              args:
              - 1
              - var: actor.abilities.charisma.modifier
            recovery:
            - event: rest.short_completed
              amount: all
            - event: rest.long_completed
              amount: all
          conversion:
            action: none
            consume:
              resource: spell_slot
              count: 1
              minimum_level: 1
            restore:
              resource: bard.bardic_inspiration
              amount: 1
              expended_only: true
          class_scope:
            class: bard
            minimum_level: 5
    contract:
      id: dnd2024:class_features_bard_level_5_font_of_inspiration
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - rest.short_completed
      - rest.long_completed
      - resource.convert
      capabilities:
      - character_features
      - resources
      - spellcasting
    requires:
    - dnd2024:class_features_bard_level_5_font_of_inspiration
    citations:
    - SRD 5.2.1 pp. 32; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/bard_level_5_font_of_inspiration.md
  dnd2024:class_features/bard_level_10_magical_secrets:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/bard_level_10_magical_secrets
        implementation: dnd2024:class_features_bard_level_10_magical_secrets
        parameters:
          spell_list_expansion:
          - bard
          - cleric
          - druid
          - wizard
          applies_to:
            new_preparations:
              event: bard.level_gained
              minimum_level: 10
              prepared_count_increased: true
            replacements: true
          counts_as_class: bard
          other_preparation_constraints_unchanged: true
          class_scope:
            class: bard
            minimum_level: 10
    contract:
      id: dnd2024:class_features_bard_level_10_magical_secrets
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.level_gained
      - spell.preparation_changed
      capabilities:
      - character_features
      - spellcasting
    requires:
    - dnd2024:class_features_bard_level_10_magical_secrets
    citations:
    - SRD 5.2.1 pp. 33; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/bard_level_10_magical_secrets.md
  dnd2024:class_features/bard_level_19_epic_boon:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/bard_level_19_epic_boon
        implementation: dnd2024:class_features_bard_level_19_epic_boon
        parameters:
          feat_choice:
            count: 1
            category_preference: epic_boon
            allowed_categories: any
            must_meet_prerequisites: true
          class_scope:
            class: bard
            minimum_level: 19
    contract:
      id: dnd2024:class_features_bard_level_19_epic_boon
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.feature_acquired
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_bard_level_19_epic_boon
    citations:
    - SRD 5.2.1 pp. 33; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/bard_level_19_epic_boon.md
  dnd2024:subclasses/bard_college_of_lore:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:subclasses/bard_college_of_lore
        implementation: dnd2024:subclasses_bard_college_of_lore
        parameters:
          class: bard
          minimum_class_level: 3
          feature_grants:
          - class_level: 14
            definition:
              key: dnd2024:class_features/college_of_lore_level_14_peerless_skill
          - class_level: 3
            definition:
              key: dnd2024:class_features/college_of_lore_level_3_bonus_proficiencies
          - class_level: 3
            definition:
              key: dnd2024:class_features/college_of_lore_level_3_cutting_words
          - class_level: 6
            definition:
              key: dnd2024:class_features/college_of_lore_level_6_magical_discoveries
          grant_eligible_features:
            'on':
            - subclass.selected
            - character.level_gained
            comparison: class_level_at_least_threshold
            persistent: true
    contract:
      id: dnd2024:subclasses_bard_college_of_lore
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - feature_grants
      events:
      - subclass.selected
      - character.level_gained
      capabilities:
      - character_features
      - proficiencies
    requires:
    - dnd2024:subclasses_bard_college_of_lore
    citations:
    - SRD 5.2.1 pp. 35; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/subclasses/bard_college_of_lore.md
  dnd2024:class_features/college_of_lore_level_3_cutting_words:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/college_of_lore_level_3_cutting_words
        implementation: dnd2024:class_features_college_of_lore_level_3_cutting_words
        parameters:
          action: reaction
          trigger_any:
          - visible_creature.damage_rolled
          - visible_creature.ability_check_succeeded
          - visible_creature.attack_roll_succeeded
          range_feet: 60
          resource_cost:
            id: bard.bardic_inspiration
            amount: 1
          roll:
            var: actor.bardic_inspiration_die
          modify: subtract_from_trigger_roll
          recompute_success: true
          class_scope:
            class: bard
            minimum_level: 3
    contract:
      id: dnd2024:class_features_college_of_lore_level_3_cutting_words
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - damage.rolled
      - ability.check_succeeded
      - attack.roll_succeeded
      capabilities:
      - character_features
      - resources
      - damage
    requires:
    - dnd2024:class_features_college_of_lore_level_3_cutting_words
    citations:
    - SRD 5.2.1 pp. 35; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/college_of_lore_level_3_cutting_words.md
  dnd2024:class_features/college_of_lore_level_14_peerless_skill:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/college_of_lore_level_14_peerless_skill
        implementation: dnd2024:class_features_college_of_lore_level_14_peerless_skill
        parameters:
          trigger:
          - actor.ability_check_failed
          - actor.attack_roll_failed
          resource_cost:
            id: bard.bardic_inspiration
            amount: 1
          roll:
            var: actor.bardic_inspiration_die
          modify: add_to_d20
          recompute_success: true
          refund_on_failure: true
          class_scope:
            class: bard
            minimum_level: 14
    contract:
      id: dnd2024:class_features_college_of_lore_level_14_peerless_skill
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - ability.check_failed
      - attack.roll_failed
      capabilities:
      - character_features
      - resources
    requires:
    - dnd2024:class_features_college_of_lore_level_14_peerless_skill
    citations:
    - SRD 5.2.1 pp. 35; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/college_of_lore_level_14_peerless_skill.md
  dnd2024:class_features/cleric_level_1_spellcasting:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/cleric_level_1_spellcasting
        implementation: dnd2024:class_features_cleric_level_1_spellcasting
        parameters:
          class: cleric
          spellcasting_ability: wisdom
          spellcasting_focus: holy_symbol
          save_dc:
            op: add
            args:
            - 8
            - var: actor.abilities.wisdom.modifier
            - var: actor.proficiency_bonus
          spell_attack_bonus:
            op: add
            args:
            - var: actor.proficiency_bonus
            - var: actor.abilities.wisdom.modifier
          slots:
            progression:
              key: dnd2024:tables/cleric_progression
            recovery:
            - rest.long_completed
            recover: all
            pool: spellcasting
          prepared_spells:
            initial_count: 4
            initial_spell_level: 1
            progression:
              table:
                key: dnd2024:tables/cleric_progression
              row:
                var: actor.class_levels.cleric
              column: prepared_spells
            eligible_list: cleric
            eligible_levels:
              minimum: 1
              maximum: available_class_slot_level
            on_count_increase: choose_until_count_met
            replacement:
              event: rest.long_completed
              class: cleric
              maximum: all
            always_prepared_from_other_class_features:
              counts_against_limit: false
              counts_as_class_spell: cleric
          cantrips:
            initial_count: 3
            list: cleric
            additional_levels:
            - 4
            - 10
            additional_each: 1
            replace:
              event: character.level_gained
              class: cleric
              maximum: 1
              same_list: true
          class_scope:
            class: cleric
            minimum_level: 1
    contract:
      id: dnd2024:class_features_cleric_level_1_spellcasting
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - spellcasting
      - prepared_spells
      - resources
      events:
      - character.level_gained
      - rest.long_completed
      - rest.short_completed
      - spell.prepare
      - spell.cast
      capabilities:
      - character_features
      - spellcasting
      - proficiencies
      - d20_tests
    requires:
    - dnd2024:class_features_cleric_level_1_spellcasting
    citations:
    - SRD 5.2.1 pp. 36, 37; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/cleric_level_1_spellcasting.md
  dnd2024:class_features/cleric_level_2_channel_divinity:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/cleric_level_2_channel_divinity
        implementation: dnd2024:class_features_cleric_level_2_channel_divinity
        parameters:
          resource:
            id: cleric.channel_divinity
            maximum:
              table:
                key: dnd2024:tables/cleric_progression
              row:
                var: actor.class_levels.cleric
              column: channel_divinity
            recovery:
            - event: rest.short_completed
              amount: 1
            - event: rest.long_completed
              amount: all
          cost_per_use: 1
          choose_effect: 1
          dc:
            op: add
            args:
            - 8
            - var: actor.abilities.wisdom.modifier
            - var: actor.proficiency_bonus
          options:
            divine_spark:
              action: magic
              requires_item: holy_symbol
              target:
                type: creature
                self: false
                visible: true
                range_feet: 30
              amount:
                op: add
                args:
                - dice:
                    count_by_cleric_level:
                      2: 1
                      7: 2
                      13: 3
                      18: 4
                    sides: 8
                - var: actor.abilities.wisdom.modifier
              choice:
              - heal
              - damage
              damage:
                types_choice:
                - necrotic
                - radiant
                save: constitution
                success_multiplier: 0.5
                rounding: down
            turn_undead:
              action: magic
              requires_item: holy_symbol
              targets:
                type: undead
                selection: actor_choice
                range_feet: 30
              save: wisdom
              failure:
                conditions:
                - frightened
                - incapacitated
                duration:
                  minutes: 1
                on_target_turn: move_as_far_as_possible_from_actor
                ends_on:
                - target.damage_taken
                - actor.incapacitated
                - actor.died
          class_scope:
            class: cleric
            minimum_level: 2
    contract:
      id: dnd2024:class_features_cleric_level_2_channel_divinity
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - action.magic
      - rest.short_completed
      - rest.long_completed
      - turn.start
      - damage.taken
      - condition.applied
      - participant.died
      capabilities:
      - character_features
      - resources
      - damage
      - conditions
      - proficiencies
      - d20_tests
      - healing
    requires:
    - dnd2024:class_features_cleric_level_2_channel_divinity
    citations:
    - SRD 5.2.1 pp. 37; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/cleric_level_2_channel_divinity.md
  dnd2024:class_features/cleric_level_4_ability_score_improvement:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/cleric_level_4_ability_score_improvement
        implementation: dnd2024:class_features_cleric_level_4_ability_score_improvement
        parameters:
          class: cleric
          grant_at_class_levels:
          - 4
          - 8
          - 12
          - 16
          feat_choice:
            count_each: 1
            default:
              key: dnd2024:feats/general_feats_ability_score_improvement
            allowed_categories: any
            must_meet_prerequisites: true
          class_scope:
            class: cleric
            minimum_level: 4
    contract:
      id: dnd2024:class_features_cleric_level_4_ability_score_improvement
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - feat_grants
      events:
      - character.level_gained
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_cleric_level_4_ability_score_improvement
    citations:
    - SRD 5.2.1 pp. 37; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/cleric_level_4_ability_score_improvement.md
  dnd2024:class_features/cleric_level_7_blessed_strikes:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/cleric_level_7_blessed_strikes
        implementation: dnd2024:class_features_cleric_level_7_blessed_strikes
        parameters:
          choice:
            count: 1
            options:
              divine_strike:
                trigger: actor.weapon_attack_hits_creature
                limit:
                  count: 1
                  period: actor.turn
                damage:
                  dice:
                    count: 1
                    sides: 8
                  type_choice:
                  - necrotic
                  - radiant
              potent_spellcasting:
                class: cleric
                spell_level: 0
                damage_bonus:
                  var: actor.abilities.wisdom.modifier
          legacy_subclass_same_options: only_selected_option_applies
          class_scope:
            class: cleric
            minimum_level: 7
    contract:
      id: dnd2024:class_features_cleric_level_7_blessed_strikes
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.feature_acquired
      - attack.hit
      - spell.damage
      capabilities:
      - character_features
      - spellcasting
      - damage
    requires:
    - dnd2024:class_features_cleric_level_7_blessed_strikes
    citations:
    - SRD 5.2.1 pp. 38; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/cleric_level_7_blessed_strikes.md
  dnd2024:class_features/cleric_level_14_improved_blessed_strikes:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/cleric_level_14_improved_blessed_strikes
        implementation: dnd2024:class_features_cleric_level_14_improved_blessed_strikes
        parameters:
          modify_selected_blessed_strikes:
            divine_strike:
              damage_dice:
                count: 2
                sides: 8
            potent_spellcasting:
              trigger: cleric.cantrip_cast_deals_damage_to_creature
              optional: true
              target:
                type: creature
                self_allowed: true
                range_feet: 60
                count: 1
              temporary_hit_points:
                op: multiply
                args:
                - 2
                - var: actor.abilities.wisdom.modifier
          class_scope:
            class: cleric
            minimum_level: 14
    contract:
      id: dnd2024:class_features_cleric_level_14_improved_blessed_strikes
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - attack.hit
      - spell.damage
      capabilities:
      - character_features
      - spellcasting
      - damage
    requires:
    - dnd2024:class_features_cleric_level_14_improved_blessed_strikes
    citations:
    - SRD 5.2.1 pp. 38; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/cleric_level_14_improved_blessed_strikes.md
  dnd2024:class_features/cleric_level_20_greater_divine_intervention:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/cleric_level_20_greater_divine_intervention
        implementation: dnd2024:class_features_cleric_level_20_greater_divine_intervention
        parameters:
          extends_feature:
            key: dnd2024:class_features/cleric_level_10_divine_intervention
          additional_spell_choice:
            key: dnd2024:spells/wish
          if_chosen:
            divine_intervention_recovery:
              event: rest.long_completed
              required_count:
                dice:
                  count: 2
                  sides: 4
          class_scope:
            class: cleric
            minimum_level: 20
    contract:
      id: dnd2024:class_features_cleric_level_20_greater_divine_intervention
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - feature.divine_intervention
      - rest.long_completed
      capabilities:
      - character_features
      - spellcasting
    requires:
    - dnd2024:class_features_cleric_level_20_greater_divine_intervention
    citations:
    - SRD 5.2.1 pp. 38; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/cleric_level_20_greater_divine_intervention.md
  dnd2024:class_features/life_domain_level_3_disciple_of_life:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/life_domain_level_3_disciple_of_life
        implementation: dnd2024:class_features_life_domain_level_3_disciple_of_life
        parameters:
          trigger: spell.healing
          requires:
            caster: actor
            expended_spell_slot: true
            turn: spell.cast_turn
          additional_healing:
            op: add
            args:
            - 2
            - var: spell.slot_level
          class_scope:
            class: cleric
            minimum_level: 3
    contract:
      id: dnd2024:class_features_life_domain_level_3_disciple_of_life
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.healing
      capabilities:
      - character_features
      - spellcasting
      - healing
    requires:
    - dnd2024:class_features_life_domain_level_3_disciple_of_life
    citations:
    - SRD 5.2.1 pp. 40; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/life_domain_level_3_disciple_of_life.md
  dnd2024:class_features/life_domain_level_3_preserve_life:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/life_domain_level_3_preserve_life
        implementation: dnd2024:class_features_life_domain_level_3_preserve_life
        parameters:
          action: magic
          requires_item: holy_symbol
          resource_cost:
            id: cleric.channel_divinity
            amount: 1
          healing_pool:
            op: multiply
            args:
            - 5
            - var: actor.class_levels.cleric
          targets:
            bloodied: true
            range_feet: 30
            self_allowed: true
            selection: actor_choice
          distribution:
            chooser: actor
            type: integer_allocation
            sum_maximum: healing_pool
            per_target_hp_cap:
              op: multiply
              args:
              - var: target.hit_point_maximum
              - 0.5
          class_scope:
            class: cleric
            minimum_level: 3
    contract:
      id: dnd2024:class_features_life_domain_level_3_preserve_life
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - healing
      - resource_expenditure
      events:
      - action.magic
      capabilities:
      - character_features
      - resources
      - healing
    requires:
    - dnd2024:class_features_life_domain_level_3_preserve_life
    citations:
    - SRD 5.2.1 pp. 40; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/life_domain_level_3_preserve_life.md
  dnd2024:class_features/life_domain_level_17_supreme_healing:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/life_domain_level_17_supreme_healing
        implementation: dnd2024:class_features_life_domain_level_17_supreme_healing
        parameters:
          trigger: healing.dice_pending
          sources:
          - actor.spell
          - actor.channel_divinity
          replace_each_die: maximum_face
          class_scope:
            class: cleric
            minimum_level: 17
    contract:
      id: dnd2024:class_features_life_domain_level_17_supreme_healing
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - healing.dice_pending
      capabilities:
      - character_features
      - spellcasting
      - healing
    requires:
    - dnd2024:class_features_life_domain_level_17_supreme_healing
    citations:
    - SRD 5.2.1 pp. 40; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/life_domain_level_17_supreme_healing.md
  dnd2024:class_features/druid_level_1_spellcasting:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/druid_level_1_spellcasting
        implementation: dnd2024:class_features_druid_level_1_spellcasting
        parameters:
          class: druid
          spellcasting_ability: wisdom
          spellcasting_focus: druidic_focus
          save_dc:
            op: add
            args:
            - 8
            - var: actor.abilities.wisdom.modifier
            - var: actor.proficiency_bonus
          spell_attack_bonus:
            op: add
            args:
            - var: actor.proficiency_bonus
            - var: actor.abilities.wisdom.modifier
          slots:
            progression:
              key: dnd2024:tables/druid_progression
            recovery:
            - rest.long_completed
            recover: all
            pool: spellcasting
          prepared_spells:
            initial_count: 4
            initial_spell_level: 1
            progression:
              table:
                key: dnd2024:tables/druid_progression
              row:
                var: actor.class_levels.druid
              column: prepared_spells
            eligible_list: druid
            eligible_levels:
              minimum: 1
              maximum: available_class_slot_level
            on_count_increase: choose_until_count_met
            replacement:
              event: rest.long_completed
              class: druid
              maximum: all
            always_prepared_from_other_class_features:
              counts_against_limit: false
              counts_as_class_spell: druid
          cantrips:
            initial_count: 2
            list: druid
            additional_levels:
            - 4
            - 10
            additional_each: 1
            replace:
              event: character.level_gained
              class: druid
              maximum: 1
              same_list: true
          class_scope:
            class: druid
            minimum_level: 1
    contract:
      id: dnd2024:class_features_druid_level_1_spellcasting
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - spellcasting
      - prepared_spells
      - resources
      events:
      - character.level_gained
      - rest.long_completed
      - rest.short_completed
      - spell.prepare
      - spell.cast
      capabilities:
      - character_features
      - spellcasting
      - proficiencies
      - d20_tests
    requires:
    - dnd2024:class_features_druid_level_1_spellcasting
    citations:
    - SRD 5.2.1 pp. 41, 42; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/druid_level_1_spellcasting.md
  dnd2024:class_features/druid_level_1_primal_order:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/druid_level_1_primal_order
        implementation: dnd2024:class_features_druid_level_1_primal_order
        parameters:
          choose: 1
          options:
            magician:
              extra_cantrip:
                count: 1
                list: druid
              ability_check_bonus:
                ability: intelligence
                skills:
                - arcana
                - nature
                amount:
                  op: max
                  args:
                  - 1
                  - var: actor.abilities.wisdom.modifier
            warden:
              weapon_proficiencies:
              - martial
              armor_training:
              - medium
          class_scope:
            class: druid
            minimum_level: 1
    contract:
      id: dnd2024:class_features_druid_level_1_primal_order
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.feature_acquired
      - ability.check
      capabilities:
      - character_features
      - proficiencies
    requires:
    - dnd2024:class_features_druid_level_1_primal_order
    citations:
    - SRD 5.2.1 pp. 42; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/druid_level_1_primal_order.md
  dnd2024:class_features/druid_level_2_wild_companion:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/druid_level_2_wild_companion
        implementation: dnd2024:class_features_druid_level_2_wild_companion
        parameters:
          action: magic
          cost_choice:
          - resource: spell_slot
            amount: 1
            minimum_level: 1
          - resource: druid.wild_shape
            amount: 1
          cast:
            spell:
              key: dnd2024:spells/find_familiar
            material_components: false
            creature_type: fey
            familiar_ends: actor.rest.long_completed
          class_scope:
            class: druid
            minimum_level: 2
    contract:
      id: dnd2024:class_features_druid_level_2_wild_companion
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - action.magic
      - rest.long_completed
      capabilities:
      - character_features
      - resources
      - spellcasting
    requires:
    - dnd2024:class_features_druid_level_2_wild_companion
    citations:
    - SRD 5.2.1 pp. 43; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/druid_level_2_wild_companion.md
  dnd2024:class_features/druid_level_4_ability_score_improvement:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/druid_level_4_ability_score_improvement
        implementation: dnd2024:class_features_druid_level_4_ability_score_improvement
        parameters:
          class: druid
          grant_at_class_levels:
          - 4
          - 8
          - 12
          - 16
          feat_choice:
            count_each: 1
            default:
              key: dnd2024:feats/general_feats_ability_score_improvement
            allowed_categories: any
            must_meet_prerequisites: true
          class_scope:
            class: druid
            minimum_level: 4
    contract:
      id: dnd2024:class_features_druid_level_4_ability_score_improvement
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - feat_grants
      events:
      - character.level_gained
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_druid_level_4_ability_score_improvement
    citations:
    - SRD 5.2.1 pp. 43; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/druid_level_4_ability_score_improvement.md
  dnd2024:class_features/druid_level_7_elemental_fury:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/druid_level_7_elemental_fury
        implementation: dnd2024:class_features_druid_level_7_elemental_fury
        parameters:
          choose: 1
          options:
            potent_spellcasting:
              spell_list: druid
              spell_level: 0
              damage_bonus:
                var: actor.abilities.wisdom.modifier
            primal_strike:
              trigger: attack.hits_creature
              attack_sources:
              - weapon
              - wild_shape.beast_attack
              limit:
                count: 1
                period: actor.turn
              extra_damage:
                dice:
                  count: 1
                  sides: 8
                type_choice:
                - cold
                - fire
                - lightning
                - thunder
                choice_time: attack.hit
          class_scope:
            class: druid
            minimum_level: 7
    contract:
      id: dnd2024:class_features_druid_level_7_elemental_fury
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.damage
      - attack.hit
      capabilities:
      - character_features
      - spellcasting
      - damage
    requires:
    - dnd2024:class_features_druid_level_7_elemental_fury
    citations:
    - SRD 5.2.1 pp. 43; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/druid_level_7_elemental_fury.md
  dnd2024:class_features/druid_level_18_beast_spells:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/druid_level_18_beast_spells
        implementation: dnd2024:class_features_druid_level_18_beast_spells
        parameters:
          while: wild_shape.beast_form
          allow_spellcasting: true
          exclude_spells:
            material_component_cost_specified: true
            or_material_component_consumed: true
          class_scope:
            class: druid
            minimum_level: 18
    contract:
      id: dnd2024:class_features_druid_level_18_beast_spells
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast_pending
      capabilities:
      - character_features
      - spellcasting
    requires:
    - dnd2024:class_features_druid_level_18_beast_spells
    citations:
    - SRD 5.2.1 pp. 43; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/druid_level_18_beast_spells.md
  dnd2024:class_features/druid_level_20_archdruid:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/druid_level_20_archdruid
        implementation: dnd2024:class_features_druid_level_20_archdruid
        parameters:
          evergreen_wild_shape:
            trigger: initiative.rolled
            requires:
              wild_shape_uses: 0
            restore_uses: 1
          nature_magician:
            action: none
            resource:
              id: archdruid.nature_magician
              maximum: 1
              recovery:
              - event: rest.long_completed
                amount: all
            cost_uses: 1
            convert:
              resource: druid.wild_shape
              choose_unexpended_count: true
              minimum_use_count: 1
              create_spell_slot_count: 1
              slot_level:
                op: multiply
                args:
                - 2
                - var: chosen_use_count
          aging:
            body_years_per_elapsed_year: 0.1
          class_scope:
            class: druid
            minimum_level: 20
    contract:
      id: dnd2024:class_features_druid_level_20_archdruid
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - initiative.rolled
      - resource.convert
      - rest.long_completed
      - time.elapsed
      capabilities:
      - character_features
      - resources
      - spellcasting
      - initiative
    requires:
    - dnd2024:class_features_druid_level_20_archdruid
    citations:
    - SRD 5.2.1 pp. 43; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/druid_level_20_archdruid.md
  dnd2024:class_features/circle_of_the_land_level_3_circle_of_the_land_spells:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/circle_of_the_land_level_3_circle_of_the_land_spells
        implementation: dnd2024:class_features_circle_of_the_land_level_3_circle_of_the_land_spells
        parameters:
          land_choice:
            event: rest.long_completed
            count: 1
            options:
            - arid
            - polar
            - temperate
            - tropical
          always_prepared:
            arid:
            - minimum_druid_level: 3
              spells:
              - key: dnd2024:spells/blur
              - key: dnd2024:spells/burning_hands
              - key: dnd2024:spells/fire_bolt
            - minimum_druid_level: 5
              spells:
              - key: dnd2024:spells/fireball
            - minimum_druid_level: 7
              spells:
              - key: dnd2024:spells/blight
            - minimum_druid_level: 9
              spells:
              - key: dnd2024:spells/wall_of_stone
            polar:
            - minimum_druid_level: 3
              spells:
              - key: dnd2024:spells/fog_cloud
              - key: dnd2024:spells/hold_person
              - key: dnd2024:spells/ray_of_frost
            - minimum_druid_level: 5
              spells:
              - key: dnd2024:spells/sleet_storm
            - minimum_druid_level: 7
              spells:
              - key: dnd2024:spells/ice_storm
            - minimum_druid_level: 9
              spells:
              - key: dnd2024:spells/cone_of_cold
            temperate:
            - minimum_druid_level: 3
              spells:
              - key: dnd2024:spells/misty_step
              - key: dnd2024:spells/shocking_grasp
              - key: dnd2024:spells/sleep
            - minimum_druid_level: 5
              spells:
              - key: dnd2024:spells/lightning_bolt
            - minimum_druid_level: 7
              spells:
              - key: dnd2024:spells/freedom_of_movement
            - minimum_druid_level: 9
              spells:
              - key: dnd2024:spells/tree_stride
            tropical:
            - minimum_druid_level: 3
              spells:
              - key: dnd2024:spells/acid_splash
              - key: dnd2024:spells/ray_of_sickness
              - key: dnd2024:spells/web
            - minimum_druid_level: 5
              spells:
              - key: dnd2024:spells/stinking_cloud
            - minimum_druid_level: 7
              spells:
              - key: dnd2024:spells/polymorph
            - minimum_druid_level: 9
              spells:
              - key: dnd2024:spells/insect_plague
          active_list: chosen_land
          include_lower_levels: true
          counts_against_prepared_limit: false
          spell_class: druid
          class_scope:
            class: druid
            minimum_level: 3
    contract:
      id: dnd2024:class_features_circle_of_the_land_level_3_circle_of_the_land_spells
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - prepared_spells
      events:
      - rest.long_completed
      - character.level_gained
      capabilities:
      - character_features
      - spellcasting
      - movement
    requires:
    - dnd2024:class_features_circle_of_the_land_level_3_circle_of_the_land_spells
    citations:
    - SRD 5.2.1 pp. 46; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/circle_of_the_land_level_3_circle_of_the_land_spells.md
  dnd2024:class_features/circle_of_the_land_level_6_natural_recovery:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/circle_of_the_land_level_6_natural_recovery
        implementation: dnd2024:class_features_circle_of_the_land_level_6_natural_recovery
        parameters:
          free_cast:
            resource:
              id: land.natural_recovery_cast
              maximum: 1
              recovery:
              - event: rest.long_completed
                amount: all
            cost: 1
            spell_eligibility:
              prepared_by:
                key: dnd2024:class_features/circle_of_the_land_level_3_circle_of_the_land_spells
              minimum_spell_level: 1
            expend_slot: false
          slot_recovery:
            event: rest.short_completed
            resource:
              id: land.natural_recovery_slots
              maximum: 1
              recovery:
              - event: rest.long_completed
                amount: all
            cost: 1
            choose_expended_slots: true
            maximum_combined_levels:
              op: ceil
              args:
              - op: divide
                args:
                - var: actor.class_levels.druid
                - 2
            maximum_slot_level: 5
          class_scope:
            class: druid
            minimum_level: 6
    contract:
      id: dnd2024:class_features_circle_of_the_land_level_6_natural_recovery
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - spellcasting
      - resources
      events:
      - spell.cast
      - rest.short_completed
      - rest.long_completed
      capabilities:
      - character_features
      - resources
      - spellcasting
    requires:
    - dnd2024:class_features_circle_of_the_land_level_6_natural_recovery
    citations:
    - SRD 5.2.1 pp. 46; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/circle_of_the_land_level_6_natural_recovery.md
  dnd2024:class_features/circle_of_the_land_level_14_nature_s_sanctuary:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/circle_of_the_land_level_14_nature_s_sanctuary
        implementation: dnd2024:class_features_circle_of_the_land_level_14_nature_s_sanctuary
        parameters:
          action: magic
          resource_cost:
            id: druid.wild_shape
            amount: 1
          area:
            shape: cube
            side_feet: 15
            placement: ground
            range_feet: 120
            geometry_validation: explicit_location_choice
          duration:
            minutes: 1
          ends_on:
          - actor.incapacitated
          - actor.died
          within_area:
            actor_and_allies:
              cover: half
            allies:
              resistance:
                var: actor.natures_ward.current_resistance
          move_area:
            action: bonus_action
            maximum_feet: 60
            destination:
              ground: true
              within_actor_feet: 120
          class_scope:
            class: druid
            minimum_level: 14
    contract:
      id: dnd2024:class_features_circle_of_the_land_level_14_nature_s_sanctuary
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - action.magic
      - action.bonus
      - participant.moved
      - condition.applied
      - participant.died
      capabilities:
      - character_features
      - resources
    requires:
    - dnd2024:class_features_circle_of_the_land_level_14_nature_s_sanctuary
    citations:
    - SRD 5.2.1 pp. 46; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/circle_of_the_land_level_14_nature_s_sanctuary.md
  dnd2024:class_features/fighter_level_1_fighting_style:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/fighter_level_1_fighting_style
        implementation: dnd2024:class_features_fighter_level_1_fighting_style
        parameters:
          feat_choice:
            count: 1
            category: fighting_style
          replace:
            event: fighter.level_gained
            category: fighting_style
            maximum: 1
          class_scope:
            class: fighter
            minimum_level: 1
    contract:
      id: dnd2024:class_features_fighter_level_1_fighting_style
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - feat_grants
      events:
      - character.feature_acquired
      - character.level_gained
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_fighter_level_1_fighting_style
    citations:
    - SRD 5.2.1 pp. 47; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/fighter_level_1_fighting_style.md
  dnd2024:class_features/fighter_level_1_weapon_mastery:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/fighter_level_1_weapon_mastery
        implementation: dnd2024:class_features_fighter_level_1_weapon_mastery
        parameters:
          mastery_choices:
            count:
              table:
                key: dnd2024:tables/fighter_progression
              row:
                var: actor.class_levels.fighter
              column: weapon_mastery
            eligible:
              weapon_category:
              - simple
              - martial
            replacement:
              event: rest.long_completed
              maximum: 1
          class_scope:
            class: fighter
            minimum_level: 1
    contract:
      id: dnd2024:class_features_fighter_level_1_weapon_mastery
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.level_gained
      - rest.long_completed
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_fighter_level_1_weapon_mastery
    citations:
    - SRD 5.2.1 pp. 48; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/fighter_level_1_weapon_mastery.md
  dnd2024:class_features/fighter_level_2_tactical_mind:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/fighter_level_2_tactical_mind
        implementation: dnd2024:class_features_fighter_level_2_tactical_mind
        parameters:
          trigger: actor.ability_check_failed
          resource_cost:
            id: fighter.second_wind
            amount: 1
          roll:
            count: 1
            sides: 10
          modify: add_to_d20
          recompute_success: true
          refund_on_failure: true
          healing: false
          class_scope:
            class: fighter
            minimum_level: 2
    contract:
      id: dnd2024:class_features_fighter_level_2_tactical_mind
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - ability.check_failed
      capabilities:
      - character_features
      - resources
      - healing
    requires:
    - dnd2024:class_features_fighter_level_2_tactical_mind
    citations:
    - SRD 5.2.1 pp. 48; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/fighter_level_2_tactical_mind.md
  dnd2024:class_features/fighter_level_4_ability_score_improvement:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/fighter_level_4_ability_score_improvement
        implementation: dnd2024:class_features_fighter_level_4_ability_score_improvement
        parameters:
          class: fighter
          grant_at_class_levels:
          - 4
          - 6
          - 8
          - 12
          - 14
          - 16
          feat_choice:
            count_each: 1
            default:
              key: dnd2024:feats/general_feats_ability_score_improvement
            allowed_categories: any
            must_meet_prerequisites: true
          class_scope:
            class: fighter
            minimum_level: 4
    contract:
      id: dnd2024:class_features_fighter_level_4_ability_score_improvement
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - feat_grants
      events:
      - character.level_gained
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_fighter_level_4_ability_score_improvement
    citations:
    - SRD 5.2.1 pp. 48; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/fighter_level_4_ability_score_improvement.md
  dnd2024:class_features/fighter_level_5_tactical_shift:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/fighter_level_5_tactical_shift
        implementation: dnd2024:class_features_fighter_level_5_tactical_shift
        parameters:
          trigger: actor.second_wind_activated_with_bonus_action
          optional_movement:
            maximum_feet:
              op: multiply
              args:
              - var: actor.speed
              - 0.5
            opportunity_attacks: false
          class_scope:
            class: fighter
            minimum_level: 5
    contract:
      id: dnd2024:class_features_fighter_level_5_tactical_shift
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - second_wind.activated
      capabilities:
      - character_features
      - movement
    requires:
    - dnd2024:class_features_fighter_level_5_tactical_shift
    citations:
    - SRD 5.2.1 pp. 48; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/fighter_level_5_tactical_shift.md
  dnd2024:class_features/fighter_level_9_tactical_master:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/fighter_level_9_tactical_master
        implementation: dnd2024:class_features_fighter_level_9_tactical_master
        parameters:
          trigger: weapon.attack
          requires:
            weapon_mastery_available_to_actor: true
          replace_mastery_for_this_attack:
            optional: true
            choices:
            - push
            - sap
            - slow
          class_scope:
            class: fighter
            minimum_level: 9
    contract:
      id: dnd2024:class_features_fighter_level_9_tactical_master
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - attack.roll_pending
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_fighter_level_9_tactical_master
    citations:
    - SRD 5.2.1 pp. 48; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/fighter_level_9_tactical_master.md
  dnd2024:class_features/fighter_level_13_studied_attacks:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/fighter_level_13_studied_attacks
        implementation: dnd2024:class_features_fighter_level_13_studied_attacks
        parameters:
          trigger: actor.attack_missed_creature
          grant:
            advantage: true
            scope: next_attack_against_same_creature
            expires: actor.next_turn.end
            consume_on: matching_attack_roll
          class_scope:
            class: fighter
            minimum_level: 13
    contract:
      id: dnd2024:class_features_fighter_level_13_studied_attacks
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - attack.missed
      - attack.roll_pending
      - turn.end
      capabilities:
      - character_features
      - d20_tests
    requires:
    - dnd2024:class_features_fighter_level_13_studied_attacks
    citations:
    - SRD 5.2.1 pp. 48; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/fighter_level_13_studied_attacks.md
  dnd2024:class_features/fighter_level_20_three_extra_attacks:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/fighter_level_20_three_extra_attacks
        implementation: dnd2024:class_features_fighter_level_20_three_extra_attacks
        parameters:
          action: attack
          turn: actor
          attack_count: 4
          class_scope:
            class: fighter
            minimum_level: 20
    contract:
      id: dnd2024:class_features_fighter_level_20_three_extra_attacks
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - action.attack
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_fighter_level_20_three_extra_attacks
    citations:
    - SRD 5.2.1 pp. 48; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/fighter_level_20_three_extra_attacks.md
  dnd2024:class_features/champion_level_3_improved_critical:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/champion_level_3_improved_critical
        implementation: dnd2024:class_features_champion_level_3_improved_critical
        parameters:
          attack_sources:
          - weapon
          - unarmed_strike
          critical_d20_faces:
          - 19
          - 20
          class_scope:
            class: fighter
            minimum_level: 3
    contract:
      id: dnd2024:class_features_champion_level_3_improved_critical
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - attack.roll
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_champion_level_3_improved_critical
    citations:
    - SRD 5.2.1 pp. 49; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/champion_level_3_improved_critical.md
  dnd2024:class_features/champion_level_7_additional_fighting_style:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/champion_level_7_additional_fighting_style
        implementation: dnd2024:class_features_champion_level_7_additional_fighting_style
        parameters:
          feat_choice:
            count: 1
            category: fighting_style
            additional: true
          class_scope:
            class: fighter
            minimum_level: 7
    contract:
      id: dnd2024:class_features_champion_level_7_additional_fighting_style
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - feat_grants
      events:
      - character.feature_acquired
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_champion_level_7_additional_fighting_style
    citations:
    - SRD 5.2.1 pp. 49; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/champion_level_7_additional_fighting_style.md
  dnd2024:class_features/champion_level_15_superior_critical:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/champion_level_15_superior_critical
        implementation: dnd2024:class_features_champion_level_15_superior_critical
        parameters:
          attack_sources:
          - weapon
          - unarmed_strike
          critical_d20_faces:
          - 18
          - 19
          - 20
          class_scope:
            class: fighter
            minimum_level: 15
    contract:
      id: dnd2024:class_features_champion_level_15_superior_critical
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - attack.roll
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_champion_level_15_superior_critical
    citations:
    - SRD 5.2.1 pp. 49; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/champion_level_15_superior_critical.md
  dnd2024:classes/monk:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:classes/monk
        implementation: dnd2024:classes_monk
        parameters:
          class: monk
          primary_abilities:
          - dexterity
          - wisdom
          hit_point_die:
            count_per_class_level: 1
            sides: 8
          first_class_traits:
            save_proficiencies:
            - strength
            - dexterity
            skill_proficiencies:
              count: 2
              choices:
              - acrobatics
              - athletics
              - history
              - insight
              - religion
              - stealth
            weapon_proficiencies:
              simple: true
              martial_required_property_any:
              - light
            armor_training: []
            tool_proficiency_choice:
              count: 1
              categories:
              - artisans_tools
              - musical_instrument
          starting_equipment:
            choose: 1
            options:
              equipment:
                items:
                - definition:
                    key: dnd2024:equipment/spear
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/dagger
                  quantity: 5
                - definition:
                    key: dnd2024:equipment/explorer_s_pack_10_gp
                  quantity: 1
                currency:
                  gp: 11
                matching_proficient_tool: true
              currency:
                currency:
                  gp: 50
          progression:
            key: dnd2024:tables/monk_progression
          feature_grants:
          - minimum_class_level: 10
            definition:
              key: dnd2024:class_features/monk_level_10_heightened_focus
          - minimum_class_level: 10
            definition:
              key: dnd2024:class_features/monk_level_10_self_restoration
          - minimum_class_level: 13
            definition:
              key: dnd2024:class_features/monk_level_13_deflect_energy
          - minimum_class_level: 14
            definition:
              key: dnd2024:class_features/monk_level_14_disciplined_survivor
          - minimum_class_level: 15
            definition:
              key: dnd2024:class_features/monk_level_15_perfect_focus
          - minimum_class_level: 18
            definition:
              key: dnd2024:class_features/monk_level_18_superior_defense
          - minimum_class_level: 19
            definition:
              key: dnd2024:class_features/monk_level_19_epic_boon
          - minimum_class_level: 1
            definition:
              key: dnd2024:class_features/monk_level_1_martial_arts
          - minimum_class_level: 1
            definition:
              key: dnd2024:class_features/monk_level_1_unarmored_defense
          - minimum_class_level: 20
            definition:
              key: dnd2024:class_features/monk_level_20_body_and_mind
          - minimum_class_level: 2
            definition:
              key: dnd2024:class_features/monk_level_2_monk_s_focus
          - minimum_class_level: 2
            definition:
              key: dnd2024:class_features/monk_level_2_unarmored_movement
          - minimum_class_level: 2
            definition:
              key: dnd2024:class_features/monk_level_2_uncanny_metabolism
          - minimum_class_level: 3
            definition:
              key: dnd2024:class_features/monk_level_3_deflect_attacks
          - minimum_class_level: 3
            definition:
              key: dnd2024:class_features/monk_level_3_monk_subclass
          - minimum_class_level: 4
            definition:
              key: dnd2024:class_features/monk_level_4_ability_score_improvement
          - minimum_class_level: 4
            definition:
              key: dnd2024:class_features/monk_level_4_slow_fall
          - minimum_class_level: 5
            definition:
              key: dnd2024:class_features/monk_level_5_extra_attack
          - minimum_class_level: 5
            definition:
              key: dnd2024:class_features/monk_level_5_stunning_strike
          - minimum_class_level: 6
            definition:
              key: dnd2024:class_features/monk_level_6_empowered_strikes
          - minimum_class_level: 7
            definition:
              key: dnd2024:class_features/monk_level_7_evasion
          - minimum_class_level: 9
            definition:
              key: dnd2024:class_features/monk_level_9_acrobatic_movement
    contract:
      id: dnd2024:classes_monk
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - class_traits
      - equipment_choices
      - progression
      - feature_grants
      events:
      - character.class_selected
      - character.level_gained
      capabilities:
      - character_features
      - proficiencies
      - d20_tests
      - movement
      - equipment
    requires:
    - dnd2024:classes_monk
    citations:
    - SRD 5.2.1 pp. 49; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/classes/monk.md
  dnd2024:class_features/monk_level_1_unarmored_defense:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_1_unarmored_defense
        implementation: dnd2024:class_features_monk_level_1_unarmored_defense
        parameters:
          requires:
            armor_worn: false
            shield_wielded: false
          base_armor_class:
            op: add
            args:
            - 10
            - var: actor.abilities.dexterity.modifier
            - var: actor.abilities.wisdom.modifier
          class_scope:
            class: monk
            minimum_level: 1
    contract:
      id: dnd2024:class_features_monk_level_1_unarmored_defense
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - armor_class.calculate
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_monk_level_1_unarmored_defense
    citations:
    - SRD 5.2.1 pp. 50; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_1_unarmored_defense.md
  dnd2024:class_features/monk_level_2_unarmored_movement:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_2_unarmored_movement
        implementation: dnd2024:class_features_monk_level_2_unarmored_movement
        parameters:
          requires:
            armor_worn: false
            shield_wielded: false
          speed_bonus_feet:
            table:
              key: dnd2024:tables/monk_progression
            row:
              var: actor.class_levels.monk
            column: unarmored_movement
          class_scope:
            class: monk
            minimum_level: 2
    contract:
      id: dnd2024:class_features_monk_level_2_unarmored_movement
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - speed.calculate
      capabilities:
      - character_features
      - movement
    requires:
    - dnd2024:class_features_monk_level_2_unarmored_movement
    citations:
    - SRD 5.2.1 pp. 51; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_2_unarmored_movement.md
  dnd2024:class_features/monk_level_3_deflect_attacks:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_3_deflect_attacks
        implementation: dnd2024:class_features_monk_level_3_deflect_attacks
        parameters:
          action: reaction
          trigger: attack.hit_actor
          requires:
            damage_types_include_any:
            - bludgeoning
            - piercing
            - slashing
          reduce_total_damage:
            op: add
            args:
            - dice:
                count: 1
                sides: 10
            - var: actor.abilities.dexterity.modifier
            - var: actor.class_levels.monk
          if_damage_reduced_to_zero:
            optional: true
            cost:
              resource: monk.focus_points
              amount: 1
            target:
              visible: true
              type: creature
              if_melee_attack:
                range_feet: 5
              if_ranged_attack:
                range_feet: 60
                total_cover: false
            save:
              ability: dexterity
              dc:
                op: add
                args:
                - 8
                - var: actor.abilities.wisdom.modifier
                - var: actor.proficiency_bonus
              failure_damage:
                amount:
                  op: add
                  args:
                  - dice:
                      count: 2
                      sides:
                        martial_arts_die_sides:
                          table:
                            key: dnd2024:tables/monk_progression
                          row:
                            var: actor.class_levels.monk
                          column: martial_arts
                  - var: actor.abilities.dexterity.modifier
                type: trigger_attack.damage_type
              success_damage: 0
          class_scope:
            class: monk
            minimum_level: 3
    contract:
      id: dnd2024:class_features_monk_level_3_deflect_attacks
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - attack.hit
      - damage.pending
      capabilities:
      - character_features
      - resources
      - damage
      - proficiencies
      - d20_tests
    requires:
    - dnd2024:class_features_monk_level_3_deflect_attacks
    citations:
    - SRD 5.2.1 pp. 51; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_3_deflect_attacks.md
  dnd2024:class_features/monk_level_4_ability_score_improvement:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_4_ability_score_improvement
        implementation: dnd2024:class_features_monk_level_4_ability_score_improvement
        parameters:
          class: monk
          grant_at_class_levels:
          - 4
          - 8
          - 12
          - 16
          feat_choice:
            count_each: 1
            default:
              key: dnd2024:feats/general_feats_ability_score_improvement
            allowed_categories: any
            must_meet_prerequisites: true
          class_scope:
            class: monk
            minimum_level: 4
    contract:
      id: dnd2024:class_features_monk_level_4_ability_score_improvement
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - feat_grants
      events:
      - character.level_gained
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_monk_level_4_ability_score_improvement
    citations:
    - SRD 5.2.1 pp. 51; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_4_ability_score_improvement.md
  dnd2024:class_features/monk_level_5_extra_attack:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_5_extra_attack
        implementation: dnd2024:class_features_monk_level_5_extra_attack
        parameters:
          action: attack
          turn: actor
          attack_count: 2
          class_scope:
            class: monk
            minimum_level: 5
    contract:
      id: dnd2024:class_features_monk_level_5_extra_attack
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - action.attack
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_monk_level_5_extra_attack
    citations:
    - SRD 5.2.1 pp. 51; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_5_extra_attack.md
  dnd2024:class_features/monk_level_6_empowered_strikes:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_6_empowered_strikes
        implementation: dnd2024:class_features_monk_level_6_empowered_strikes
        parameters:
          trigger: unarmed_strike.damage
          damage_type_choice:
          - force
          - normal
          class_scope:
            class: monk
            minimum_level: 6
    contract:
      id: dnd2024:class_features_monk_level_6_empowered_strikes
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - damage.pending
      capabilities:
      - character_features
      - damage
    requires:
    - dnd2024:class_features_monk_level_6_empowered_strikes
    citations:
    - SRD 5.2.1 pp. 51; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_6_empowered_strikes.md
  dnd2024:class_features/monk_level_9_acrobatic_movement:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_9_acrobatic_movement
        implementation: dnd2024:class_features_monk_level_9_acrobatic_movement
        parameters:
          requires:
            armor_worn: false
            shield_wielded: false
            turn: actor
          movement_surfaces:
          - vertical
          - liquid
          falling_during_movement: false
          class_scope:
            class: monk
            minimum_level: 9
    contract:
      id: dnd2024:class_features_monk_level_9_acrobatic_movement
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - participant.moved
      capabilities:
      - character_features
      - movement
    requires:
    - dnd2024:class_features_monk_level_9_acrobatic_movement
    citations:
    - SRD 5.2.1 pp. 51; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_9_acrobatic_movement.md
  dnd2024:class_features/monk_level_10_self_restoration:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_10_self_restoration
        implementation: dnd2024:class_features_monk_level_10_self_restoration
        parameters:
          condition_removal:
            event: actor.turn.end
            optional: true
            choose_maximum: 1
            conditions:
            - charmed
            - frightened
            - poisoned
          exhaustion_prevention_sources:
          - lack_of_food
          - lack_of_drink
          class_scope:
            class: monk
            minimum_level: 10
    contract:
      id: dnd2024:class_features_monk_level_10_self_restoration
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - turn.end
      - exhaustion.pending
      capabilities:
      - character_features
      - conditions
    requires:
    - dnd2024:class_features_monk_level_10_self_restoration
    citations:
    - SRD 5.2.1 pp. 52; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_10_self_restoration.md
  dnd2024:class_features/monk_level_14_disciplined_survivor:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_14_disciplined_survivor
        implementation: dnd2024:class_features_monk_level_14_disciplined_survivor
        parameters:
          saving_throw_proficiency: all
          failure_reroll:
            optional: true
            cost:
              resource: monk.focus_points
              amount: 1
            must_use_new_roll: true
          class_scope:
            class: monk
            minimum_level: 14
    contract:
      id: dnd2024:class_features_monk_level_14_disciplined_survivor
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - saving_throw.calculate
      - saving_throw.failed
      capabilities:
      - character_features
      - resources
      - proficiencies
    requires:
    - dnd2024:class_features_monk_level_14_disciplined_survivor
    citations:
    - SRD 5.2.1 pp. 52; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_14_disciplined_survivor.md
  dnd2024:class_features/monk_level_18_superior_defense:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_18_superior_defense
        implementation: dnd2024:class_features_monk_level_18_superior_defense
        parameters:
          trigger: actor.turn.start
          optional: true
          cost:
            resource: monk.focus_points
            amount: 3
          duration:
            minutes: 1
          ends_on:
          - actor.incapacitated
          resistance:
            all_damage_types: true
            except:
            - force
          class_scope:
            class: monk
            minimum_level: 18
    contract:
      id: dnd2024:class_features_monk_level_18_superior_defense
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - turn.start
      - condition.applied
      - damage.pending
      capabilities:
      - character_features
      - resources
      - damage
    requires:
    - dnd2024:class_features_monk_level_18_superior_defense
    citations:
    - SRD 5.2.1 pp. 52; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_18_superior_defense.md
  dnd2024:class_features/monk_level_20_body_and_mind:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_20_body_and_mind
        implementation: dnd2024:class_features_monk_level_20_body_and_mind
        parameters:
          ability_increases:
          - ability: dexterity
            amount: 4
            maximum: 25
          - ability: wisdom
            amount: 4
            maximum: 25
          class_scope:
            class: monk
            minimum_level: 20
    contract:
      id: dnd2024:class_features_monk_level_20_body_and_mind
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.feature_acquired
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_monk_level_20_body_and_mind
    citations:
    - SRD 5.2.1 pp. 52; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_20_body_and_mind.md
  dnd2024:class_features/warrior_of_the_open_hand_level_3_open_hand_technique:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/warrior_of_the_open_hand_level_3_open_hand_technique
        implementation: dnd2024:class_features_warrior_of_the_open_hand_level_3_open_hand_technique
        parameters:
          trigger: actor.flurry_of_blows_attack_hits_creature
          optional: true
          choose: 1
          save_dc:
            op: add
            args:
            - 8
            - var: actor.abilities.wisdom.modifier
            - var: actor.proficiency_bonus
          options:
            addle:
              opportunity_attacks: false
              end: target.next_turn.start
            push:
              save: strength
              failure:
                push:
                  maximum_feet: 15
                  direction: away_from_actor
            topple:
              save: dexterity
              failure:
                condition: prone
          class_scope:
            class: monk
            minimum_level: 3
    contract:
      id: dnd2024:class_features_warrior_of_the_open_hand_level_3_open_hand_technique
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - attack.hit
      - turn.start
      capabilities:
      - character_features
      - conditions
      - proficiencies
      - d20_tests
    requires:
    - dnd2024:class_features_warrior_of_the_open_hand_level_3_open_hand_technique
    citations:
    - SRD 5.2.1 pp. 52; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/warrior_of_the_open_hand_level_3_open_hand_technique.md
  dnd2024:class_features/warrior_of_the_open_hand_level_11_fleet_step:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/warrior_of_the_open_hand_level_11_fleet_step
        implementation: dnd2024:class_features_warrior_of_the_open_hand_level_11_fleet_step
        parameters:
          trigger: actor.bonus_action_completed
          exclude: step_of_the_wind
          grant:
            feature:
              key: dnd2024:class_features/monk_level_2_monk_s_focus
            option: step_of_the_wind
            timing: immediately_after_trigger
            additional_bonus_action_cost: false
            normal_optional_focus_costs_apply: true
          class_scope:
            class: monk
            minimum_level: 11
    contract:
      id: dnd2024:class_features_warrior_of_the_open_hand_level_11_fleet_step
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - action.bonus_completed
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_warrior_of_the_open_hand_level_11_fleet_step
    citations:
    - SRD 5.2.1 pp. 52; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/warrior_of_the_open_hand_level_11_fleet_step.md
  dnd2024:classes/paladin:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:classes/paladin
        implementation: dnd2024:classes_paladin
        parameters:
          class: paladin
          primary_abilities:
          - strength
          - charisma
          hit_point_die:
            count_per_class_level: 1
            sides: 10
          first_class_traits:
            save_proficiencies:
            - wisdom
            - charisma
            skill_proficiencies:
              count: 2
              choices:
              - athletics
              - insight
              - intimidation
              - medicine
              - persuasion
              - religion
            weapon_proficiencies:
              simple: true
              martial: true
            armor_training:
            - light
            - medium
            - heavy
            - shield
          starting_equipment:
            choose: 1
            options:
              equipment:
                items:
                - definition:
                    key: dnd2024:equipment/chain_mail
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/shield
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/longsword
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/javelin
                  quantity: 6
                - definition:
                    key: dnd2024:equipment/holy_symbol_varies
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/priest_s_pack_33_gp
                  quantity: 1
                currency:
                  gp: 9
              currency:
                currency:
                  gp: 150
          progression:
            key: dnd2024:tables/paladin_progression
          feature_grants:
          - minimum_class_level: 10
            definition:
              key: dnd2024:class_features/paladin_level_10_aura_of_courage
          - minimum_class_level: 11
            definition:
              key: dnd2024:class_features/paladin_level_11_radiant_strikes
          - minimum_class_level: 14
            definition:
              key: dnd2024:class_features/paladin_level_14_restoring_touch
          - minimum_class_level: 18
            definition:
              key: dnd2024:class_features/paladin_level_18_aura_expansion
          - minimum_class_level: 19
            definition:
              key: dnd2024:class_features/paladin_level_19_epic_boon
          - minimum_class_level: 1
            definition:
              key: dnd2024:class_features/paladin_level_1_lay_on_hands
          - minimum_class_level: 1
            definition:
              key: dnd2024:class_features/paladin_level_1_spellcasting
          - minimum_class_level: 1
            definition:
              key: dnd2024:class_features/paladin_level_1_weapon_mastery
          - minimum_class_level: 2
            definition:
              key: dnd2024:class_features/paladin_level_2_fighting_style
          - minimum_class_level: 2
            definition:
              key: dnd2024:class_features/paladin_level_2_paladin_s_smite
          - minimum_class_level: 3
            definition:
              key: dnd2024:class_features/paladin_level_3_channel_divinity
          - minimum_class_level: 3
            definition:
              key: dnd2024:class_features/paladin_level_3_paladin_subclass
          - minimum_class_level: 4
            definition:
              key: dnd2024:class_features/paladin_level_4_ability_score_improvement
          - minimum_class_level: 5
            definition:
              key: dnd2024:class_features/paladin_level_5_extra_attack
          - minimum_class_level: 5
            definition:
              key: dnd2024:class_features/paladin_level_5_faithful_steed
          - minimum_class_level: 6
            definition:
              key: dnd2024:class_features/paladin_level_6_aura_of_protection
          - minimum_class_level: 9
            definition:
              key: dnd2024:class_features/paladin_level_9_abjure_foes
    contract:
      id: dnd2024:classes_paladin
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - class_traits
      - equipment_choices
      - progression
      - feature_grants
      events:
      - character.class_selected
      - character.level_gained
      capabilities:
      - character_features
      - spellcasting
      - proficiencies
      - d20_tests
      - equipment
      - auras
    requires:
    - dnd2024:classes_paladin
    citations:
    - SRD 5.2.1 pp. 53; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/classes/paladin.md
  dnd2024:class_features/paladin_level_1_spellcasting:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/paladin_level_1_spellcasting
        implementation: dnd2024:class_features_paladin_level_1_spellcasting
        parameters:
          class: paladin
          spellcasting_ability: charisma
          spellcasting_focus: holy_symbol
          save_dc:
            op: add
            args:
            - 8
            - var: actor.abilities.charisma.modifier
            - var: actor.proficiency_bonus
          spell_attack_bonus:
            op: add
            args:
            - var: actor.proficiency_bonus
            - var: actor.abilities.charisma.modifier
          slots:
            progression:
              key: dnd2024:tables/paladin_progression
            recovery:
            - rest.long_completed
            recover: all
            pool: spellcasting
          prepared_spells:
            initial_count: 2
            initial_spell_level: 1
            progression:
              table:
                key: dnd2024:tables/paladin_progression
              row:
                var: actor.class_levels.paladin
              column: prepared_spells
            eligible_list: paladin
            eligible_levels:
              minimum: 1
              maximum: available_class_slot_level
            on_count_increase: choose_until_count_met
            replacement:
              event: rest.long_completed
              class: paladin
              maximum: 1
            always_prepared_from_other_class_features:
              counts_against_limit: false
              counts_as_class_spell: paladin
          class_scope:
            class: paladin
            minimum_level: 1
    contract:
      id: dnd2024:class_features_paladin_level_1_spellcasting
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - spellcasting
      - prepared_spells
      - resources
      events:
      - character.level_gained
      - rest.long_completed
      - rest.short_completed
      - spell.prepare
      - spell.cast
      capabilities:
      - character_features
      - spellcasting
      - proficiencies
      - d20_tests
    requires:
    - dnd2024:class_features_paladin_level_1_spellcasting
    citations:
    - SRD 5.2.1 pp. 54; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/paladin_level_1_spellcasting.md
  dnd2024:class_features/paladin_level_2_fighting_style:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/paladin_level_2_fighting_style
        implementation: dnd2024:class_features_paladin_level_2_fighting_style
        parameters:
          choose: 1
          options:
            fighting_style:
              feat_category: fighting_style
              count: 1
            blessed_warrior:
              cantrips:
                count: 2
                list: cleric
              counts_as_class: paladin
              ability: charisma
              replace:
                event: paladin.level_gained
                maximum: 1
                list: cleric
                spell_level: 0
          class_scope:
            class: paladin
            minimum_level: 2
    contract:
      id: dnd2024:class_features_paladin_level_2_fighting_style
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - feat_or_spell_grants
      events:
      - character.feature_acquired
      - character.level_gained
      capabilities:
      - character_features
      - spellcasting
    requires:
    - dnd2024:class_features_paladin_level_2_fighting_style
    citations:
    - SRD 5.2.1 pp. 54; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/paladin_level_2_fighting_style.md
  dnd2024:class_features/paladin_level_3_channel_divinity:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/paladin_level_3_channel_divinity
        implementation: dnd2024:class_features_paladin_level_3_channel_divinity
        parameters:
          resource:
            id: paladin.channel_divinity
            maximum:
              by_paladin_level:
                3: 2
                11: 3
            recovery:
            - event: rest.short_completed
              amount: 1
            - event: rest.long_completed
              amount: all
          choose_effect_each_use: 1
          cost_each_use: 1
          save_dc:
            op: add
            args:
            - 8
            - var: actor.abilities.charisma.modifier
            - var: actor.proficiency_bonus
          divine_sense:
            action: bonus_action
            duration:
              minutes: 10
            ends_on:
            - actor.incapacitated
            radius_feet: 60
            detect_creatures:
              types:
              - celestial
              - fiend
              - undead
              reveal:
              - location
              - creature_type
            detect_objects_and_places:
              tags_any:
              - consecrated
              - desecrated
              reveal:
              - presence
          class_scope:
            class: paladin
            minimum_level: 3
    contract:
      id: dnd2024:class_features_paladin_level_3_channel_divinity
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - action.bonus
      - rest.short_completed
      - rest.long_completed
      - condition.applied
      - participant.moved
      capabilities:
      - character_features
      - resources
      - proficiencies
      - d20_tests
      - senses
    requires:
    - dnd2024:class_features_paladin_level_3_channel_divinity
    citations:
    - SRD 5.2.1 pp. 54, 55; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/paladin_level_3_channel_divinity.md
  dnd2024:class_features/paladin_level_4_ability_score_improvement:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/paladin_level_4_ability_score_improvement
        implementation: dnd2024:class_features_paladin_level_4_ability_score_improvement
        parameters:
          class: paladin
          grant_at_class_levels:
          - 4
          - 8
          - 12
          - 16
          feat_choice:
            count_each: 1
            default:
              key: dnd2024:feats/general_feats_ability_score_improvement
            allowed_categories: any
            must_meet_prerequisites: true
          class_scope:
            class: paladin
            minimum_level: 4
    contract:
      id: dnd2024:class_features_paladin_level_4_ability_score_improvement
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - feat_grants
      events:
      - character.level_gained
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_paladin_level_4_ability_score_improvement
    citations:
    - SRD 5.2.1 pp. 55; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/paladin_level_4_ability_score_improvement.md
  dnd2024:class_features/paladin_level_5_faithful_steed:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/paladin_level_5_faithful_steed
        implementation: dnd2024:class_features_paladin_level_5_faithful_steed
        parameters:
          always_prepared:
          - key: dnd2024:spells/find_steed
          counts_against_prepared_limit: false
          free_cast:
            resource:
              id: paladin.faithful_steed
              maximum: 1
              recovery:
              - event: rest.long_completed
                amount: all
            cost: 1
            expend_slot: false
          class_scope:
            class: paladin
            minimum_level: 5
    contract:
      id: dnd2024:class_features_paladin_level_5_faithful_steed
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - prepared_spells
      - resources
      events:
      - spell.cast
      - rest.long_completed
      capabilities:
      - character_features
      - resources
      - spellcasting
    requires:
    - dnd2024:class_features_paladin_level_5_faithful_steed
    citations:
    - SRD 5.2.1 pp. 55; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/paladin_level_5_faithful_steed.md
  dnd2024:class_features/paladin_level_9_abjure_foes:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/paladin_level_9_abjure_foes
        implementation: dnd2024:class_features_paladin_level_9_abjure_foes
        parameters:
          action: magic
          resource_cost:
            id: paladin.channel_divinity
            amount: 1
          present_item_any:
          - holy_symbol
          - weapon
          targets:
            maximum:
              op: max
              args:
              - 1
              - var: actor.abilities.charisma.modifier
            visible: true
            range_feet: 60
            type: creature
          save:
            ability: wisdom
            dc:
              op: add
              args:
              - 8
              - var: actor.abilities.charisma.modifier
              - var: actor.proficiency_bonus
            failure:
              condition: frightened
              source: actor
              duration:
                minutes: 1
              ends_on:
              - target.damage_taken
              turn_activity_limit:
                choose_only_one:
                - move
                - action
                - bonus_action
          class_scope:
            class: paladin
            minimum_level: 9
    contract:
      id: dnd2024:class_features_paladin_level_9_abjure_foes
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - action.magic
      - damage.taken
      - turn.start
      capabilities:
      - character_features
      - resources
      - damage
      - conditions
      - proficiencies
      - d20_tests
    requires:
    - dnd2024:class_features_paladin_level_9_abjure_foes
    citations:
    - SRD 5.2.1 pp. 55; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/paladin_level_9_abjure_foes.md
  dnd2024:class_features/paladin_level_11_radiant_strikes:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/paladin_level_11_radiant_strikes
        implementation: dnd2024:class_features_paladin_level_11_radiant_strikes
        parameters:
          trigger: actor.attack_hits_target
          attack_sources:
          - melee_weapon
          - unarmed_strike
          extra_damage:
            dice:
              count: 1
              sides: 8
            type: radiant
          class_scope:
            class: paladin
            minimum_level: 11
    contract:
      id: dnd2024:class_features_paladin_level_11_radiant_strikes
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - attack.hit
      capabilities:
      - character_features
      - damage
    requires:
    - dnd2024:class_features_paladin_level_11_radiant_strikes
    citations:
    - SRD 5.2.1 pp. 55; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/paladin_level_11_radiant_strikes.md
  dnd2024:class_features/paladin_level_18_aura_expansion:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/paladin_level_18_aura_expansion
        implementation: dnd2024:class_features_paladin_level_18_aura_expansion
        parameters:
          modifies_aura: paladin.aura_of_protection
          shape: emanation
          radius_feet: 30
          class_scope:
            class: paladin
            minimum_level: 18
    contract:
      id: dnd2024:class_features_paladin_level_18_aura_expansion
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - aura.calculate
      capabilities:
      - character_features
      - auras
    requires:
    - dnd2024:class_features_paladin_level_18_aura_expansion
    citations:
    - SRD 5.2.1 pp. 55; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/paladin_level_18_aura_expansion.md
  dnd2024:subclasses/paladin_paladin_subclass_oath_of_devotion:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:subclasses/paladin_paladin_subclass_oath_of_devotion
        implementation: dnd2024:subclasses_paladin_paladin_subclass_oath_of_devotion
        parameters:
          class: paladin
          minimum_class_level: 3
          feature_grants:
          - class_level: 15
            definition:
              key: dnd2024:class_features/oath_of_devotion_level_15_smite_of_protection
          - class_level: 20
            definition:
              key: dnd2024:class_features/oath_of_devotion_level_20_holy_nimbus
          - class_level: 3
            definition:
              key: dnd2024:class_features/oath_of_devotion_level_3_oath_of_devotion_spells
          - class_level: 3
            definition:
              key: dnd2024:class_features/oath_of_devotion_level_3_sacred_weapon
          - class_level: 7
            definition:
              key: dnd2024:class_features/oath_of_devotion_level_7_aura_of_devotion
          grant_eligible_features:
            'on':
            - subclass.selected
            - character.level_gained
            comparison: class_level_at_least_threshold
            persistent: true
    contract:
      id: dnd2024:subclasses_paladin_paladin_subclass_oath_of_devotion
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - feature_grants
      events:
      - subclass.selected
      - character.level_gained
      capabilities:
      - character_features
      - spellcasting
      - auras
    requires:
    - dnd2024:subclasses_paladin_paladin_subclass_oath_of_devotion
    citations:
    - SRD 5.2.1 pp. 56; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/subclasses/paladin_paladin_subclass_oath_of_devotion.md
  dnd2024:class_features/oath_of_devotion_level_3_sacred_weapon:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/oath_of_devotion_level_3_sacred_weapon
        implementation: dnd2024:class_features_oath_of_devotion_level_3_sacred_weapon
        parameters:
          trigger: actor.attack_action
          optional: true
          resource_cost:
            id: paladin.channel_divinity
            amount: 1
          target_item:
            type: melee_weapon
            held_by_actor: true
            count: 1
          duration:
            minutes: 10
          ends_on:
          - feature.used_again
          - weapon.not_carried_by_actor
          - actor.voluntary_end_no_action
          attack_bonus:
            actor_only: true
            weapon: selected
            amount:
              op: max
              args:
              - 1
              - var: actor.abilities.charisma.modifier
          hit_damage_type_choice:
          - normal
          - radiant
          light:
            bright_radius_feet: 20
            additional_dim_radius_feet: 20
          class_scope:
            class: paladin
            minimum_level: 3
    contract:
      id: dnd2024:class_features_oath_of_devotion_level_3_sacred_weapon
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - action.attack
      - attack.roll
      - attack.hit
      - item.carrying_changed
      capabilities:
      - character_features
      - resources
      - damage
    requires:
    - dnd2024:class_features_oath_of_devotion_level_3_sacred_weapon
    citations:
    - SRD 5.2.1 pp. 56; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/oath_of_devotion_level_3_sacred_weapon.md
  dnd2024:class_features/oath_of_devotion_level_15_smite_of_protection:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/oath_of_devotion_level_15_smite_of_protection
        implementation: dnd2024:class_features_oath_of_devotion_level_15_smite_of_protection
        parameters:
          trigger:
            spell_cast:
              key: dnd2024:spells/divine_smite
            caster: actor
          aura: paladin.aura_of_protection
          targets:
          - actor
          - allies
          effect:
            cover: half
          end: actor.next_turn.start
          class_scope:
            class: paladin
            minimum_level: 15
    contract:
      id: dnd2024:class_features_oath_of_devotion_level_15_smite_of_protection
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast
      - turn.start
      - participant.moved
      capabilities:
      - character_features
      - spellcasting
      - auras
    requires:
    - dnd2024:class_features_oath_of_devotion_level_15_smite_of_protection
    citations:
    - SRD 5.2.1 pp. 57; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/oath_of_devotion_level_15_smite_of_protection.md
  dnd2024:classes/ranger:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:classes/ranger
        implementation: dnd2024:classes_ranger
        parameters:
          class: ranger
          primary_abilities:
          - dexterity
          - wisdom
          hit_point_die:
            count_per_class_level: 1
            sides: 10
          first_class_traits:
            save_proficiencies:
            - strength
            - dexterity
            skill_proficiencies:
              count: 3
              choices:
              - animal_handling
              - athletics
              - insight
              - investigation
              - nature
              - perception
              - stealth
              - survival
            weapon_proficiencies:
              simple: true
              martial: true
            armor_training:
            - light
            - medium
            - shield
          starting_equipment:
            choose: 1
            options:
              equipment:
                items:
                - definition:
                    key: dnd2024:equipment/studded_leather_armor
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/scimitar
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/shortsword
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/longbow
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/ammunition_varies
                  quantity: 20
                  variant: arrows
                - definition:
                    key: dnd2024:equipment/quiver_1_gp
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/druidic_focus_varies
                  quantity: 1
                  variant: sprig_of_mistletoe
                - definition:
                    key: dnd2024:equipment/explorer_s_pack_10_gp
                  quantity: 1
                currency:
                  gp: 7
              currency:
                currency:
                  gp: 150
          progression:
            key: dnd2024:tables/ranger_progression
          feature_grants:
          - minimum_class_level: 10
            definition:
              key: dnd2024:class_features/ranger_level_10_tireless
          - minimum_class_level: 13
            definition:
              key: dnd2024:class_features/ranger_level_13_relentless_hunter
          - minimum_class_level: 14
            definition:
              key: dnd2024:class_features/ranger_level_14_nature_s_veil
          - minimum_class_level: 17
            definition:
              key: dnd2024:class_features/ranger_level_17_precise_hunter
          - minimum_class_level: 18
            definition:
              key: dnd2024:class_features/ranger_level_18_feral_senses
          - minimum_class_level: 19
            definition:
              key: dnd2024:class_features/ranger_level_19_epic_boon
          - minimum_class_level: 1
            definition:
              key: dnd2024:class_features/ranger_level_1_favored_enemy
          - minimum_class_level: 1
            definition:
              key: dnd2024:class_features/ranger_level_1_spellcasting
          - minimum_class_level: 1
            definition:
              key: dnd2024:class_features/ranger_level_1_weapon_mastery
          - minimum_class_level: 20
            definition:
              key: dnd2024:class_features/ranger_level_20_foe_slayer
          - minimum_class_level: 2
            definition:
              key: dnd2024:class_features/ranger_level_2_deft_explorer
          - minimum_class_level: 2
            definition:
              key: dnd2024:class_features/ranger_level_2_fighting_style
          - minimum_class_level: 3
            definition:
              key: dnd2024:class_features/ranger_level_3_ranger_subclass
          - minimum_class_level: 4
            definition:
              key: dnd2024:class_features/ranger_level_4_ability_score_improvement
          - minimum_class_level: 5
            definition:
              key: dnd2024:class_features/ranger_level_5_extra_attack
          - minimum_class_level: 6
            definition:
              key: dnd2024:class_features/ranger_level_6_roving
          - minimum_class_level: 9
            definition:
              key: dnd2024:class_features/ranger_level_9_expertise
    contract:
      id: dnd2024:classes_ranger
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - class_traits
      - equipment_choices
      - progression
      - feature_grants
      events:
      - character.class_selected
      - character.level_gained
      capabilities:
      - character_features
      - spellcasting
      - proficiencies
      - d20_tests
      - equipment
      - senses
    requires:
    - dnd2024:classes_ranger
    citations:
    - SRD 5.2.1 pp. 57; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/classes/ranger.md
  dnd2024:class_features/ranger_level_1_favored_enemy:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/ranger_level_1_favored_enemy
        implementation: dnd2024:class_features_ranger_level_1_favored_enemy
        parameters:
          always_prepared:
          - key: dnd2024:spells/hunter_s_mark
          counts_against_prepared_limit: false
          free_cast:
            resource:
              id: ranger.favored_enemy
              maximum:
                table:
                  key: dnd2024:tables/ranger_progression
                row:
                  var: actor.class_levels.ranger
                column: favored_enemy
              recovery:
              - event: rest.long_completed
                amount: all
            cost: 1
            expend_slot: false
          class_scope:
            class: ranger
            minimum_level: 1
    contract:
      id: dnd2024:class_features_ranger_level_1_favored_enemy
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - prepared_spells
      - resources
      events:
      - spell.cast
      - rest.long_completed
      capabilities:
      - character_features
      - resources
      - spellcasting
    requires:
    - dnd2024:class_features_ranger_level_1_favored_enemy
    citations:
    - SRD 5.2.1 pp. 58; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/ranger_level_1_favored_enemy.md
  dnd2024:class_features/ranger_level_2_deft_explorer:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/ranger_level_2_deft_explorer
        implementation: dnd2024:class_features_ranger_level_2_deft_explorer
        parameters:
          expertise:
            count: 1
            eligible:
              proficient: true
              expertise: false
            grant: expertise
          languages:
            count: 2
            selection: actor_choice
            eligible_tables:
            - standard_languages
            - rare_languages
          class_scope:
            class: ranger
            minimum_level: 2
    contract:
      id: dnd2024:class_features_ranger_level_2_deft_explorer
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.feature_acquired
      capabilities:
      - character_features
      - proficiencies
    requires:
    - dnd2024:class_features_ranger_level_2_deft_explorer
    citations:
    - SRD 5.2.1 pp. 59; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/ranger_level_2_deft_explorer.md
  dnd2024:class_features/ranger_level_3_ranger_subclass:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/ranger_level_3_ranger_subclass
        implementation: dnd2024:class_features_ranger_level_3_ranger_subclass
        parameters:
          subclass_choice:
            count: 1
            class: ranger
          feature_grant_policy:
            class_level_at_least_feature_level: true
            events:
            - subclass.selected
            - character.level_gained
            persistent: true
          class_scope:
            class: ranger
            minimum_level: 3
    contract:
      id: dnd2024:class_features_ranger_level_3_ranger_subclass
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - subclass
      - feature_grants
      events:
      - subclass.selected
      - character.level_gained
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_ranger_level_3_ranger_subclass
    citations:
    - SRD 5.2.1 pp. 59; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/ranger_level_3_ranger_subclass.md
  dnd2024:class_features/ranger_level_5_extra_attack:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/ranger_level_5_extra_attack
        implementation: dnd2024:class_features_ranger_level_5_extra_attack
        parameters:
          action: attack
          turn: actor
          attack_count: 2
          class_scope:
            class: ranger
            minimum_level: 5
    contract:
      id: dnd2024:class_features_ranger_level_5_extra_attack
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - action.attack
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_ranger_level_5_extra_attack
    citations:
    - SRD 5.2.1 pp. 59; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/ranger_level_5_extra_attack.md
  dnd2024:class_features/ranger_level_9_expertise:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/ranger_level_9_expertise
        implementation: dnd2024:class_features_ranger_level_9_expertise
        parameters:
          expertise:
            count: 2
            eligible:
              proficient: true
              expertise: false
            grant: expertise
          class_scope:
            class: ranger
            minimum_level: 9
    contract:
      id: dnd2024:class_features_ranger_level_9_expertise
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.feature_acquired
      capabilities:
      - character_features
      - proficiencies
    requires:
    - dnd2024:class_features_ranger_level_9_expertise
    citations:
    - SRD 5.2.1 pp. 59; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/ranger_level_9_expertise.md
  dnd2024:class_features/ranger_level_13_relentless_hunter:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/ranger_level_13_relentless_hunter
        implementation: dnd2024:class_features_ranger_level_13_relentless_hunter
        parameters:
          spell:
            key: dnd2024:spells/hunter_s_mark
          concentration_end_prevention:
            cause: damage_taken
          class_scope:
            class: ranger
            minimum_level: 13
    contract:
      id: dnd2024:class_features_ranger_level_13_relentless_hunter
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - concentration.damage_check
      capabilities:
      - character_features
      - spellcasting
      - damage
    requires:
    - dnd2024:class_features_ranger_level_13_relentless_hunter
    citations:
    - SRD 5.2.1 pp. 59; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/ranger_level_13_relentless_hunter.md
  dnd2024:class_features/ranger_level_17_precise_hunter:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/ranger_level_17_precise_hunter
        implementation: dnd2024:class_features_ranger_level_17_precise_hunter
        parameters:
          advantage:
            attack_target: actor.current_hunters_mark_target
          class_scope:
            class: ranger
            minimum_level: 17
    contract:
      id: dnd2024:class_features_ranger_level_17_precise_hunter
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - attack.roll
      capabilities:
      - character_features
      - d20_tests
    requires:
    - dnd2024:class_features_ranger_level_17_precise_hunter
    citations:
    - SRD 5.2.1 pp. 59; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/ranger_level_17_precise_hunter.md
  dnd2024:class_features/ranger_level_19_epic_boon:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/ranger_level_19_epic_boon
        implementation: dnd2024:class_features_ranger_level_19_epic_boon
        parameters:
          feat_choice:
            count: 1
            category_preference: epic_boon
            allowed_categories: any
            must_meet_prerequisites: true
          class_scope:
            class: ranger
            minimum_level: 19
    contract:
      id: dnd2024:class_features_ranger_level_19_epic_boon
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.feature_acquired
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_ranger_level_19_epic_boon
    citations:
    - SRD 5.2.1 pp. 59; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/ranger_level_19_epic_boon.md
  dnd2024:subclasses/ranger_ranger_subclass_hunter:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:subclasses/ranger_ranger_subclass_hunter
        implementation: dnd2024:subclasses_ranger_ranger_subclass_hunter
        parameters:
          class: ranger
          minimum_class_level: 3
          feature_grants:
          - class_level: 11
            definition:
              key: dnd2024:class_features/hunter_level_11_superior_hunter_s_prey
          - class_level: 15
            definition:
              key: dnd2024:class_features/hunter_level_15_superior_hunter_s_defense
          - class_level: 3
            definition:
              key: dnd2024:class_features/hunter_level_3_hunter_s_lore
          - class_level: 3
            definition:
              key: dnd2024:class_features/hunter_level_3_hunter_s_prey
          - class_level: 7
            definition:
              key: dnd2024:class_features/hunter_level_7_defensive_tactics
          grant_eligible_features:
            'on':
            - subclass.selected
            - character.level_gained
            comparison: class_level_at_least_threshold
            persistent: true
    contract:
      id: dnd2024:subclasses_ranger_ranger_subclass_hunter
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - feature_grants
      events:
      - subclass.selected
      - character.level_gained
      capabilities:
      - character_features
    requires:
    - dnd2024:subclasses_ranger_ranger_subclass_hunter
    citations:
    - SRD 5.2.1 pp. 61; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/subclasses/ranger_ranger_subclass_hunter.md
  dnd2024:class_features/hunter_level_3_hunter_s_prey:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/hunter_level_3_hunter_s_prey
        implementation: dnd2024:class_features_hunter_level_3_hunter_s_prey
        parameters:
          choose: 1
          replacement:
            events:
            - rest.short_completed
            - rest.long_completed
            choose_other_option: true
          options:
            colossus_slayer:
              trigger: actor.weapon_hits_creature
              requires:
                target_missing_hit_points: true
              extra_damage:
                dice:
                  count: 1
                  sides: 8
                type: weapon.damage_type
              limit:
                count: 1
                period: turn
            horde_breaker:
              trigger: actor.weapon_attack
              limit:
                count: 1
                period: actor.turn
              extra_attack:
                weapon: same
                target:
                  type: creature
                  different_from_original: true
                  within_original_target_feet: 5
                  within_weapon_range: true
                  not_attacked_by_actor_this_turn: true
          class_scope:
            class: ranger
            minimum_level: 3
    contract:
      id: dnd2024:class_features_hunter_level_3_hunter_s_prey
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - rest.short_completed
      - rest.long_completed
      - attack.hit
      - attack.roll
      capabilities:
      - character_features
      - damage
    requires:
    - dnd2024:class_features_hunter_level_3_hunter_s_prey
    citations:
    - SRD 5.2.1 pp. 61; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/hunter_level_3_hunter_s_prey.md
  dnd2024:class_features/hunter_level_11_superior_hunter_s_prey:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/hunter_level_11_superior_hunter_s_prey
        implementation: dnd2024:class_features_hunter_level_11_superior_hunter_s_prey
        parameters:
          trigger: actor.damages_own_hunters_mark_target
          limit:
            count: 1
            period: turn
          optional: true
          additional_target:
            type: creature
            different_from_original: true
            visible_to_actor: true
            within_original_target_feet: 30
          damage: triggering_hunters_mark.extra_damage
          class_scope:
            class: ranger
            minimum_level: 11
    contract:
      id: dnd2024:class_features_hunter_level_11_superior_hunter_s_prey
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - damage.dealt
      capabilities:
      - character_features
      - damage
    requires:
    - dnd2024:class_features_hunter_level_11_superior_hunter_s_prey
    citations:
    - SRD 5.2.1 pp. 61; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/hunter_level_11_superior_hunter_s_prey.md
  dnd2024:classes/rogue:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:classes/rogue
        implementation: dnd2024:classes_rogue
        parameters:
          class: rogue
          primary_abilities:
          - dexterity
          hit_point_die:
            count_per_class_level: 1
            sides: 8
          first_class_traits:
            save_proficiencies:
            - dexterity
            - intelligence
            skill_proficiencies:
              count: 4
              choices:
              - acrobatics
              - athletics
              - deception
              - insight
              - intimidation
              - investigation
              - perception
              - persuasion
              - sleight_of_hand
              - stealth
            weapon_proficiencies:
              simple: true
              martial_required_property_any:
              - finesse
              - light
            armor_training:
            - light
            tool_proficiencies:
            - key: dnd2024:equipment/other_tools_thieves_tools_25_gp
          starting_equipment:
            choose: 1
            options:
              equipment:
                items:
                - definition:
                    key: dnd2024:equipment/leather_armor
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/dagger
                  quantity: 2
                - definition:
                    key: dnd2024:equipment/shortsword
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/shortbow
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/ammunition_varies
                  quantity: 20
                  variant: arrows
                - definition:
                    key: dnd2024:equipment/quiver_1_gp
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/other_tools_thieves_tools_25_gp
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/burglar_s_pack_16_gp
                  quantity: 1
                currency:
                  gp: 8
              currency:
                currency:
                  gp: 100
          progression:
            key: dnd2024:tables/rogue_progression
          feature_grants:
          - minimum_class_level: 11
            definition:
              key: dnd2024:class_features/rogue_level_11_improved_cunning_strike
          - minimum_class_level: 14
            definition:
              key: dnd2024:class_features/rogue_level_14_devious_strikes
          - minimum_class_level: 15
            definition:
              key: dnd2024:class_features/rogue_level_15_slippery_mind
          - minimum_class_level: 18
            definition:
              key: dnd2024:class_features/rogue_level_18_elusive
          - minimum_class_level: 19
            definition:
              key: dnd2024:class_features/rogue_level_19_epic_boon
          - minimum_class_level: 1
            definition:
              key: dnd2024:class_features/rogue_level_1_expertise
          - minimum_class_level: 1
            definition:
              key: dnd2024:class_features/rogue_level_1_sneak_attack
          - minimum_class_level: 1
            definition:
              key: dnd2024:class_features/rogue_level_1_thieves_cant
          - minimum_class_level: 1
            definition:
              key: dnd2024:class_features/rogue_level_1_weapon_mastery
          - minimum_class_level: 20
            definition:
              key: dnd2024:class_features/rogue_level_20_stroke_of_luck
          - minimum_class_level: 2
            definition:
              key: dnd2024:class_features/rogue_level_2_cunning_action
          - minimum_class_level: 3
            definition:
              key: dnd2024:class_features/rogue_level_3_rogue_subclass
          - minimum_class_level: 3
            definition:
              key: dnd2024:class_features/rogue_level_3_steady_aim
          - minimum_class_level: 4
            definition:
              key: dnd2024:class_features/rogue_level_4_ability_score_improvement
          - minimum_class_level: 5
            definition:
              key: dnd2024:class_features/rogue_level_5_cunning_strike
          - minimum_class_level: 5
            definition:
              key: dnd2024:class_features/rogue_level_5_uncanny_dodge
          - minimum_class_level: 7
            definition:
              key: dnd2024:class_features/rogue_level_7_evasion
          - minimum_class_level: 7
            definition:
              key: dnd2024:class_features/rogue_level_7_reliable_talent
    contract:
      id: dnd2024:classes_rogue
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - class_traits
      - equipment_choices
      - progression
      - feature_grants
      events:
      - character.class_selected
      - character.level_gained
      capabilities:
      - character_features
      - proficiencies
      - d20_tests
      - equipment
    requires:
    - dnd2024:classes_rogue
    citations:
    - SRD 5.2.1 pp. 61; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/classes/rogue.md
  dnd2024:class_features/rogue_level_1_sneak_attack:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/rogue_level_1_sneak_attack
        implementation: dnd2024:class_features_rogue_level_1_sneak_attack
        parameters:
          limit:
            count: 1
            period: turn
          trigger: actor.attack_hits_creature
          requires:
            weapon_any:
            - finesse_property
            - ranged_weapon
            any:
            - attack_advantage: true
            - attack_disadvantage: false
              ally_within_target_feet: 5
              ally_incapacitated: false
          extra_damage:
            dice:
              count:
                op: ceil
                args:
                - op: divide
                  args:
                  - var: actor.class_levels.rogue
                  - 2
              sides: 6
            type: weapon.damage_type
          class_scope:
            class: rogue
            minimum_level: 1
    contract:
      id: dnd2024:class_features_rogue_level_1_sneak_attack
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - attack.hit
      capabilities:
      - character_features
      - damage
      - d20_tests
    requires:
    - dnd2024:class_features_rogue_level_1_sneak_attack
    citations:
    - SRD 5.2.1 pp. 61, 62; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/rogue_level_1_sneak_attack.md
  dnd2024:class_features/rogue_level_1_weapon_mastery:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/rogue_level_1_weapon_mastery
        implementation: dnd2024:class_features_rogue_level_1_weapon_mastery
        parameters:
          mastery_choices:
            count: 2
            eligible:
              actor_proficient: true
            replacement:
              event: rest.long_completed
              maximum: 2
          class_scope:
            class: rogue
            minimum_level: 1
    contract:
      id: dnd2024:class_features_rogue_level_1_weapon_mastery
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - rest.long_completed
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_rogue_level_1_weapon_mastery
    citations:
    - SRD 5.2.1 pp. 62; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/rogue_level_1_weapon_mastery.md
  dnd2024:class_features/rogue_level_3_rogue_subclass:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/rogue_level_3_rogue_subclass
        implementation: dnd2024:class_features_rogue_level_3_rogue_subclass
        parameters:
          subclass_choice:
            count: 1
            class: rogue
          feature_grant_policy:
            class_level_at_least_feature_level: true
            events:
            - subclass.selected
            - character.level_gained
            persistent: true
          class_scope:
            class: rogue
            minimum_level: 3
    contract:
      id: dnd2024:class_features_rogue_level_3_rogue_subclass
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - subclass
      - feature_grants
      events:
      - subclass.selected
      - character.level_gained
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_rogue_level_3_rogue_subclass
    citations:
    - SRD 5.2.1 pp. 62; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/rogue_level_3_rogue_subclass.md
  dnd2024:class_features/rogue_level_4_ability_score_improvement:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/rogue_level_4_ability_score_improvement
        implementation: dnd2024:class_features_rogue_level_4_ability_score_improvement
        parameters:
          class: rogue
          grant_at_class_levels:
          - 4
          - 8
          - 10
          - 12
          - 16
          feat_choice:
            count_each: 1
            default:
              key: dnd2024:feats/general_feats_ability_score_improvement
            allowed_categories: any
            must_meet_prerequisites: true
          class_scope:
            class: rogue
            minimum_level: 4
    contract:
      id: dnd2024:class_features_rogue_level_4_ability_score_improvement
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - feat_grants
      events:
      - character.level_gained
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_rogue_level_4_ability_score_improvement
    citations:
    - SRD 5.2.1 pp. 63; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/rogue_level_4_ability_score_improvement.md
  dnd2024:class_features/rogue_level_5_uncanny_dodge:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/rogue_level_5_uncanny_dodge
        implementation: dnd2024:class_features_rogue_level_5_uncanny_dodge
        parameters:
          action: reaction
          trigger: visible_attacker.attack_hits_actor
          damage_multiplier: 0.5
          rounding: down
          scope: trigger_attack.damage_against_actor
          class_scope:
            class: rogue
            minimum_level: 5
    contract:
      id: dnd2024:class_features_rogue_level_5_uncanny_dodge
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - attack.hit
      - damage.pending
      capabilities:
      - character_features
      - damage
    requires:
    - dnd2024:class_features_rogue_level_5_uncanny_dodge
    citations:
    - SRD 5.2.1 pp. 63; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/rogue_level_5_uncanny_dodge.md
  dnd2024:class_features/rogue_level_7_reliable_talent:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/rogue_level_7_reliable_talent
        implementation: dnd2024:class_features_rogue_level_7_reliable_talent
        parameters:
          requires:
            test: ability_check
            uses_skill_or_tool_proficiency: true
          optional_d20_floor: 10
          class_scope:
            class: rogue
            minimum_level: 7
    contract:
      id: dnd2024:class_features_rogue_level_7_reliable_talent
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - ability.check.rolled
      capabilities:
      - character_features
      - proficiencies
    requires:
    - dnd2024:class_features_rogue_level_7_reliable_talent
    citations:
    - SRD 5.2.1 pp. 63; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/rogue_level_7_reliable_talent.md
  dnd2024:class_features/rogue_level_14_devious_strikes:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/rogue_level_14_devious_strikes
        implementation: dnd2024:class_features_rogue_level_14_devious_strikes
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
    contract:
      id: dnd2024:class_features_rogue_level_14_devious_strikes
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
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
    requires:
    - dnd2024:class_features_rogue_level_14_devious_strikes
    citations:
    - SRD 5.2.1 pp. 63; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/rogue_level_14_devious_strikes.md
  dnd2024:class_features/rogue_level_18_elusive:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/rogue_level_18_elusive
        implementation: dnd2024:class_features_rogue_level_18_elusive
        parameters:
          unless:
            actor_condition: incapacitated
          attacks_against_actor_can_have_advantage: false
          class_scope:
            class: rogue
            minimum_level: 18
    contract:
      id: dnd2024:class_features_rogue_level_18_elusive
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - attack.roll
      capabilities:
      - character_features
      - conditions
      - d20_tests
    requires:
    - dnd2024:class_features_rogue_level_18_elusive
    citations:
    - SRD 5.2.1 pp. 63; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/rogue_level_18_elusive.md
  dnd2024:class_features/rogue_level_20_stroke_of_luck:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/rogue_level_20_stroke_of_luck
        implementation: dnd2024:class_features_rogue_level_20_stroke_of_luck
        parameters:
          trigger: actor.d20_test.failed
          optional: true
          replace_d20: 20
          recompute_result: true
          resource:
            id: rogue.stroke_of_luck
            maximum: 1
            recovery:
            - event: rest.short_completed
              amount: all
            - event: rest.long_completed
              amount: all
          cost: 1
          class_scope:
            class: rogue
            minimum_level: 20
    contract:
      id: dnd2024:class_features_rogue_level_20_stroke_of_luck
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - d20_test.failed
      - rest.short_completed
      - rest.long_completed
      capabilities:
      - character_features
      - resources
    requires:
    - dnd2024:class_features_rogue_level_20_stroke_of_luck
    citations:
    - SRD 5.2.1 pp. 63; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/rogue_level_20_stroke_of_luck.md
  dnd2024:class_features/thief_level_3_fast_hands:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/thief_level_3_fast_hands
        implementation: dnd2024:class_features_thief_level_3_fast_hands
        parameters:
          action: bonus_action
          choose: 1
          options:
            sleight_of_hand:
              ability: dexterity
              skill: sleight_of_hand
              tasks:
                pick_lock:
                  tool: thieves_tools
                disarm_trap:
                  tool: thieves_tools
                pick_pocket: {}
            utilize:
              take_action: utilize
            magic_item:
              take_action: magic
              scope: use_magic_item
              item_requires_action: magic
          class_scope:
            class: rogue
            minimum_level: 3
    contract:
      id: dnd2024:class_features_thief_level_3_fast_hands
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - action.bonus
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_thief_level_3_fast_hands
    citations:
    - SRD 5.2.1 pp. 64; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/thief_level_3_fast_hands.md
  dnd2024:class_features/thief_level_9_supreme_sneak:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/thief_level_9_supreme_sneak
        implementation: dnd2024:class_features_thief_level_9_supreme_sneak
        parameters:
          extends: rogue.cunning_strike
          option: stealth_attack
          cost:
            resource: sneak_attack.d6_dice
            amount: 1
          requires:
            invisible_source: hide_action
          attack_ends_invisible: false
          conditional_retention:
            check: actor.turn.end
            requires_cover_any:
            - three_quarters
            - total
            on_requirement_failure: end_hide_invisible
          class_scope:
            class: rogue
            minimum_level: 9
    contract:
      id: dnd2024:class_features_thief_level_9_supreme_sneak
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - sneak_attack.damage_pending
      - attack.roll
      - turn.end
      capabilities:
      - character_features
      - resources
      - conditions
    requires:
    - dnd2024:class_features_thief_level_9_supreme_sneak
    citations:
    - SRD 5.2.1 pp. 64; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/thief_level_9_supreme_sneak.md
  dnd2024:class_features/thief_level_17_thief_s_reflexes:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/thief_level_17_thief_s_reflexes
        implementation: dnd2024:class_features_thief_level_17_thief_s_reflexes
        parameters:
          combat_round: 1
          turns:
          - initiative: normal
          - initiative:
              op: subtract
              args:
              - var: actor.initiative
              - 10
          class_scope:
            class: rogue
            minimum_level: 17
    contract:
      id: dnd2024:class_features_thief_level_17_thief_s_reflexes
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - turn_schedule
      events:
      - combat.started
      - initiative.ordered
      capabilities:
      - character_features
      - initiative
    requires:
    - dnd2024:class_features_thief_level_17_thief_s_reflexes
    citations:
    - SRD 5.2.1 pp. 64; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/thief_level_17_thief_s_reflexes.md
  dnd2024:class_features/sorcerer_level_1_spellcasting:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/sorcerer_level_1_spellcasting
        implementation: dnd2024:class_features_sorcerer_level_1_spellcasting
        parameters:
          class: sorcerer
          spellcasting_ability: charisma
          spellcasting_focus: arcane_focus
          save_dc:
            op: add
            args:
            - 8
            - var: actor.abilities.charisma.modifier
            - var: actor.proficiency_bonus
          spell_attack_bonus:
            op: add
            args:
            - var: actor.proficiency_bonus
            - var: actor.abilities.charisma.modifier
          slots:
            progression:
              key: dnd2024:tables/sorcerer_progression
            recovery:
            - rest.long_completed
            recover: all
            pool: spellcasting
          prepared_spells:
            initial_count: 2
            initial_spell_level: 1
            progression:
              table:
                key: dnd2024:tables/sorcerer_progression
              row:
                var: actor.class_levels.sorcerer
              column: prepared_spells
            eligible_list: sorcerer
            eligible_levels:
              minimum: 1
              maximum: available_class_slot_level
            on_count_increase: choose_until_count_met
            replacement:
              event: character.level_gained
              class: sorcerer
              maximum: 1
            always_prepared_from_other_class_features:
              counts_against_limit: false
              counts_as_class_spell: sorcerer
          cantrips:
            initial_count: 4
            list: sorcerer
            additional_levels:
            - 4
            - 10
            additional_each: 1
            replace:
              event: character.level_gained
              class: sorcerer
              maximum: 1
              same_list: true
          class_scope:
            class: sorcerer
            minimum_level: 1
    contract:
      id: dnd2024:class_features_sorcerer_level_1_spellcasting
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - spellcasting
      - prepared_spells
      - resources
      events:
      - character.level_gained
      - rest.long_completed
      - rest.short_completed
      - spell.prepare
      - spell.cast
      capabilities:
      - character_features
      - spellcasting
      - proficiencies
      - d20_tests
    requires:
    - dnd2024:class_features_sorcerer_level_1_spellcasting
    citations:
    - SRD 5.2.1 pp. 64, 65; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/sorcerer_level_1_spellcasting.md
  dnd2024:class_features/sorcerer_level_2_font_of_magic:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/sorcerer_level_2_font_of_magic
        implementation: dnd2024:class_features_sorcerer_level_2_font_of_magic
        parameters:
          resource:
            id: sorcerer.sorcery_points
            maximum:
              var: actor.class_levels.sorcerer
            recovery:
            - event: rest.long_completed
              amount: all
          cannot_exceed_maximum: true
          slot_to_points:
            action: none
            expend_slot_count: 1
            restore_points:
              var: slot.level
          points_to_slot:
            action: bonus_action
            create_slot_count: 1
            options:
            - slot_level: 1
              point_cost: 2
              minimum_sorcerer_level: 2
            - slot_level: 2
              point_cost: 3
              minimum_sorcerer_level: 3
            - slot_level: 3
              point_cost: 5
              minimum_sorcerer_level: 5
            - slot_level: 4
              point_cost: 6
              minimum_sorcerer_level: 7
            - slot_level: 5
              point_cost: 7
              minimum_sorcerer_level: 9
            slot_expires: actor.rest.long_completed
          class_scope:
            class: sorcerer
            minimum_level: 2
    contract:
      id: dnd2024:class_features_sorcerer_level_2_font_of_magic
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - resources
      events:
      - resource.convert
      - action.bonus
      - rest.long_completed
      capabilities:
      - character_features
      - resources
    requires:
    - dnd2024:class_features_sorcerer_level_2_font_of_magic
    citations:
    - SRD 5.2.1 pp. 66; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/sorcerer_level_2_font_of_magic.md
  dnd2024:class_features/sorcerer_level_3_sorcerer_subclass:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/sorcerer_level_3_sorcerer_subclass
        implementation: dnd2024:class_features_sorcerer_level_3_sorcerer_subclass
        parameters:
          subclass_choice:
            count: 1
            class: sorcerer
          feature_grant_policy:
            class_level_at_least_feature_level: true
            events:
            - subclass.selected
            - character.level_gained
            persistent: true
          class_scope:
            class: sorcerer
            minimum_level: 3
    contract:
      id: dnd2024:class_features_sorcerer_level_3_sorcerer_subclass
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - subclass
      - feature_grants
      events:
      - subclass.selected
      - character.level_gained
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_sorcerer_level_3_sorcerer_subclass
    citations:
    - SRD 5.2.1 pp. 66; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/sorcerer_level_3_sorcerer_subclass.md
  dnd2024:class_features/sorcerer_level_5_sorcerous_restoration:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/sorcerer_level_5_sorcerous_restoration
        implementation: dnd2024:class_features_sorcerer_level_5_sorcerous_restoration
        parameters:
          trigger: rest.short_completed
          optional: true
          resource:
            id: sorcerer.sorcerous_restoration
            maximum: 1
            recovery:
            - event: rest.long_completed
              amount: all
          cost: 1
          restore:
            resource: sorcerer.sorcery_points
            maximum:
              op: floor
              args:
              - op: divide
                args:
                - var: actor.class_levels.sorcerer
                - 2
            expended_only: true
          class_scope:
            class: sorcerer
            minimum_level: 5
    contract:
      id: dnd2024:class_features_sorcerer_level_5_sorcerous_restoration
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - resources
      events:
      - rest.short_completed
      - rest.long_completed
      capabilities:
      - character_features
      - resources
    requires:
    - dnd2024:class_features_sorcerer_level_5_sorcerous_restoration
    citations:
    - SRD 5.2.1 pp. 66; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/sorcerer_level_5_sorcerous_restoration.md
  dnd2024:class_features/sorcerer_level_19_epic_boon:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/sorcerer_level_19_epic_boon
        implementation: dnd2024:class_features_sorcerer_level_19_epic_boon
        parameters:
          feat_choice:
            count: 1
            category_preference: epic_boon
            allowed_categories: any
            must_meet_prerequisites: true
          class_scope:
            class: sorcerer
            minimum_level: 19
    contract:
      id: dnd2024:class_features_sorcerer_level_19_epic_boon
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.feature_acquired
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_sorcerer_level_19_epic_boon
    citations:
    - SRD 5.2.1 pp. 66; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/sorcerer_level_19_epic_boon.md
  dnd2024:subclasses/sorcerer_sorcerer_subclass_draconic_sorcery:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:subclasses/sorcerer_sorcerer_subclass_draconic_sorcery
        implementation: dnd2024:subclasses_sorcerer_sorcerer_subclass_draconic_sorcery
        parameters:
          class: sorcerer
          minimum_class_level: 3
          feature_grants:
          - class_level: 14
            definition:
              key: dnd2024:class_features/draconic_sorcery_level_14_dragon_wings
          - class_level: 18
            definition:
              key: dnd2024:class_features/draconic_sorcery_level_18_dragon_companion
          - class_level: 3
            definition:
              key: dnd2024:class_features/draconic_sorcery_level_3_draconic_resilience
          - class_level: 3
            definition:
              key: dnd2024:class_features/draconic_sorcery_level_3_draconic_spells
          - class_level: 6
            definition:
              key: dnd2024:class_features/draconic_sorcery_level_6_elemental_affinity
          grant_eligible_features:
            'on':
            - subclass.selected
            - character.level_gained
            comparison: class_level_at_least_threshold
            persistent: true
    contract:
      id: dnd2024:subclasses_sorcerer_sorcerer_subclass_draconic_sorcery
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - feature_grants
      events:
      - subclass.selected
      - character.level_gained
      capabilities:
      - character_features
      - spellcasting
    requires:
    - dnd2024:subclasses_sorcerer_sorcerer_subclass_draconic_sorcery
    citations:
    - SRD 5.2.1 pp. 69; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/subclasses/sorcerer_sorcerer_subclass_draconic_sorcery.md
  dnd2024:class_features/draconic_sorcery_level_3_draconic_spells:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/draconic_sorcery_level_3_draconic_spells
        implementation: dnd2024:class_features_draconic_sorcery_level_3_draconic_spells
        parameters:
          class: sorcerer
          always_prepared:
          - minimum_class_level: 3
            spells:
            - key: dnd2024:spells/alter_self
            - key: dnd2024:spells/chromatic_orb
            - key: dnd2024:spells/command
            - key: dnd2024:spells/dragon_s_breath
          - minimum_class_level: 5
            spells:
            - key: dnd2024:spells/fear
            - key: dnd2024:spells/fly
          - minimum_class_level: 7
            spells:
            - key: dnd2024:spells/arcane_eye
            - key: dnd2024:spells/charm_monster
          - minimum_class_level: 9
            spells:
            - key: dnd2024:spells/legend_lore
            - key: dnd2024:spells/summon_dragon
          include_lower_levels: true
          counts_against_prepared_limit: false
          class_scope:
            class: sorcerer
            minimum_level: 3
    contract:
      id: dnd2024:class_features_draconic_sorcery_level_3_draconic_spells
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - prepared_spells
      events:
      - character.level_gained
      capabilities:
      - character_features
      - spellcasting
    requires:
    - dnd2024:class_features_draconic_sorcery_level_3_draconic_spells
    citations:
    - SRD 5.2.1 pp. 70; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/draconic_sorcery_level_3_draconic_spells.md
  dnd2024:class_features/draconic_sorcery_level_14_dragon_wings:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/draconic_sorcery_level_14_dragon_wings
        implementation: dnd2024:class_features_draconic_sorcery_level_14_dragon_wings
        parameters:
          action: bonus_action
          resource:
            id: sorcerer.dragon_wings
            maximum: 1
            recovery:
            - event: rest.long_completed
              amount: all
          cost: 1
          duration:
            hours: 1
          fly_speed_feet: 60
          dismiss:
            action: none
          recharge_alternative:
            action: none
            resource_cost:
              id: sorcerer.sorcery_points
              amount: 3
            restore_uses: 1
          class_scope:
            class: sorcerer
            minimum_level: 14
    contract:
      id: dnd2024:class_features_draconic_sorcery_level_14_dragon_wings
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - action.bonus
      - resource.convert
      - rest.long_completed
      capabilities:
      - character_features
      - resources
      - movement
    requires:
    - dnd2024:class_features_draconic_sorcery_level_14_dragon_wings
    citations:
    - SRD 5.2.1 pp. 70; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/draconic_sorcery_level_14_dragon_wings.md
  dnd2024:classes/warlock:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:classes/warlock
        implementation: dnd2024:classes_warlock
        parameters:
          class: warlock
          primary_abilities:
          - charisma
          hit_point_die:
            count_per_class_level: 1
            sides: 8
          first_class_traits:
            save_proficiencies:
            - wisdom
            - charisma
            skill_proficiencies:
              count: 2
              choices:
              - arcana
              - deception
              - history
              - intimidation
              - investigation
              - nature
              - religion
            weapon_proficiencies:
              simple: true
            armor_training:
            - light
          starting_equipment:
            choose: 1
            options:
              equipment:
                items:
                - definition:
                    key: dnd2024:equipment/leather_armor
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/sickle
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/dagger
                  quantity: 2
                - definition:
                    key: dnd2024:equipment/arcane_focus_varies
                  quantity: 1
                  variant: orb
                - definition:
                    key: dnd2024:equipment/book_25_gp
                  quantity: 1
                  variant: occult_lore
                - definition:
                    key: dnd2024:equipment/scholar_s_pack_40_gp
                  quantity: 1
                currency:
                  gp: 15
              currency:
                currency:
                  gp: 100
          progression:
            key: dnd2024:tables/warlock_progression
          feature_grants:
          - minimum_class_level: 11
            definition:
              key: dnd2024:class_features/warlock_level_11_mystic_arcanum
          - minimum_class_level: 19
            definition:
              key: dnd2024:class_features/warlock_level_19_epic_boon
          - minimum_class_level: 1
            definition:
              key: dnd2024:class_features/warlock_level_1_eldritch_invocations
          - minimum_class_level: 1
            definition:
              key: dnd2024:class_features/warlock_level_1_pact_magic
          - minimum_class_level: 20
            definition:
              key: dnd2024:class_features/warlock_level_20_eldritch_master
          - minimum_class_level: 2
            definition:
              key: dnd2024:class_features/warlock_level_2_magical_cunning
          - minimum_class_level: 3
            definition:
              key: dnd2024:class_features/warlock_level_3_warlock_subclass
          - minimum_class_level: 4
            definition:
              key: dnd2024:class_features/warlock_level_4_ability_score_improvement
          - minimum_class_level: 9
            definition:
              key: dnd2024:class_features/warlock_level_9_contact_patron
    contract:
      id: dnd2024:classes_warlock
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - class_traits
      - equipment_choices
      - progression
      - feature_grants
      events:
      - character.class_selected
      - character.level_gained
      capabilities:
      - character_features
      - proficiencies
      - d20_tests
      - equipment
    requires:
    - dnd2024:classes_warlock
    citations:
    - SRD 5.2.1 pp. 70; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/classes/warlock.md
  dnd2024:class_features/warlock_level_1_pact_magic:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/warlock_level_1_pact_magic
        implementation: dnd2024:class_features_warlock_level_1_pact_magic
        parameters:
          class: warlock
          spellcasting_ability: charisma
          spellcasting_focus: arcane_focus
          save_dc:
            op: add
            args:
            - 8
            - var: actor.abilities.charisma.modifier
            - var: actor.proficiency_bonus
          spell_attack_bonus:
            op: add
            args:
            - var: actor.proficiency_bonus
            - var: actor.abilities.charisma.modifier
          slots:
            progression:
              key: dnd2024:tables/warlock_progression
            recovery:
            - rest.short_completed
            - rest.long_completed
            recover: all
            pool: pact_magic
            count:
              table:
                key: dnd2024:tables/warlock_progression
              row:
                var: actor.class_levels.warlock
              column: pact_slots
            level:
              table:
                key: dnd2024:tables/warlock_progression
              row:
                var: actor.class_levels.warlock
              column: pact_slot_level
            uniform_level: true
            spell_level_range:
            - 1
            - 5
            cast_at_slot_level: true
          prepared_spells:
            initial_count: 2
            initial_spell_level: 1
            progression:
              table:
                key: dnd2024:tables/warlock_progression
              row:
                var: actor.class_levels.warlock
              column: prepared_spells
            eligible_list: warlock
            eligible_levels:
              minimum: 1
              maximum: pact_slot_level
            on_count_increase: choose_until_count_met
            replacement:
              event: character.level_gained
              class: warlock
              maximum: 1
            always_prepared_from_other_class_features:
              counts_against_limit: false
              counts_as_class_spell: warlock
          cantrips:
            initial_count: 2
            list: warlock
            additional_levels:
            - 4
            - 10
            additional_each: 1
            replace:
              event: character.level_gained
              class: warlock
              maximum: 1
              same_list: true
          class_scope:
            class: warlock
            minimum_level: 1
    contract:
      id: dnd2024:class_features_warlock_level_1_pact_magic
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - spellcasting
      - prepared_spells
      - resources
      events:
      - character.level_gained
      - rest.long_completed
      - rest.short_completed
      - spell.prepare
      - spell.cast
      capabilities:
      - character_features
      - spellcasting
      - proficiencies
      - d20_tests
    requires:
    - dnd2024:class_features_warlock_level_1_pact_magic
    citations:
    - SRD 5.2.1 pp. 71, 72; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/warlock_level_1_pact_magic.md
  dnd2024:class_features/warlock_level_3_warlock_subclass:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/warlock_level_3_warlock_subclass
        implementation: dnd2024:class_features_warlock_level_3_warlock_subclass
        parameters:
          subclass_choice:
            count: 1
            class: warlock
          feature_grant_policy:
            class_level_at_least_feature_level: true
            events:
            - subclass.selected
            - character.level_gained
            persistent: true
          class_scope:
            class: warlock
            minimum_level: 3
    contract:
      id: dnd2024:class_features_warlock_level_3_warlock_subclass
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - subclass
      - feature_grants
      events:
      - subclass.selected
      - character.level_gained
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_warlock_level_3_warlock_subclass
    citations:
    - SRD 5.2.1 pp. 72; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/warlock_level_3_warlock_subclass.md
  dnd2024:class_features/warlock_level_9_contact_patron:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/warlock_level_9_contact_patron
        implementation: dnd2024:class_features_warlock_level_9_contact_patron
        parameters:
          always_prepared:
          - key: dnd2024:spells/contact_other_plane
          counts_against_prepared_limit: false
          free_cast:
            resource:
              id: warlock.contact_patron
              maximum: 1
              recovery:
              - event: rest.long_completed
                amount: all
            cost: 1
            contact: actor.patron
            expend_slot: false
            spell_saving_throw: automatic_success
          class_scope:
            class: warlock
            minimum_level: 9
    contract:
      id: dnd2024:class_features_warlock_level_9_contact_patron
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast
      - rest.long_completed
      capabilities:
      - character_features
      - resources
      - spellcasting
    requires:
    - dnd2024:class_features_warlock_level_9_contact_patron
    citations:
    - SRD 5.2.1 pp. 72; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/warlock_level_9_contact_patron.md
  dnd2024:class_features/warlock_level_19_epic_boon:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/warlock_level_19_epic_boon
        implementation: dnd2024:class_features_warlock_level_19_epic_boon
        parameters:
          feat_choice:
            count: 1
            category_preference: epic_boon
            allowed_categories: any
            must_meet_prerequisites: true
          class_scope:
            class: warlock
            minimum_level: 19
    contract:
      id: dnd2024:class_features_warlock_level_19_epic_boon
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.feature_acquired
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_warlock_level_19_epic_boon
    citations:
    - SRD 5.2.1 pp. 72; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/warlock_level_19_epic_boon.md
  dnd2024:subclasses/warlock_warlock_subclass_fiend_patron:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:subclasses/warlock_warlock_subclass_fiend_patron
        implementation: dnd2024:subclasses_warlock_warlock_subclass_fiend_patron
        parameters:
          class: warlock
          minimum_class_level: 3
          feature_grants:
          - class_level: 10
            definition:
              key: dnd2024:class_features/fiend_patron_level_10_fiendish_resilience
          - class_level: 14
            definition:
              key: dnd2024:class_features/fiend_patron_level_14_hurl_through_hell
          - class_level: 3
            definition:
              key: dnd2024:class_features/fiend_patron_level_3_dark_one_s_blessing
          - class_level: 3
            definition:
              key: dnd2024:class_features/fiend_patron_level_3_fiend_spells
          - class_level: 6
            definition:
              key: dnd2024:class_features/fiend_patron_level_6_dark_one_s_own_luck
          grant_eligible_features:
            'on':
            - subclass.selected
            - character.level_gained
            comparison: class_level_at_least_threshold
            persistent: true
    contract:
      id: dnd2024:subclasses_warlock_warlock_subclass_fiend_patron
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - feature_grants
      events:
      - subclass.selected
      - character.level_gained
      capabilities:
      - character_features
      - spellcasting
    requires:
    - dnd2024:subclasses_warlock_warlock_subclass_fiend_patron
    citations:
    - SRD 5.2.1 pp. 76; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/subclasses/warlock_warlock_subclass_fiend_patron.md
  dnd2024:class_features/fiend_patron_level_3_fiend_spells:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/fiend_patron_level_3_fiend_spells
        implementation: dnd2024:class_features_fiend_patron_level_3_fiend_spells
        parameters:
          class: warlock
          always_prepared:
          - minimum_class_level: 3
            spells:
            - key: dnd2024:spells/burning_hands
            - key: dnd2024:spells/command
            - key: dnd2024:spells/scorching_ray
            - key: dnd2024:spells/suggestion
          - minimum_class_level: 5
            spells:
            - key: dnd2024:spells/fireball
            - key: dnd2024:spells/stinking_cloud
          - minimum_class_level: 7
            spells:
            - key: dnd2024:spells/fire_shield
            - key: dnd2024:spells/wall_of_fire
          - minimum_class_level: 9
            spells:
            - key: dnd2024:spells/geas
            - key: dnd2024:spells/insect_plague
          include_lower_levels: true
          counts_against_prepared_limit: false
          class_scope:
            class: warlock
            minimum_level: 3
    contract:
      id: dnd2024:class_features_fiend_patron_level_3_fiend_spells
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - prepared_spells
      events:
      - character.level_gained
      capabilities:
      - character_features
      - spellcasting
    requires:
    - dnd2024:class_features_fiend_patron_level_3_fiend_spells
    citations:
    - SRD 5.2.1 pp. 76; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/fiend_patron_level_3_fiend_spells.md
  dnd2024:class_features/fiend_patron_level_10_fiendish_resilience:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/fiend_patron_level_10_fiendish_resilience
        implementation: dnd2024:class_features_fiend_patron_level_10_fiendish_resilience
        parameters:
          choose_damage_type:
            count: 1
            exclude:
            - force
            events:
            - rest.short_completed
            - rest.long_completed
          grant: resistance
          duration: until_different_choice
          class_scope:
            class: warlock
            minimum_level: 10
    contract:
      id: dnd2024:class_features_fiend_patron_level_10_fiendish_resilience
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - rest.short_completed
      - rest.long_completed
      - damage.pending
      capabilities:
      - character_features
      - damage
    requires:
    - dnd2024:class_features_fiend_patron_level_10_fiendish_resilience
    citations:
    - SRD 5.2.1 pp. 76; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/fiend_patron_level_10_fiendish_resilience.md
  dnd2024:classes/wizard:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:classes/wizard
        implementation: dnd2024:classes_wizard
        parameters:
          class: wizard
          primary_abilities:
          - intelligence
          hit_point_die:
            count_per_class_level: 1
            sides: 6
          first_class_traits:
            save_proficiencies:
            - intelligence
            - wisdom
            skill_proficiencies:
              count: 2
              choices:
              - arcana
              - history
              - insight
              - investigation
              - medicine
              - nature
              - religion
            weapon_proficiencies:
              simple: true
            armor_training: []
          starting_equipment:
            choose: 1
            options:
              equipment:
                items:
                - definition:
                    key: dnd2024:equipment/dagger
                  quantity: 2
                - definition:
                    key: dnd2024:equipment/arcane_focus_varies
                  quantity: 1
                  variant: quarterstaff
                - definition:
                    key: dnd2024:equipment/robe_1_gp
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/scholar_s_pack_40_gp
                  quantity: 1
                currency:
                  gp: 5
                additional_items:
                - kind: spellbook
                  quantity: 1
              currency:
                currency:
                  gp: 55
          progression:
            key: dnd2024:tables/wizard_progression
          feature_grants:
          - minimum_class_level: 18
            definition:
              key: dnd2024:class_features/wizard_level_18_spell_mastery
          - minimum_class_level: 19
            definition:
              key: dnd2024:class_features/wizard_level_19_epic_boon
          - minimum_class_level: 1
            definition:
              key: dnd2024:class_features/wizard_level_1_arcane_recovery
          - minimum_class_level: 1
            definition:
              key: dnd2024:class_features/wizard_level_1_ritual_adept
          - minimum_class_level: 1
            definition:
              key: dnd2024:class_features/wizard_level_1_spellcasting
          - minimum_class_level: 20
            definition:
              key: dnd2024:class_features/wizard_level_20_signature_spells
          - minimum_class_level: 2
            definition:
              key: dnd2024:class_features/wizard_level_2_scholar
          - minimum_class_level: 3
            definition:
              key: dnd2024:class_features/wizard_level_3_wizard_subclass
          - minimum_class_level: 4
            definition:
              key: dnd2024:class_features/wizard_level_4_ability_score_improvement
          - minimum_class_level: 5
            definition:
              key: dnd2024:class_features/wizard_level_5_memorize_spell
    contract:
      id: dnd2024:classes_wizard
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - class_traits
      - equipment_choices
      - progression
      - feature_grants
      events:
      - character.class_selected
      - character.level_gained
      capabilities:
      - character_features
      - spellcasting
      - proficiencies
      - d20_tests
      - equipment
    requires:
    - dnd2024:classes_wizard
    citations:
    - SRD 5.2.1 pp. 77; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/classes/wizard.md
  dnd2024:class_features/wizard_level_1_ritual_adept:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/wizard_level_1_ritual_adept
        implementation: dnd2024:class_features_wizard_level_1_ritual_adept
        parameters:
          ritual_casting:
            eligible:
              ritual_tag: true
              in_actor_spellbook: true
            must_be_prepared: false
            must_read_from_book: true
          class_scope:
            class: wizard
            minimum_level: 1
    contract:
      id: dnd2024:class_features_wizard_level_1_ritual_adept
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.ritual_cast_pending
      capabilities:
      - character_features
      - spellcasting
    requires:
    - dnd2024:class_features_wizard_level_1_ritual_adept
    citations:
    - SRD 5.2.1 pp. 78; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/wizard_level_1_ritual_adept.md
  dnd2024:class_features/wizard_level_2_scholar:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/wizard_level_2_scholar
        implementation: dnd2024:class_features_wizard_level_2_scholar
        parameters:
          expertise:
            count: 1
            eligible:
              proficient: true
              expertise: false
              skills:
              - arcana
              - history
              - investigation
              - medicine
              - nature
              - religion
            grant: expertise
          class_scope:
            class: wizard
            minimum_level: 2
    contract:
      id: dnd2024:class_features_wizard_level_2_scholar
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.feature_acquired
      capabilities:
      - character_features
      - proficiencies
    requires:
    - dnd2024:class_features_wizard_level_2_scholar
    citations:
    - SRD 5.2.1 pp. 78; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/wizard_level_2_scholar.md
  dnd2024:class_features/wizard_level_4_ability_score_improvement:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/wizard_level_4_ability_score_improvement
        implementation: dnd2024:class_features_wizard_level_4_ability_score_improvement
        parameters:
          class: wizard
          grant_at_class_levels:
          - 4
          - 8
          - 12
          - 16
          feat_choice:
            count_each: 1
            default:
              key: dnd2024:feats/general_feats_ability_score_improvement
            allowed_categories: any
            must_meet_prerequisites: true
          class_scope:
            class: wizard
            minimum_level: 4
    contract:
      id: dnd2024:class_features_wizard_level_4_ability_score_improvement
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - feat_grants
      events:
      - character.level_gained
      capabilities:
      - character_features
    requires:
    - dnd2024:class_features_wizard_level_4_ability_score_improvement
    citations:
    - SRD 5.2.1 pp. 79; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/wizard_level_4_ability_score_improvement.md
  dnd2024:class_features/wizard_level_18_spell_mastery:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/wizard_level_18_spell_mastery
        implementation: dnd2024:class_features_wizard_level_18_spell_mastery
        parameters:
          choose_spells:
          - count: 1
            level: 1
          - count: 1
            level: 2
          eligibility:
            in_spellbook: true
            casting_time: action
          always_prepared: true
          counts_against_prepared_limit: false
          free_cast:
            at_will: true
            cast_level: base_spell_level
            expend_slot: false
          higher_level_casting:
            expend_slot: true
          replace:
            event: rest.long_completed
            requires: study_spellbook
            maximum: 1
            same_level: true
            same_eligibility: true
          class_scope:
            class: wizard
            minimum_level: 18
    contract:
      id: dnd2024:class_features_wizard_level_18_spell_mastery
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.feature_acquired
      - rest.long_completed
      - spell.cast
      capabilities:
      - character_features
      - spellcasting
    requires:
    - dnd2024:class_features_wizard_level_18_spell_mastery
    citations:
    - SRD 5.2.1 pp. 79; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/wizard_level_18_spell_mastery.md
  dnd2024:class_features/wizard_level_20_signature_spells:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/wizard_level_20_signature_spells
        implementation: dnd2024:class_features_wizard_level_20_signature_spells
        parameters:
          choose_spells:
            count: 2
            level: 3
            in_spellbook: true
          always_prepared: true
          counts_against_prepared_limit: false
          free_cast:
            per_spell_resource:
              id: wizard.signature_spell
              maximum: 1
              recovery:
              - event: rest.short_completed
                amount: all
              - event: rest.long_completed
                amount: all
            cost: 1
            cast_level: 3
            expend_slot: false
          higher_level_casting:
            expend_slot: true
          class_scope:
            class: wizard
            minimum_level: 20
    contract:
      id: dnd2024:class_features_wizard_level_20_signature_spells
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.feature_acquired
      - rest.short_completed
      - rest.long_completed
      - spell.cast
      capabilities:
      - character_features
      - resources
      - spellcasting
    requires:
    - dnd2024:class_features_wizard_level_20_signature_spells
    citations:
    - SRD 5.2.1 pp. 79; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/wizard_level_20_signature_spells.md
  dnd2024:class_features/evoker_level_3_evocation_savant:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/evoker_level_3_evocation_savant
        implementation: dnd2024:class_features_evoker_level_3_evocation_savant
        parameters:
          initial_spellbook_additions:
            count: 2
            spell_list: wizard
            school: evocation
            minimum_spell_level: 1
            maximum_spell_level: 2
            cost_gp: 0
          subsequent_spellbook_additions:
            event: wizard.new_spell_slot_level_access
            count: 1
            spell_list: wizard
            school: evocation
            requires_available_slot_level: true
            cost_gp: 0
          class_scope:
            class: wizard
            minimum_level: 3
    contract:
      id: dnd2024:class_features_evoker_level_3_evocation_savant
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - spellbook_additions
      events:
      - character.feature_acquired
      - character.level_gained
      capabilities:
      - character_features
      - spellcasting
    requires:
    - dnd2024:class_features_evoker_level_3_evocation_savant
    citations:
    - SRD 5.2.1 pp. 82; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/evoker_level_3_evocation_savant.md
  dnd2024:class_features/evoker_level_6_sculpt_spells:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/evoker_level_6_sculpt_spells
        implementation: dnd2024:class_features_evoker_level_6_sculpt_spells
        parameters:
          trigger: actor.casts_evocation_spell
          targets:
            not_actor: true
            affected_by_spell: true
            visible_to_actor: true
            maximum:
              op: add
              args:
              - 1
              - var: spell.level
            selection: actor_choice
          saving_throws_against_spell: automatic_success
          damage_on_success:
            if_normally_half: 0
          class_scope:
            class: wizard
            minimum_level: 6
    contract:
      id: dnd2024:class_features_evoker_level_6_sculpt_spells
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast
      - saving_throw
      - damage.pending
      capabilities:
      - character_features
      - spellcasting
      - damage
    requires:
    - dnd2024:class_features_evoker_level_6_sculpt_spells
    citations:
    - SRD 5.2.1 pp. 82; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/evoker_level_6_sculpt_spells.md
  dnd2024:class_features/evoker_level_14_overchannel:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/evoker_level_14_overchannel
        implementation: dnd2024:class_features_evoker_level_14_overchannel
        parameters:
          trigger: actor.casts_damaging_wizard_spell
          requires_slot_level:
          - 1
          - 2
          - 3
          - 4
          - 5
          optional: true
          spell_damage: maximum
          spell_damage_scope: cast_turn
          usage_counter:
            id: wizard.overchannel
            increment_each_use: 1
            reset: rest.long_completed
          backlash:
            when_use_number_minimum: 2
            timing: immediately_after_cast
            damage:
              dice:
                count:
                  op: multiply
                  args:
                  - var: slot.level
                  - var: overchannel.use_number
                sides: 12
              type: necrotic
            ignores:
            - resistance
            - immunity
          class_scope:
            class: wizard
            minimum_level: 14
    contract:
      id: dnd2024:class_features_evoker_level_14_overchannel
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast
      - spell.damage
      - rest.long_completed
      capabilities:
      - character_features
      - spellcasting
      - damage
    requires:
    - dnd2024:class_features_evoker_level_14_overchannel
    citations:
    - SRD 5.2.1 pp. 82; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/evoker_level_14_overchannel.md
  dnd2024:backgrounds/criminal:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:backgrounds/criminal
        implementation: dnd2024:backgrounds_criminal
        parameters:
          ability_score_choices:
          - dexterity
          - constitution
          - intelligence
          ability_increase:
            patterns:
            - - 2
              - 1
            - - 1
              - 1
              - 1
            distinct_abilities: true
            maximum_score: 20
          feat:
            key: dnd2024:feats/origin_feats_alert
          skill_proficiencies:
          - sleight_of_hand
          - stealth
          tool_proficiency:
            key: dnd2024:equipment/other_tools_thieves_tools_25_gp
          equipment:
            choose: 1
            options:
              equipment:
                items:
                - definition:
                    key: dnd2024:equipment/dagger
                  quantity: 2
                - definition:
                    key: dnd2024:equipment/other_tools_thieves_tools_25_gp
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/crowbar_2_gp
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/pouch_5_sp
                  quantity: 2
                - definition:
                    key: dnd2024:equipment/clothes_traveler_s_2_gp
                  quantity: 1
                currency:
                  gp: 16
              currency:
                gp: 50
    contract:
      id: dnd2024:backgrounds_criminal
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - ability_increases
      - proficiencies
      - feat_grant
      - equipment_choices
      events:
      - character.background_selected
      capabilities:
      - character_features
      - proficiencies
      - equipment
    requires:
    - dnd2024:backgrounds_criminal
    citations:
    - SRD 5.2.1 pp. 83; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/backgrounds/criminal.md
  dnd2024:backgrounds/soldier:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:backgrounds/soldier
        implementation: dnd2024:backgrounds_soldier
        parameters:
          ability_score_choices:
          - strength
          - dexterity
          - constitution
          ability_increase:
            patterns:
            - - 2
              - 1
            - - 1
              - 1
              - 1
            distinct_abilities: true
            maximum_score: 20
          feat:
            key: dnd2024:feats/origin_feats_savage_attacker
          skill_proficiencies:
          - athletics
          - intimidation
          tool_proficiency:
            count: 1
            category: gaming_set
            selection: actor_choice
          equipment:
            choose: 1
            options:
              equipment:
                items:
                - definition:
                    key: dnd2024:equipment/spear
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/shortbow
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/ammunition_varies
                  quantity: 20
                - definition:
                    key: dnd2024:equipment/healer_s_kit_5_gp
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/quiver_1_gp
                  quantity: 1
                - definition:
                    key: dnd2024:equipment/clothes_traveler_s_2_gp
                  quantity: 1
                currency:
                  gp: 14
                ammunition_variant: arrows
                matching_proficient_gaming_set:
                  quantity: 1
              currency:
                gp: 50
    contract:
      id: dnd2024:backgrounds_soldier
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - ability_increases
      - proficiencies
      - feat_grant
      - equipment_choices
      events:
      - character.background_selected
      capabilities:
      - character_features
      - proficiencies
      - equipment
      - healing
    requires:
    - dnd2024:backgrounds_soldier
    citations:
    - SRD 5.2.1 pp. 83; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/backgrounds/soldier.md
  dnd2024:species/dwarf:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:species/dwarf
        implementation: dnd2024:species_dwarf
        parameters:
          creature_type: humanoid
          size: medium
          height_feet:
            typical_minimum: 4
            typical_maximum: 5
          speed_feet: 30
          darkvision_feet: 120
          dwarven_resilience:
            damage_resistance:
            - poison
            save_advantage:
              condition: poisoned
              purposes:
              - avoid
              - end
          dwarven_toughness:
            hit_point_maximum_bonus:
              var: actor.character_level
            recalculate_on_level_gain: true
          stonecunning:
            action: bonus_action
            resource:
              id: dwarf.stonecunning
              maximum:
                var: actor.proficiency_bonus
              recovery:
              - event: rest.long_completed
                amount: all
            cost: 1
            duration:
              minutes: 10
            tremorsense_feet: 60
            sense_requires:
              stone_surface_contact: true
              stone_types:
              - natural
              - worked
    contract:
      id: dnd2024:species_dwarf
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.species_selected
      - character.level_gained
      - saving_throw
      - action.bonus
      - rest.long_completed
      - sense.query
      capabilities:
      - character_features
      - resources
      - damage
      - conditions
      - d20_tests
      - proficiencies
      - movement
      - senses
    requires:
    - dnd2024:species_dwarf
    citations:
    - SRD 5.2.1 pp. 84; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/species/dwarf.md
  dnd2024:species/gnome:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:species/gnome
        implementation: dnd2024:species_gnome
        parameters:
          creature_type: humanoid
          size: small
          height_feet:
            typical_minimum: 3
            typical_maximum: 4
          speed_feet: 30
          darkvision_feet: 60
          save_advantage_abilities:
          - intelligence
          - wisdom
          - charisma
          lineage_choice:
            count: 1
            spellcasting_ability_choice:
            - intelligence
            - wisdom
            - charisma
            ability_choice_time: lineage_selection
            options:
              forest:
                cantrips:
                - key: dnd2024:spells/minor_illusion
                always_prepared:
                - key: dnd2024:spells/speak_with_animals
                free_cast:
                  spell:
                    key: dnd2024:spells/speak_with_animals
                  resource:
                    id: gnome.speak_with_animals
                    maximum:
                      var: actor.proficiency_bonus
                    recovery:
                    - event: rest.long_completed
                      amount: all
                  cost: 1
                  expend_slot: false
                cast_with_available_slots: true
              rock:
                cantrips:
                - key: dnd2024:spells/mending
                - key: dnd2024:spells/prestidigitation
                clockwork_device:
                  creation:
                    cast:
                      key: dnd2024:spells/prestidigitation
                    minutes: 10
                  size: tiny
                  armor_class: 5
                  hit_points: 1
                  effect_choice:
                    source_spell:
                      key: dnd2024:spells/prestidigitation
                    count: 1
                    nested_options_chosen_at_creation: true
                    fixed_until_destroyed: true
                  activation:
                    action: bonus_action
                    requires_touch: true
                    user: any_creature
                    produce_chosen_effect: true
                  maximum_simultaneous_devices: 3
                  expires:
                    hours_after_creation: 8
                  dismantle:
                    actor: creator
                    action: utilize
                    requires_touch: true
    contract:
      id: dnd2024:species_gnome
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.species_selected
      - saving_throw
      - spell.cast
      - action.bonus
      - action.utilize
      - rest.long_completed
      - time.elapsed
      capabilities:
      - character_features
      - resources
      - spellcasting
      - d20_tests
      - proficiencies
      - movement
    requires:
    - dnd2024:species_gnome
    citations:
    - SRD 5.2.1 pp. 85; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/species/gnome.md
  dnd2024:species/halfling:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:species/halfling
        implementation: dnd2024:species_halfling
        parameters:
          creature_type: humanoid
          size: small
          height_feet:
            typical_minimum: 2
            typical_maximum: 3
          speed_feet: 30
          brave:
            save_advantage:
              condition: frightened
              purposes:
              - avoid
              - end
          nimbleness:
            move_through_creature_space:
              minimum_size_difference: 1
            stop_in_occupied_space: false
          luck:
            trigger: actor.d20_test.d20_is_1
            optional_reroll: true
            must_use_new_roll: true
          naturally_stealthy:
            hide_obscurement_qualifier:
              only_obscured_by_creature: true
              minimum_size_difference: 1
    contract:
      id: dnd2024:species_halfling
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.species_selected
      - saving_throw
      - participant.moved
      - d20_test.rolled
      - action.hide_pending
      capabilities:
      - character_features
      - conditions
      - d20_tests
      - movement
    requires:
    - dnd2024:species_halfling
    citations:
    - SRD 5.2.1 pp. 86; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/species/halfling.md
  dnd2024:species/orc:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:species/orc
        implementation: dnd2024:species_orc
        parameters:
          creature_type: humanoid
          size: medium
          height_feet:
            typical_minimum: 6
            typical_maximum: 7
          speed_feet: 30
          darkvision_feet: 120
          adrenaline_rush:
            action: bonus_action
            take_action: dash
            temporary_hit_points:
              var: actor.proficiency_bonus
            resource:
              id: orc.adrenaline_rush
              maximum:
                var: actor.proficiency_bonus
              recovery:
              - event: rest.short_completed
                amount: all
              - event: rest.long_completed
                amount: all
            cost: 1
          relentless_endurance:
            trigger: actor.reduced_to_zero_hit_points
            requires:
              killed_outright: false
            optional: true
            instead_hit_points: 1
            resource:
              id: orc.relentless_endurance
              maximum: 1
              recovery:
              - event: rest.long_completed
                amount: all
            cost: 1
    contract:
      id: dnd2024:species_orc
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - character.species_selected
      - action.bonus
      - hit_points.reduced_to_zero
      - rest.short_completed
      - rest.long_completed
      capabilities:
      - character_features
      - resources
      - proficiencies
      - movement
    requires:
    - dnd2024:species_orc
    citations:
    - SRD 5.2.1 pp. 86; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/species/orc.md
  dnd2024:feats/origin_feats_alert:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:feats/origin_feats_alert
        implementation: dnd2024:feats_origin_feats_alert
        parameters:
          feat_category: origin
          initiative_bonus:
            optional: true
            amount:
              var: actor.proficiency_bonus
          initiative_swap:
            timing: immediately_after_actor_initiative_roll
            optional: true
            target:
              willing: true
              ally: true
              same_combat: true
              incapacitated: false
            requires:
              actor_incapacitated: false
            swap_values:
            - actor.initiative
            - target.initiative
    contract:
      id: dnd2024:feats_origin_feats_alert
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - initiative.rolled
      capabilities:
      - character_features
      - proficiencies
      - initiative
    requires:
    - dnd2024:feats_origin_feats_alert
    citations:
    - SRD 5.2.1 pp. 87; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/feats/origin_feats_alert.md
  dnd2024:feats/origin_feats_savage_attacker:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:feats/origin_feats_savage_attacker
        implementation: dnd2024:feats_origin_feats_savage_attacker
        parameters:
          feat_category: origin
          trigger: actor.weapon_hits_target
          limit:
            count: 1
            period: turn
          optional: true
          damage_dice:
            scope: weapon_damage_dice
            roll_count: 2
            choose_one_roll_total: true
    contract:
      id: dnd2024:feats_origin_feats_savage_attacker
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - weapon.damage_roll_pending
      capabilities:
      - character_features
      - damage
    requires:
    - dnd2024:feats_origin_feats_savage_attacker
    citations:
    - SRD 5.2.1 pp. 87; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/feats/origin_feats_savage_attacker.md
  dnd2024:feats/general_feats_ability_score_improvement:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:feats/general_feats_ability_score_improvement
        implementation: dnd2024:feats_general_feats_ability_score_improvement
        parameters:
          feat_category: general
          prerequisites:
            minimum_character_level: 4
          repeatable: true
          ability_increase:
            patterns:
            - - 2
            - - 1
              - 1
            distinct_abilities: true
            eligible_abilities: all
            maximum_score: 20
    contract:
      id: dnd2024:feats_general_feats_ability_score_improvement
      inputs:
        actor: participant
        selection:
          type: choice
          required: false
      outputs:
      - ability_increases
      events:
      - feat.selected
      capabilities:
      - character_features
    requires:
    - dnd2024:feats_general_feats_ability_score_improvement
    citations:
    - SRD 5.2.1 pp. 87; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/feats/general_feats_ability_score_improvement.md
  dnd2024:feats/fighting_style_feats_archery:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:feats/fighting_style_feats_archery
        implementation: dnd2024:feats_fighting_style_feats_archery
        parameters:
          feat_category: fighting_style
          prerequisites:
            feature: fighting_style
          attack_bonus:
            weapon_category: ranged
            amount: 2
    contract:
      id: dnd2024:feats_fighting_style_feats_archery
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - attack.roll
      capabilities:
      - character_features
    requires:
    - dnd2024:feats_fighting_style_feats_archery
    citations:
    - SRD 5.2.1 pp. 87; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/feats/fighting_style_feats_archery.md
  dnd2024:feats/fighting_style_feats_great_weapon_fighting:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:feats/fighting_style_feats_great_weapon_fighting
        implementation: dnd2024:feats_fighting_style_feats_great_weapon_fighting
        parameters:
          feat_category: fighting_style
          prerequisites:
            feature: fighting_style
          requires:
            weapon_category: melee
            held_with_hands: 2
            weapon_property_any:
            - two_handed
            - versatile
          optional_damage_die_floor: 3
          scope: attack_damage_dice
    contract:
      id: dnd2024:feats_fighting_style_feats_great_weapon_fighting
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - damage.rolled
      capabilities:
      - character_features
      - damage
    requires:
    - dnd2024:feats_fighting_style_feats_great_weapon_fighting
    citations:
    - SRD 5.2.1 pp. 88; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/feats/fighting_style_feats_great_weapon_fighting.md
  dnd2024:feats/epic_boon_feats_boon_of_combat_prowess:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:feats/epic_boon_feats_boon_of_combat_prowess
        implementation: dnd2024:feats_epic_boon_feats_boon_of_combat_prowess
        parameters:
          feat_category: epic_boon
          prerequisites:
            minimum_character_level: 19
          ability_increase:
            count: 1
            amount: 1
            choices: all
            maximum_score: 30
          peerless_aim:
            trigger: actor.attack_missed
            optional: true
            replace_result: hit
            resource:
              id: boon.combat_prowess
              maximum: 1
              recovery:
              - event: actor.turn.start
                amount: all
            cost: 1
    contract:
      id: dnd2024:feats_epic_boon_feats_boon_of_combat_prowess
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - attack.missed
      - turn.start
      capabilities:
      - character_features
      - resources
    requires:
    - dnd2024:feats_epic_boon_feats_boon_of_combat_prowess
    citations:
    - SRD 5.2.1 pp. 88; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/feats/epic_boon_feats_boon_of_combat_prowess.md
  dnd2024:feats/epic_boon_feats_boon_of_fate:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:feats/epic_boon_feats_boon_of_fate
        implementation: dnd2024:feats_epic_boon_feats_boon_of_fate
        parameters:
          feat_category: epic_boon
          prerequisites:
            minimum_character_level: 19
          ability_increase:
            count: 1
            amount: 1
            choices: all
            maximum_score: 30
          improve_fate:
            trigger_any:
            - creature.d20_test_succeeded
            - creature.d20_test_failed
            target:
              self_allowed: true
              range_feet: 60
            optional: true
            roll:
              count: 2
              sides: 4
            modify_choice:
            - add_to_d20
            - subtract_from_d20
            recompute_result: true
            resource:
              id: boon.fate
              maximum: 1
              recovery:
              - event: actor.initiative.rolled
                amount: all
              - event: rest.short_completed
                amount: all
              - event: rest.long_completed
                amount: all
            cost: 1
    contract:
      id: dnd2024:feats_epic_boon_feats_boon_of_fate
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - d20_test.succeeded
      - d20_test.failed
      - initiative.rolled
      - rest.short_completed
      - rest.long_completed
      capabilities:
      - character_features
      - resources
      - initiative
    requires:
    - dnd2024:feats_epic_boon_feats_boon_of_fate
    citations:
    - SRD 5.2.1 pp. 88; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/feats/epic_boon_feats_boon_of_fate.md
  dnd2024:feats/epic_boon_feats_boon_of_spell_recall:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:feats/epic_boon_feats_boon_of_spell_recall
        implementation: dnd2024:feats_epic_boon_feats_boon_of_spell_recall
        parameters:
          feat_category: epic_boon
          prerequisites:
            minimum_character_level: 19
            feature: spellcasting
          ability_increase:
            count: 1
            amount: 1
            choices:
            - intelligence
            - wisdom
            - charisma
            maximum_score: 30
          free_casting:
            trigger: actor.casts_spell_with_slot
            slot_levels:
            - 1
            - 2
            - 3
            - 4
            roll:
              count: 1
              sides: 4
            if:
              roll_equals: slot.level
            then:
              slot_expended: false
    contract:
      id: dnd2024:feats_epic_boon_feats_boon_of_spell_recall
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - spell.cast
      capabilities:
      - character_features
      - spellcasting
    requires:
    - dnd2024:feats_epic_boon_feats_boon_of_spell_recall
    citations:
    - SRD 5.2.1 pp. 88; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/feats/epic_boon_feats_boon_of_spell_recall.md
  dnd2024:feats/epic_boon_feats_boon_of_truesight:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:feats/epic_boon_feats_boon_of_truesight
        implementation: dnd2024:feats_epic_boon_feats_boon_of_truesight
        parameters:
          feat_category: epic_boon
          prerequisites:
            minimum_character_level: 19
          ability_increase:
            count: 1
            amount: 1
            choices: all
            maximum_score: 30
          truesight_feet: 60
    contract:
      id: dnd2024:feats_epic_boon_feats_boon_of_truesight
      inputs:
        actor: participant
        event:
          type: event
          required: false
        selection:
          type: choice
          required: false
      outputs:
      - effects
      events:
      - sense.query
      capabilities:
      - character_features
    requires:
    - dnd2024:feats_epic_boon_feats_boon_of_truesight
    citations:
    - SRD 5.2.1 pp. 88; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/feats/epic_boon_feats_boon_of_truesight.md
---

# Character options conversion, part 2

All 143 assigned notes are specified: 143 rule contracts, 0 informational notes. The six subclass introductions are selectable feature bundles with canonical grants. Class core-trait contracts preserve their complete source tables; their separate feature notes and progression tables compose class behavior.

Source: the bundled official SRD 5.2.1, pp. 28–88, https://www.dndbeyond.com/srd. Each contract cites its exact page span. The original PDF hash is retained because these supplemental drafts cite PDF page spans. Numeric class progression references use the existing structured tables.

The source extraction interleaves adjacent table columns with Rage, Bardic Inspiration, Spellcasting, Fighting Style, Sneak Attack, Weapon Mastery, and Gnome prose. These columns belong to the corresponding class progression or Elf lineage table, and are not extra feature clauses. Gnome grants only its Forest or Rock lineage mechanics. The Paladin Channel Divinity span also includes oath advice; that advice imposes no automatic class-loss rule.

Staggering Blow places no explicit expiration on the next-save disadvantage; only its Opportunity Attack restriction ends at the start of the Barbarian’s next turn. Overchannel backlash starts at two d12 per slot level on the second use and rises by one d12 each further use. Nature’s Sanctuary uses the current Nature’s Ward resistance, including changes while the area persists. No engine implementation is included.
