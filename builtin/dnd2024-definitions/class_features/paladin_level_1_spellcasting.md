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
---

# Level 1: Spellcasting

## Source text

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

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/paladin_level_1_spellcasting ` |
| ` implementation ` | ` dnd2024:class_features_paladin_level_1_spellcasting ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` paladin ` |
| ` spellcasting_ability ` | ` charisma ` |
| ` spellcasting_focus ` | ` holy_symbol ` |

#### Save dc

<!-- splat:yaml path=splat_rule/parameters/save_dc -->

```yaml
op: add
args:
- 8
- var: actor.abilities.charisma.modifier
- var: actor.proficiency_bonus
```

#### Spell attack bonus

<!-- splat:yaml path=splat_rule/parameters/spell_attack_bonus -->

```yaml
op: add
args:
- var: actor.proficiency_bonus
- var: actor.abilities.charisma.modifier
```

#### Slots

<!-- splat:yaml path=splat_rule/parameters/slots -->

```yaml
progression:
  key: dnd2024:tables/paladin_progression
recovery:
- rest.long_completed
recover: all
pool: spellcasting
```

#### Prepared spells

<!-- splat:yaml path=splat_rule/parameters/prepared_spells -->

```yaml
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
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` paladin ` |
| ` minimum_level ` | ` 1 ` |
