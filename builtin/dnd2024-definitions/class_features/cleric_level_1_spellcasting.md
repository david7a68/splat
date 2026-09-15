---
splat_schema: 1
splat_id: f168e85d-67e7-545d-b90f-399f1c3a5612
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_1_spellcasting
splat_name: 'Level 1: Spellcasting'
splat_origin:
  key: dnd2024:reference/pages/036
splat_sources:
- title: SRD 5.2.1
  locator: pp. 36, 37
  url: https://www.dndbeyond.com/srd
---

# Level 1: Spellcasting

## Source text

<!-- SRD page 36 -->

You have learned to cast spells through prayer and\
meditation. See “Spells” for the rules on spellcast-\
ing. The information below details how you use\
those rules with Cleric spells, which appear on the\
Cleric spell list later in the class’s description.\
Cantrips. You know three cantrips of your choice\
from the Cleric spell list. Guidance, Sacred Flame,\
and Thaumaturgy are recommended.\
Whenever you gain a Cleric level, you can replace\
one of your cantrips with another cantrip of your\
choice from the Cleric spell list.\
Prepared ——Spell Slots per Spell Level——\
Cantrips Spells 1 2 3 4 5 6 7 8 9\
3 4 2 — — — — — — — —\
3 5 3 — — — — — — — —\
3 6 4 2 — — — — — — —\
4 7 4 3 — — — — — — —\
4 9 4 3 2 — — — — — —\
4 10 4 3 3 — — — — — —\
4 11 4 3 3 1 — — — — —\
4 12 4 3 3 2 — — — — —\
4 14 4 3 3 3 1 — — — —\
5 15 4 3 3 3 2 — — — —\
5 16 4 3 3 3 2 1 — — —\
5 16 4 3 3 3 2 1 — — —\
5 17 4 3 3 3 2 1 1 — —\
5 17 4 3 3 3 2 1 1 — —\
5 18 4 3 3 3 2 1 1 1 —\
5 18 4 3 3 3 2 1 1 1 —\
5 19 4 3 3 3 2 1 1 1 1\
5 20 4 3 3 3 3 1 1 1 1\
5 21 4 3 3 3 3 2 1 1 1\
5 22 4 3 3 3 3 2 2 1 1\

<!-- SRD page 37 -->

When you reach Cleric levels 4 and 10, you learn\
another cantrip of your choice from the Cleric spell\
list, as shown in the Cantrips column of the Cleric\
Features table.\
Spell Slots. The Cleric Features table shows how\
many spell slots you have to cast your level 1+\
spells. You regain all expended slots when you fin-\
ish a Long Rest.\
Prepared Spells of Level 1+. You prepare the list\
of level 1+ spells that are available for you to cast\
with this feature. To start, choose four level 1 spells\
from the Cleric spell list. Bless, Cure Wounds, Guiding\
Bolt, and Shield of Faith are recommended.\
The number of spells on your list increases as you\
gain Cleric levels, as shown in the Prepared Spells\
column of the Cleric Features table. Whenever that\
number increases, choose additional spells from the\
Cleric spell list until the number of spells on your\
list matches the number on the table. The chosen\
spells must be of a level for which you have spell\
slots. For example, if you’re a level 3 Cleric, your list\
of prepared spells can include six spells of levels 1\
and 2 in any combination.\
If another Cleric feature gives you spells that\
you always have prepared, those spells don’t count\
against the number of spells you can prepare with\
this feature, but those spells otherwise count as\
Cleric spells for you.\
Changing Your Prepared Spells. Whenever you\
finish a Long Rest, you can change your list of pre-\
pared spells, replacing any of the spells there with\
other Cleric spells for which you have spell slots.\
Spellcasting Ability. Wisdom is your spellcasting\
ability for your Cleric spells.\
Spellcasting Focus. You can use a Holy Symbol as\
a Spellcasting Focus for your Cleric spells.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/cleric_level_1_spellcasting ` |
| ` implementation ` | ` dnd2024:class_features_cleric_level_1_spellcasting ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` cleric ` |
| ` spellcasting_ability ` | ` wisdom ` |
| ` spellcasting_focus ` | ` holy_symbol ` |

#### Save dc

<!-- splat:yaml path=splat_rule/parameters/save_dc -->

```yaml
op: add
args:
- 8
- var: actor.abilities.wisdom.modifier
- var: actor.proficiency_bonus
```

#### Spell attack bonus

<!-- splat:yaml path=splat_rule/parameters/spell_attack_bonus -->

```yaml
op: add
args:
- var: actor.proficiency_bonus
- var: actor.abilities.wisdom.modifier
```

#### Slots

<!-- splat:yaml path=splat_rule/parameters/slots -->

```yaml
progression:
  key: dnd2024:tables/cleric_progression
recovery:
- rest.long_completed
recover: all
pool: spellcasting
```

#### Prepared spells

<!-- splat:yaml path=splat_rule/parameters/prepared_spells -->

```yaml
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
```

#### Cantrips

<!-- splat:yaml path=splat_rule/parameters/cantrips -->

```yaml
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
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` cleric ` |
| ` minimum_level ` | ` 1 ` |
