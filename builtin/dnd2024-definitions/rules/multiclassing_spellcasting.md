---
splat_schema: 1
splat_id: cfb889a4-de9b-5ded-9aeb-d96d9d17507a
splat_kind: rule
splat_key: dnd2024:rules/multiclassing_spellcasting
splat_name: Spellcasting
splat_origin:
  key: dnd2024:reference/pages/025
splat_sources:
- title: SRD 5.2.1
  locator: pp. 25, 26
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/multiclassing_spellcasting
  parameters:
    multiclass_table_requires_spellcasting_classes_min: 2
    single_spellcasting_class: use_class_slots
    spell_preparation:
      separate_by_class: true
      use_only_class_level: true
      spell_associated_with_class: true
      casting_ability: associated_class_ability
    cantrip_scaling: total_character_level_unless_spell_overrides
    effective_caster_level:
      full_level_classes:
      - bard
      - cleric
      - druid
      - sorcerer
      - wizard
      half_level_classes:
      - paladin
      - ranger
      half_rounding: up
      round_each_class_before_sum: true
    slots_by_effective_level:
    - level: 1
      slots:
        '1': 2
        '2': 0
        '3': 0
        '4': 0
        '5': 0
        '6': 0
        '7': 0
        '8': 0
        '9': 0
    - level: 2
      slots:
        '1': 3
        '2': 0
        '3': 0
        '4': 0
        '5': 0
        '6': 0
        '7': 0
        '8': 0
        '9': 0
    - level: 3
      slots:
        '1': 4
        '2': 2
        '3': 0
        '4': 0
        '5': 0
        '6': 0
        '7': 0
        '8': 0
        '9': 0
    - level: 4
      slots:
        '1': 4
        '2': 3
        '3': 0
        '4': 0
        '5': 0
        '6': 0
        '7': 0
        '8': 0
        '9': 0
    - level: 5
      slots:
        '1': 4
        '2': 3
        '3': 2
        '4': 0
        '5': 0
        '6': 0
        '7': 0
        '8': 0
        '9': 0
    - level: 6
      slots:
        '1': 4
        '2': 3
        '3': 3
        '4': 0
        '5': 0
        '6': 0
        '7': 0
        '8': 0
        '9': 0
    - level: 7
      slots:
        '1': 4
        '2': 3
        '3': 3
        '4': 1
        '5': 0
        '6': 0
        '7': 0
        '8': 0
        '9': 0
    - level: 8
      slots:
        '1': 4
        '2': 3
        '3': 3
        '4': 2
        '5': 0
        '6': 0
        '7': 0
        '8': 0
        '9': 0
    - level: 9
      slots:
        '1': 4
        '2': 3
        '3': 3
        '4': 3
        '5': 1
        '6': 0
        '7': 0
        '8': 0
        '9': 0
    - level: 10
      slots:
        '1': 4
        '2': 3
        '3': 3
        '4': 3
        '5': 2
        '6': 0
        '7': 0
        '8': 0
        '9': 0
    - level: 11
      slots:
        '1': 4
        '2': 3
        '3': 3
        '4': 3
        '5': 2
        '6': 1
        '7': 0
        '8': 0
        '9': 0
    - level: 12
      slots:
        '1': 4
        '2': 3
        '3': 3
        '4': 3
        '5': 2
        '6': 1
        '7': 0
        '8': 0
        '9': 0
    - level: 13
      slots:
        '1': 4
        '2': 3
        '3': 3
        '4': 3
        '5': 2
        '6': 1
        '7': 1
        '8': 0
        '9': 0
    - level: 14
      slots:
        '1': 4
        '2': 3
        '3': 3
        '4': 3
        '5': 2
        '6': 1
        '7': 1
        '8': 0
        '9': 0
    - level: 15
      slots:
        '1': 4
        '2': 3
        '3': 3
        '4': 3
        '5': 2
        '6': 1
        '7': 1
        '8': 1
        '9': 0
    - level: 16
      slots:
        '1': 4
        '2': 3
        '3': 3
        '4': 3
        '5': 2
        '6': 1
        '7': 1
        '8': 1
        '9': 0
    - level: 17
      slots:
        '1': 4
        '2': 3
        '3': 3
        '4': 3
        '5': 2
        '6': 1
        '7': 1
        '8': 1
        '9': 1
    - level: 18
      slots:
        '1': 4
        '2': 3
        '3': 3
        '4': 3
        '5': 3
        '6': 1
        '7': 1
        '8': 1
        '9': 1
    - level: 19
      slots:
        '1': 4
        '2': 3
        '3': 3
        '4': 3
        '5': 3
        '6': 2
        '7': 1
        '8': 1
        '9': 1
    - level: 20
      slots:
        '1': 4
        '2': 3
        '3': 3
        '4': 3
        '5': 3
        '6': 2
        '7': 2
        '8': 1
        '9': 1
    slots_allow_casting_any_prepared_spell_with_sufficient_level: true
    higher_slots_do_not_unlock_preparation: true
    upcast_effects: spell_definition
    pact_magic:
      pact_slots_cast_other_class_prepared_spells: true
      spellcasting_slots_cast_prepared_warlock_spells: true
  implementation: dnd2024:rules_multiclassing_spellcasting
