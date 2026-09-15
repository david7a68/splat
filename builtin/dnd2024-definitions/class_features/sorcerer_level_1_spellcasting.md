---
splat_schema: 1
splat_id: 8f1b19b2-4af8-5f3c-88ea-c7f2fec52817
splat_kind: rule
splat_key: dnd2024:class_features/sorcerer_level_1_spellcasting
splat_name: 'Level 1: Spellcasting'
splat_origin:
  key: dnd2024:reference/pages/064
splat_sources:
- title: SRD 5.2.1
  locator: pp. 64, 65
  url: https://www.dndbeyond.com/srd
---

# Level 1: Spellcasting

## Source text

<!-- SRD page 64 -->

Drawing from your innate magic, you can cast\
spells. See “Spells” for the rules on spellcasting. The\
information below details how you use those rules\
with Sorcerer spells, which appear in the Sorcerer\
spell list later in the class’s description.\
Cantrips. You know four Sorcerer cantrips of your\
choice. Light, Prestidigitation, Shocking Grasp, and\
Sorcerous Burst are recommended. Whenever you\
gain a Sorcerer level, you can replace one of your\
cantrips from this feature with another Sorcerer\
cantrip of your choice.\
When you reach Sorcerer levels 4 and 10, you\
learn another Sorcerer cantrip of your choice, as\
shown in the Cantrips column of the Sorcerer Fea-\
tures table.\
Spell Slots. The Sorcerer Features table shows\
how many spell slots you have to cast your level 1+\

<!-- SRD page 65 -->

Sorcerer Features\
Proficiency Sorcery\
Level Bonus Class Features Points Cantrips\
1 +2 Spellcasting, —\
Innate Sorcery\
2 +2 Font of Magic, 2\
Metamagic\
3 +2 Sorcerer Subclass 3\
4 +2 Ability Score 4\
Improvement\
5 +3 Sorcerous 5\
Restoration\
6 +3 Subclass feature 6\
7 +3 Sorcery Incarnate 7\
8 +3 Ability Score 8\
Improvement\
9 +4 — 9\
10 +4 Metamagic 10\
11 +4 — 11\
12 +4 Ability Score 12\
Improvement\
13 +5 — 13\
14 +5 Subclass feature 14\
15 +5 — 15\
16 +5 Ability Score 16\
Improvement\
17 +6 Metamagic 17\
18 +6 Subclass feature 18\
19 +6 Epic Boon 19\
20 +6 Arcane Apotheosis 20\
spells. You regain all expended slots when you fin-\
ish a Long Rest.\
Prepared Spells of Level 1+. You prepare the list\
of level 1+ spells that are available for you to cast\
with this feature. To start, choose two level 1 Sor-\
cerer spells. Burning Hands and Detect Magic are\
recommended.\
The number of spells on your list increases as\
you gain Sorcerer levels, as shown in the Prepared\
Spells column of the Sorcerer Features table. When-\
ever that number increases, choose additional\
Sorcerer spells until the number of spells on your\
list matches the number in the Sorcerer Features\
table. The chosen spells must be of a level for which\
you have spell slots. For example, if you’re a level 3\
Sorcerer, your list of prepared spells can include six\
Sorcerer spells of level 1 or 2 in any combination.\
If another Sorcerer feature gives you spells that\
you always have prepared, those spells don’t count\
Prepared ——Spell Slots per Spell Level——\
Spells 1 2 3 4 5 6 7 8 9\
4 2 2 — — — — — — — —\
4 4 3 — — — — — — — —\
4 6 4 2 — — — — — — —\
5 7 4 3 — — — — — — —\
5 9 4 3 2 — — — — — —\
5 10 4 3 3 — — — — — —\
5 11 4 3 3 1 — — — — —\
5 12 4 3 3 2 — — — — —\
5 14 4 3 3 3 1 — — — —\
6 15 4 3 3 3 2 — — — —\
6 16 4 3 3 3 2 1 — — —\
6 16 4 3 3 3 2 1 — — —\
6 17 4 3 3 3 2 1 1 — —\
6 17 4 3 3 3 2 1 1 — —\
6 18 4 3 3 3 2 1 1 1 —\
6 18 4 3 3 3 2 1 1 1 —\
6 19 4 3 3 3 2 1 1 1 1\
6 20 4 3 3 3 3 1 1 1 1\
6 21 4 3 3 3 3 2 1 1 1\
6 22 4 3 3 3 3 2 2 1 1\
against the number of spells you can prepare with\
this feature, but those spells otherwise count as\
Sorcerer spells for you.\
Changing Your Prepared Spells. Whenever you\
gain a Sorcerer level, you can replace one spell on\
your list with another Sorcerer spell for which you\
have spell slots.\
Spellcasting Ability. Charisma is your spellcast-\
ing ability for your Sorcerer spells.\
Spellcasting Focus. You can use an Arcane Focus\
as a Spellcasting Focus for your Sorcerer spells.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/sorcerer_level_1_spellcasting ` |
| ` implementation ` | ` dnd2024:class_features_sorcerer_level_1_spellcasting ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` sorcerer ` |
| ` spellcasting_ability ` | ` charisma ` |
| ` spellcasting_focus ` | ` arcane_focus ` |

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
  key: dnd2024:tables/sorcerer_progression
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
```

#### Cantrips

<!-- splat:yaml path=splat_rule/parameters/cantrips -->

```yaml
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
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` sorcerer ` |
| ` minimum_level ` | ` 1 ` |
