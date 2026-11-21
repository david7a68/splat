---
format: 1
source: SRD 5.2.1
definitions:
  dnd2024:classes/barbarian:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:classes/barbarian
        implementation: dnd2024:classes_barbarian
        parameters:
          class: barbarian
          primary_abilities:
          - strength
          hit_point_die:
            sides: 12
            per_class_level: 1
          saving_throw_proficiencies:
          - strength
          - constitution
          skill_proficiencies:
            count: 2
            options:
            - animal_handling
            - athletics
            - intimidation
            - nature
            - perception
            - survival
          weapon_proficiencies:
          - simple
          - martial
          armor_training:
          - light
          - medium
          - shield
          tool_proficiencies: []
          starting_equipment:
            count: 1
            options:
            - id: a
              grants:
              - item_type: greataxe
                quantity: 1
              - item_type: handaxe
                quantity: 4
              - item_type: explorer_s_pack
                quantity: 1
              - item_type: gp
                quantity: 15
            - id: b
              grants:
              - item_type: gp
                quantity: 75
          progression:
            key: dnd2024:tables/barbarian_progression
          feature_grants:
          - feature: &id001
              key: dnd2024:class_features/barbarian_level_11_relentless_rage
            minimum_class_level: 11
          - feature: &id002
              key: dnd2024:class_features/barbarian_level_13_improved_brutal_strike
            minimum_class_level: 13
          - feature: &id003
              key: dnd2024:class_features/barbarian_level_15_persistent_rage
            minimum_class_level: 15
          - feature: &id004
              key: dnd2024:class_features/barbarian_level_17_improved_brutal_strike
            minimum_class_level: 17
          - feature: &id005
              key: dnd2024:class_features/barbarian_level_18_indomitable_might
            minimum_class_level: 18
          - feature: &id006
              key: dnd2024:class_features/barbarian_level_19_epic_boon
            minimum_class_level: 19
          - feature: &id007
              key: dnd2024:class_features/barbarian_level_1_rage
            minimum_class_level: 1
          - feature: &id008
              key: dnd2024:class_features/barbarian_level_1_unarmored_defense
            minimum_class_level: 1
          - feature: &id009
              key: dnd2024:class_features/barbarian_level_1_weapon_mastery
            minimum_class_level: 1
          - feature: &id010
              key: dnd2024:class_features/barbarian_level_20_primal_champion
            minimum_class_level: 20
          - feature: &id011
              key: dnd2024:class_features/barbarian_level_2_danger_sense
            minimum_class_level: 2
          - feature: &id012
              key: dnd2024:class_features/barbarian_level_2_reckless_attack
            minimum_class_level: 2
          - feature: &id013
              key: dnd2024:class_features/barbarian_level_3_barbarian_subclass
            minimum_class_level: 3
          - feature: &id014
              key: dnd2024:class_features/barbarian_level_3_primal_knowledge
            minimum_class_level: 3
          - feature: &id015
              key: dnd2024:class_features/barbarian_level_4_ability_score_improvement
            minimum_class_level: 4
          - feature: &id016
              key: dnd2024:class_features/barbarian_level_5_extra_attack
            minimum_class_level: 5
          - feature: &id017
              key: dnd2024:class_features/barbarian_level_5_fast_movement
            minimum_class_level: 5
          - feature: &id018
              key: dnd2024:class_features/barbarian_level_7_feral_instinct
            minimum_class_level: 7
          - feature: &id019
              key: dnd2024:class_features/barbarian_level_7_instinctive_pounce
            minimum_class_level: 7
          - feature: &id020
              key: dnd2024:class_features/barbarian_level_9_brutal_strike
            minimum_class_level: 9
      splat_rule_group:
        category: class
        members:
        - *id001
        - *id002
        - *id003
        - *id004
        - *id005
        - *id006
        - *id007
        - *id008
        - *id009
        - *id010
        - *id011
        - *id012
        - *id013
        - *id014
        - *id015
        - *id016
        - *id017
        - *id018
        - *id019
        - *id020
    contract:
      id: dnd2024:classes_barbarian
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - proficiencies
      - inventory_changes
      events:
      - character.class_selected
      - character.level_gained
      capabilities:
      - character_creation
      - class_progression
      - inventory
      - proficiencies
    requires:
    - dnd2024:classes_barbarian
    citations:
    - SRD 5.2.1 pp. 28; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/classes/barbarian.md
  dnd2024:class_features/barbarian_level_1_unarmored_defense:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/barbarian_level_1_unarmored_defense
        implementation: dnd2024:class_features_barbarian_level_1_unarmored_defense
        parameters:
          when:
            armor_worn: false
          base_ac:
            op: add
            args:
            - 10
            - stat: abilities.dexterity.modifier
            - stat: abilities.constitution.modifier
          shield_permitted: true
    contract:
      id: dnd2024:class_features_barbarian_level_1_unarmored_defense
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - character.feature_gained
      capabilities:
      - character_features
      - effects
    requires:
    - dnd2024:class_features_barbarian_level_1_unarmored_defense
    citations:
    - SRD 5.2.1 pp. 29; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/barbarian_level_1_unarmored_defense.md
  dnd2024:class_features/barbarian_level_2_danger_sense:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/barbarian_level_2_danger_sense
        implementation: dnd2024:class_features_barbarian_level_2_danger_sense
        parameters:
          roll: saving_throw
          ability: dexterity
          advantage: true
          unless_condition: incapacitated
    contract:
      id: dnd2024:class_features_barbarian_level_2_danger_sense
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - saving_throw.pending
      capabilities:
      - character_features
      - effects
    requires:
    - dnd2024:class_features_barbarian_level_2_danger_sense
    citations:
    - SRD 5.2.1 pp. 29; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/barbarian_level_2_danger_sense.md
  dnd2024:class_features/barbarian_level_3_barbarian_subclass:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/barbarian_level_3_barbarian_subclass
        implementation: dnd2024:class_features_barbarian_level_3_barbarian_subclass
        parameters:
          subclass_choice:
            count: 1
            class: barbarian
            srd_options:
            - key: dnd2024:subclasses/barbarian_path_of_the_berserker
          grant_features:
            from: selected_subclass
            when_required_class_level_lte_current: true
            reevaluate_on_class_level_gain: true
    contract:
      id: dnd2024:class_features_barbarian_level_3_barbarian_subclass
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.feature_gained
      - character.level_gained
      capabilities:
      - character_choices
      - class_progression
    requires:
    - dnd2024:class_features_barbarian_level_3_barbarian_subclass
    citations:
    - SRD 5.2.1 pp. 29; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/barbarian_level_3_barbarian_subclass.md
  dnd2024:class_features/barbarian_level_4_ability_score_improvement:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/barbarian_level_4_ability_score_improvement
        implementation: dnd2024:class_features_barbarian_level_4_ability_score_improvement
        parameters:
          class: barbarian
          grant_at_levels:
          - 4
          - 8
          - 12
          - 16
          feat_choice:
            count_per_milestone: 1
            category: any
            require_prerequisites: true
            default:
              key: dnd2024:feats/general_feats_ability_score_improvement
    contract:
      id: dnd2024:class_features_barbarian_level_4_ability_score_improvement
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.level_gained
      capabilities:
      - character_choices
      - feats
    requires:
    - dnd2024:class_features_barbarian_level_4_ability_score_improvement
    citations:
    - SRD 5.2.1 pp. 29; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/barbarian_level_4_ability_score_improvement.md
  dnd2024:class_features/barbarian_level_5_fast_movement:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/barbarian_level_5_fast_movement
        implementation: dnd2024:class_features_barbarian_level_5_fast_movement
        parameters:
          speed_bonus_feet: 10
          unless_armor_category: heavy
    contract:
      id: dnd2024:class_features_barbarian_level_5_fast_movement
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - character.feature_gained
      capabilities:
      - character_features
      - effects
    requires:
    - dnd2024:class_features_barbarian_level_5_fast_movement
    citations:
    - SRD 5.2.1 pp. 29; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/barbarian_level_5_fast_movement.md
  dnd2024:class_features/barbarian_level_7_instinctive_pounce:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/barbarian_level_7_instinctive_pounce
        implementation: dnd2024:class_features_barbarian_level_7_instinctive_pounce
        parameters:
          trigger: rage.entered
          action_cost: included_in_rage_bonus_action
          optional_move:
            maximum_feet: &id088
              op: divide
              args:
              - &id094
                stat: speed
              - 2
    contract:
      id: dnd2024:class_features_barbarian_level_7_instinctive_pounce
      inputs:
        actor: participant
        destination:
          type: location
          required: false
      outputs:
      - feature_state
      - movement
      events:
      - rage.entered
      capabilities:
      - movement
      - action_economy
    requires:
    - dnd2024:class_features_barbarian_level_7_instinctive_pounce
    citations:
    - SRD 5.2.1 pp. 29; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/barbarian_level_7_instinctive_pounce.md
  dnd2024:class_features/barbarian_level_11_relentless_rage:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/barbarian_level_11_relentless_rage
        implementation: dnd2024:class_features_barbarian_level_11_relentless_rage
        parameters:
          trigger:
            hit_points_would_be: 0
            rage_active: true
            instant_death: false
          optional: true
          save:
            ability: constitution
            dc:
              op: add
              args:
              - 10
              - op: multiply
                args:
                - 5
                - stat: relentless_rage.uses_since_rest
            success:
              replace_hit_points:
                op: multiply
                args:
                - 2
                - stat: classes.barbarian.level
          on_use:
            increment: relentless_rage.uses_since_rest
          reset:
            events:
            - rest.short_rest_completed
            - rest.long_rest_completed
            uses_since_rest: 0
    contract:
      id: dnd2024:class_features_barbarian_level_11_relentless_rage
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - resource_changes
      events:
      - hit_points.zero_pending
      - rest.short_rest_completed
      - rest.long_rest_completed
      capabilities:
      - saving_throws
      - hit_points
      - resources
    requires:
    - dnd2024:class_features_barbarian_level_11_relentless_rage
    citations:
    - SRD 5.2.1 pp. 30; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/barbarian_level_11_relentless_rage.md
  dnd2024:class_features/barbarian_level_15_persistent_rage:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/barbarian_level_15_persistent_rage
        implementation: dnd2024:class_features_barbarian_level_15_persistent_rage
        parameters:
          initiative_recovery:
            optional: true
            resource: rage
            amount: all_expended
            uses:
              maximum: 1
              recover:
                event: rest.long_rest_completed
                amount: all
          rage_override:
            duration:
              minutes: 10
            requires_round_extension: false
            early_end_conditions:
            - unconscious
            early_end_armor: heavy
            replaces_incapacitated_end_trigger: true
    contract:
      id: dnd2024:class_features_barbarian_level_15_persistent_rage
      inputs:
        actor: participant
      outputs:
      - feature_state
      - resource_changes
      events:
      - initiative.rolled
      - rage.entered
      - condition.applied
      - equipment.donned
      capabilities:
      - resources
      - effects
      - rage
    requires:
    - dnd2024:class_features_barbarian_level_15_persistent_rage
    citations:
    - SRD 5.2.1 pp. 30; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/barbarian_level_15_persistent_rage.md
  dnd2024:class_features/barbarian_level_18_indomitable_might:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/barbarian_level_18_indomitable_might
        implementation: dnd2024:class_features_barbarian_level_18_indomitable_might
        parameters:
          rolls:
          - ability_check
          - saving_throw
          ability: strength
          optional_total_floor:
            stat: abilities.strength.score
    contract:
      id: dnd2024:class_features_barbarian_level_18_indomitable_might
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - ability_check.rolled
      - saving_throw.rolled
      capabilities:
      - character_features
      - effects
    requires:
    - dnd2024:class_features_barbarian_level_18_indomitable_might
    citations:
    - SRD 5.2.1 pp. 30; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/barbarian_level_18_indomitable_might.md
  dnd2024:class_features/barbarian_level_20_primal_champion:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/barbarian_level_20_primal_champion
        implementation: dnd2024:class_features_barbarian_level_20_primal_champion
        parameters:
          ability_score_increases:
          - ability: strength
            amount: 4
            maximum: 25
          - ability: constitution
            amount: 4
            maximum: 25
    contract:
      id: dnd2024:class_features_barbarian_level_20_primal_champion
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - character.feature_gained
      capabilities:
      - character_features
      - effects
    requires:
    - dnd2024:class_features_barbarian_level_20_primal_champion
    citations:
    - SRD 5.2.1 pp. 30; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/barbarian_level_20_primal_champion.md
  dnd2024:class_features/path_of_the_berserker_level_3_frenzy:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/path_of_the_berserker_level_3_frenzy
        implementation: dnd2024:class_features_path_of_the_berserker_level_3_frenzy
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
    contract:
      id: dnd2024:class_features_path_of_the_berserker_level_3_frenzy
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - damage
      events:
      - attack.hit
      capabilities:
      - attacks
      - damage
      - rage
    requires:
    - dnd2024:class_features_path_of_the_berserker_level_3_frenzy
    citations:
    - SRD 5.2.1 pp. 30; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/path_of_the_berserker_level_3_frenzy.md
  dnd2024:class_features/path_of_the_berserker_level_10_retaliation:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/path_of_the_berserker_level_10_retaliation
        implementation: dnd2024:class_features_path_of_the_berserker_level_10_retaliation
        parameters:
          trigger:
            event: damage.taken
            source_type: creature
            source_distance_max_feet: 5
          action: reaction
          attack:
            count: 1
            target: damage_source
            range: melee
            allowed:
            - weapon
            - unarmed_strike
    contract:
      id: dnd2024:class_features_path_of_the_berserker_level_10_retaliation
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      events:
      - damage.taken
      capabilities:
      - attacks
      - reactions
    requires:
    - dnd2024:class_features_path_of_the_berserker_level_10_retaliation
    citations:
    - SRD 5.2.1 pp. 30; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/path_of_the_berserker_level_10_retaliation.md
  dnd2024:classes/bard:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:classes/bard
        implementation: dnd2024:classes_bard
        parameters:
          class: bard
          primary_abilities:
          - charisma
          hit_point_die:
            sides: 8
            per_class_level: 1
          saving_throw_proficiencies:
          - dexterity
          - charisma
          skill_proficiencies:
            count: 3
            options: any_skill
          weapon_proficiencies:
          - simple
          armor_training:
          - light
          tool_proficiencies:
          - category: musical_instrument
            count: 3
          starting_equipment:
            count: 1
            options:
            - id: a
              grants:
              - item_type: leather_armor
                quantity: 1
              - item_type: dagger
                quantity: 2
              - item_type: musical_instrument_choice
                quantity: 1
              - item_type: entertainer_s_pack
                quantity: 1
              - item_type: gp
                quantity: 19
            - id: b
              grants:
              - item_type: gp
                quantity: 90
          progression:
            key: dnd2024:tables/bard_progression
          feature_grants:
          - feature: &id021
              key: dnd2024:class_features/bard_level_10_magical_secrets
            minimum_class_level: 10
          - feature: &id022
              key: dnd2024:class_features/bard_level_18_superior_inspiration
            minimum_class_level: 18
          - feature: &id023
              key: dnd2024:class_features/bard_level_19_epic_boon
            minimum_class_level: 19
          - feature: &id024
              key: dnd2024:class_features/bard_level_1_bardic_inspiration
            minimum_class_level: 1
          - feature: &id025
              key: dnd2024:class_features/bard_level_1_spellcasting
            minimum_class_level: 1
          - feature: &id026
              key: dnd2024:class_features/bard_level_20_words_of_creation
            minimum_class_level: 20
          - feature: &id027
              key: dnd2024:class_features/bard_level_2_expertise
            minimum_class_level: 2
          - feature: &id028
              key: dnd2024:class_features/bard_level_2_jack_of_all_trades
            minimum_class_level: 2
          - feature: &id029
              key: dnd2024:class_features/bard_level_3_bard_subclass
            minimum_class_level: 3
          - feature: &id030
              key: dnd2024:class_features/bard_level_4_ability_score_improvement
            minimum_class_level: 4
          - feature: &id031
              key: dnd2024:class_features/bard_level_5_font_of_inspiration
            minimum_class_level: 5
          - feature: &id032
              key: dnd2024:class_features/bard_level_7_countercharm
            minimum_class_level: 7
      splat_rule_group:
        category: class
        members:
        - *id021
        - *id022
        - *id023
        - *id024
        - *id025
        - *id026
        - *id027
        - *id028
        - *id029
        - *id030
        - *id031
        - *id032
    contract:
      id: dnd2024:classes_bard
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - proficiencies
      - inventory_changes
      events:
      - character.class_selected
      - character.level_gained
      capabilities:
      - character_creation
      - class_progression
      - inventory
      - proficiencies
    requires:
    - dnd2024:classes_bard
    citations:
    - SRD 5.2.1 pp. 31; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/classes/bard.md
  dnd2024:class_features/bard_level_1_spellcasting:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/bard_level_1_spellcasting
        implementation: dnd2024:class_features_bard_level_1_spellcasting
        parameters:
          class: bard
          spell_list: bard
          ability: charisma
          focus_types:
          - musical_instrument
          slots:
            progression:
              key: dnd2024:tables/bard_progression
            columns_by_spell_level:
              1: spell_slots_1
              2: spell_slots_2
              3: spell_slots_3
              4: spell_slots_4
              5: spell_slots_5
              6: spell_slots_6
              7: spell_slots_7
              8: spell_slots_8
              9: spell_slots_9
            recovery:
              event: rest.long_rest_completed
              amount: all_expended
          prepared_spells:
            initial_count: 4
            initial_spell_level: 1
            count:
              table:
                key: dnd2024:tables/bard_progression
              row:
                stat: classes.bard.level
              column: prepared_spells
            choose_on_count_increase: true
            level_must_have_slot: true
            replace:
              event: character.class_level_gained
              maximum: 1
            other_class_feature_always_prepared:
              counts_toward_limit: false
              counts_as_class_spell: true
          cantrips:
            initial_count: 2
            spell_level: 0
            count:
              table:
                key: dnd2024:tables/bard_progression
              row:
                stat: classes.bard.level
              column: cantrips
            gain_extra_at_levels:
            - 4
            - 10
            replace:
              event: character.class_level_gained
              maximum: 1
    contract:
      id: dnd2024:class_features_bard_level_1_spellcasting
      inputs:
        actor: participant
      outputs:
      - feature_state
      - resource_changes
      - spell_grants
      events:
      - character.class_level_gained
      - rest.long_rest_completed
      - spell.cast_pending
      capabilities:
      - spellcasting
      - spell_preparation
      - resources
    requires:
    - dnd2024:class_features_bard_level_1_spellcasting
    citations:
    - SRD 5.2.1 pp. 32; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/bard_level_1_spellcasting.md
  dnd2024:class_features/bard_level_2_jack_of_all_trades:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/bard_level_2_jack_of_all_trades
        implementation: dnd2024:class_features_bard_level_2_jack_of_all_trades
        parameters:
          roll: ability_check
          requires_skill: true
          skill_proficiency: false
          already_uses_proficiency_bonus: false
          optional_bonus:
            op: floor
            args:
            - op: divide
              args:
              - &id089
                stat: proficiency_bonus
              - 2
    contract:
      id: dnd2024:class_features_bard_level_2_jack_of_all_trades
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - ability_check.pending
      capabilities:
      - character_features
      - effects
    requires:
    - dnd2024:class_features_bard_level_2_jack_of_all_trades
    citations:
    - SRD 5.2.1 pp. 32; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/bard_level_2_jack_of_all_trades.md
  dnd2024:class_features/bard_level_4_ability_score_improvement:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/bard_level_4_ability_score_improvement
        implementation: dnd2024:class_features_bard_level_4_ability_score_improvement
        parameters:
          class: bard
          grant_at_levels:
          - 4
          - 8
          - 12
          - 16
          feat_choice:
            count_per_milestone: 1
            category: any
            require_prerequisites: true
            default:
              key: dnd2024:feats/general_feats_ability_score_improvement
    contract:
      id: dnd2024:class_features_bard_level_4_ability_score_improvement
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.level_gained
      capabilities:
      - character_choices
      - feats
    requires:
    - dnd2024:class_features_bard_level_4_ability_score_improvement
    citations:
    - SRD 5.2.1 pp. 32; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/bard_level_4_ability_score_improvement.md
  dnd2024:class_features/bard_level_7_countercharm:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/bard_level_7_countercharm
        implementation: dnd2024:class_features_bard_level_7_countercharm
        parameters:
          trigger:
            saving_throw: failed
            effect_applies_any_condition:
            - charmed
            - frightened
            target: self_or_creature
            target_range_feet: 30
          action: reaction
          reroll:
            advantage: true
    contract:
      id: dnd2024:class_features_bard_level_7_countercharm
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      events:
      - saving_throw.failed
      capabilities:
      - saving_throws
      - reactions
    requires:
    - dnd2024:class_features_bard_level_7_countercharm
    citations:
    - SRD 5.2.1 pp. 33; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/bard_level_7_countercharm.md
  dnd2024:class_features/bard_level_18_superior_inspiration:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/bard_level_18_superior_inspiration
        implementation: dnd2024:class_features_bard_level_18_superior_inspiration
        parameters:
          resource: bardic_inspiration
          recover_to_minimum: 2
    contract:
      id: dnd2024:class_features_bard_level_18_superior_inspiration
      inputs:
        actor: participant
      outputs:
      - feature_state
      - resource_changes
      events:
      - initiative.rolled
      capabilities:
      - resources
    requires:
    - dnd2024:class_features_bard_level_18_superior_inspiration
    citations:
    - SRD 5.2.1 pp. 33; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/bard_level_18_superior_inspiration.md
  dnd2024:class_features/bard_level_20_words_of_creation:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/bard_level_20_words_of_creation
        implementation: dnd2024:class_features_bard_level_20_words_of_creation
        parameters:
          always_prepared:
          - key: dnd2024:spells/power_word_heal
          - key: dnd2024:spells/power_word_kill
          when_casting_these_spells:
            optional_additional_creature_targets: 1
            additional_target_within_feet_of_first: 10
    contract:
      id: dnd2024:class_features_bard_level_20_words_of_creation
      inputs:
        actor: participant
      outputs:
      - feature_state
      - spell_grants
      events:
      - character.feature_gained
      - spell.cast_pending
      capabilities:
      - spellcasting
      - targeting
    requires:
    - dnd2024:class_features_bard_level_20_words_of_creation
    citations:
    - SRD 5.2.1 pp. 33; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/bard_level_20_words_of_creation.md
  dnd2024:class_features/college_of_lore_level_3_bonus_proficiencies:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/college_of_lore_level_3_bonus_proficiencies
        implementation: dnd2024:class_features_college_of_lore_level_3_bonus_proficiencies
        parameters:
          skill_proficiency_choice:
            count: 3
            category: any_skill
    contract:
      id: dnd2024:class_features_college_of_lore_level_3_bonus_proficiencies
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      - proficiencies
      events:
      - character.feature_gained
      capabilities:
      - character_choices
      - proficiencies
    requires:
    - dnd2024:class_features_college_of_lore_level_3_bonus_proficiencies
    citations:
    - SRD 5.2.1 pp. 35; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/college_of_lore_level_3_bonus_proficiencies.md
  dnd2024:class_features/college_of_lore_level_6_magical_discoveries:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/college_of_lore_level_6_magical_discoveries
        implementation: dnd2024:class_features_college_of_lore_level_6_magical_discoveries
        parameters:
          spell_choice:
            count: 2
            lists:
            - cleric
            - druid
            - wizard
            mix_lists: true
            allowed_level:
              cantrip: true
              otherwise_has_bard_progression_slot: true
          always_prepared: true
          counts_as_class_spell: bard
          counts_toward_bard_prepared_limit: false
          replace:
            event: character.bard_level_gained
            maximum: 1
            same_eligibility: true
    contract:
      id: dnd2024:class_features_college_of_lore_level_6_magical_discoveries
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      - spell_grants
      events:
      - character.feature_gained
      - character.class_level_gained
      capabilities:
      - spellcasting
      - character_choices
    requires:
    - dnd2024:class_features_college_of_lore_level_6_magical_discoveries
    citations:
    - SRD 5.2.1 pp. 35; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/college_of_lore_level_6_magical_discoveries.md
  dnd2024:classes/cleric:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:classes/cleric
        implementation: dnd2024:classes_cleric
        parameters:
          class: cleric
          primary_abilities:
          - wisdom
          hit_point_die:
            sides: 8
            per_class_level: 1
          saving_throw_proficiencies:
          - wisdom
          - charisma
          skill_proficiencies:
            count: 2
            options:
            - history
            - insight
            - medicine
            - persuasion
            - religion
          weapon_proficiencies:
          - simple
          armor_training:
          - light
          - medium
          - shield
          tool_proficiencies: []
          starting_equipment:
            count: 1
            options:
            - id: a
              grants:
              - item_type: chain_shirt
                quantity: 1
              - item_type: shield
                quantity: 1
              - item_type: mace
                quantity: 1
              - item_type: holy_symbol
                quantity: 1
              - item_type: priest_s_pack
                quantity: 1
              - item_type: gp
                quantity: 7
            - id: b
              grants:
              - item_type: gp
                quantity: 110
          progression:
            key: dnd2024:tables/cleric_progression
          feature_grants:
          - feature: &id033
              key: dnd2024:class_features/cleric_level_10_divine_intervention
            minimum_class_level: 10
          - feature: &id034
              key: dnd2024:class_features/cleric_level_14_improved_blessed_strikes
            minimum_class_level: 14
          - feature: &id035
              key: dnd2024:class_features/cleric_level_19_epic_boon
            minimum_class_level: 19
          - feature: &id036
              key: dnd2024:class_features/cleric_level_1_divine_order
            minimum_class_level: 1
          - feature: &id037
              key: dnd2024:class_features/cleric_level_1_spellcasting
            minimum_class_level: 1
          - feature: &id038
              key: dnd2024:class_features/cleric_level_20_greater_divine_intervention
            minimum_class_level: 20
          - feature: &id039
              key: dnd2024:class_features/cleric_level_2_channel_divinity
            minimum_class_level: 2
          - feature: &id040
              key: dnd2024:class_features/cleric_level_3_cleric_subclass
            minimum_class_level: 3
          - feature: &id041
              key: dnd2024:class_features/cleric_level_4_ability_score_improvement
            minimum_class_level: 4
          - feature: &id042
              key: dnd2024:class_features/cleric_level_5_sear_undead
            minimum_class_level: 5
          - feature: &id043
              key: dnd2024:class_features/cleric_level_7_blessed_strikes
            minimum_class_level: 7
      splat_rule_group:
        category: class
        members:
        - *id033
        - *id034
        - *id035
        - *id036
        - *id037
        - *id038
        - *id039
        - *id040
        - *id041
        - *id042
        - *id043
    contract:
      id: dnd2024:classes_cleric
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - proficiencies
      - inventory_changes
      events:
      - character.class_selected
      - character.level_gained
      capabilities:
      - character_creation
      - class_progression
      - inventory
      - proficiencies
    requires:
    - dnd2024:classes_cleric
    citations:
    - SRD 5.2.1 pp. 36; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/classes/cleric.md
  dnd2024:class_features/cleric_level_1_divine_order:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/cleric_level_1_divine_order
        implementation: dnd2024:class_features_cleric_level_1_divine_order
        parameters:
          choice:
            count: 1
            options:
            - id: protector
              weapon_proficiencies:
              - martial
              armor_training:
              - heavy
            - id: thaumaturge
              cantrip_choice:
                list: cleric
                count: 1
              check_bonus:
                ability: intelligence
                skills:
                - arcana
                - religion
                amount:
                  op: max
                  args:
                  - 1
                  - stat: abilities.wisdom.modifier
    contract:
      id: dnd2024:class_features_cleric_level_1_divine_order
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      - spell_grants
      - proficiencies
      events:
      - character.feature_gained
      capabilities:
      - character_choices
      - proficiencies
      - spellcasting
    requires:
    - dnd2024:class_features_cleric_level_1_divine_order
    citations:
    - SRD 5.2.1 pp. 37; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/cleric_level_1_divine_order.md
  dnd2024:class_features/cleric_level_3_cleric_subclass:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/cleric_level_3_cleric_subclass
        implementation: dnd2024:class_features_cleric_level_3_cleric_subclass
        parameters:
          subclass_choice:
            count: 1
            class: cleric
            srd_options:
            - key: dnd2024:subclasses/cleric_cleric_subclass_life_domain
          grant_features:
            from: selected_subclass
            when_required_class_level_lte_current: true
            reevaluate_on_class_level_gain: true
    contract:
      id: dnd2024:class_features_cleric_level_3_cleric_subclass
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.feature_gained
      - character.level_gained
      capabilities:
      - character_choices
      - class_progression
    requires:
    - dnd2024:class_features_cleric_level_3_cleric_subclass
    citations:
    - SRD 5.2.1 pp. 37; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/cleric_level_3_cleric_subclass.md
  dnd2024:class_features/cleric_level_5_sear_undead:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/cleric_level_5_sear_undead
        implementation: dnd2024:class_features_cleric_level_5_sear_undead
        parameters:
          trigger: turn_undead.used
          optional: true
          damage_roll:
            shared_for_all_targets: true
            roll:
              dice:
                count:
                  op: max
                  args:
                  - 1
                  - stat: abilities.wisdom.modifier
                sides: 8
            type: radiant
          targets:
            creature_type: undead
            failed_triggering_save: true
          damage_ends_turn_undead: false
    contract:
      id: dnd2024:class_features_cleric_level_5_sear_undead
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - damage
      events:
      - turn_undead.resolved
      capabilities:
      - damage
      - effects
    requires:
    - dnd2024:class_features_cleric_level_5_sear_undead
    citations:
    - SRD 5.2.1 pp. 37; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/cleric_level_5_sear_undead.md
  dnd2024:class_features/cleric_level_10_divine_intervention:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/cleric_level_10_divine_intervention
        implementation: dnd2024:class_features_cleric_level_10_divine_intervention
        parameters:
          action: magic
          spell_choice:
            list: cleric
            maximum_level: 5
            exclude_casting_time: reaction
          cast_as_part_of_action: true
          spell_slot_cost: 0
          material_components_required: false
          uses:
            maximum: 1
            recover:
              event: rest.long_rest_completed
              amount: all
    contract:
      id: dnd2024:class_features_cleric_level_10_divine_intervention
      inputs:
        actor: participant
      outputs:
      - feature_state
      - resource_changes
      - spell_grants
      events:
      - action.magic
      capabilities:
      - spellcasting
      - resources
    requires:
    - dnd2024:class_features_cleric_level_10_divine_intervention
    citations:
    - SRD 5.2.1 pp. 38; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/cleric_level_10_divine_intervention.md
  dnd2024:class_features/cleric_level_19_epic_boon:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/cleric_level_19_epic_boon
        implementation: dnd2024:class_features_cleric_level_19_epic_boon
        parameters:
          minimum_class_level: 19
          feat_choice:
            count: 1
            category: any
            require_prerequisites: true
            suggested_category: epic_boon
    contract:
      id: dnd2024:class_features_cleric_level_19_epic_boon
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.level_gained
      capabilities:
      - character_choices
      - feats
    requires:
    - dnd2024:class_features_cleric_level_19_epic_boon
    citations:
    - SRD 5.2.1 pp. 38; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/cleric_level_19_epic_boon.md
  dnd2024:subclasses/cleric_cleric_subclass_life_domain:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:subclasses/cleric_cleric_subclass_life_domain
        implementation: dnd2024:subclasses_cleric_cleric_subclass_life_domain
        parameters:
          class: cleric
          subclass: life_domain
          feature_grants:
          - feature: &id044
              key: dnd2024:class_features/life_domain_level_17_supreme_healing
            minimum_class_level: 17
          - feature: &id045
              key: dnd2024:class_features/life_domain_level_3_disciple_of_life
            minimum_class_level: 3
          - feature: &id046
              key: dnd2024:class_features/life_domain_level_3_life_domain_spells
            minimum_class_level: 3
          - feature: &id047
              key: dnd2024:class_features/life_domain_level_3_preserve_life
            minimum_class_level: 3
          - feature: &id048
              key: dnd2024:class_features/life_domain_level_6_blessed_healer
            minimum_class_level: 6
      splat_rule_group:
        category: subclass
        members:
        - *id044
        - *id045
        - *id046
        - *id047
        - *id048
    contract:
      id: dnd2024:subclasses_cleric_cleric_subclass_life_domain
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - character.subclass_selected
      - character.class_level_gained
      capabilities:
      - class_progression
    requires:
    - dnd2024:subclasses_cleric_cleric_subclass_life_domain
    citations:
    - SRD 5.2.1 pp. 40; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/subclasses/cleric_cleric_subclass_life_domain.md
  dnd2024:class_features/life_domain_level_3_life_domain_spells:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/life_domain_level_3_life_domain_spells
        implementation: dnd2024:class_features_life_domain_level_3_life_domain_spells
        parameters:
          class: cleric
          always_prepared_by_class_level:
          - level: 3
            spells:
            - key: dnd2024:spells/aid
            - key: dnd2024:spells/bless
            - key: dnd2024:spells/cure_wounds
            - key: dnd2024:spells/lesser_restoration
          - level: 5
            spells:
            - key: dnd2024:spells/mass_healing_word
            - key: dnd2024:spells/revivify
          - level: 7
            spells:
            - key: dnd2024:spells/aura_of_life
            - key: dnd2024:spells/death_ward
          - level: 9
            spells:
            - key: dnd2024:spells/greater_restoration
            - key: dnd2024:spells/mass_cure_wounds
          retain_at_higher_levels: true
          counts_toward_class_prepared_limit: false
    contract:
      id: dnd2024:class_features_life_domain_level_3_life_domain_spells
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - character.class_level_gained
      capabilities:
      - spell_preparation
    requires:
    - dnd2024:class_features_life_domain_level_3_life_domain_spells
    citations:
    - SRD 5.2.1 pp. 40; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/life_domain_level_3_life_domain_spells.md
  dnd2024:class_features/life_domain_level_6_blessed_healer:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/life_domain_level_6_blessed_healer
        implementation: dnd2024:class_features_life_domain_level_6_blessed_healer
        parameters:
          trigger:
            spell_cast: true
            expended_spell_slot: true
            restores_hit_points_to_other_creature: true
          heal_self:
            op: add
            args:
            - 2
            - stat: spell.slot_level
          timing: immediately_after_cast
    contract:
      id: dnd2024:class_features_life_domain_level_6_blessed_healer
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - healing
      events:
      - spell.cast_completed
      capabilities:
      - healing
    requires:
    - dnd2024:class_features_life_domain_level_6_blessed_healer
    citations:
    - SRD 5.2.1 pp. 40; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/life_domain_level_6_blessed_healer.md
  dnd2024:classes/druid:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:classes/druid
        implementation: dnd2024:classes_druid
        parameters:
          class: druid
          primary_abilities:
          - wisdom
          hit_point_die:
            sides: 8
            per_class_level: 1
          saving_throw_proficiencies:
          - intelligence
          - wisdom
          skill_proficiencies:
            count: 2
            options:
            - animal_handling
            - arcana
            - insight
            - medicine
            - nature
            - perception
            - religion
            - survival
          weapon_proficiencies:
          - simple
          armor_training:
          - light
          - shield
          tool_proficiencies:
          - herbalism_kit
          starting_equipment:
            count: 1
            options:
            - id: a
              grants:
              - item_type: leather_armor
                quantity: 1
              - item_type: shield
                quantity: 1
              - item_type: sickle
                quantity: 1
              - item_type: druidic_focus_quarterstaff
                quantity: 1
              - item_type: explorer_s_pack
                quantity: 1
              - item_type: herbalism_kit
                quantity: 1
              - item_type: gp
                quantity: 9
            - id: b
              grants:
              - item_type: gp
                quantity: 50
          progression:
            key: dnd2024:tables/druid_progression
          feature_grants:
          - feature: &id049
              key: dnd2024:class_features/druid_level_15_improved_elemental_fury
            minimum_class_level: 15
          - feature: &id050
              key: dnd2024:class_features/druid_level_18_beast_spells
            minimum_class_level: 18
          - feature: &id051
              key: dnd2024:class_features/druid_level_19_epic_boon
            minimum_class_level: 19
          - feature: &id052
              key: dnd2024:class_features/druid_level_1_druidic
            minimum_class_level: 1
          - feature: &id053
              key: dnd2024:class_features/druid_level_1_primal_order
            minimum_class_level: 1
          - feature: &id054
              key: dnd2024:class_features/druid_level_1_spellcasting
            minimum_class_level: 1
          - feature: &id055
              key: dnd2024:class_features/druid_level_20_archdruid
            minimum_class_level: 20
          - feature: &id056
              key: dnd2024:class_features/druid_level_2_wild_companion
            minimum_class_level: 2
          - feature: &id057
              key: dnd2024:class_features/druid_level_2_wild_shape
            minimum_class_level: 2
          - feature: &id058
              key: dnd2024:class_features/druid_level_3_druid_subclass
            minimum_class_level: 3
          - feature: &id059
              key: dnd2024:class_features/druid_level_4_ability_score_improvement
            minimum_class_level: 4
          - feature: &id060
              key: dnd2024:class_features/druid_level_5_wild_resurgence
            minimum_class_level: 5
          - feature: &id061
              key: dnd2024:class_features/druid_level_7_elemental_fury
            minimum_class_level: 7
      splat_rule_group:
        category: class
        members:
        - *id049
        - *id050
        - *id051
        - *id052
        - *id053
        - *id054
        - *id055
        - *id056
        - *id057
        - *id058
        - *id059
        - *id060
        - *id061
    contract:
      id: dnd2024:classes_druid
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - proficiencies
      - inventory_changes
      events:
      - character.class_selected
      - character.level_gained
      capabilities:
      - character_creation
      - class_progression
      - inventory
      - proficiencies
    requires:
    - dnd2024:classes_druid
    citations:
    - SRD 5.2.1 pp. 41; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/classes/druid.md
  dnd2024:class_features/druid_level_1_druidic:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/druid_level_1_druidic
        implementation: dnd2024:class_features_druid_level_1_druidic
        parameters:
          language_grant: druidic
          always_prepared:
          - key: dnd2024:spells/speak_with_animals
          hidden_messages:
            can_create: true
            druidic_knowers:
              automatically_spot: true
              can_read: true
            others:
              spot_check:
                ability: intelligence
                skill: investigation
                dc: 15
              decipher_requires_magic: true
    contract:
      id: dnd2024:class_features_druid_level_1_druidic
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - character.feature_gained
      capabilities:
      - languages
      - spell_preparation
      - ability_checks
    requires:
    - dnd2024:class_features_druid_level_1_druidic
    citations:
    - SRD 5.2.1 pp. 42; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/druid_level_1_druidic.md
  dnd2024:class_features/druid_level_2_wild_shape:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/druid_level_2_wild_shape
        implementation: dnd2024:class_features_druid_level_2_wild_shape
        parameters:
          action: bonus_action
          resource:
            id: wild_shape
            initial_maximum: 2
            maximum_by_class_level:
              2: 2
              6: 3
              17: 4
            recover:
            - event: rest.short_rest_completed
              amount: 1
            - event: rest.long_rest_completed
              amount: all_expended
          cost: 1
          form_choice:
            from: known_beast_forms
            known_count_by_druid_level:
              2: 4
              4: 6
              8: 8
            maximum_challenge_rating_by_druid_level:
              2: 0.25
              4: 0.5
              8: 1
            fly_speed_allowed_minimum_druid_level: 8
            replace:
              event: rest.long_rest_completed
              maximum: 1
              must_remain_eligible: true
            other_source_beasts:
              choice_authority: game_master
              type: allow_source
              required: true
          duration:
            hours:
              op: divide
              args:
              - stat: classes.druid.level
              - 2
          ends_on:
          - wild_shape.used_again
          - condition.incapacitated
          - death
          end_voluntarily:
            action: bonus_action
          temporary_hit_points:
            stat: classes.druid.level
          statistics:
            replace_with: beast_stat_block
            retain:
            - personality
            - memories
            - speech
            - creature_type
            - hit_points
            - hit_point_dice
            - intelligence
            - wisdom
            - charisma
            - class_features
            - languages
            - feats
            proficiencies:
              retain:
              - skills
              - saving_throws
              use_own_proficiency_bonus: true
              gain_beast_proficiencies: true
              modifier_rule: higher_of_own_and_beast
          spellcasting:
            can_cast: false
            concentration_preserved: true
            existing_spells_unaffected: true
          objects:
            handling_depends_on: form_limbs
            equipment_choice_per_item:
            - drop_in_space
            - merge
            - wear
            wearing_eligibility:
              authority: game_master
              decision_type: boolean
              criteria:
              - size
              - shape
            equipment_resizes: false
            unwearable_choices:
            - drop_in_space
            - merge
            merged_equipment_effects_active: false
            worn_equipment_effects_active: true
    contract:
      id: dnd2024:class_features_druid_level_2_wild_shape
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
        form: definition
        equipment_decisions: choice
      outputs:
      - feature_state
      - choices
      - resource_changes
      - spell_grants
      events:
      - action.bonus
      - rest.short_rest_completed
      - rest.long_rest_completed
      - condition.applied
      - death
      - character.class_level_gained
      capabilities:
      - shapechanging
      - character_choices
      - resources
      - equipment
      - spellcasting
    requires:
    - dnd2024:class_features_druid_level_2_wild_shape
    citations:
    - SRD 5.2.1 pp. 42, 43; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/druid_level_2_wild_shape.md
  dnd2024:class_features/druid_level_3_druid_subclass:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/druid_level_3_druid_subclass
        implementation: dnd2024:class_features_druid_level_3_druid_subclass
        parameters:
          subclass_choice:
            count: 1
            class: druid
            srd_options:
            - key: dnd2024:subclasses/druid_circle_of_the_land
          grant_features:
            from: selected_subclass
            when_required_class_level_lte_current: true
            reevaluate_on_class_level_gain: true
    contract:
      id: dnd2024:class_features_druid_level_3_druid_subclass
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.feature_gained
      - character.level_gained
      capabilities:
      - character_choices
      - class_progression
    requires:
    - dnd2024:class_features_druid_level_3_druid_subclass
    citations:
    - SRD 5.2.1 pp. 43; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/druid_level_3_druid_subclass.md
  dnd2024:class_features/druid_level_5_wild_resurgence:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/druid_level_5_wild_resurgence
        implementation: dnd2024:class_features_druid_level_5_wild_resurgence
        parameters:
          conversions:
          - from: spell_slot
            amount: 1
            to: wild_shape_use
            gain: 1
            requires_remaining_wild_shape: 0
            action: none
            limit:
              count: 1
              period: own_turn
          - from: wild_shape_use
            amount: 1
            to: spell_slot
            slot_level: 1
            gain: 1
            action: none
            uses:
              maximum: 1
              recover:
                event: rest.long_rest_completed
                amount: all
    contract:
      id: dnd2024:class_features_druid_level_5_wild_resurgence
      inputs:
        actor: participant
      outputs:
      - feature_state
      - resource_changes
      events:
      - turn.active
      - resource.conversion_requested
      capabilities:
      - resources
      - spell_slots
    requires:
    - dnd2024:class_features_druid_level_5_wild_resurgence
    citations:
    - SRD 5.2.1 pp. 43; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/druid_level_5_wild_resurgence.md
  dnd2024:class_features/druid_level_15_improved_elemental_fury:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/druid_level_15_improved_elemental_fury
        implementation: dnd2024:class_features_druid_level_15_improved_elemental_fury
        parameters:
          by_elemental_fury_choice:
            potent_spellcasting:
              spell_class: druid
              spell_level: 0
              minimum_base_range_feet: 10
              range_bonus_feet: 300
            primal_strike:
              replace_extra_damage:
                dice:
                  count: 2
                  sides: 8
    contract:
      id: dnd2024:class_features_druid_level_15_improved_elemental_fury
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - damage
      - spell_grants
      events:
      - spell.cast_pending
      - attack.hit
      capabilities:
      - spellcasting
      - damage
    requires:
    - dnd2024:class_features_druid_level_15_improved_elemental_fury
    citations:
    - SRD 5.2.1 pp. 43; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/druid_level_15_improved_elemental_fury.md
  dnd2024:class_features/druid_level_19_epic_boon:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/druid_level_19_epic_boon
        implementation: dnd2024:class_features_druid_level_19_epic_boon
        parameters:
          minimum_class_level: 19
          feat_choice:
            count: 1
            category: any
            require_prerequisites: true
            suggested_category: epic_boon
    contract:
      id: dnd2024:class_features_druid_level_19_epic_boon
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.level_gained
      capabilities:
      - character_choices
      - feats
    requires:
    - dnd2024:class_features_druid_level_19_epic_boon
    citations:
    - SRD 5.2.1 pp. 43; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/druid_level_19_epic_boon.md
  dnd2024:subclasses/druid_circle_of_the_land:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:subclasses/druid_circle_of_the_land
        implementation: dnd2024:subclasses_druid_circle_of_the_land
        parameters:
          class: druid
          subclass: circle_of_the_land
          feature_grants:
          - feature: &id062
              key: dnd2024:class_features/circle_of_the_land_level_10_nature_s_ward
            minimum_class_level: 10
          - feature: &id063
              key: dnd2024:class_features/circle_of_the_land_level_14_nature_s_sanctuary
            minimum_class_level: 14
          - feature: &id064
              key: dnd2024:class_features/circle_of_the_land_level_3_circle_of_the_land_spells
            minimum_class_level: 3
          - feature: &id065
              key: dnd2024:class_features/circle_of_the_land_level_3_land_s_aid
            minimum_class_level: 3
          - feature: &id066
              key: dnd2024:class_features/circle_of_the_land_level_6_natural_recovery
            minimum_class_level: 6
      splat_rule_group:
        category: subclass
        members:
        - *id062
        - *id063
        - *id064
        - *id065
        - *id066
    contract:
      id: dnd2024:subclasses_druid_circle_of_the_land
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - character.subclass_selected
      - character.class_level_gained
      capabilities:
      - class_progression
    requires:
    - dnd2024:subclasses_druid_circle_of_the_land
    citations:
    - SRD 5.2.1 pp. 46; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/subclasses/druid_circle_of_the_land.md
  dnd2024:class_features/circle_of_the_land_level_3_land_s_aid:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/circle_of_the_land_level_3_land_s_aid
        implementation: dnd2024:class_features_circle_of_the_land_level_3_land_s_aid
        parameters:
          action: magic
          cost:
            resource: wild_shape
            amount: 1
          origin:
            range_feet: 60
            type: point
          area:
            shape: sphere
            radius_feet: 10
          damage_targets:
            creatures: chosen_in_area
          save:
            ability: constitution
            dc:
              stat: spell_save_dc
            success_damage_multiplier: 0.5
          damage:
            type: necrotic
            dice_by_druid_level:
              3:
                dice:
                  count: 2
                  sides: 6
              10:
                dice:
                  count: 3
                  sides: 6
              14:
                dice:
                  count: 4
                  sides: 6
          healing:
            target_count: 1
            target_choice: creature_in_area
            dice_by_druid_level:
              3:
                dice:
                  count: 2
                  sides: 6
              10:
                dice:
                  count: 3
                  sides: 6
              14:
                dice:
                  count: 4
                  sides: 6
    contract:
      id: dnd2024:class_features_circle_of_the_land_level_3_land_s_aid
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - resource_changes
      - damage
      - healing
      events:
      - action.magic
      capabilities:
      - areas
      - saving_throws
      - damage
      - healing
      - resources
    requires:
    - dnd2024:class_features_circle_of_the_land_level_3_land_s_aid
    citations:
    - SRD 5.2.1 pp. 46; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/circle_of_the_land_level_3_land_s_aid.md
  dnd2024:class_features/circle_of_the_land_level_10_nature_s_ward:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/circle_of_the_land_level_10_nature_s_ward
        implementation: dnd2024:class_features_circle_of_the_land_level_10_nature_s_ward
        parameters:
          condition_immunities:
          - poisoned
          damage_resistance_by_current_circle_land:
            arid: fire
            polar: cold
            temperate: lightning
            tropical: poison
    contract:
      id: dnd2024:class_features_circle_of_the_land_level_10_nature_s_ward
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - character.feature_gained
      capabilities:
      - immunities
      - resistances
    requires:
    - dnd2024:class_features_circle_of_the_land_level_10_nature_s_ward
    citations:
    - SRD 5.2.1 pp. 46; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/circle_of_the_land_level_10_nature_s_ward.md
  dnd2024:classes/fighter:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:classes/fighter
        implementation: dnd2024:classes_fighter
        parameters:
          class: fighter
          primary_abilities:
          - strength
          - dexterity
          hit_point_die:
            sides: 10
            per_class_level: 1
          saving_throw_proficiencies:
          - strength
          - constitution
          skill_proficiencies:
            count: 2
            options:
            - acrobatics
            - animal_handling
            - athletics
            - history
            - insight
            - intimidation
            - persuasion
            - perception
            - survival
          weapon_proficiencies:
          - simple
          - martial
          armor_training:
          - light
          - medium
          - heavy
          - shield
          tool_proficiencies: []
          starting_equipment:
            count: 1
            options:
            - id: a
              grants:
              - item_type: chain_mail
                quantity: 1
              - item_type: greatsword
                quantity: 1
              - item_type: flail
                quantity: 1
              - item_type: javelin
                quantity: 8
              - item_type: dungeoneer_s_pack
                quantity: 1
              - item_type: gp
                quantity: 4
            - id: b
              grants:
              - item_type: studded_leather_armor
                quantity: 1
              - item_type: scimitar
                quantity: 1
              - item_type: shortsword
                quantity: 1
              - item_type: longbow
                quantity: 1
              - item_type: arrow
                quantity: 20
              - item_type: quiver
                quantity: 1
              - item_type: dungeoneer_s_pack
                quantity: 1
              - item_type: gp
                quantity: 11
            - id: c
              grants:
              - item_type: gp
                quantity: 155
          progression:
            key: dnd2024:tables/fighter_progression
          feature_grants:
          - feature: &id067
              key: dnd2024:class_features/fighter_level_11_two_extra_attacks
            minimum_class_level: 11
          - feature: &id068
              key: dnd2024:class_features/fighter_level_13_studied_attacks
            minimum_class_level: 13
          - feature: &id069
              key: dnd2024:class_features/fighter_level_19_epic_boon
            minimum_class_level: 19
          - feature: &id070
              key: dnd2024:class_features/fighter_level_1_fighting_style
            minimum_class_level: 1
          - feature: &id071
              key: dnd2024:class_features/fighter_level_1_second_wind
            minimum_class_level: 1
          - feature: &id072
              key: dnd2024:class_features/fighter_level_1_weapon_mastery
            minimum_class_level: 1
          - feature: &id073
              key: dnd2024:class_features/fighter_level_20_three_extra_attacks
            minimum_class_level: 20
          - feature: &id074
              key: dnd2024:class_features/fighter_level_2_action_surge
            minimum_class_level: 2
          - feature: &id075
              key: dnd2024:class_features/fighter_level_2_tactical_mind
            minimum_class_level: 2
          - feature: &id076
              key: dnd2024:class_features/fighter_level_3_fighter_subclass
            minimum_class_level: 3
          - feature: &id077
              key: dnd2024:class_features/fighter_level_4_ability_score_improvement
            minimum_class_level: 4
          - feature: &id078
              key: dnd2024:class_features/fighter_level_5_extra_attack
            minimum_class_level: 5
          - feature: &id079
              key: dnd2024:class_features/fighter_level_5_tactical_shift
            minimum_class_level: 5
          - feature: &id080
              key: dnd2024:class_features/fighter_level_9_indomitable
            minimum_class_level: 9
          - feature: &id081
              key: dnd2024:class_features/fighter_level_9_tactical_master
            minimum_class_level: 9
      splat_rule_group:
        category: class
        members:
        - *id067
        - *id068
        - *id069
        - *id070
        - *id071
        - *id072
        - *id073
        - *id074
        - *id075
        - *id076
        - *id077
        - *id078
        - *id079
        - *id080
        - *id081
    contract:
      id: dnd2024:classes_fighter
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - proficiencies
      - inventory_changes
      events:
      - character.class_selected
      - character.level_gained
      capabilities:
      - character_creation
      - class_progression
      - inventory
      - proficiencies
    requires:
    - dnd2024:classes_fighter
    citations:
    - SRD 5.2.1 pp. 47; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/classes/fighter.md
  dnd2024:class_features/fighter_level_1_second_wind:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/fighter_level_1_second_wind
        implementation: dnd2024:class_features_fighter_level_1_second_wind
        parameters:
          action: bonus_action
          heal_self:
            op: add
            args:
            - dice:
                count: 1
                sides: 10
            - stat: classes.fighter.level
          resource:
            maximum_by_fighter_level:
              1: 2
              4: 3
              10: 4
            cost: 1
            recover:
            - event: rest.short_rest_completed
              amount: 1
            - event: rest.long_rest_completed
              amount: all_expended
    contract:
      id: dnd2024:class_features_fighter_level_1_second_wind
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - resource_changes
      - healing
      events:
      - action.bonus
      - rest.short_rest_completed
      - rest.long_rest_completed
      capabilities:
      - healing
      - resources
    requires:
    - dnd2024:class_features_fighter_level_1_second_wind
    citations:
    - SRD 5.2.1 pp. 48; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/fighter_level_1_second_wind.md
    - SRD 5.2.1 p. 47; builtin/srd-5.2.1/source.txt (full page layout)
  dnd2024:class_features/fighter_level_2_action_surge:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/fighter_level_2_action_surge
        implementation: dnd2024:class_features_fighter_level_2_action_surge
        parameters:
          on_own_turn: true
          additional_actions: 1
          excluded_actions:
          - magic
          uses:
            maximum_by_fighter_level:
              2: 1
              17: 2
            recover:
              events:
              - rest.short_rest_completed
              - rest.long_rest_completed
              amount: all_expended
          limit:
            count: 1
            period: turn
    contract:
      id: dnd2024:class_features_fighter_level_2_action_surge
      inputs:
        actor: participant
      outputs:
      - feature_state
      - resource_changes
      events:
      - turn.active
      capabilities:
      - action_economy
      - resources
    requires:
    - dnd2024:class_features_fighter_level_2_action_surge
    citations:
    - SRD 5.2.1 pp. 48; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/fighter_level_2_action_surge.md
  dnd2024:class_features/fighter_level_3_fighter_subclass:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/fighter_level_3_fighter_subclass
        implementation: dnd2024:class_features_fighter_level_3_fighter_subclass
        parameters:
          subclass_choice:
            count: 1
            class: fighter
            srd_options:
            - key: dnd2024:subclasses/fighter_fighter_subclass_champion
          grant_features:
            from: selected_subclass
            when_required_class_level_lte_current: true
            reevaluate_on_class_level_gain: true
    contract:
      id: dnd2024:class_features_fighter_level_3_fighter_subclass
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.feature_gained
      - character.level_gained
      capabilities:
      - character_choices
      - class_progression
    requires:
    - dnd2024:class_features_fighter_level_3_fighter_subclass
    citations:
    - SRD 5.2.1 pp. 48; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/fighter_level_3_fighter_subclass.md
  dnd2024:class_features/fighter_level_5_extra_attack:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/fighter_level_5_extra_attack
        implementation: dnd2024:class_features_fighter_level_5_extra_attack
        parameters:
          action: attack
          turn: own
          replace_attack_count: 2
    contract:
      id: dnd2024:class_features_fighter_level_5_extra_attack
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      events:
      - action.attack
      capabilities:
      - attacks
    requires:
    - dnd2024:class_features_fighter_level_5_extra_attack
    citations:
    - SRD 5.2.1 pp. 48; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/fighter_level_5_extra_attack.md
  dnd2024:class_features/fighter_level_9_indomitable:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/fighter_level_9_indomitable
        implementation: dnd2024:class_features_fighter_level_9_indomitable
        parameters:
          trigger: self_saving_throw_failed
          optional_reroll: true
          reroll_bonus:
            stat: classes.fighter.level
          must_use_new_roll: true
          uses:
            maximum_by_fighter_level:
              9: 1
              13: 2
              17: 3
            recover:
              event: rest.long_rest_completed
              amount: all_expended
    contract:
      id: dnd2024:class_features_fighter_level_9_indomitable
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - resource_changes
      events:
      - saving_throw.failed
      capabilities:
      - saving_throws
      - resources
    requires:
    - dnd2024:class_features_fighter_level_9_indomitable
    citations:
    - SRD 5.2.1 pp. 48; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/fighter_level_9_indomitable.md
  dnd2024:class_features/fighter_level_11_two_extra_attacks:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/fighter_level_11_two_extra_attacks
        implementation: dnd2024:class_features_fighter_level_11_two_extra_attacks
        parameters:
          action: attack
          turn: own
          replace_attack_count: 3
    contract:
      id: dnd2024:class_features_fighter_level_11_two_extra_attacks
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      events:
      - action.attack
      capabilities:
      - attacks
    requires:
    - dnd2024:class_features_fighter_level_11_two_extra_attacks
    citations:
    - SRD 5.2.1 pp. 48; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/fighter_level_11_two_extra_attacks.md
  dnd2024:class_features/fighter_level_19_epic_boon:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/fighter_level_19_epic_boon
        implementation: dnd2024:class_features_fighter_level_19_epic_boon
        parameters:
          minimum_class_level: 19
          feat_choice:
            count: 1
            category: any
            require_prerequisites: true
            suggested_category: epic_boon
    contract:
      id: dnd2024:class_features_fighter_level_19_epic_boon
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.level_gained
      capabilities:
      - character_choices
      - feats
    requires:
    - dnd2024:class_features_fighter_level_19_epic_boon
    citations:
    - SRD 5.2.1 pp. 48; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/fighter_level_19_epic_boon.md
  dnd2024:subclasses/fighter_fighter_subclass_champion:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:subclasses/fighter_fighter_subclass_champion
        implementation: dnd2024:subclasses_fighter_fighter_subclass_champion
        parameters:
          class: fighter
          subclass: champion
          feature_grants:
          - feature: &id082
              key: dnd2024:class_features/champion_level_10_heroic_warrior
            minimum_class_level: 10
          - feature: &id083
              key: dnd2024:class_features/champion_level_15_superior_critical
            minimum_class_level: 15
          - feature: &id084
              key: dnd2024:class_features/champion_level_18_survivor
            minimum_class_level: 18
          - feature: &id085
              key: dnd2024:class_features/champion_level_3_improved_critical
            minimum_class_level: 3
          - feature: &id086
              key: dnd2024:class_features/champion_level_3_remarkable_athlete
            minimum_class_level: 3
          - feature: &id087
              key: dnd2024:class_features/champion_level_7_additional_fighting_style
            minimum_class_level: 7
      splat_rule_group:
        category: subclass
        members:
        - *id082
        - *id083
        - *id084
        - *id085
        - *id086
        - *id087
    contract:
      id: dnd2024:subclasses_fighter_fighter_subclass_champion
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - character.subclass_selected
      - character.class_level_gained
      capabilities:
      - class_progression
    requires:
    - dnd2024:subclasses_fighter_fighter_subclass_champion
    citations:
    - SRD 5.2.1 pp. 49; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/subclasses/fighter_fighter_subclass_champion.md
  dnd2024:class_features/champion_level_3_remarkable_athlete:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/champion_level_3_remarkable_athlete
        implementation: dnd2024:class_features_champion_level_3_remarkable_athlete
        parameters:
          advantage_on:
          - roll: initiative
          - roll: ability_check
            ability: strength
            skill: athletics
          after_critical_hit:
            optional_move_maximum_feet: *id088
            provokes_opportunity_attacks: false
    contract:
      id: dnd2024:class_features_champion_level_3_remarkable_athlete
      inputs:
        actor: participant
        destination:
          type: location
          required: false
      outputs:
      - feature_state
      - movement
      events:
      - initiative.pending
      - ability_check.pending
      - attack.critical_hit
      capabilities:
      - roll_modifiers
      - movement
    requires:
    - dnd2024:class_features_champion_level_3_remarkable_athlete
    citations:
    - SRD 5.2.1 pp. 49; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/champion_level_3_remarkable_athlete.md
  dnd2024:class_features/champion_level_10_heroic_warrior:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/champion_level_10_heroic_warrior
        implementation: dnd2024:class_features_champion_level_10_heroic_warrior
        parameters:
          when:
            combat_active: true
            heroic_inspiration: false
          optional_grant: heroic_inspiration
    contract:
      id: dnd2024:class_features_champion_level_10_heroic_warrior
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - turn.started
      capabilities:
      - inspiration
    requires:
    - dnd2024:class_features_champion_level_10_heroic_warrior
    citations:
    - SRD 5.2.1 pp. 49; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/champion_level_10_heroic_warrior.md
  dnd2024:class_features/champion_level_18_survivor:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/champion_level_18_survivor
        implementation: dnd2024:class_features_champion_level_18_survivor
        parameters:
          death_saves:
            advantage: true
            natural_d20_results_count_as_20:
            - 18
            - 19
            - 20
          turn_start_healing:
            requires:
              bloodied: true
              minimum_hit_points: 1
            amount:
              op: add
              args:
              - 5
              - stat: abilities.constitution.modifier
    contract:
      id: dnd2024:class_features_champion_level_18_survivor
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - healing
      events:
      - death_save.pending
      - death_save.rolled
      - turn.started
      capabilities:
      - saving_throws
      - healing
    requires:
    - dnd2024:class_features_champion_level_18_survivor
    citations:
    - SRD 5.2.1 pp. 49; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/champion_level_18_survivor.md
  dnd2024:class_features/monk_level_1_martial_arts:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_1_martial_arts
        implementation: dnd2024:class_features_monk_level_1_martial_arts
        parameters:
          monk_weapon_categories:
          - category: simple
            range: melee
          - category: martial
            range: melee
            requires_property: light
          benefit_requirements:
            armor_worn: false
            shield_wielded: false
            weapons_wielded: none_or_only_monk_weapons
          bonus_unarmed_strike:
            action: bonus_action
            count: 1
          damage_die_substitution:
            optional: true
            attacks:
            - unarmed_strike
            - monk_weapon
            dice_by_monk_level:
              1:
                dice:
                  count: 1
                  sides: 6
              5:
                dice:
                  count: 1
                  sides: 8
              11:
                dice:
                  count: 1
                  sides: 10
              17:
                dice:
                  count: 1
                  sides: 12
          dexterity_substitution:
            optional: true
            replaces: strength
            for:
            - unarmed_strike_attack
            - unarmed_strike_damage
            - monk_weapon_attack
            - monk_weapon_damage
            - unarmed_grapple_save_dc
            - unarmed_shove_save_dc
    contract:
      id: dnd2024:class_features_monk_level_1_martial_arts
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      events:
      - attack.pending
      - action.bonus
      capabilities:
      - attacks
      - ability_substitution
    requires:
    - dnd2024:class_features_monk_level_1_martial_arts
    citations:
    - SRD 5.2.1 pp. 50; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_1_martial_arts.md
  dnd2024:class_features/monk_level_2_monk_s_focus:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_2_monk_s_focus
        implementation: dnd2024:class_features_monk_level_2_monk_s_focus
        parameters:
          resource:
            id: focus_points
            maximum:
              stat: classes.monk.level
            recover:
              events:
              - rest.short_rest_completed
              - rest.long_rest_completed
              amount: all_expended
          feature_save_dc:
            op: add
            args:
            - 8
            - stat: abilities.wisdom.modifier
            - *id089
          options:
          - id: flurry_of_blows
            action: bonus_action
            focus_cost: 1
            unarmed_strikes: 2
          - id: patient_defense
            action: bonus_action
            focus_cost: 0
            actions:
            - disengage
            paid_alternative:
              focus_cost: 1
              actions:
              - disengage
              - dodge
          - id: step_of_the_wind
            action: bonus_action
            focus_cost: 0
            actions:
            - dash
            paid_alternative:
              focus_cost: 1
              actions:
              - disengage
              - dash
              jump_distance_multiplier: 2
              duration: current_turn
    contract:
      id: dnd2024:class_features_monk_level_2_monk_s_focus
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
        destination:
          type: location
          required: false
      outputs:
      - feature_state
      - resource_changes
      - movement
      events:
      - action.bonus
      - rest.short_rest_completed
      - rest.long_rest_completed
      capabilities:
      - resources
      - attacks
      - action_economy
      - movement
    requires:
    - dnd2024:class_features_monk_level_2_monk_s_focus
    citations:
    - SRD 5.2.1 pp. 50, 51; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_2_monk_s_focus.md
  dnd2024:class_features/monk_level_2_uncanny_metabolism:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_2_uncanny_metabolism
        implementation: dnd2024:class_features_monk_level_2_uncanny_metabolism
        parameters:
          optional: true
          recover:
            resource: focus_points
            amount: all_expended
          heal_self:
            op: add
            args:
            - stat: classes.monk.level
            - dice:
                count: 1
                sides:
                  stat: martial_arts_die.sides
          uses:
            maximum: 1
            recover:
              event: rest.long_rest_completed
              amount: all
    contract:
      id: dnd2024:class_features_monk_level_2_uncanny_metabolism
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - resource_changes
      - healing
      events:
      - initiative.rolled
      capabilities:
      - resources
      - healing
    requires:
    - dnd2024:class_features_monk_level_2_uncanny_metabolism
    citations:
    - SRD 5.2.1 pp. 51; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_2_uncanny_metabolism.md
  dnd2024:class_features/monk_level_3_monk_subclass:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_3_monk_subclass
        implementation: dnd2024:class_features_monk_level_3_monk_subclass
        parameters:
          subclass_choice:
            count: 1
            class: monk
            srd_options:
            - key: dnd2024:subclasses/monk_monk_subclass_warrior_of_the_open_hand
          grant_features:
            from: selected_subclass
            when_required_class_level_lte_current: true
            reevaluate_on_class_level_gain: true
    contract:
      id: dnd2024:class_features_monk_level_3_monk_subclass
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.feature_gained
      - character.level_gained
      capabilities:
      - character_choices
      - class_progression
    requires:
    - dnd2024:class_features_monk_level_3_monk_subclass
    citations:
    - SRD 5.2.1 pp. 51; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_3_monk_subclass.md
  dnd2024:class_features/monk_level_4_slow_fall:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_4_slow_fall
        implementation: dnd2024:class_features_monk_level_4_slow_fall
        parameters:
          trigger: self_falls
          action: reaction
          fall_damage_reduction:
            op: multiply
            args:
            - 5
            - stat: classes.monk.level
    contract:
      id: dnd2024:class_features_monk_level_4_slow_fall
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - damage
      events:
      - fall.started
      capabilities:
      - damage
      - reactions
    requires:
    - dnd2024:class_features_monk_level_4_slow_fall
    citations:
    - SRD 5.2.1 pp. 51; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_4_slow_fall.md
  dnd2024:class_features/monk_level_5_stunning_strike:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_5_stunning_strike
        implementation: dnd2024:class_features_monk_level_5_stunning_strike
        parameters:
          trigger:
            hit_creature: true
            attack_types:
            - monk_weapon
            - unarmed_strike
          limit:
            count: 1
            period: turn
          focus_cost: 1
          save:
            ability: constitution
            dc:
              op: add
              args:
              - 8
              - stat: abilities.wisdom.modifier
              - *id089
            failure:
              condition: stunned
              until: source.next_turn_start
            success:
              speed_multiplier: 0.5
              next_attack_against_target_advantage: true
              until: source.next_turn_start
    contract:
      id: dnd2024:class_features_monk_level_5_stunning_strike
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - resource_changes
      - conditions
      events:
      - attack.hit
      capabilities:
      - saving_throws
      - conditions
      - resources
    requires:
    - dnd2024:class_features_monk_level_5_stunning_strike
    citations:
    - SRD 5.2.1 pp. 51; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_5_stunning_strike.md
  dnd2024:class_features/monk_level_7_evasion:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_7_evasion
        implementation: dnd2024:class_features_monk_level_7_evasion
        parameters:
          when:
            saving_throw_ability: dexterity
            normal_success_damage_multiplier: 0.5
            incapacitated: false
          damage_multipliers:
            success: 0
            failure: 0.5
    contract:
      id: dnd2024:class_features_monk_level_7_evasion
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - damage
      events:
      - saving_throw.resolved
      capabilities:
      - damage
      - saving_throws
    requires:
    - dnd2024:class_features_monk_level_7_evasion
    citations:
    - SRD 5.2.1 pp. 51; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_7_evasion.md
  dnd2024:class_features/monk_level_10_heightened_focus:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_10_heightened_focus
        implementation: dnd2024:class_features_monk_level_10_heightened_focus
        parameters:
          flurry_of_blows:
            focus_cost: 1
            replace_unarmed_strike_count: 3
          patient_defense:
            when_focus_spent: true
            temporary_hit_points:
              dice:
                count: 2
                sides:
                  stat: martial_arts_die.sides
          step_of_the_wind:
            when_focus_spent: true
            companion_choice:
              count: 1
              willing: true
              maximum_distance_feet: 5
              maximum_size: large
            moves_with_source_until: current_turn_end
            companion_movement_provokes_opportunity_attacks: false
    contract:
      id: dnd2024:class_features_monk_level_10_heightened_focus
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
        destination:
          type: location
          required: false
      outputs:
      - feature_state
      - movement
      events:
      - focus_feature.used
      capabilities:
      - attacks
      - temporary_hit_points
      - movement
    requires:
    - dnd2024:class_features_monk_level_10_heightened_focus
    citations:
    - SRD 5.2.1 pp. 51, 52; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_10_heightened_focus.md
  dnd2024:class_features/monk_level_13_deflect_energy:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_13_deflect_energy
        implementation: dnd2024:class_features_monk_level_13_deflect_energy
        parameters:
          feature:
            key: dnd2024:class_features/monk_level_3_deflect_attacks
          replace_eligible_attack_damage_types: all
    contract:
      id: dnd2024:class_features_monk_level_13_deflect_energy
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - damage
      events:
      - attack.hit
      capabilities:
      - damage
      - reactions
    requires:
    - dnd2024:class_features_monk_level_13_deflect_energy
    citations:
    - SRD 5.2.1 pp. 52; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_13_deflect_energy.md
  dnd2024:class_features/monk_level_15_perfect_focus:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_15_perfect_focus
        implementation: dnd2024:class_features_monk_level_15_perfect_focus
        parameters:
          when_uncanny_metabolism_used: false
          resource: focus_points
          recover_to_minimum: 4
    contract:
      id: dnd2024:class_features_monk_level_15_perfect_focus
      inputs:
        actor: participant
      outputs:
      - feature_state
      - resource_changes
      events:
      - initiative.rolled
      capabilities:
      - resources
    requires:
    - dnd2024:class_features_monk_level_15_perfect_focus
    citations:
    - SRD 5.2.1 pp. 52; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_15_perfect_focus.md
  dnd2024:class_features/monk_level_19_epic_boon:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/monk_level_19_epic_boon
        implementation: dnd2024:class_features_monk_level_19_epic_boon
        parameters:
          minimum_class_level: 19
          feat_choice:
            count: 1
            category: any
            require_prerequisites: true
            suggested_category: epic_boon
    contract:
      id: dnd2024:class_features_monk_level_19_epic_boon
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.level_gained
      capabilities:
      - character_choices
      - feats
    requires:
    - dnd2024:class_features_monk_level_19_epic_boon
    citations:
    - SRD 5.2.1 pp. 52; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/monk_level_19_epic_boon.md
  dnd2024:subclasses/monk_monk_subclass_warrior_of_the_open_hand:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:subclasses/monk_monk_subclass_warrior_of_the_open_hand
        implementation: dnd2024:subclasses_monk_monk_subclass_warrior_of_the_open_hand
        parameters:
          class: monk
          subclass: warrior_of_the_open_hand
          feature_grants:
          - feature: &id090
              key: dnd2024:class_features/warrior_of_the_open_hand_level_11_fleet_step
            minimum_class_level: 11
          - feature: &id091
              key: dnd2024:class_features/warrior_of_the_open_hand_level_17_quivering_palm
            minimum_class_level: 17
          - feature: &id092
              key: dnd2024:class_features/warrior_of_the_open_hand_level_3_open_hand_technique
            minimum_class_level: 3
          - feature: &id093
              key: dnd2024:class_features/warrior_of_the_open_hand_level_6_wholeness_of_body
            minimum_class_level: 6
      splat_rule_group:
        category: subclass
        members:
        - *id090
        - *id091
        - *id092
        - *id093
    contract:
      id: dnd2024:subclasses_monk_monk_subclass_warrior_of_the_open_hand
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - character.subclass_selected
      - character.class_level_gained
      capabilities:
      - class_progression
    requires:
    - dnd2024:subclasses_monk_monk_subclass_warrior_of_the_open_hand
    citations:
    - SRD 5.2.1 pp. 52; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/subclasses/monk_monk_subclass_warrior_of_the_open_hand.md
  dnd2024:class_features/warrior_of_the_open_hand_level_6_wholeness_of_body:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/warrior_of_the_open_hand_level_6_wholeness_of_body
        implementation: dnd2024:class_features_warrior_of_the_open_hand_level_6_wholeness_of_body
        parameters:
          action: bonus_action
          heal_self:
            op: max
            args:
            - 1
            - op: add
              args:
              - dice:
                  count: 1
                  sides:
                    stat: martial_arts_die.sides
              - stat: abilities.wisdom.modifier
          uses:
            maximum:
              op: max
              args:
              - 1
              - stat: abilities.wisdom.modifier
            recover:
              event: rest.long_rest_completed
              amount: all
    contract:
      id: dnd2024:class_features_warrior_of_the_open_hand_level_6_wholeness_of_body
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - resource_changes
      - healing
      events:
      - action.bonus
      capabilities:
      - healing
      - resources
    requires:
    - dnd2024:class_features_warrior_of_the_open_hand_level_6_wholeness_of_body
    citations:
    - SRD 5.2.1 pp. 52; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/warrior_of_the_open_hand_level_6_wholeness_of_body.md
  dnd2024:class_features/warrior_of_the_open_hand_level_17_quivering_palm:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/warrior_of_the_open_hand_level_17_quivering_palm
        implementation: dnd2024:class_features_warrior_of_the_open_hand_level_17_quivering_palm
        parameters:
          apply:
            trigger: unarmed_strike.hit_creature
            focus_cost: 4
            duration:
              days:
                stat: classes.monk.level
            perceptible: false
            passive_damage: 0
          maximum_active_creatures: 1
          detonate:
            action_options:
            - action: action
            - action: replace_one_attack
              requires_action: attack
              turn: own
            requires_same_plane: true
            save:
              ability: constitution
              dc:
                op: add
                args:
                - 8
                - stat: abilities.wisdom.modifier
                - *id089
              failure_damage:
                dice:
                  count: 10
                  sides: 12
              success_damage_multiplier: 0.5
              damage_type: force
            ends_vibrations: true
          dismiss:
            action: none
            harmless: true
    contract:
      id: dnd2024:class_features_warrior_of_the_open_hand_level_17_quivering_palm
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - resource_changes
      - damage
      events:
      - attack.hit
      - action.attack
      - action.taken
      capabilities:
      - effects
      - resources
      - damage
      - saving_throws
    requires:
    - dnd2024:class_features_warrior_of_the_open_hand_level_17_quivering_palm
    citations:
    - SRD 5.2.1 pp. 52; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/warrior_of_the_open_hand_level_17_quivering_palm.md
  dnd2024:class_features/paladin_level_1_lay_on_hands:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/paladin_level_1_lay_on_hands
        implementation: dnd2024:class_features_paladin_level_1_lay_on_hands
        parameters:
          resource:
            id: lay_on_hands_pool
            maximum:
              op: multiply
              args:
              - 5
              - stat: classes.paladin.level
            recover:
              event: rest.long_rest_completed
              amount: all_expended
          action: bonus_action
          target:
            type: creature
            range: touch
            self_allowed: true
          healing:
            chosen_points:
              type: integer
              minimum: 0
              maximum: remaining_pool
            hit_points_per_pool_point: 1
          remove_condition:
            condition: poisoned
            pool_cost: 5
            cost_points_also_heal: false
    contract:
      id: dnd2024:class_features_paladin_level_1_lay_on_hands
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - resource_changes
      - healing
      - conditions
      events:
      - action.bonus
      - rest.long_rest_completed
      capabilities:
      - healing
      - conditions
      - resources
    requires:
    - dnd2024:class_features_paladin_level_1_lay_on_hands
    citations:
    - SRD 5.2.1 pp. 53, 54; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/paladin_level_1_lay_on_hands.md
  dnd2024:class_features/paladin_level_1_weapon_mastery:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/paladin_level_1_weapon_mastery
        implementation: dnd2024:class_features_paladin_level_1_weapon_mastery
        parameters:
          mastery_choice:
            count: 2
            category: weapon_kind
            requires_proficiency: true
          grants: chosen_weapons_mastery_properties
          replace:
            event: rest.long_rest_completed
            maximum: all
    contract:
      id: dnd2024:class_features_paladin_level_1_weapon_mastery
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.feature_gained
      - rest.long_rest_completed
      capabilities:
      - weapon_mastery
      - character_choices
    requires:
    - dnd2024:class_features_paladin_level_1_weapon_mastery
    citations:
    - SRD 5.2.1 pp. 54; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/paladin_level_1_weapon_mastery.md
  dnd2024:class_features/paladin_level_2_paladin_s_smite:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/paladin_level_2_paladin_s_smite
        implementation: dnd2024:class_features_paladin_level_2_paladin_s_smite
        parameters:
          always_prepared:
          - key: dnd2024:spells/divine_smite
          free_cast:
            spell:
              key: dnd2024:spells/divine_smite
            spell_slot_cost: 0
            uses:
              maximum: 1
              recover:
                event: rest.long_rest_completed
                amount: all
    contract:
      id: dnd2024:class_features_paladin_level_2_paladin_s_smite
      inputs:
        actor: participant
      outputs:
      - feature_state
      - resource_changes
      - spell_grants
      events:
      - spell.cast_pending
      - rest.long_rest_completed
      capabilities:
      - spellcasting
      - resources
    requires:
    - dnd2024:class_features_paladin_level_2_paladin_s_smite
    citations:
    - SRD 5.2.1 pp. 54; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/paladin_level_2_paladin_s_smite.md
  dnd2024:class_features/paladin_level_3_paladin_subclass:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/paladin_level_3_paladin_subclass
        implementation: dnd2024:class_features_paladin_level_3_paladin_subclass
        parameters:
          subclass_choice:
            count: 1
            class: paladin
            srd_options:
            - key: dnd2024:subclasses/paladin_paladin_subclass_oath_of_devotion
          grant_features:
            from: selected_subclass
            when_required_class_level_lte_current: true
            reevaluate_on_class_level_gain: true
    contract:
      id: dnd2024:class_features_paladin_level_3_paladin_subclass
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.feature_gained
      - character.level_gained
      capabilities:
      - character_choices
      - class_progression
    requires:
    - dnd2024:class_features_paladin_level_3_paladin_subclass
    citations:
    - SRD 5.2.1 pp. 55; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/paladin_level_3_paladin_subclass.md
  dnd2024:class_features/paladin_level_5_extra_attack:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/paladin_level_5_extra_attack
        implementation: dnd2024:class_features_paladin_level_5_extra_attack
        parameters:
          action: attack
          turn: own
          replace_attack_count: 2
    contract:
      id: dnd2024:class_features_paladin_level_5_extra_attack
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      events:
      - action.attack
      capabilities:
      - attacks
    requires:
    - dnd2024:class_features_paladin_level_5_extra_attack
    citations:
    - SRD 5.2.1 pp. 55; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/paladin_level_5_extra_attack.md
  dnd2024:class_features/paladin_level_6_aura_of_protection:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/paladin_level_6_aura_of_protection
        implementation: dnd2024:class_features_paladin_level_6_aura_of_protection
        parameters:
          aura:
            shape: emanation
            radius_feet: 10
            origin: self
            visible: false
            inactive_when_condition: incapacitated
          targets:
          - self
          - allies_in_aura
          saving_throw_bonus:
            op: max
            args:
            - 1
            - stat: abilities.charisma.modifier
          stacking:
            maximum_aura_of_protection_benefits: 1
            selector: affected_creature
    contract:
      id: dnd2024:class_features_paladin_level_6_aura_of_protection
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      events:
      - saving_throw.pending
      - aura.membership_changed
      - condition.applied
      capabilities:
      - auras
      - saving_throws
    requires:
    - dnd2024:class_features_paladin_level_6_aura_of_protection
    citations:
    - SRD 5.2.1 pp. 55; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/paladin_level_6_aura_of_protection.md
  dnd2024:class_features/paladin_level_10_aura_of_courage:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/paladin_level_10_aura_of_courage
        implementation: dnd2024:class_features_paladin_level_10_aura_of_courage
        parameters:
          aura: own_aura_of_protection
          targets:
          - self
          - allies
          condition_immunity: frightened
          existing_condition: suppressed_while_in_aura
    contract:
      id: dnd2024:class_features_paladin_level_10_aura_of_courage
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - conditions
      events:
      - aura.membership_changed
      - condition.pending
      capabilities:
      - auras
      - conditions
    requires:
    - dnd2024:class_features_paladin_level_10_aura_of_courage
    citations:
    - SRD 5.2.1 pp. 55; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/paladin_level_10_aura_of_courage.md
  dnd2024:class_features/paladin_level_14_restoring_touch:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/paladin_level_14_restoring_touch
        implementation: dnd2024:class_features_paladin_level_14_restoring_touch
        parameters:
          during: lay_on_hands
          remove_conditions_choice:
            options:
            - blinded
            - charmed
            - deafened
            - frightened
            - paralyzed
            - stunned
            maximum: all_present
          pool_cost_per_condition: 5
          cost_points_also_heal: false
    contract:
      id: dnd2024:class_features_paladin_level_14_restoring_touch
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - resource_changes
      - conditions
      events:
      - lay_on_hands.used
      capabilities:
      - conditions
      - resources
    requires:
    - dnd2024:class_features_paladin_level_14_restoring_touch
    citations:
    - SRD 5.2.1 pp. 55; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/paladin_level_14_restoring_touch.md
  dnd2024:class_features/paladin_level_19_epic_boon:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/paladin_level_19_epic_boon
        implementation: dnd2024:class_features_paladin_level_19_epic_boon
        parameters:
          minimum_class_level: 19
          feat_choice:
            count: 1
            category: any
            require_prerequisites: true
            suggested_category: epic_boon
    contract:
      id: dnd2024:class_features_paladin_level_19_epic_boon
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.level_gained
      capabilities:
      - character_choices
      - feats
    requires:
    - dnd2024:class_features_paladin_level_19_epic_boon
    citations:
    - SRD 5.2.1 pp. 55; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/paladin_level_19_epic_boon.md
  dnd2024:class_features/oath_of_devotion_level_3_oath_of_devotion_spells:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/oath_of_devotion_level_3_oath_of_devotion_spells
        implementation: dnd2024:class_features_oath_of_devotion_level_3_oath_of_devotion_spells
        parameters:
          class: paladin
          always_prepared_by_class_level:
          - level: 3
            spells:
            - key: dnd2024:spells/protection_from_evil_and_good
            - key: dnd2024:spells/shield_of_faith
          - level: 5
            spells:
            - key: dnd2024:spells/aid
            - key: dnd2024:spells/zone_of_truth
          - level: 9
            spells:
            - key: dnd2024:spells/beacon_of_hope
            - key: dnd2024:spells/dispel_magic
          - level: 13
            spells:
            - key: dnd2024:spells/freedom_of_movement
            - key: dnd2024:spells/guardian_of_faith
          - level: 17
            spells:
            - key: dnd2024:spells/commune
            - key: dnd2024:spells/flame_strike
          retain_at_higher_levels: true
          counts_toward_class_prepared_limit: false
    contract:
      id: dnd2024:class_features_oath_of_devotion_level_3_oath_of_devotion_spells
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - character.class_level_gained
      capabilities:
      - spell_preparation
    requires:
    - dnd2024:class_features_oath_of_devotion_level_3_oath_of_devotion_spells
    citations:
    - SRD 5.2.1 pp. 56; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/oath_of_devotion_level_3_oath_of_devotion_spells.md
  dnd2024:class_features/oath_of_devotion_level_7_aura_of_devotion:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/oath_of_devotion_level_7_aura_of_devotion
        implementation: dnd2024:class_features_oath_of_devotion_level_7_aura_of_devotion
        parameters:
          aura: own_aura_of_protection
          targets:
          - self
          - allies
          condition_immunity: charmed
          existing_condition: suppressed_while_in_aura
    contract:
      id: dnd2024:class_features_oath_of_devotion_level_7_aura_of_devotion
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - conditions
      events:
      - aura.membership_changed
      - condition.pending
      capabilities:
      - auras
      - conditions
    requires:
    - dnd2024:class_features_oath_of_devotion_level_7_aura_of_devotion
    citations:
    - SRD 5.2.1 pp. 56; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/oath_of_devotion_level_7_aura_of_devotion.md
  dnd2024:class_features/oath_of_devotion_level_20_holy_nimbus:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/oath_of_devotion_level_20_holy_nimbus
        implementation: dnd2024:class_features_oath_of_devotion_level_20_holy_nimbus
        parameters:
          action: bonus_action
          duration:
            minutes: 10
          voluntary_end_action: none
          uses:
            maximum: 1
            recover:
              event: rest.long_rest_completed
              amount: all
          restore_use:
            spell_slot_level: 5
            spell_slot_cost: 1
            action: none
          benefits:
            saving_throw_advantage:
              forced_by_creature_types:
              - fiend
              - undead
            aura_damage:
              aura: own_aura_of_protection
              trigger: enemy.turn_started_in_aura
              amount:
                op: add
                args:
                - stat: abilities.charisma.modifier
                - *id089
              type: radiant
            aura_light:
              brightness: bright
              sunlight: true
    contract:
      id: dnd2024:class_features_oath_of_devotion_level_20_holy_nimbus
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - resource_changes
      - damage
      events:
      - action.bonus
      - saving_throw.pending
      - turn.started
      capabilities:
      - auras
      - damage
      - resources
      - light
    requires:
    - dnd2024:class_features_oath_of_devotion_level_20_holy_nimbus
    citations:
    - SRD 5.2.1 pp. 57; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/oath_of_devotion_level_20_holy_nimbus.md
  dnd2024:class_features/ranger_level_1_spellcasting:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/ranger_level_1_spellcasting
        implementation: dnd2024:class_features_ranger_level_1_spellcasting
        parameters:
          class: ranger
          spell_list: ranger
          ability: wisdom
          focus_types:
          - druidic_focus
          slots:
            progression:
              key: dnd2024:tables/ranger_progression
            columns_by_spell_level:
              1: spell_slots_1
              2: spell_slots_2
              3: spell_slots_3
              4: spell_slots_4
              5: spell_slots_5
            recovery:
              event: rest.long_rest_completed
              amount: all_expended
          prepared_spells:
            initial_count: 2
            initial_spell_level: 1
            count:
              table:
                key: dnd2024:tables/ranger_progression
              row:
                stat: classes.ranger.level
              column: prepared_spells
            choose_on_count_increase: true
            level_must_have_slot: true
            replace:
              event: rest.long_rest_completed
              maximum: 1
            other_class_feature_always_prepared:
              counts_toward_limit: false
              counts_as_class_spell: true
    contract:
      id: dnd2024:class_features_ranger_level_1_spellcasting
      inputs:
        actor: participant
      outputs:
      - feature_state
      - resource_changes
      - spell_grants
      events:
      - character.class_level_gained
      - rest.long_rest_completed
      - spell.cast_pending
      capabilities:
      - spellcasting
      - spell_preparation
      - resources
    requires:
    - dnd2024:class_features_ranger_level_1_spellcasting
    citations:
    - SRD 5.2.1 pp. 57, 58; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/ranger_level_1_spellcasting.md
  dnd2024:class_features/ranger_level_1_weapon_mastery:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/ranger_level_1_weapon_mastery
        implementation: dnd2024:class_features_ranger_level_1_weapon_mastery
        parameters:
          mastery_choice:
            count: 2
            category: weapon_kind
            requires_proficiency: true
          grants: chosen_weapons_mastery_properties
          replace:
            event: rest.long_rest_completed
            maximum: all
    contract:
      id: dnd2024:class_features_ranger_level_1_weapon_mastery
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.feature_gained
      - rest.long_rest_completed
      capabilities:
      - weapon_mastery
      - character_choices
    requires:
    - dnd2024:class_features_ranger_level_1_weapon_mastery
    citations:
    - SRD 5.2.1 pp. 58, 59; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/ranger_level_1_weapon_mastery.md
  dnd2024:class_features/ranger_level_2_fighting_style:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/ranger_level_2_fighting_style
        implementation: dnd2024:class_features_ranger_level_2_fighting_style
        parameters:
          choice:
            count: 1
            options:
            - id: fighting_style
              feat_choice:
                category: fighting_style
                count: 1
            - id: druidic_warrior
              cantrip_choice:
                list: druid
                count: 2
              count_as_class_spells: ranger
              ability: wisdom
              replace:
                event: character.ranger_level_gained
                maximum: 1
                list: druid
                spell_level: 0
    contract:
      id: dnd2024:class_features_ranger_level_2_fighting_style
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      - spell_grants
      events:
      - character.feature_gained
      - character.class_level_gained
      capabilities:
      - character_choices
      - feats
      - spellcasting
    requires:
    - dnd2024:class_features_ranger_level_2_fighting_style
    citations:
    - SRD 5.2.1 pp. 59; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/ranger_level_2_fighting_style.md
  dnd2024:class_features/ranger_level_4_ability_score_improvement:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/ranger_level_4_ability_score_improvement
        implementation: dnd2024:class_features_ranger_level_4_ability_score_improvement
        parameters:
          class: ranger
          grant_at_levels:
          - 4
          - 8
          - 12
          - 16
          feat_choice:
            count_per_milestone: 1
            category: any
            require_prerequisites: true
            default:
              key: dnd2024:feats/general_feats_ability_score_improvement
    contract:
      id: dnd2024:class_features_ranger_level_4_ability_score_improvement
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.level_gained
      capabilities:
      - character_choices
      - feats
    requires:
    - dnd2024:class_features_ranger_level_4_ability_score_improvement
    citations:
    - SRD 5.2.1 pp. 59; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/ranger_level_4_ability_score_improvement.md
  dnd2024:class_features/ranger_level_6_roving:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/ranger_level_6_roving
        implementation: dnd2024:class_features_ranger_level_6_roving
        parameters:
          speed_bonus:
            feet: 10
            unless_armor: heavy
          movement_speeds:
            climb: *id094
            swim: *id094
    contract:
      id: dnd2024:class_features_ranger_level_6_roving
      inputs:
        actor: participant
        destination:
          type: location
          required: false
      outputs:
      - feature_state
      - movement
      events:
      - character.feature_gained
      capabilities:
      - movement
    requires:
    - dnd2024:class_features_ranger_level_6_roving
    citations:
    - SRD 5.2.1 pp. 59; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/ranger_level_6_roving.md
  dnd2024:class_features/ranger_level_10_tireless:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/ranger_level_10_tireless
        implementation: dnd2024:class_features_ranger_level_10_tireless
        parameters:
          temporary_hit_points:
            action: magic
            amount:
              op: max
              args:
              - 1
              - op: add
                args:
                - dice:
                    count: 1
                    sides: 8
                - stat: abilities.wisdom.modifier
            uses:
              maximum:
                op: max
                args:
                - 1
                - stat: abilities.wisdom.modifier
              recover:
                event: rest.long_rest_completed
                amount: all
          short_rest:
            exhaustion_reduction: 1
            minimum_exhaustion: 0
    contract:
      id: dnd2024:class_features_ranger_level_10_tireless
      inputs:
        actor: participant
      outputs:
      - feature_state
      - resource_changes
      events:
      - action.magic
      - rest.short_rest_completed
      capabilities:
      - temporary_hit_points
      - resources
      - exhaustion
    requires:
    - dnd2024:class_features_ranger_level_10_tireless
    citations:
    - SRD 5.2.1 pp. 59; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/ranger_level_10_tireless.md
  dnd2024:class_features/ranger_level_14_nature_s_veil:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/ranger_level_14_nature_s_veil
        implementation: dnd2024:class_features_ranger_level_14_nature_s_veil
        parameters:
          action: bonus_action
          condition: invisible
          until: source.next_turn_end
          uses:
            maximum:
              op: max
              args:
              - 1
              - stat: abilities.wisdom.modifier
            recover:
              event: rest.long_rest_completed
              amount: all
    contract:
      id: dnd2024:class_features_ranger_level_14_nature_s_veil
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - resource_changes
      - conditions
      events:
      - action.bonus
      capabilities:
      - conditions
      - resources
    requires:
    - dnd2024:class_features_ranger_level_14_nature_s_veil
    citations:
    - SRD 5.2.1 pp. 59; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/ranger_level_14_nature_s_veil.md
  dnd2024:class_features/ranger_level_18_feral_senses:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/ranger_level_18_feral_senses
        implementation: dnd2024:class_features_ranger_level_18_feral_senses
        parameters:
          blindsight_feet: 30
    contract:
      id: dnd2024:class_features_ranger_level_18_feral_senses
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - character.feature_gained
      capabilities:
      - senses
    requires:
    - dnd2024:class_features_ranger_level_18_feral_senses
    citations:
    - SRD 5.2.1 pp. 59; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/ranger_level_18_feral_senses.md
  dnd2024:class_features/ranger_level_20_foe_slayer:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/ranger_level_20_foe_slayer
        implementation: dnd2024:class_features_ranger_level_20_foe_slayer
        parameters:
          spell:
            key: dnd2024:spells/hunter_s_mark
          replace_damage_die_sides: 10
    contract:
      id: dnd2024:class_features_ranger_level_20_foe_slayer
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - damage
      events:
      - spell.damage_rolled
      capabilities:
      - damage
    requires:
    - dnd2024:class_features_ranger_level_20_foe_slayer
    citations:
    - SRD 5.2.1 pp. 59; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/ranger_level_20_foe_slayer.md
  dnd2024:class_features/hunter_level_3_hunter_s_lore:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/hunter_level_3_hunter_s_lore
        implementation: dnd2024:class_features_hunter_level_3_hunter_s_lore
        parameters:
          while_target_marked_by: own_hunters_mark
          reveal_target_properties:
          - immunities
          - resistances
          - vulnerabilities
          reveal_absence: true
    contract:
      id: dnd2024:class_features_hunter_level_3_hunter_s_lore
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - spell.effect_applied
      - spell.effect_ended
      capabilities:
      - knowledge
      - effects
    requires:
    - dnd2024:class_features_hunter_level_3_hunter_s_lore
    citations:
    - SRD 5.2.1 pp. 61; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/hunter_level_3_hunter_s_lore.md
  dnd2024:class_features/hunter_level_7_defensive_tactics:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/hunter_level_7_defensive_tactics
        implementation: dnd2024:class_features_hunter_level_7_defensive_tactics
        parameters:
          choice:
            count: 1
            options:
            - id: escape_the_horde
              opportunity_attacks_against_self: disadvantage
            - id: multiattack_defense
              trigger: creature.attack_roll_hits_self
              source_further_attacks_against_self: disadvantage
              until: current_turn_end
          replace_choice:
            events:
            - rest.short_rest_completed
            - rest.long_rest_completed
            with: other_option
    contract:
      id: dnd2024:class_features_hunter_level_7_defensive_tactics
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - attack.pending
      - attack.hit
      - rest.short_rest_completed
      - rest.long_rest_completed
      capabilities:
      - character_choices
      - attacks
    requires:
    - dnd2024:class_features_hunter_level_7_defensive_tactics
    citations:
    - SRD 5.2.1 pp. 61; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/hunter_level_7_defensive_tactics.md
  dnd2024:class_features/hunter_level_15_superior_hunter_s_defense:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/hunter_level_15_superior_hunter_s_defense
        implementation: dnd2024:class_features_hunter_level_15_superior_hunter_s_defense
        parameters:
          trigger: damage.taken
          action: reaction
          resistance:
            type: triggering_damage_type
            applies_to_triggering_damage: true
            until: current_turn_end
    contract:
      id: dnd2024:class_features_hunter_level_15_superior_hunter_s_defense
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - damage
      events:
      - damage.taken
      capabilities:
      - damage
      - reactions
    requires:
    - dnd2024:class_features_hunter_level_15_superior_hunter_s_defense
    citations:
    - SRD 5.2.1 pp. 61; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/hunter_level_15_superior_hunter_s_defense.md
  dnd2024:class_features/rogue_level_1_expertise:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/rogue_level_1_expertise
        implementation: dnd2024:class_features_rogue_level_1_expertise
        parameters:
          expertise_choices:
          - rogue_level: 1
            count: 2
          - rogue_level: 6
            count: 2
          eligibility: own_skill_proficiencies
    contract:
      id: dnd2024:class_features_rogue_level_1_expertise
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      - proficiencies
      events:
      - character.class_level_gained
      capabilities:
      - proficiencies
      - character_choices
    requires:
    - dnd2024:class_features_rogue_level_1_expertise
    citations:
    - SRD 5.2.1 pp. 61; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/rogue_level_1_expertise.md
  dnd2024:class_features/rogue_level_1_thieves_cant:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/rogue_level_1_thieves_cant
        implementation: dnd2024:class_features_rogue_level_1_thieves_cant
        parameters:
          language_grants:
          - thieves_cant
          language_choice:
            count: 1
            source: character_creation_language_tables
    contract:
      id: dnd2024:class_features_rogue_level_1_thieves_cant
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.feature_gained
      capabilities:
      - languages
      - character_choices
    requires:
    - dnd2024:class_features_rogue_level_1_thieves_cant
    citations:
    - SRD 5.2.1 pp. 62; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/rogue_level_1_thieves_cant.md
  dnd2024:class_features/rogue_level_2_cunning_action:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/rogue_level_2_cunning_action
        implementation: dnd2024:class_features_rogue_level_2_cunning_action
        parameters:
          turn: own
          action: bonus_action
          choose_action:
          - dash
          - disengage
          - hide
    contract:
      id: dnd2024:class_features_rogue_level_2_cunning_action
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - action.bonus
      capabilities:
      - action_economy
    requires:
    - dnd2024:class_features_rogue_level_2_cunning_action
    citations:
    - SRD 5.2.1 pp. 62; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/rogue_level_2_cunning_action.md
  dnd2024:class_features/rogue_level_3_steady_aim:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/rogue_level_3_steady_aim
        implementation: dnd2024:class_features_rogue_level_3_steady_aim
        parameters:
          action: bonus_action
          requires_movement_so_far_this_turn: 0
          next_attack_roll:
            advantage: true
            expires: current_turn_end
          speed_override:
            feet: 0
            until: current_turn_end
    contract:
      id: dnd2024:class_features_rogue_level_3_steady_aim
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
        destination:
          type: location
          required: false
      outputs:
      - feature_state
      - movement
      events:
      - action.bonus
      capabilities:
      - attacks
      - movement
    requires:
    - dnd2024:class_features_rogue_level_3_steady_aim
    citations:
    - SRD 5.2.1 pp. 62; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/rogue_level_3_steady_aim.md
  dnd2024:class_features/rogue_level_5_cunning_strike:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/rogue_level_5_cunning_strike
        implementation: dnd2024:class_features_rogue_level_5_cunning_strike
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
            - *id089
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
              maximum_feet: *id088
              provokes_opportunity_attacks: false
    contract:
      id: dnd2024:class_features_rogue_level_5_cunning_strike
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
        destination:
          type: location
          required: false
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
    requires:
    - dnd2024:class_features_rogue_level_5_cunning_strike
    citations:
    - SRD 5.2.1 pp. 63; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/rogue_level_5_cunning_strike.md
  dnd2024:class_features/rogue_level_7_evasion:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/rogue_level_7_evasion
        implementation: dnd2024:class_features_rogue_level_7_evasion
        parameters:
          when:
            saving_throw_ability: dexterity
            normal_success_damage_multiplier: 0.5
            incapacitated: false
          damage_multipliers:
            success: 0
            failure: 0.5
    contract:
      id: dnd2024:class_features_rogue_level_7_evasion
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - damage
      events:
      - saving_throw.resolved
      capabilities:
      - damage
      - saving_throws
    requires:
    - dnd2024:class_features_rogue_level_7_evasion
    citations:
    - SRD 5.2.1 pp. 63; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/rogue_level_7_evasion.md
  dnd2024:class_features/rogue_level_11_improved_cunning_strike:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/rogue_level_11_improved_cunning_strike
        implementation: dnd2024:class_features_rogue_level_11_improved_cunning_strike
        parameters:
          feature: cunning_strike
          replace_maximum_effect_choices: 2
          pay_each_effect_die_cost: true
    contract:
      id: dnd2024:class_features_rogue_level_11_improved_cunning_strike
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - sneak_attack.damage_pending
      capabilities:
      - damage_dice_costs
    requires:
    - dnd2024:class_features_rogue_level_11_improved_cunning_strike
    citations:
    - SRD 5.2.1 pp. 63; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/rogue_level_11_improved_cunning_strike.md
  dnd2024:class_features/rogue_level_15_slippery_mind:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/rogue_level_15_slippery_mind
        implementation: dnd2024:class_features_rogue_level_15_slippery_mind
        parameters:
          saving_throw_proficiencies:
          - wisdom
          - charisma
    contract:
      id: dnd2024:class_features_rogue_level_15_slippery_mind
      inputs:
        actor: participant
      outputs:
      - feature_state
      - proficiencies
      events:
      - character.feature_gained
      capabilities:
      - proficiencies
    requires:
    - dnd2024:class_features_rogue_level_15_slippery_mind
    citations:
    - SRD 5.2.1 pp. 63; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/rogue_level_15_slippery_mind.md
  dnd2024:class_features/rogue_level_19_epic_boon:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/rogue_level_19_epic_boon
        implementation: dnd2024:class_features_rogue_level_19_epic_boon
        parameters:
          minimum_class_level: 19
          feat_choice:
            count: 1
            category: any
            require_prerequisites: true
            suggested_category: epic_boon
    contract:
      id: dnd2024:class_features_rogue_level_19_epic_boon
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.level_gained
      capabilities:
      - character_choices
      - feats
    requires:
    - dnd2024:class_features_rogue_level_19_epic_boon
    citations:
    - SRD 5.2.1 pp. 63; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/rogue_level_19_epic_boon.md
  dnd2024:subclasses/rogue_rogue_subclass_thief:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:subclasses/rogue_rogue_subclass_thief
        implementation: dnd2024:subclasses_rogue_rogue_subclass_thief
        parameters:
          class: rogue
          subclass: thief
          feature_grants:
          - feature: &id095
              key: dnd2024:class_features/thief_level_13_use_magic_device
            minimum_class_level: 13
          - feature: &id096
              key: dnd2024:class_features/thief_level_17_thief_s_reflexes
            minimum_class_level: 17
          - feature: &id097
              key: dnd2024:class_features/thief_level_3_fast_hands
            minimum_class_level: 3
          - feature: &id098
              key: dnd2024:class_features/thief_level_3_second_story_work
            minimum_class_level: 3
          - feature: &id099
              key: dnd2024:class_features/thief_level_9_supreme_sneak
            minimum_class_level: 9
      splat_rule_group:
        category: subclass
        members:
        - *id095
        - *id096
        - *id097
        - *id098
        - *id099
    contract:
      id: dnd2024:subclasses_rogue_rogue_subclass_thief
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - character.subclass_selected
      - character.class_level_gained
      capabilities:
      - class_progression
    requires:
    - dnd2024:subclasses_rogue_rogue_subclass_thief
    citations:
    - SRD 5.2.1 pp. 64; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/subclasses/rogue_rogue_subclass_thief.md
  dnd2024:class_features/thief_level_3_second_story_work:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/thief_level_3_second_story_work
        implementation: dnd2024:class_features_thief_level_3_second_story_work
        parameters:
          climb_speed: *id094
          jump_distance_ability_substitution:
            optional: true
            from: strength
            to: dexterity
    contract:
      id: dnd2024:class_features_thief_level_3_second_story_work
      inputs:
        actor: participant
        destination:
          type: location
          required: false
      outputs:
      - feature_state
      - movement
      events:
      - character.feature_gained
      capabilities:
      - movement
    requires:
    - dnd2024:class_features_thief_level_3_second_story_work
    citations:
    - SRD 5.2.1 pp. 64; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/thief_level_3_second_story_work.md
  dnd2024:class_features/thief_level_13_use_magic_device:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/thief_level_13_use_magic_device
        implementation: dnd2024:class_features_thief_level_13_use_magic_device
        parameters:
          maximum_attuned_items: 4
          charged_item_use:
            roll:
              dice:
                count: 1
                sides: 6
            result_6:
              use_property: true
              expend_charges: false
          spell_scrolls:
            allowed_spell_lists: any
            spellcasting_ability: intelligence
            automatic_cast_maximum_spell_level: 1
            higher_levels_check:
              ability: intelligence
              skill: arcana
              dc:
                op: add
                args:
                - 10
                - stat: scroll.spell_level
              success: cast_scroll_spell
              failure: scroll_disintegrates
    contract:
      id: dnd2024:class_features_thief_level_13_use_magic_device
      inputs:
        actor: participant
      outputs:
      - feature_state
      - spell_grants
      events:
      - item.charges_spending_pending
      - scroll.cast_pending
      capabilities:
      - magic_items
      - attunement
      - ability_checks
      - spellcasting
    requires:
    - dnd2024:class_features_thief_level_13_use_magic_device
    citations:
    - SRD 5.2.1 pp. 64; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/thief_level_13_use_magic_device.md
  dnd2024:classes/sorcerer:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:classes/sorcerer
        implementation: dnd2024:classes_sorcerer
        parameters:
          class: sorcerer
          primary_abilities:
          - charisma
          hit_point_die:
            sides: 6
            per_class_level: 1
          saving_throw_proficiencies:
          - constitution
          - charisma
          skill_proficiencies:
            count: 2
            options:
            - arcana
            - deception
            - insight
            - intimidation
            - persuasion
            - religion
          weapon_proficiencies:
          - simple
          armor_training: []
          tool_proficiencies: []
          starting_equipment:
            count: 1
            options:
            - id: a
              grants:
              - item_type: spear
                quantity: 1
              - item_type: dagger
                quantity: 2
              - item_type: arcane_focus_crystal
                quantity: 1
              - item_type: dungeoneer_s_pack
                quantity: 1
              - item_type: gp
                quantity: 28
            - id: b
              grants:
              - item_type: gp
                quantity: 50
          progression:
            key: dnd2024:tables/sorcerer_progression
          feature_grants:
          - feature: &id100
              key: dnd2024:class_features/sorcerer_level_19_epic_boon
            minimum_class_level: 19
          - feature: &id101
              key: dnd2024:class_features/sorcerer_level_1_innate_sorcery
            minimum_class_level: 1
          - feature: &id102
              key: dnd2024:class_features/sorcerer_level_1_spellcasting
            minimum_class_level: 1
          - feature: &id103
              key: dnd2024:class_features/sorcerer_level_20_arcane_apotheosis
            minimum_class_level: 20
          - feature: &id104
              key: dnd2024:class_features/sorcerer_level_2_font_of_magic
            minimum_class_level: 2
          - feature: &id105
              key: dnd2024:class_features/sorcerer_level_2_metamagic
            minimum_class_level: 2
          - feature: &id106
              key: dnd2024:class_features/sorcerer_level_3_sorcerer_subclass
            minimum_class_level: 3
          - feature: &id107
              key: dnd2024:class_features/sorcerer_level_4_ability_score_improvement
            minimum_class_level: 4
          - feature: &id108
              key: dnd2024:class_features/sorcerer_level_5_sorcerous_restoration
            minimum_class_level: 5
          - feature: &id109
              key: dnd2024:class_features/sorcerer_level_7_sorcery_incarnate
            minimum_class_level: 7
      splat_rule_group:
        category: class
        members:
        - *id100
        - *id101
        - *id102
        - *id103
        - *id104
        - *id105
        - *id106
        - *id107
        - *id108
        - *id109
    contract:
      id: dnd2024:classes_sorcerer
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - proficiencies
      - inventory_changes
      events:
      - character.class_selected
      - character.level_gained
      capabilities:
      - character_creation
      - class_progression
      - inventory
      - proficiencies
    requires:
    - dnd2024:classes_sorcerer
    citations:
    - SRD 5.2.1 pp. 64; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/classes/sorcerer.md
  dnd2024:class_features/sorcerer_level_1_innate_sorcery:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/sorcerer_level_1_innate_sorcery
        implementation: dnd2024:class_features_sorcerer_level_1_innate_sorcery
        parameters:
          action: bonus_action
          duration:
            minutes: 1
          uses:
            maximum: 2
            recover:
              event: rest.long_rest_completed
              amount: all
          benefits:
            sorcerer_spell_save_dc_bonus: 1
            sorcerer_spell_attack_advantage: true
    contract:
      id: dnd2024:class_features_sorcerer_level_1_innate_sorcery
      inputs:
        actor: participant
      outputs:
      - feature_state
      - resource_changes
      - spell_grants
      events:
      - action.bonus
      - spell.cast_pending
      capabilities:
      - spellcasting
      - resources
    requires:
    - dnd2024:class_features_sorcerer_level_1_innate_sorcery
    citations:
    - SRD 5.2.1 pp. 65, 66; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/sorcerer_level_1_innate_sorcery.md
  dnd2024:class_features/sorcerer_level_2_metamagic:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/sorcerer_level_2_metamagic
        implementation: dnd2024:class_features_sorcerer_level_2_metamagic
        parameters:
          metamagic_choices:
            count_by_sorcerer_level:
              2: 2
              10: 4
              17: 6
            source:
              key: dnd2024:rules/sorcerer_metamagic_options
            replace:
              event: character.sorcerer_level_gained
              maximum: 1
              new_option_must_be_unknown: true
          pay_option_sorcery_point_cost: true
          maximum_options_per_cast: 1
          option_specific_stacking_exception_allowed: true
    contract:
      id: dnd2024:class_features_sorcerer_level_2_metamagic
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      - resource_changes
      events:
      - character.class_level_gained
      - spell.cast_pending
      capabilities:
      - metamagic
      - resources
      - character_choices
    requires:
    - dnd2024:class_features_sorcerer_level_2_metamagic
    citations:
    - SRD 5.2.1 pp. 66; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/sorcerer_level_2_metamagic.md
  dnd2024:class_features/sorcerer_level_4_ability_score_improvement:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/sorcerer_level_4_ability_score_improvement
        implementation: dnd2024:class_features_sorcerer_level_4_ability_score_improvement
        parameters:
          class: sorcerer
          grant_at_levels:
          - 4
          - 8
          - 12
          - 16
          feat_choice:
            count_per_milestone: 1
            category: any
            require_prerequisites: true
            default:
              key: dnd2024:feats/general_feats_ability_score_improvement
    contract:
      id: dnd2024:class_features_sorcerer_level_4_ability_score_improvement
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.level_gained
      capabilities:
      - character_choices
      - feats
    requires:
    - dnd2024:class_features_sorcerer_level_4_ability_score_improvement
    citations:
    - SRD 5.2.1 pp. 66; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/sorcerer_level_4_ability_score_improvement.md
  dnd2024:class_features/sorcerer_level_7_sorcery_incarnate:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/sorcerer_level_7_sorcery_incarnate
        implementation: dnd2024:class_features_sorcerer_level_7_sorcery_incarnate
        parameters:
          innate_sorcery_alternative_cost:
            requires_remaining_uses: 0
            sorcery_points: 2
            action: same_activation_bonus_action
          while_innate_sorcery_active:
            maximum_metamagic_options_per_cast: 2
    contract:
      id: dnd2024:class_features_sorcerer_level_7_sorcery_incarnate
      inputs:
        actor: participant
      outputs:
      - feature_state
      - resource_changes
      events:
      - action.bonus
      - spell.cast_pending
      capabilities:
      - metamagic
      - resources
    requires:
    - dnd2024:class_features_sorcerer_level_7_sorcery_incarnate
    citations:
    - SRD 5.2.1 pp. 66; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/sorcerer_level_7_sorcery_incarnate.md
  dnd2024:class_features/sorcerer_level_20_arcane_apotheosis:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/sorcerer_level_20_arcane_apotheosis
        implementation: dnd2024:class_features_sorcerer_level_20_arcane_apotheosis
        parameters:
          requires_innate_sorcery_active: true
          free_metamagic:
            maximum_options: 1
            period: each_own_turn
            sorcery_point_cost: 0
    contract:
      id: dnd2024:class_features_sorcerer_level_20_arcane_apotheosis
      inputs:
        actor: participant
      outputs:
      - feature_state
      - resource_changes
      events:
      - spell.metamagic_applied
      capabilities:
      - metamagic
      - resources
    requires:
    - dnd2024:class_features_sorcerer_level_20_arcane_apotheosis
    citations:
    - SRD 5.2.1 pp. 66; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/sorcerer_level_20_arcane_apotheosis.md
  dnd2024:class_features/draconic_sorcery_level_3_draconic_resilience:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/draconic_sorcery_level_3_draconic_resilience
        implementation: dnd2024:class_features_draconic_sorcery_level_3_draconic_resilience
        parameters:
          hit_point_maximum_bonus:
            at_sorcerer_level_3: 3
            per_subsequent_sorcerer_level: 1
          base_ac:
            when_armor_worn: false
            value:
              op: add
              args:
              - 10
              - stat: abilities.dexterity.modifier
              - stat: abilities.charisma.modifier
    contract:
      id: dnd2024:class_features_draconic_sorcery_level_3_draconic_resilience
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - character.class_level_gained
      - armor_class.calculated
      capabilities:
      - hit_points
      - armor_class
    requires:
    - dnd2024:class_features_draconic_sorcery_level_3_draconic_resilience
    citations:
    - SRD 5.2.1 pp. 69; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/draconic_sorcery_level_3_draconic_resilience.md
  dnd2024:class_features/draconic_sorcery_level_6_elemental_affinity:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/draconic_sorcery_level_6_elemental_affinity
        implementation: dnd2024:class_features_draconic_sorcery_level_6_elemental_affinity
        parameters:
          damage_type_choice:
            count: 1
            options:
            - acid
            - cold
            - fire
            - lightning
            - poison
          resistance: chosen_damage_type
          spell_damage_bonus:
            requires_spell_deals_chosen_type: true
            maximum_rolls_per_spell: 1
            bonus:
              stat: abilities.charisma.modifier
            optional: true
    contract:
      id: dnd2024:class_features_draconic_sorcery_level_6_elemental_affinity
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - choices
      - damage
      events:
      - character.feature_gained
      - spell.damage_rolled
      capabilities:
      - damage
      - resistances
      - character_choices
    requires:
    - dnd2024:class_features_draconic_sorcery_level_6_elemental_affinity
    citations:
    - SRD 5.2.1 pp. 70; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/draconic_sorcery_level_6_elemental_affinity.md
  dnd2024:class_features/draconic_sorcery_level_18_dragon_companion:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/draconic_sorcery_level_18_dragon_companion
        implementation: dnd2024:class_features_draconic_sorcery_level_18_dragon_companion
        parameters:
          spell:
            key: dnd2024:spells/summon_dragon
          material_component_required: false
          free_cast:
            spell_slot_cost: 0
            uses:
              maximum: 1
              recover:
                event: rest.long_rest_completed
                amount: all
          optional_cast_modification:
            timing: casting_started
            requires_concentration: false
            duration:
              minutes: 1
    contract:
      id: dnd2024:class_features_draconic_sorcery_level_18_dragon_companion
      inputs:
        actor: participant
      outputs:
      - feature_state
      - resource_changes
      - spell_grants
      events:
      - spell.cast_started
      capabilities:
      - spellcasting
      - concentration
      - resources
    requires:
    - dnd2024:class_features_draconic_sorcery_level_18_dragon_companion
    citations:
    - SRD 5.2.1 pp. 70; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/draconic_sorcery_level_18_dragon_companion.md
  dnd2024:class_features/warlock_level_1_eldritch_invocations:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/warlock_level_1_eldritch_invocations
        implementation: dnd2024:class_features_warlock_level_1_eldritch_invocations
        parameters:
          invocation_choice:
            source:
              key: dnd2024:rules/warlock_eldritch_invocation_options
            initial_count: 1
            count:
              table:
                key: dnd2024:tables/warlock_progression
              row:
                stat: classes.warlock.level
              column: eldritch_invocations
            requires_prerequisites: true
            repeat_only_if_option_repeatable: true
          replace:
            event: character.warlock_level_gained
            maximum: 1
            new_requires_prerequisites: true
            cannot_remove_if_prerequisite_of_retained_invocation: true
    contract:
      id: dnd2024:class_features_warlock_level_1_eldritch_invocations
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.class_level_gained
      capabilities:
      - character_choices
      - invocations
    requires:
    - dnd2024:class_features_warlock_level_1_eldritch_invocations
    citations:
    - SRD 5.2.1 pp. 70, 71; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/warlock_level_1_eldritch_invocations.md
  dnd2024:class_features/warlock_level_2_magical_cunning:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/warlock_level_2_magical_cunning
        implementation: dnd2024:class_features_warlock_level_2_magical_cunning
        parameters:
          activity_duration:
            minutes: 1
          on_completion:
            recover_resource: pact_magic_spell_slots
            maximum_recovered:
              op: ceil
              args:
              - op: divide
                args:
                - stat: pact_magic.maximum_slots
                - 2
          uses:
            maximum: 1
            recover:
              event: rest.long_rest_completed
              amount: all
    contract:
      id: dnd2024:class_features_warlock_level_2_magical_cunning
      inputs:
        actor: participant
      outputs:
      - feature_state
      - resource_changes
      events:
      - activity.completed
      capabilities:
      - resources
      - spell_slots
    requires:
    - dnd2024:class_features_warlock_level_2_magical_cunning
    citations:
    - SRD 5.2.1 pp. 72; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/warlock_level_2_magical_cunning.md
  dnd2024:class_features/warlock_level_4_ability_score_improvement:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/warlock_level_4_ability_score_improvement
        implementation: dnd2024:class_features_warlock_level_4_ability_score_improvement
        parameters:
          class: warlock
          grant_at_levels:
          - 4
          - 8
          - 12
          - 16
          feat_choice:
            count_per_milestone: 1
            category: any
            require_prerequisites: true
            default:
              key: dnd2024:feats/general_feats_ability_score_improvement
    contract:
      id: dnd2024:class_features_warlock_level_4_ability_score_improvement
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.level_gained
      capabilities:
      - character_choices
      - feats
    requires:
    - dnd2024:class_features_warlock_level_4_ability_score_improvement
    citations:
    - SRD 5.2.1 pp. 72; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/warlock_level_4_ability_score_improvement.md
  dnd2024:class_features/warlock_level_11_mystic_arcanum:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/warlock_level_11_mystic_arcanum
        implementation: dnd2024:class_features_warlock_level_11_mystic_arcanum
        parameters:
          arcanum_choices:
          - warlock_level: 11
            spell_level: 6
          - warlock_level: 13
            spell_level: 7
          - warlock_level: 15
            spell_level: 8
          - warlock_level: 17
            spell_level: 9
          count_each: 1
          spell_list: warlock
          each_spell:
            spell_slot_cost: 0
            uses:
              maximum: 1
              recover:
                event: rest.long_rest_completed
                amount: all
          replace:
            event: character.warlock_level_gained
            maximum: 1
            same_spell_level: true
            list: warlock
    contract:
      id: dnd2024:class_features_warlock_level_11_mystic_arcanum
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      - resource_changes
      - spell_grants
      events:
      - character.class_level_gained
      - spell.cast_pending
      - rest.long_rest_completed
      capabilities:
      - spellcasting
      - resources
      - character_choices
    requires:
    - dnd2024:class_features_warlock_level_11_mystic_arcanum
    citations:
    - SRD 5.2.1 pp. 72; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/warlock_level_11_mystic_arcanum.md
  dnd2024:class_features/warlock_level_20_eldritch_master:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/warlock_level_20_eldritch_master
        implementation: dnd2024:class_features_warlock_level_20_eldritch_master
        parameters:
          feature: magical_cunning
          replace_recovery_amount: all_expended_pact_magic_slots
    contract:
      id: dnd2024:class_features_warlock_level_20_eldritch_master
      inputs:
        actor: participant
      outputs:
      - feature_state
      - resource_changes
      events:
      - magical_cunning.completed
      capabilities:
      - resources
    requires:
    - dnd2024:class_features_warlock_level_20_eldritch_master
    citations:
    - SRD 5.2.1 pp. 72; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/warlock_level_20_eldritch_master.md
  dnd2024:class_features/fiend_patron_level_3_dark_one_s_blessing:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/fiend_patron_level_3_dark_one_s_blessing
        implementation: dnd2024:class_features_fiend_patron_level_3_dark_one_s_blessing
        parameters:
          triggers:
          - enemy_reduced_to_zero_by: self
          - enemy_reduced_to_zero_by: other
            enemy_distance_max_feet: 10
          temporary_hit_points:
            op: max
            args:
            - 1
            - op: add
              args:
              - stat: abilities.charisma.modifier
              - stat: classes.warlock.level
    contract:
      id: dnd2024:class_features_fiend_patron_level_3_dark_one_s_blessing
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - creature.hit_points_became_zero
      capabilities:
      - temporary_hit_points
    requires:
    - dnd2024:class_features_fiend_patron_level_3_dark_one_s_blessing
    citations:
    - SRD 5.2.1 pp. 76; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/fiend_patron_level_3_dark_one_s_blessing.md
  dnd2024:class_features/fiend_patron_level_6_dark_one_s_own_luck:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/fiend_patron_level_6_dark_one_s_own_luck
        implementation: dnd2024:class_features_fiend_patron_level_6_dark_one_s_own_luck
        parameters:
          roll_types:
          - ability_check
          - saving_throw
          timing: after_roll_before_effects
          optional_bonus:
            dice:
              count: 1
              sides: 10
          maximum_uses_per_roll: 1
          uses:
            maximum:
              op: max
              args:
              - 1
              - stat: abilities.charisma.modifier
            recover:
              event: rest.long_rest_completed
              amount: all
    contract:
      id: dnd2024:class_features_fiend_patron_level_6_dark_one_s_own_luck
      inputs:
        actor: participant
      outputs:
      - feature_state
      - resource_changes
      events:
      - ability_check.rolled
      - saving_throw.rolled
      capabilities:
      - roll_modifiers
      - resources
    requires:
    - dnd2024:class_features_fiend_patron_level_6_dark_one_s_own_luck
    citations:
    - SRD 5.2.1 pp. 76; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/fiend_patron_level_6_dark_one_s_own_luck.md
  dnd2024:class_features/fiend_patron_level_14_hurl_through_hell:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/fiend_patron_level_14_hurl_through_hell
        implementation: dnd2024:class_features_fiend_patron_level_14_hurl_through_hell
        parameters:
          trigger: attack_roll.hit_creature
          limit:
            count: 1
            period: turn
          save:
            ability: charisma
            dc:
              stat: spell_save_dc
            success: no_effect
            failure:
              banish_to: lower_planes
              damage:
                roll:
                  dice:
                    count: 8
                    sides: 10
                type: psychic
                unless_creature_type: fiend
              condition: incapacitated
              until: source.next_turn_end
              return:
                destination: previous_space
                if_occupied: nearest_unoccupied_space
          uses:
            maximum: 1
            recover:
              event: rest.long_rest_completed
              amount: all
          restore_use:
            resource: pact_magic_spell_slot
            amount: 1
            action: none
    contract:
      id: dnd2024:class_features_fiend_patron_level_14_hurl_through_hell
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - resource_changes
      - damage
      - conditions
      events:
      - attack.hit
      - turn.ended
      capabilities:
      - planar_travel
      - conditions
      - damage
      - saving_throws
      - resources
    requires:
    - dnd2024:class_features_fiend_patron_level_14_hurl_through_hell
    citations:
    - SRD 5.2.1 pp. 76; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/fiend_patron_level_14_hurl_through_hell.md
  dnd2024:class_features/wizard_level_1_spellcasting:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/wizard_level_1_spellcasting
        implementation: dnd2024:class_features_wizard_level_1_spellcasting
        parameters:
          class: wizard
          spell_list: wizard
          ability: intelligence
          focus_types:
          - arcane_focus
          - own_spellbook
          slots:
            progression:
              key: dnd2024:tables/wizard_progression
            columns_by_spell_level:
              1: spell_slots_1
              2: spell_slots_2
              3: spell_slots_3
              4: spell_slots_4
              5: spell_slots_5
              6: spell_slots_6
              7: spell_slots_7
              8: spell_slots_8
              9: spell_slots_9
            recovery:
              event: rest.long_rest_completed
              amount: all_expended
          prepared_spells:
            initial_count: 4
            initial_spell_level: 1
            count:
              table:
                key: dnd2024:tables/wizard_progression
              row:
                stat: classes.wizard.level
              column: prepared_spells
            choose_on_count_increase: true
            level_must_have_slot: true
            replace:
              event: rest.long_rest_completed
              maximum: all
              select_from: own_spellbook
            other_class_feature_always_prepared:
              counts_toward_limit: false
              counts_as_class_spell: true
            select_from: own_spellbook
          cantrips:
            initial_count: 3
            spell_level: 0
            count:
              table:
                key: dnd2024:tables/wizard_progression
              row:
                stat: classes.wizard.level
              column: cantrips
            gain_extra_at_levels:
            - 4
            - 10
            replace:
              event: rest.long_rest_completed
              maximum: 1
          spellbook:
            size: tiny
            weight_lb: 3
            pages: 100
            readers:
            - owner
            - identify_caster
            appearance_and_materials:
              authority: player
              type: cosmetic_choice
            initial_spells:
              count: 6
              list: wizard
              spell_level: 1
            add_per_wizard_level_after_1:
              count: 2
              list: wizard
              level_must_have_slot_in_wizard_table: true
            contains: known_level_1_plus_spells
    contract:
      id: dnd2024:class_features_wizard_level_1_spellcasting
      inputs:
        actor: participant
      outputs:
      - feature_state
      - resource_changes
      - spell_grants
      events:
      - character.class_level_gained
      - rest.long_rest_completed
      - spell.cast_pending
      capabilities:
      - spellcasting
      - spellbooks
      - spell_preparation
      - resources
    requires:
    - dnd2024:class_features_wizard_level_1_spellcasting
    citations:
    - SRD 5.2.1 pp. 77, 78; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/wizard_level_1_spellcasting.md
  dnd2024:class_features/wizard_level_1_arcane_recovery:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/wizard_level_1_arcane_recovery
        implementation: dnd2024:class_features_wizard_level_1_arcane_recovery
        parameters:
          requires_activity: study_own_spellbook_during_short_rest
          recover_choice:
            resource: expended_spell_slots
            maximum_slot_level: 5
            maximum_combined_slot_levels:
              op: ceil
              args:
              - op: divide
                args:
                - stat: classes.wizard.level
                - 2
          uses:
            maximum: 1
            recover:
              event: rest.long_rest_completed
              amount: all
    contract:
      id: dnd2024:class_features_wizard_level_1_arcane_recovery
      inputs:
        actor: participant
      outputs:
      - feature_state
      - resource_changes
      events:
      - rest.short_rest_completed
      capabilities:
      - spell_slots
      - resources
    requires:
    - dnd2024:class_features_wizard_level_1_arcane_recovery
    citations:
    - SRD 5.2.1 pp. 78; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/wizard_level_1_arcane_recovery.md
  dnd2024:class_features/wizard_level_3_wizard_subclass:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/wizard_level_3_wizard_subclass
        implementation: dnd2024:class_features_wizard_level_3_wizard_subclass
        parameters:
          subclass_choice:
            count: 1
            class: wizard
            srd_options:
            - key: dnd2024:subclasses/wizard_wizard_subclass_evoker
          grant_features:
            from: selected_subclass
            when_required_class_level_lte_current: true
            reevaluate_on_class_level_gain: true
          spellbook_copying_sidebar:
            copy_found_spell:
              list: wizard
              minimum_spell_level: 1
              must_be_level_can_prepare: true
              hours_per_spell_level: 2
              gp_per_spell_level: 50
              result: add_to_spellbook
            copy_own_book_spell:
              hours_per_spell_level: 1
              gp_per_spell_level: 10
              result: add_to_destination_spellbook
            replace_lost_book:
              eligible_spells: currently_prepared_wizard_spells
              hours_per_spell_level: 1
              gp_per_spell_level: 10
              other_spells_require_rediscovery: true
    contract:
      id: dnd2024:class_features_wizard_level_3_wizard_subclass
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
        spellbook_operation:
          type: choice
          required: false
        spells:
          type: definition_list
          required: false
      outputs:
      - feature_state
      - choices
      - inventory_changes
      events:
      - character.feature_gained
      - character.level_gained
      capabilities:
      - character_choices
      - class_progression
      - spellbooks
      - downtime
      - inventory
    requires:
    - dnd2024:class_features_wizard_level_3_wizard_subclass
    citations:
    - SRD 5.2.1 pp. 78, 79; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/wizard_level_3_wizard_subclass.md
    - SRD 5.2.1 p. 78; builtin/srd-5.2.1/source.txt (full page layout)
  dnd2024:class_features/wizard_level_5_memorize_spell:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/wizard_level_5_memorize_spell
        implementation: dnd2024:class_features_wizard_level_5_memorize_spell
        parameters:
          requires: study_own_spellbook
          replace_prepared_spells:
            maximum: 1
            feature: wizard_spellcasting
            minimum_spell_level: 1
            replacement_from: own_spellbook
            replacement_minimum_level: 1
    contract:
      id: dnd2024:class_features_wizard_level_5_memorize_spell
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - rest.short_rest_completed
      capabilities:
      - spell_preparation
    requires:
    - dnd2024:class_features_wizard_level_5_memorize_spell
    citations:
    - SRD 5.2.1 pp. 79; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/wizard_level_5_memorize_spell.md
  dnd2024:class_features/wizard_level_19_epic_boon:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/wizard_level_19_epic_boon
        implementation: dnd2024:class_features_wizard_level_19_epic_boon
        parameters:
          minimum_class_level: 19
          feat_choice:
            count: 1
            category: any
            require_prerequisites: true
            suggested_category: epic_boon
    contract:
      id: dnd2024:class_features_wizard_level_19_epic_boon
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      events:
      - character.level_gained
      capabilities:
      - character_choices
      - feats
    requires:
    - dnd2024:class_features_wizard_level_19_epic_boon
    citations:
    - SRD 5.2.1 pp. 79; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/wizard_level_19_epic_boon.md
  dnd2024:subclasses/wizard_wizard_subclass_evoker:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:subclasses/wizard_wizard_subclass_evoker
        implementation: dnd2024:subclasses_wizard_wizard_subclass_evoker
        parameters:
          class: wizard
          subclass: evoker
          feature_grants:
          - feature: &id110
              key: dnd2024:class_features/evoker_level_10_empowered_evocation
            minimum_class_level: 10
          - feature: &id111
              key: dnd2024:class_features/evoker_level_14_overchannel
            minimum_class_level: 14
          - feature: &id112
              key: dnd2024:class_features/evoker_level_3_evocation_savant
            minimum_class_level: 3
          - feature: &id113
              key: dnd2024:class_features/evoker_level_3_potent_cantrip
            minimum_class_level: 3
          - feature: &id114
              key: dnd2024:class_features/evoker_level_6_sculpt_spells
            minimum_class_level: 6
      splat_rule_group:
        category: subclass
        members:
        - *id110
        - *id111
        - *id112
        - *id113
        - *id114
    contract:
      id: dnd2024:subclasses_wizard_wizard_subclass_evoker
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - character.subclass_selected
      - character.class_level_gained
      capabilities:
      - class_progression
    requires:
    - dnd2024:subclasses_wizard_wizard_subclass_evoker
    citations:
    - SRD 5.2.1 pp. 82; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/subclasses/wizard_wizard_subclass_evoker.md
  dnd2024:class_features/evoker_level_3_potent_cantrip:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/evoker_level_3_potent_cantrip
        implementation: dnd2024:class_features_evoker_level_3_potent_cantrip
        parameters:
          spell_level: 0
          target_type: creature
          triggers:
          - cantrip_attack_misses
          - target_succeeds_cantrip_save
          damage_multiplier: 0.5
          additional_cantrip_effects: false
          requires_cantrip_damage: true
    contract:
      id: dnd2024:class_features_evoker_level_3_potent_cantrip
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - damage
      events:
      - spell.attack_missed
      - spell.save_succeeded
      capabilities:
      - damage
    requires:
    - dnd2024:class_features_evoker_level_3_potent_cantrip
    citations:
    - SRD 5.2.1 pp. 82; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/evoker_level_3_potent_cantrip.md
  dnd2024:class_features/evoker_level_10_empowered_evocation:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:class_features/evoker_level_10_empowered_evocation
        implementation: dnd2024:class_features_evoker_level_10_empowered_evocation
        parameters:
          spell_class: wizard
          spell_school: evocation
          optional_bonus:
            stat: abilities.intelligence.modifier
          maximum_damage_rolls_per_spell: 1
    contract:
      id: dnd2024:class_features_evoker_level_10_empowered_evocation
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - damage
      events:
      - spell.damage_rolled
      capabilities:
      - damage
    requires:
    - dnd2024:class_features_evoker_level_10_empowered_evocation
    citations:
    - SRD 5.2.1 pp. 82; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/class_features/evoker_level_10_empowered_evocation.md
  dnd2024:backgrounds/acolyte:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:backgrounds/acolyte
        implementation: dnd2024:backgrounds_acolyte
        parameters:
          ability_score_choices:
            eligible:
            - intelligence
            - wisdom
            - charisma
            allocations:
            - - 2
              - 1
            - - 1
              - 1
              - 1
            distinct_abilities: true
            maximum_score: 20
          feat_grant:
            definition:
              key: dnd2024:feats/origin_feats_magic_initiate
            spell_list: cleric
          skill_proficiencies:
          - insight
          - religion
          tool_proficiencies:
          - calligraphers_supplies
          equipment_choice:
            count: 1
            options:
            - id: a
              grants:
              - item_type: calligraphers_supplies
                quantity: 1
              - item_type: book_prayers
                quantity: 1
              - item_type: holy_symbol
                quantity: 1
              - item_type: parchment_sheet
                quantity: 10
              - item_type: robe
                quantity: 1
              - item_type: gp
                quantity: 8
            - id: b
              grants:
              - item_type: gp
                quantity: 50
    contract:
      id: dnd2024:backgrounds_acolyte
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - proficiencies
      - inventory_changes
      events:
      - character.background_selected
      capabilities:
      - character_creation
      - ability_scores
      - proficiencies
      - feats
      - inventory
    requires:
    - dnd2024:backgrounds_acolyte
    citations:
    - SRD 5.2.1 pp. 83; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/backgrounds/acolyte.md
  dnd2024:backgrounds/sage:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:backgrounds/sage
        implementation: dnd2024:backgrounds_sage
        parameters:
          ability_score_choices:
            eligible:
            - constitution
            - intelligence
            - wisdom
            allocations:
            - - 2
              - 1
            - - 1
              - 1
              - 1
            distinct_abilities: true
            maximum_score: 20
          feat_grant:
            definition:
              key: dnd2024:feats/origin_feats_magic_initiate
            spell_list: wizard
          skill_proficiencies:
          - arcana
          - history
          tool_proficiencies:
          - calligraphers_supplies
          equipment_choice:
            count: 1
            options:
            - id: a
              grants:
              - item_type: quarterstaff
                quantity: 1
              - item_type: calligraphers_supplies
                quantity: 1
              - item_type: book_history
                quantity: 1
              - item_type: parchment_sheet
                quantity: 8
              - item_type: robe
                quantity: 1
              - item_type: gp
                quantity: 8
            - id: b
              grants:
              - item_type: gp
                quantity: 50
    contract:
      id: dnd2024:backgrounds_sage
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - proficiencies
      - inventory_changes
      events:
      - character.background_selected
      capabilities:
      - character_creation
      - ability_scores
      - proficiencies
      - feats
      - inventory
    requires:
    - dnd2024:backgrounds_sage
    citations:
    - SRD 5.2.1 pp. 83; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/backgrounds/sage.md
  dnd2024:species/dragonborn:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:species/dragonborn
        implementation: dnd2024:species_dragonborn
        parameters:
          creature_type: humanoid
          size: medium
          height_feet_approximate:
            minimum: 5
            maximum: 7
          speed_feet: 30
          ancestry_choice:
            count: 1
            options:
            - dragon: black
              damage_type: acid
            - dragon: blue
              damage_type: lightning
            - dragon: brass
              damage_type: fire
            - dragon: bronze
              damage_type: lightning
            - dragon: copper
              damage_type: acid
            - dragon: gold
              damage_type: fire
            - dragon: green
              damage_type: poison
            - dragon: red
              damage_type: fire
            - dragon: silver
              damage_type: cold
            - dragon: white
              damage_type: cold
          breath_weapon:
            action: replace_one_attack_in_own_turn_attack_action
            magical: true
            area_choice_each_use:
            - shape: cone
              length_feet: 15
            - shape: line
              length_feet: 30
              width_feet: 5
            targets: all_creatures_in_area
            save:
              ability: dexterity
              dc:
                op: add
                args:
                - 8
                - stat: abilities.constitution.modifier
                - *id089
              success_damage_multiplier: 0.5
            damage_type: ancestry_damage_type
            dice_by_character_level:
              1:
                dice:
                  count: 1
                  sides: 10
              5:
                dice:
                  count: 2
                  sides: 10
              11:
                dice:
                  count: 3
                  sides: 10
              17:
                dice:
                  count: 4
                  sides: 10
            uses:
              maximum: *id089
              recover:
                event: rest.long_rest_completed
                amount: all
          damage_resistance: ancestry_damage_type
          darkvision_feet: 60
          draconic_flight:
            minimum_character_level: 5
            action: bonus_action
            fly_speed: *id094
            duration:
              minutes: 10
            ends_on_condition: incapacitated
            retract_action: none
            uses:
              maximum: 1
              recover:
                event: rest.long_rest_completed
                amount: all
    contract:
      id: dnd2024:species_dragonborn
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
        destination:
          type: location
          required: false
      outputs:
      - feature_state
      - resource_changes
      - damage
      - movement
      events:
      - character.species_selected
      - action.attack
      - action.bonus
      - rest.long_rest_completed
      capabilities:
      - character_creation
      - damage
      - saving_throws
      - resources
      - movement
      - senses
    requires:
    - dnd2024:species_dragonborn
    citations:
    - SRD 5.2.1 pp. 84; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/species/dragonborn.md
  dnd2024:species/elf:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:species/elf
        implementation: dnd2024:species_elf
        parameters:
          creature_type: humanoid
          size: medium
          height_feet_approximate:
            minimum: 5
            maximum: 6
          speed_feet: 30
          darkvision_feet: 60
          elven_lineage:
            choice:
              count: 1
              options:
              - id: drow
                level_1:
                  darkvision_feet: 120
                  cantrip:
                    key: dnd2024:spells/dancing_lights
                level_3_spell:
                  key: dnd2024:spells/faerie_fire
                level_5_spell:
                  key: dnd2024:spells/darkness
              - id: high_elf
                level_1:
                  cantrip:
                    key: dnd2024:spells/prestidigitation
                  replace_cantrip:
                    event: rest.long_rest_completed
                    list: wizard
                    maximum: 1
                level_3_spell:
                  key: dnd2024:spells/detect_magic
                level_5_spell:
                  key: dnd2024:spells/misty_step
              - id: wood_elf
                level_1:
                  speed_feet: 35
                  cantrip:
                    key: dnd2024:spells/druidcraft
                level_3_spell:
                  key: dnd2024:spells/longstrider
                level_5_spell:
                  key: dnd2024:spells/pass_without_trace
            level_basis: character_level
            spellcasting_ability_choice:
              count: 1
              options:
              - intelligence
              - wisdom
              - charisma
            leveled_spell_grants:
              always_prepared: true
              each_spell_free_cast:
                spell_slot_cost: 0
                uses:
                  maximum: 1
                  recover:
                    event: rest.long_rest_completed
                    amount: all
              can_use_appropriate_spell_slots: true
          fey_ancestry:
            saving_throw_advantage:
              avoid_or_end_condition: charmed
          keen_senses:
            skill_proficiency_choice:
              count: 1
              options:
              - insight
              - perception
              - survival
          trance:
            sleep_required: false
            magical_sleep_immune: true
            long_rest_hours: 4
            required_activity: trancelike_meditation
            conscious_during_rest: true
    contract:
      id: dnd2024:species_elf
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - spell_grants
      - proficiencies
      events:
      - character.species_selected
      - character.level_gained
      - rest.long_rest_completed
      - saving_throw.pending
      capabilities:
      - character_creation
      - spellcasting
      - senses
      - proficiencies
      - rest
    requires:
    - dnd2024:species_elf
    citations:
    - SRD 5.2.1 pp. 84, 85; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/species/elf.md
    - SRD 5.2.1 p. 85; builtin/srd-5.2.1/source.txt (full page layout)
  dnd2024:species/goliath:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:species/goliath
        implementation: dnd2024:species_goliath
        parameters:
          creature_type: humanoid
          size: medium
          height_feet_approximate:
            minimum: 7
            maximum: 8
          speed_feet: 35
          giant_ancestry:
            choice:
              count: 1
              options:
              - id: clouds_jaunt
                action: bonus_action
                teleport:
                  maximum_feet: 30
                  destination_visible: true
                  destination_unoccupied: true
                  magical: true
              - id: fires_burn
                trigger: attack_roll_hit_and_damage
                extra_damage:
                  roll:
                    dice:
                      count: 1
                      sides: 10
                  type: fire
              - id: frosts_chill
                trigger: attack_roll_hit_and_damage
                extra_damage:
                  roll:
                    dice:
                      count: 1
                      sides: 6
                  type: cold
                target_speed_reduction_feet: 10
                until: source.next_turn_start
              - id: hills_tumble
                trigger: attack_roll_hit_and_damage
                target_type: creature
                maximum_size: large
                condition: prone
              - id: stones_endurance
                trigger: damage.taken
                action: reaction
                damage_reduction:
                  op: add
                  args:
                  - dice:
                      count: 1
                      sides: 12
                  - stat: abilities.constitution.modifier
              - id: storms_thunder
                trigger: damage.taken_from_creature
                source_maximum_distance_feet: 60
                action: reaction
                damage_to_source:
                  roll:
                    dice:
                      count: 1
                      sides: 8
                  type: thunder
            uses:
              maximum: *id089
              recover:
                event: rest.long_rest_completed
                amount: all
          large_form:
            minimum_character_level: 5
            action: bonus_action
            requires_space_for_size: large
            size: large
            duration:
              minutes: 10
            end_action: none
            strength_check_advantage: true
            speed_bonus_feet: 10
            uses:
              maximum: 1
              recover:
                event: rest.long_rest_completed
                amount: all
          powerful_build:
            advantage_on_checks_to_end_condition: grappled
            carrying_capacity_size_categories_bonus: 1
    contract:
      id: dnd2024:species_goliath
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
        destination:
          type: location
          required: false
      outputs:
      - feature_state
      - resource_changes
      - damage
      - movement
      - conditions
      events:
      - character.species_selected
      - action.bonus
      - attack.damage_completed
      - damage.taken
      - ability_check.pending
      capabilities:
      - character_creation
      - teleportation
      - damage
      - conditions
      - resources
      - movement
      - carrying_capacity
    requires:
    - dnd2024:species_goliath
    citations:
    - SRD 5.2.1 pp. 85, 86; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/species/goliath.md
  dnd2024:species/human:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:species/human
        implementation: dnd2024:species_human
        parameters:
          creature_type: humanoid
          size_choice:
            count: 1
            options:
            - size: medium
              height_feet_approximate:
                minimum: 4
                maximum: 7
            - size: small
              height_feet_approximate:
                minimum: 2
                maximum: 4
          speed_feet: 30
          resourceful:
            event: rest.long_rest_completed
            grant: heroic_inspiration
          skillful:
            proficiency_choice:
              count: 1
              category: any_skill
          versatile:
            feat_choice:
              count: 1
              category: origin
    contract:
      id: dnd2024:species_human
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - proficiencies
      events:
      - character.species_selected
      - rest.long_rest_completed
      capabilities:
      - character_creation
      - inspiration
      - proficiencies
      - feats
    requires:
    - dnd2024:species_human
    citations:
    - SRD 5.2.1 pp. 86; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/species/human.md
    - SRD 5.2.1 p. 86; builtin/srd-5.2.1/source.txt (full page layout)
  dnd2024:species/tiefling:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:species/tiefling
        implementation: dnd2024:species_tiefling
        parameters:
          creature_type: humanoid
          size_choice:
            count: 1
            options:
            - size: medium
              height_feet_approximate:
                minimum: 4
                maximum: 7
            - size: small
              height_feet_approximate:
                minimum: 3
                maximum: 4
          speed_feet: 30
          darkvision_feet: 60
          fiendish_legacy:
            choice:
              count: 1
              options:
              - id: abyssal
                level_1:
                  damage_resistance: poison
                  cantrip:
                    key: dnd2024:spells/poison_spray
                level_3_spell:
                  key: dnd2024:spells/ray_of_sickness
                level_5_spell:
                  key: dnd2024:spells/hold_person
              - id: chthonic
                level_1:
                  damage_resistance: necrotic
                  cantrip:
                    key: dnd2024:spells/chill_touch
                level_3_spell:
                  key: dnd2024:spells/false_life
                level_5_spell:
                  key: dnd2024:spells/ray_of_enfeeblement
              - id: infernal
                level_1:
                  damage_resistance: fire
                  cantrip:
                    key: dnd2024:spells/fire_bolt
                level_3_spell:
                  key: dnd2024:spells/hellish_rebuke
                level_5_spell:
                  key: dnd2024:spells/darkness
            level_basis: character_level
            spellcasting_ability_choice:
              count: 1
              options:
              - intelligence
              - wisdom
              - charisma
            leveled_spell_grants:
              always_prepared: true
              each_spell_free_cast:
                spell_slot_cost: 0
                uses:
                  maximum: 1
                  recover:
                    event: rest.long_rest_completed
                    amount: all
              can_use_appropriate_spell_slots: true
          otherworldly_presence:
            cantrip:
              key: dnd2024:spells/thaumaturgy
            ability: chosen_fiendish_legacy_spellcasting_ability
    contract:
      id: dnd2024:species_tiefling
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - spell_grants
      events:
      - character.species_selected
      - character.level_gained
      - rest.long_rest_completed
      capabilities:
      - character_creation
      - spellcasting
      - senses
      - resistances
    requires:
    - dnd2024:species_tiefling
    citations:
    - SRD 5.2.1 pp. 86; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/species/tiefling.md
    - SRD 5.2.1 p. 86; builtin/srd-5.2.1/source.txt (full page layout)
  dnd2024:feats/origin_feats_magic_initiate:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:feats/origin_feats_magic_initiate
        implementation: dnd2024:feats_origin_feats_magic_initiate
        parameters:
          feat_category: origin
          spell_list_choice:
            count: 1
            options:
            - cleric
            - druid
            - wizard
          spellcasting_ability_choice:
            count: 1
            options:
            - intelligence
            - wisdom
            - charisma
          cantrip_choice:
            count: 2
            spell_level: 0
            list: chosen_spell_list
          level_1_spell_choice:
            count: 1
            spell_level: 1
            list: chosen_spell_list
            always_prepared: true
            free_cast:
              spell_slot_cost: 0
              uses:
                maximum: 1
                recover:
                  event: rest.long_rest_completed
                  amount: all
            can_cast_with_spell_slots: true
          replace_spell:
            event: character.level_gained
            maximum: 1
            same_spell_level: true
            list: chosen_spell_list
          repeatable: true
          repeat_requirement: different_spell_list_each_selection
    contract:
      id: dnd2024:feats_origin_feats_magic_initiate
      inputs:
        actor: participant
        selections:
          type: choice
          required: false
      outputs:
      - feature_state
      - choices
      - resource_changes
      - spell_grants
      events:
      - character.feat_selected
      - character.level_gained
      - rest.long_rest_completed
      capabilities:
      - feats
      - spellcasting
      - character_choices
      - resources
    requires:
    - dnd2024:feats_origin_feats_magic_initiate
    citations:
    - SRD 5.2.1 pp. 87; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/feats/origin_feats_magic_initiate.md
  dnd2024:feats/origin_feats_skilled:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:feats/origin_feats_skilled
        implementation: dnd2024:feats_origin_feats_skilled
        parameters:
          feat_category: origin
          proficiency_choices:
            count: 3
            categories:
            - skill
            - tool
            mix_categories: true
          repeatable: true
    contract:
      id: dnd2024:feats_origin_feats_skilled
      inputs:
        actor: participant
      outputs:
      - feature_state
      - proficiencies
      events:
      - character.feat_selected
      capabilities:
      - feats
      - proficiencies
    requires:
    - dnd2024:feats_origin_feats_skilled
    citations:
    - SRD 5.2.1 pp. 87; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/feats/origin_feats_skilled.md
  dnd2024:feats/general_feats_grappler:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:feats/general_feats_grappler
        implementation: dnd2024:feats_general_feats_grappler
        parameters:
          feat_category: general
          prerequisites:
            minimum_character_level: 4
            any_ability_score_minimum:
              abilities:
              - strength
              - dexterity
              minimum: 13
          ability_score_increase:
            choice:
            - strength
            - dexterity
            amount: 1
            maximum: 20
          punch_and_grab:
            trigger:
              unarmed_strike_hit_creature: true
              part_of_attack_action: true
              turn: own
            allow_both_options:
            - damage
            - grapple
            limit:
              count: 1
              period: turn
          attack_advantage:
            target_grappled_by: self
          fast_wrestler:
            target_grappled_by: self
            maximum_target_size: own_size
            extra_movement_cost: 0
    contract:
      id: dnd2024:feats_general_feats_grappler
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
        destination:
          type: location
          required: false
      outputs:
      - feature_state
      - movement
      events:
      - character.feat_selected
      - attack.hit
      - attack.pending
      - movement.pending
      capabilities:
      - feats
      - ability_scores
      - grappling
      - attacks
      - movement
    requires:
    - dnd2024:feats_general_feats_grappler
    citations:
    - SRD 5.2.1 pp. 87; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/feats/general_feats_grappler.md
  dnd2024:feats/fighting_style_feats_defense:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:feats/fighting_style_feats_defense
        implementation: dnd2024:feats_fighting_style_feats_defense
        parameters:
          feat_category: fighting_style
          prerequisite_feature: fighting_style
          when_armor_category:
          - light
          - medium
          - heavy
          armor_class_bonus: 1
    contract:
      id: dnd2024:feats_fighting_style_feats_defense
      inputs:
        actor: participant
      outputs:
      - feature_state
      events:
      - character.feat_selected
      - armor_class.calculated
      capabilities:
      - feats
      - armor_class
    requires:
    - dnd2024:feats_fighting_style_feats_defense
    citations:
    - SRD 5.2.1 pp. 88; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/feats/fighting_style_feats_defense.md
  dnd2024:feats/fighting_style_feats_two_weapon_fighting:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:feats/fighting_style_feats_two_weapon_fighting
        implementation: dnd2024:feats_fighting_style_feats_two_weapon_fighting
        parameters:
          feat_category: fighting_style
          prerequisite_feature: fighting_style
          attack_is_extra_from_light_property: true
          optional_add_ability_modifier_to_damage: true
          unless_modifier_already_added: true
    contract:
      id: dnd2024:feats_fighting_style_feats_two_weapon_fighting
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - damage
      events:
      - attack.damage_pending
      capabilities:
      - feats
      - damage
    requires:
    - dnd2024:feats_fighting_style_feats_two_weapon_fighting
    citations:
    - SRD 5.2.1 pp. 88; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/feats/fighting_style_feats_two_weapon_fighting.md
  dnd2024:feats/epic_boon_feats_boon_of_dimensional_travel:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:feats/epic_boon_feats_boon_of_dimensional_travel
        implementation: dnd2024:feats_epic_boon_feats_boon_of_dimensional_travel
        parameters:
          feat_category: epic_boon
          minimum_character_level: 19
          ability_score_increase: &id115
            choice: any_ability
            amount: 1
            maximum: 30
          blink_steps:
            trigger: immediately_after_attack_or_magic_action
            optional: true
            teleport:
              maximum_feet: 30
              destination_unoccupied: true
              destination_visible: true
    contract:
      id: dnd2024:feats_epic_boon_feats_boon_of_dimensional_travel
      inputs:
        actor: participant
        destination:
          type: location
          required: false
      outputs:
      - feature_state
      events:
      - character.feat_selected
      - action.attack_completed
      - action.magic_completed
      capabilities:
      - feats
      - ability_scores
      - teleportation
    requires:
    - dnd2024:feats_epic_boon_feats_boon_of_dimensional_travel
    citations:
    - SRD 5.2.1 pp. 88; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/feats/epic_boon_feats_boon_of_dimensional_travel.md
  dnd2024:feats/epic_boon_feats_boon_of_irresistible_offense:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:feats/epic_boon_feats_boon_of_irresistible_offense
        implementation: dnd2024:feats_epic_boon_feats_boon_of_irresistible_offense
        parameters:
          feat_category: epic_boon
          minimum_character_level: 19
          ability_score_increase:
            choice:
            - strength
            - dexterity
            amount: 1
            maximum: 30
          ignore_damage_resistance:
          - bludgeoning
          - piercing
          - slashing
          overwhelming_strike:
            trigger:
              attack_d20_natural_result: 20
            optional_extra_damage:
              amount:
                stat: ability_chosen_for_this_feat.score
              type: triggering_attack_damage_type
    contract:
      id: dnd2024:feats_epic_boon_feats_boon_of_irresistible_offense
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - damage
      events:
      - character.feat_selected
      - damage.pending
      - attack.rolled
      capabilities:
      - feats
      - ability_scores
      - damage
    requires:
    - dnd2024:feats_epic_boon_feats_boon_of_irresistible_offense
    citations:
    - SRD 5.2.1 pp. 88; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/feats/epic_boon_feats_boon_of_irresistible_offense.md
  dnd2024:feats/epic_boon_feats_boon_of_the_night_spirit:
    source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
    disposition: specified
    fields:
      splat_rule:
        mechanic: dnd2024:feats/epic_boon_feats_boon_of_the_night_spirit
        implementation: dnd2024:feats_epic_boon_feats_boon_of_the_night_spirit
        parameters:
          feat_category: epic_boon
          minimum_character_level: 19
          ability_score_increase: *id115
          merge_with_shadows:
            requires_light:
            - dim
            - darkness
            action: bonus_action
            condition: invisible
            ends_immediately_after_subsequent:
            - action
            - bonus_action
            - reaction
          shadowy_form:
            requires_light:
            - dim
            - darkness
            damage_resistance:
              all_except:
              - psychic
              - radiant
    contract:
      id: dnd2024:feats_epic_boon_feats_boon_of_the_night_spirit
      inputs:
        actor: participant
        trigger:
          type: event
          required: false
        targets:
          type: participant_list
          required: false
      outputs:
      - feature_state
      - conditions
      events:
      - character.feat_selected
      - action.bonus
      - action.completed
      - damage.pending
      capabilities:
      - feats
      - ability_scores
      - conditions
      - resistances
    requires:
    - dnd2024:feats_epic_boon_feats_boon_of_the_night_spirit
    citations:
    - SRD 5.2.1 pp. 88; builtin/srd-5.2.1/source.pdf; builtin/dnd2024-definitions/feats/epic_boon_feats_boon_of_the_night_spirit.md
---

# SRD character option contracts, batch 1

All 143 assigned definitions have specified contracts: 6 classes, 6 subclasses, 116 class features, 2 backgrounds, 5 species, and 8 feats. No assigned definition is omitted or marked informational. Canonical definition references were checked against the bundled definition files.

The full page layout resolves three importer artifacts. The Fiendish Legacies table belongs to Tiefling, not Human (p. 86). The Elven Lineages level 3 and level 5 spell columns are included from p. 85. The spellbook copying sidebar falls inside the imported Wizard Subclass span; its full copying, replacement, time, and gold rules are accounted for in that contract (pp. 78–79). Class progression tables interleaved with feature paragraphs are referenced through the existing class progression definitions rather than treated as extra feature prose.

Level thresholds in a keyed scaling map remain in effect until the next threshold. Numeric expressions use typed stat, op/args, and dice nodes. A character level and a named class level are distinct. Selection constraints, conditional branches, costs, rest recovery, timing, and explicit Game Master equipment/source decisions are represented as data. Named equipment types describe item kinds and quantities; they are not unresolved definition references.
