---
splat_schema: 1
splat_id: 00589c13-1bb9-5efa-964d-8d161fdf7342
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_1_spellcasting
splat_name: 'Level 1: Spellcasting'
splat_origin:
  key: dnd2024:reference/pages/041
splat_sources:
- title: SRD 5.2.1
  locator: pp. 41, 42
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/druid_level_1_spellcasting
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
  implementation: dnd2024:class_features_druid_level_1_spellcasting
splat_contracts:
- id: dnd2024:class_features_druid_level_1_spellcasting
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
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 41
  - 42
  engine_gaps: &id001
  - dnd2024:class_features_druid_level_1_spellcasting
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: d768543da335ca85f432cf3be0ad796498000ad4de9b7abad4a3366d2e2bcc4e
  parent_name: Druid
  line_span:
    first: 3400
    count: 72
splat_clauses:
- key: source_rules
  text: '# Level 1: Spellcasting You have learned to cast spells through studying the mystical forces
    of nature. See “Spells” for the rules on spellcasting. The information below details Prepared ——Spell
    Slots per Spell Level—— Cantrips Spells 1 2 3 4 5 6 7 8 9 2 4 2 — — — — — — — — 2 5 3 — — — — — —
    — — 2 6 4 2 — — — — — — — 3 7 4 3 — — — — — — — 3 9 4 3 2 — — — — — — 3 10 4 3 3 — — — — — — 3 11
    4 3 3 1 — — — — — 3 12 4 3 3 2 — — — — — 3 14 4 3 3 3 1 — — — — 4 15 4 3 3 3 2 — — — — 4 16 4 3 3
    3 2 1 — — — 4 16 4 3 3 3 2 1 — — — 4 17 4 3 3 3 2 1 1 — — 4 17 4 3 3 3 2 1 1 — — 4 18 4 3 3 3 2 1
    1 1 — 4 18 4 3 3 3 2 1 1 1 — 4 19 4 3 3 3 2 1 1 1 1 4 20 4 3 3 3 3 1 1 1 1 4 21 4 3 3 3 3 2 1 1 1
    4 22 4 3 3 3 3 2 2 1 1 how you use those rules with Druid spells, which appear on the Druid spell
    list later in the class’s description. Cantrips. You know two cantrips of your choice from the Druid
    spell list. Druidcraft and Produce Flame are recommended. Whenever you gain a Druid level, you can
    replace one of your cantrips with another cantrip of your choice from the Druid spell list. When you
    reach Druid levels 4 and 10, you learn another cantrip of your choice from the Druid spell list, as
    shown in the Cantrips column of the Druid Features table. Spell Slots. The Druid Features table shows
    how many spell slots you have to cast your level 1+ spells. You regain all expended slots when you
    finish a Long Rest. Prepared Spells of Level 1+. You prepare the list of level 1+ spells that are
    available for you to cast with this feature. To start, choose four level 1 spells from the Druid spell
    list. Animal Friendship, Cure Wounds, Faerie Fire, and Thunderwave are recommended. The number of
    spells on your list increases as you gain Druid levels, as shown in the Prepared Spells column of
    the Druid Features table. Whenever that number increases, choose additional spells from the Druid
    spell list until the number of spells on your list matches the number on the table. The chosen spells
    must be of a level for which you have spell slots. For example, if you’re a level 3 Druid, your list
    of prepared spells can include six spells of levels 1 and 2 in any combination. If another Druid feature
    gives you spells that you always have prepared, those spells don’t count against the number of spells
    you can prepare with this feature, but those spells otherwise count as Druid spells for you. Changing
    Your Prepared Spells. Whenever you finish a Long Rest, you can change your list of prepared spells,
    replacing any of the spells with other Druid spells for which you have spell slots. Spellcasting Ability.
    Wisdom is your spellcasting ability for your Druid spells. Spellcasting Focus. You can use a Druidic
    Focus as a Spellcasting Focus for your Druid spells.'
  sha256: f5a4a099271e0d861ea4db0e7572f195d12ea93f53606e223aed615116947ba7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Spellcasting

<!-- SRD page 41 -->

You have learned to cast spells through studying\
the mystical forces of nature. See “Spells” for the\
rules on spellcasting. The information below details\
Prepared ——Spell Slots per Spell Level——\
Cantrips Spells 1 2 3 4 5 6 7 8 9\
2 4 2 — — — — — — — —\
2 5 3 — — — — — — — —\
2 6 4 2 — — — — — — —\
3 7 4 3 — — — — — — —\
3 9 4 3 2 — — — — — —\
3 10 4 3 3 — — — — — —\
3 11 4 3 3 1 — — — — —\
3 12 4 3 3 2 — — — — —\
3 14 4 3 3 3 1 — — — —\
4 15 4 3 3 3 2 — — — —\
4 16 4 3 3 3 2 1 — — —\
4 16 4 3 3 3 2 1 — — —\
4 17 4 3 3 3 2 1 1 — —\
4 17 4 3 3 3 2 1 1 — —\
4 18 4 3 3 3 2 1 1 1 —\
4 18 4 3 3 3 2 1 1 1 —\
4 19 4 3 3 3 2 1 1 1 1\
4 20 4 3 3 3 3 1 1 1 1\
4 21 4 3 3 3 3 2 1 1 1\
4 22 4 3 3 3 3 2 2 1 1\

<!-- SRD page 42 -->

how you use those rules with Druid spells, which\
appear on the Druid spell list later in the class’s\
description.\
Cantrips. You know two cantrips of your choice\
from the Druid spell list. Druidcraft and Produce\
Flame are recommended.\
Whenever you gain a Druid level, you can replace\
one of your cantrips with another cantrip of your\
choice from the Druid spell list.\
When you reach Druid levels 4 and 10, you learn\
another cantrip of your choice from the Druid spell\
list, as shown in the Cantrips column of the Druid\
Features table.\
Spell Slots. The Druid Features table shows how\
many spell slots you have to cast your level 1+\
spells. You regain all expended slots when you fin-\
ish a Long Rest.\
Prepared Spells of Level 1+. You prepare the\
list of level 1+ spells that are available for you to\
cast with this feature. To start, choose four level 1\
spells from the Druid spell list. Animal Friendship,\
Cure Wounds, Faerie Fire, and Thunderwave are\
recommended.\
The number of spells on your list increases as you\
gain Druid levels, as shown in the Prepared Spells\
column of the Druid Features table. Whenever that\
number increases, choose additional spells from the\
Druid spell list until the number of spells on your\
list matches the number on the table. The chosen\
spells must be of a level for which you have spell\
slots. For example, if you’re a level 3 Druid, your list\
of prepared spells can include six spells of levels 1\
and 2 in any combination.\
If another Druid feature gives you spells that\
you always have prepared, those spells don’t count\
against the number of spells you can prepare with\
this feature, but those spells otherwise count as\
Druid spells for you.\
Changing Your Prepared Spells. Whenever you\
finish a Long Rest, you can change your list of pre-\
pared spells, replacing any of the spells with other\
Druid spells for which you have spell slots.\
Spellcasting Ability. Wisdom is your spellcasting\
ability for your Druid spells.\
Spellcasting Focus. You can use a Druidic Focus\
as a Spellcasting Focus for your Druid spells.\
