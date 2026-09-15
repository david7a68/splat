---
splat_schema: 1
splat_id: 6ceac3b6-c445-5627-b0c1-8fbfc4b25129
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_1_spellcasting
splat_name: 'Level 1: Spellcasting'
splat_origin:
  key: dnd2024:reference/pages/054
splat_sources:
- title: SRD 5.2.1
  locator: pp. 54
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/paladin_level_1_spellcasting
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
  implementation: dnd2024:class_features_paladin_level_1_spellcasting
splat_contracts:
- id: dnd2024:class_features_paladin_level_1_spellcasting
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
  - 54
  engine_gaps: &id001
  - dnd2024:class_features_paladin_level_1_spellcasting
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: df10380498a53e5cea66e4d52057faa5721f4f4acdd9db8e5dc0de7142b403fd
  parent_name: Paladin
  line_span:
    first: 4493
    count: 38
splat_clauses:
- key: source_rules
  text: '# Level 1: Spellcasting You have learned to cast spells through prayer and meditation. See “Spells”
    for the rules on spellcasting. The information below details how you use those rules with Paladin
    spells, which appear in the Paladin spell list later in the class’s description. Spell Slots. The
    Paladin Features table shows how many spell slots you have to cast your level 1+ spells. You regain
    all expended slots when you finish a Long Rest. Prepared Spells of Level 1+. You prepare the list
    of level 1+ spells that are available for you to cast with this feature. To start, choose two level
    1 Paladin spells. Heroism and Searing Smite are recommended. The number of spells on your list increases
    as you gain Paladin levels, as shown in the Prepared Spells column of the Paladin Features table.
    Whenever that number increases, choose additional Paladin spells until the number of spells on your
    list matches the number in the Paladin Features table. The chosen spells must be of a level for which
    you have spell slots. For example, if you’re a level 5 Paladin, your list of prepared spells can include
    six Paladin spells of level 1 or 2 in any combination. If another Paladin feature gives you spells
    that you always have prepared, those spells don’t count against the number of spells you can prepare
    with this feature, but those spells otherwise count as Paladin spells for you. Changing Your Prepared
    Spells. Whenever you finish a Long Rest, you can replace one spell on your list with another Paladin
    spell for which you have spell slots. Spellcasting Ability. Charisma is your spellcasting ability
    for your Paladin spells. Spellcasting Focus. You can use a Holy Symbol as a Spellcasting Focus for
    your Paladin spells.'
  sha256: 608aa16e9637d1f87fa5fef6004326e54a7496c8489199832d59dd15a4b95bee
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Spellcasting

<!-- SRD page 54 -->

You have learned to cast spells through prayer and\
meditation. See “Spells” for the rules on spellcast-\
ing. The information below details how you use\
those rules with Paladin spells, which appear in the\
Paladin spell list later in the class’s description.\
Spell Slots. The Paladin Features table shows\
how many spell slots you have to cast your level 1+\
spells. You regain all expended slots when you fin-\
ish a Long Rest.\
Prepared Spells of Level 1+. You prepare the\
list of level 1+ spells that are available for you to\
cast with this feature. To start, choose two level\
1 Paladin spells. Heroism and Searing Smite are\
recommended.\
The number of spells on your list increases as\
you gain Paladin levels, as shown in the Prepared\
Spells column of the Paladin Features table. When-\
ever that number increases, choose additional\
Paladin spells until the number of spells on your\
list matches the number in the Paladin Features\
table. The chosen spells must be of a level for which\
you have spell slots. For example, if you’re a level 5\
Paladin, your list of prepared spells can include six\
Paladin spells of level 1 or 2 in any combination.\
If another Paladin feature gives you spells that\
you always have prepared, those spells don’t count\
against the number of spells you can prepare with\
this feature, but those spells otherwise count as\
Paladin spells for you.\
Changing Your Prepared Spells. Whenever you\
finish a Long Rest, you can replace one spell on your\
list with another Paladin spell for which you have\
spell slots.\
Spellcasting Ability. Charisma is your spellcast-\
ing ability for your Paladin spells.\
Spellcasting Focus. You can use a Holy Symbol as\
a Spellcasting Focus for your Paladin spells.\
