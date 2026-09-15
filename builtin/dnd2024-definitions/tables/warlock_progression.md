---
splat_schema: 1
splat_id: ff00ca23-2b98-5663-bd18-04b6c3208cd5
splat_kind: rule
splat_key: dnd2024:tables/warlock_progression
splat_name: Warlock Progression
splat_origin:
  key: dnd2024:reference/pages/071
splat_sources:
- title: SRD 5.2.1
  locator: pp. 71
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:tables/warlock_progression
  parameters:
    class: warlock
    rows:
    - level: 1
      proficiency_bonus: 2
      features_text: Eldritch Invocations, Pact Magic
      eldritch_invocations: 1
      cantrips: 2
      prepared_spells: 2
      pact_slots: 1
      pact_slot_level: 1
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/warlock_level_1_eldritch_invocations
        at_class_level: 1
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/warlock_level_1_pact_magic
        at_class_level: 1
    - level: 2
      proficiency_bonus: 2
      features_text: Magical Cunning
      eldritch_invocations: 3
      cantrips: 2
      prepared_spells: 3
      pact_slots: 2
      pact_slot_level: 1
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/warlock_level_2_magical_cunning
        at_class_level: 2
    - level: 3
      proficiency_bonus: 2
      features_text: Warlock Subclass
      eldritch_invocations: 3
      cantrips: 2
      prepared_spells: 4
      pact_slots: 2
      pact_slot_level: 2
      grants:
      - operation: choose_subclass
        feature:
          key: dnd2024:class_features/warlock_level_3_warlock_subclass
        at_class_level: 3
    - level: 4
      proficiency_bonus: 2
      features_text: Ability Score Improvement
      eldritch_invocations: 3
      cantrips: 3
      prepared_spells: 5
      pact_slots: 2
      pact_slot_level: 2
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/warlock_level_4_ability_score_improvement
        at_class_level: 4
    - level: 5
      proficiency_bonus: 3
      features_text: —
      eldritch_invocations: 5
      cantrips: 3
      prepared_spells: 6
      pact_slots: 2
      pact_slot_level: 3
      grants: []
    - level: 6
      proficiency_bonus: 3
      features_text: Subclass feature
      eldritch_invocations: 5
      cantrips: 3
      prepared_spells: 7
      pact_slots: 2
      pact_slot_level: 3
      grants:
      - operation: grant_selected_subclass_features
        class_level: 6
    - level: 7
      proficiency_bonus: 3
      features_text: —
      eldritch_invocations: 6
      cantrips: 3
      prepared_spells: 8
      pact_slots: 2
      pact_slot_level: 4
      grants: []
    - level: 8
      proficiency_bonus: 3
      features_text: Ability Score Improvement
      eldritch_invocations: 6
      cantrips: 3
      prepared_spells: 9
      pact_slots: 2
      pact_slot_level: 4
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/warlock_level_4_ability_score_improvement
        at_class_level: 8
    - level: 9
      proficiency_bonus: 4
      features_text: Contact Patron
      eldritch_invocations: 7
      cantrips: 3
      prepared_spells: 10
      pact_slots: 2
      pact_slot_level: 5
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/warlock_level_9_contact_patron
        at_class_level: 9
    - level: 10
      proficiency_bonus: 4
      features_text: Subclass feature
      eldritch_invocations: 7
      cantrips: 4
      prepared_spells: 10
      pact_slots: 2
      pact_slot_level: 5
      grants:
      - operation: grant_selected_subclass_features
        class_level: 10
    - level: 11
      proficiency_bonus: 4
      features_text: Mystic Arcanum (level 6 spell)
      eldritch_invocations: 7
      cantrips: 4
      prepared_spells: 11
      pact_slots: 3
      pact_slot_level: 5
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/warlock_level_11_mystic_arcanum
        at_class_level: 11
        arcanum_spell_level: 6
    - level: 12
      proficiency_bonus: 4
      features_text: Ability Score Improvement
      eldritch_invocations: 8
      cantrips: 4
      prepared_spells: 11
      pact_slots: 3
      pact_slot_level: 5
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/warlock_level_4_ability_score_improvement
        at_class_level: 12
    - level: 13
      proficiency_bonus: 5
      features_text: Mystic Arcanum (level 7 spell)
      eldritch_invocations: 8
      cantrips: 4
      prepared_spells: 12
      pact_slots: 3
      pact_slot_level: 5
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/warlock_level_11_mystic_arcanum
        at_class_level: 13
        arcanum_spell_level: 7
    - level: 14
      proficiency_bonus: 5
      features_text: Subclass feature
      eldritch_invocations: 8
      cantrips: 4
      prepared_spells: 12
      pact_slots: 3
      pact_slot_level: 5
      grants:
      - operation: grant_selected_subclass_features
        class_level: 14
    - level: 15
      proficiency_bonus: 5
      features_text: Mystic Arcanum (level 8 spell)
      eldritch_invocations: 9
      cantrips: 4
      prepared_spells: 13
      pact_slots: 3
      pact_slot_level: 5
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/warlock_level_11_mystic_arcanum
        at_class_level: 15
        arcanum_spell_level: 8
    - level: 16
      proficiency_bonus: 5
      features_text: Ability Score Improvement
      eldritch_invocations: 9
      cantrips: 4
      prepared_spells: 13
      pact_slots: 3
      pact_slot_level: 5
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/warlock_level_4_ability_score_improvement
        at_class_level: 16
    - level: 17
      proficiency_bonus: 6
      features_text: Mystic Arcanum (level 9 spell)
      eldritch_invocations: 9
      cantrips: 4
      prepared_spells: 14
      pact_slots: 4
      pact_slot_level: 5
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/warlock_level_11_mystic_arcanum
        at_class_level: 17
        arcanum_spell_level: 9
    - level: 18
      proficiency_bonus: 6
      features_text: —
      eldritch_invocations: 10
      cantrips: 4
      prepared_spells: 14
      pact_slots: 4
      pact_slot_level: 5
      grants: []
    - level: 19
      proficiency_bonus: 6
      features_text: Epic Boon
      eldritch_invocations: 10
      cantrips: 4
      prepared_spells: 15
      pact_slots: 4
      pact_slot_level: 5
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/warlock_level_19_epic_boon
        at_class_level: 19
    - level: 20
      proficiency_bonus: 6
      features_text: Eldritch Master
      eldritch_invocations: 10
      cantrips: 4
      prepared_spells: 15
      pact_slots: 4
      pact_slot_level: 5
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/warlock_level_20_eldritch_master
        at_class_level: 20
    class_definition:
      key: dnd2024:classes/warlock
    lookup:
      input: class_level
      minimum: 1
      maximum: 20
      match: exact
      numeric_columns: current_capacity_or_value
      feature_grants: all_crossed_levels
      repeat_application: idempotent_per_class_level_and_feature
    multiclass:
      proficiency_bonus_source: total_character_level
      spell_slot_override_rule:
        key: dnd2024:rules/multiclassing_spellcasting
      pact_magic_pool_separate: true
    column_semantics:
      cantrips:
        kind: selection_capacity
        selection: cantrips
        eligibility: owning_feature_contract
      eldritch_invocations:
        kind: selection_capacity
        selection: eldritch_invocations
        eligibility: owning_feature_contract
      pact_slot_level:
        kind: spell_level
        applies_to: pact_slots
      pact_slots:
        kind: resource_capacity
        resource: pact_slots
        recovery: owning_feature_contract
        level_change_refills: false
      prepared_spells:
        kind: selection_capacity
        selection: prepared_spells
        eligibility: owning_feature_contract
      proficiency_bonus:
        kind: bonus
        applies_to: proficiency
        use_class_row_only_if_single_class: true
  implementation: dnd2024:class_progression_warlock_progression