splat_contracts:
- id: dnd2024:rules_multiclassing_spellcasting
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - spell_slots
  - prepared_spell_constraints
  - casting_ability
  - cantrip_level
  events: []
  capabilities:
  - multiclassing
  - spellcasting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 25
  - 26
  engine_gaps: &id001
  - dnd2024:rules_multiclassing_spellcasting
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 72542bbadc257c1afd7a613b4f40e12c7e60a2cade1222991f88e92aa890be0d
  parent_name: Multiclassing
  line_span:
    first: 1977
    count: 79
splat_clauses:
- key: source_rules
  text: '# Spellcasting Your capacity for spellcasting depends partly on your combined levels in all your
    spellcasting classes and partly on your individual levels in those classes. Once you have the Spellcasting
    feature from more than one class, use the rules below. If you multiclass but have the Spellcasting
    feature from only one class, follow the rules for that class. Spells Prepared. You determine what
    spells you can prepare for each class individually, as if you were a single-classed member of that
    class. If you are a level 4 Ranger / level 3 Sorcerer, for example, you can prepare five level 1 Ranger
    spells, and you can prepare six Sorcerer spells of level 1 or 2 (as well as four Sorcerer cantrips).
    Each spell you prepare is associated with one of your classes, and you use the spellcasting ability
    of that class when you cast the spell. Cantrips. If a cantrip of yours increases in power at higher
    levels, the increase is based on your total character level, not your level in a particular class,
    unless the spell says otherwise. Spell Slots. You determine your available spell slots by adding together
    the following: • All your levels in the Bard, Cleric, Druid, Sorcerer, and Wizard classes • Half your
    levels (round up) in the Paladin and Ranger classes Then look up this total level in the Level column
    of the Multiclass Spellcaster table. You use the slots for that level to cast spells of an appropriate
    level from any class whose Spellcasting feature you have. This table might give you spell slots of
    a higher level than the spells you prepare. You can use those slots but only to cast your lower-level
    spells. If a lower-level spell that you cast, like Burning Hands, has an enhanced effect when cast
    at a higher level, you can use the enhanced effect as normal. For example, if you are a level 4 Ranger
    / level 3 Sorcerer, you count as a level 5 character when determining your spell slots, counting all
    your levels as a Sorcerer and half your Ranger levels. As shown in the Multiclass Spellcaster table,
    you have four level 1 spell slots, three level 2 slots, and two level 3 slots. However, you can’t
    prepare any level 3 spells, nor can you prepare any level 2 Ranger spells. You can use the spell slots
    of those levels to cast the spells you do prepare—and potentially enhance their effects. Pact Magic.
    If you have the Pact Magic feature from the Warlock class and the Spellcasting feature, you can use
    the spell slots you gain from Pact Magic to cast spells you have prepared from classes with the Spellcasting
    feature, and you can use the spell slots you gain from the Spellcasting feature to cast Warlock spells
    you have prepared. Multiclass Spellcaster: Spell Slots per Spell Level Level 1 2 3 4 5 6 7 8 9 1 2
    — — — — — — — — 2 3 — — — — — — — — 3 4 2 — — — — — — — 4 4 3 — — — — — — — 5 4 3 2 — — — — — — 6
    4 3 3 — — — — — — 7 4 3 3 1 — — — — — 8 4 3 3 2 — — — — — 9 4 3 3 3 1 — — — — 10 4 3 3 3 2 — — — —
    11 4 3 3 3 2 1 — — — 12 4 3 3 3 2 1 — — — 13 4 3 3 3 2 1 1 — — 14 4 3 3 3 2 1 1 — — 15 4 3 3 3 2 1
    1 1 — 16 4 3 3 3 2 1 1 1 — 17 4 3 3 3 2 1 1 1 1 18 4 3 3 3 3 1 1 1 1 19 4 3 3 3 3 2 1 1 1 20 4 3 3
    3 3 2 2 1 1'
  sha256: 519f4f0a1e7bb586602bf754a9a4bcb81966da6931a4bd32059974059fb9733a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spellcasting