splat_contracts:
- id: dnd2024:class_progression_warlock_progression
  inputs:
    actor:
      type: participant
      required: true
    class_level:
      type: integer
      required: true
    previous_class_level:
      required: false
      type: integer
    selected_subclass:
      required: false
      type: definition
    choices:
      required: false
      type: definition_list
  outputs:
  - feature_grants
  - selection_capacities
  - resource_capacities
  - class_values
  events:
  - character.class_level_changed
  capabilities:
  - class_progression
  - feature_grants
  - resource_capacities
  - character_choices
splat_conversion:
  status: specified
  category: table
  source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
  source_path: builtin/srd-5.2.1/source.txt
  pages:
  - 71
  engine_gaps: &id001
  - dnd2024:class_progression_warlock_progression
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 40729e3a6870c2f4c82fc33d22831d07d271a696ad085bafe373a1940c8ea0e5
splat_clauses:
- key: source_rules
  text: '# Warlock Progression From SRD 5.2.1 page 71. Warlock Features Proficiency Eldritch Prepared
    Spell Slot Level Bonus Class Features Invocations Cantrips Spells Slots Level 1 +2 Eldritch Invocations,
    Pact Magic 1 2 2 1 1 2 +2 Magical Cunning 3 2 3 2 1 3 +2 Warlock Subclass 3 2 4 2 2 4 +2 Ability Score
    Improvement 3 3 5 2 2 5 +3 — 5 3 6 2 3 6 +3 Subclass feature 5 3 7 2 3 7 +3 — 6 3 8 2 4 8 +3 Ability
    Score Improvement 6 3 9 2 4 9 +4 Contact Patron 7 3 10 2 5 10 +4 Subclass feature 7 4 10 2 5 11 +4
    Mystic Arcanum (level 6 spell) 7 4 11 3 5 12 +4 Ability Score Improvement 8 4 11 3 5 13 +5 Mystic
    Arcanum (level 7 spell) 8 4 12 3 5 14 +5 Subclass feature 8 4 12 3 5 15 +5 Mystic Arcanum (level 8
    spell) 9 4 13 3 5 16 +5 Ability Score Improvement 9 4 13 3 5 17 +6 Mystic Arcanum (level 9 spell)
    9 4 14 4 5 18 +6 — 10 4 14 4 5 19 +6 Epic Boon 10 4 15 4 5 20 +6 Eldritch Master 10 4 15 4 5 qualify.
    You can’t replace an invocation if it’s a pre- Spell Slots. The Warlock Features table shows requisite
    for another invocation that you have. how many spell slots you have to cast your Warlock When you
    gain certain Warlock levels, you gain spells of levels 1–5. The table also shows the level more invocations
    of your choice, as shown in the Inof those slots, all of which are the same level. You vocations column
    of the Warlock Features table. regain all expended Pact Magic spell slots when you You can’t pick
    the same invocation more than once finish a Short or Long Rest. unless its description says otherwise.
    For example, when you’re a level 5 Warlock, you have two level 3 spell slots. To cast the level 1
    spell Level 1: Pact Magic Charm Person, you must spend one of those slots, Through occult ceremony,
    you have formed a pact and you cast it as a level 3 spell. with a mysterious entity to gain magical
    powers. Prepared Spells of Level 1+. You prepare the The entity is a voice in the shadows—its identity
    list of level 1+ spells that are available for you unclear—but its boon to you is concrete: the ability
    to cast with this feature. To start, choose two to cast spells. See “Spells” for the rules on spelllevel
    1 Warlock spells. Charm Person and Hex are casting. The information below details how you use recommended.
    those rules with Warlock spells, which appear in the The number of spells on your list increases as
    you Warlock spell list later in the class’s description. gain Warlock levels, as shown in the Prepared
    Spells Cantrips. You know two Warlock cantrips of your column of the Warlock Features table. Whenever
    choice. Eldritch Blast and Prestidigitation are recthat number increases, choose additional Warommended.
    Whenever you gain a Warlock level, you lock spells until the number of spells on your list can replace
    one of your cantrips from this feature matches the number in the table. The chosen spells with another
    Warlock cantrip of your choice. must be of a level no higher than what’s shown in When you reach Warlock
    levels 4 and 10, you the table’s Slot Level column for your level. When learn another Warlock cantrip
    of your choice, as you reach level 6, for example, you learn a new Warshown in the Cantrips column
    of the Warlock Fealock spell, which can be of levels 1–3. tures table. If another Warlock feature
    gives you spells that you always have prepared, those spells don’t count 71 System Reference Document
    5.2.1'
  sha256: ece30ffb68d49f33ad7406059e03a2a27abe6358cf331318e6ddc72a06dc3109
  status: contract_specified
splat_requires_mechanics: *id001
---

# Warlock Progression

From SRD 5.2.1 page 71.

     Warlock Features
            Proficiency                                          Eldritch               Prepared     Spell     Slot
      Level Bonus       Class Features                         Invocations   Cantrips    Spells      Slots    Level
        1         +2      Eldritch Invocations, Pact Magic         1            2          2           1        1
        2         +2      Magical Cunning                          3            2          3           2        1
        3         +2      Warlock Subclass                         3            2          4           2        2
        4         +2      Ability Score Improvement                3            3          5           2        2
        5         +3      —                                        5            3          6           2        3
        6         +3      Subclass feature                         5            3          7           2        3
        7         +3      —                                        6            3          8           2        4
        8         +3      Ability Score Improvement                6            3          9           2        4
        9         +4      Contact Patron                           7            3          10          2        5
       10         +4      Subclass feature                         7            4          10          2        5
       11         +4      Mystic Arcanum (level 6 spell)           7            4          11          3        5
       12         +4      Ability Score Improvement                8            4          11          3        5
       13         +5      Mystic Arcanum (level 7 spell)           8            4          12          3        5
       14         +5      Subclass feature                         8            4          12          3        5
       15         +5      Mystic Arcanum (level 8 spell)           9            4          13          3        5
       16         +5      Ability Score Improvement                9            4          13          3        5
       17         +6      Mystic Arcanum (level 9 spell)           9            4          14          4        5
       18         +6      —                                        10           4          14          4        5
       19         +6      Epic Boon                                10           4          15          4        5
       20         +6      Eldritch Master                          10           4          15          4        5


     qualify. You can’t replace an invocation if it’s a pre-     Spell Slots. The Warlock Features table shows
     requisite for another invocation that you have.           how many spell slots you have to cast your Warlock
       When you gain certain Warlock levels, you gain          spells of levels 1–5. The table also shows the level
     more invocations of your choice, as shown in the In-      of those slots, all of which are the same level. You
     vocations column of the Warlock Features table.           regain all expended Pact Magic spell slots when you
       You can’t pick the same invocation more than once       finish a Short or Long Rest.
     unless its description says otherwise.                      For example, when you’re a level 5 Warlock, you
                                                               have two level 3 spell slots. To cast the level 1 spell
     Level 1: Pact Magic                                       Charm Person, you must spend one of those slots,
     Through occult ceremony, you have formed a pact           and you cast it as a level 3 spell.
     with a mysterious entity to gain magical powers.            Prepared Spells of Level 1+. You prepare the
     The entity is a voice in the shadows—its identity         list of level 1+ spells that are available for you
     unclear—but its boon to you is concrete: the ability      to cast with this feature. To start, choose two
     to cast spells. See “Spells” for the rules on spell-      level 1 Warlock spells. Charm Person and Hex are
     casting. The information below details how you use        recommended.
     those rules with Warlock spells, which appear in the        The number of spells on your list increases as you
     Warlock spell list later in the class’s description.      gain Warlock levels, as shown in the Prepared Spells
       Cantrips. You know two Warlock cantrips of your         column of the Warlock Features table. Whenever
     choice. Eldritch Blast and Prestidigitation are rec-      that number increases, choose additional War-
     ommended. Whenever you gain a Warlock level, you          lock spells until the number of spells on your list
     can replace one of your cantrips from this feature        matches the number in the table. The chosen spells
     with another Warlock cantrip of your choice.              must be of a level no higher than what’s shown in
       When you reach Warlock levels 4 and 10, you             the table’s Slot Level column for your level. When
     learn another Warlock cantrip of your choice, as          you reach level 6, for example, you learn a new War-
     shown in the Cantrips column of the Warlock Fea-          lock spell, which can be of levels 1–3.
     tures table.                                                If another Warlock feature gives you spells that
                                                               you always have prepared, those spells don’t count


71   System Reference Document 5.2.1