<!-- SRD page 25 -->

Your capacity for spellcasting depends partly on\
your combined levels in all your spellcasting classes\
and partly on your individual levels in those classes.\
Once you have the Spellcasting feature from more\
than one class, use the rules below. If you multiclass\
but have the Spellcasting feature from only one\
class, follow the rules for that class.\
Spells Prepared. You determine what spells you\
can prepare for each class individually, as if you\
were a single-classed member of that class. If you\
are a level 4 Ranger / level 3 Sorcerer, for example,\
you can prepare five level 1 Ranger spells, and you\
can prepare six Sorcerer spells of level 1 or 2 (as\
well as four Sorcerer cantrips).\
Each spell you prepare is associated with one of\
your classes, and you use the spellcasting ability of\
that class when you cast the spell.\
Cantrips. If a cantrip of yours increases in power\
at higher levels, the increase is based on your total\
character level, not your level in a particular class,\
unless the spell says otherwise.\
Spell Slots. You determine your available spell\
slots by adding together the following:\
•  All your levels in the Bard, Cleric, Druid, Sorcerer,\
and Wizard classes\
•  Half your levels (round up) in the Paladin and\
Ranger classes\
Then look up this total level in the Level column of\
the Multiclass Spellcaster table. You use the slots for\
that level to cast spells of an appropriate level from\
any class whose Spellcasting feature you have.\
This table might give you spell slots of a higher\
level than the spells you prepare. You can use those\
slots but only to cast your lower-level spells. If a\
lower-level spell that you cast, like Burning Hands,\
has an enhanced effect when cast at a higher level,\
you can use the enhanced effect as normal.\
For example, if you are a level 4 Ranger / level 3\
Sorcerer, you count as a level 5 character when de-\
termining your spell slots, counting all your levels\
as a Sorcerer and half your Ranger levels. As shown\

<!-- SRD page 26 -->

in the Multiclass Spellcaster table, you have four\
level 1 spell slots, three level 2 slots, and two level 3\
slots. However, you can’t prepare any level 3 spells,\
nor can you prepare any level 2 Ranger spells. You\
can use the spell slots of those levels to cast the\
spells you do prepare—and potentially enhance\
their effects.\
Pact Magic. If you have the Pact Magic feature\
from the Warlock class and the Spellcasting feature,\
you can use the spell slots you gain from Pact Magic\
to cast spells you have prepared from classes with\
the Spellcasting feature, and you can use the spell\
slots you gain from the Spellcasting feature to cast\
Warlock spells you have prepared.\
Multiclass Spellcaster:\
Spell Slots per Spell Level\
Level 1 2 3 4 5 6 7 8 9\
1 2 — — — — — — — —\
2 3 — — — — — — — —\
3 4 2 — — — — — — —\
4 4 3 — — — — — — —\
5 4 3 2 — — — — — —\
6 4 3 3 — — — — — —\
7 4 3 3 1 — — — — —\
8 4 3 3 2 — — — — —\
9 4 3 3 3 1 — — — —\
10 4 3 3 3 2 — — — —\
11 4 3 3 3 2 1 — — —\
12 4 3 3 3 2 1 — — —\
13 4 3 3 3 2 1 1 — —\
14 4 3 3 3 2 1 1 — —\
15 4 3 3 3 2 1 1 1 —\
16 4 3 3 3 2 1 1 1 —\
17 4 3 3 3 2 1 1 1 1\
18 4 3 3 3 3 1 1 1 1\
19 4 3 3 3 3 2 1 1 1\
20 4 3 3 3 3 2 2 1 1\
