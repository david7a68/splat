---
splat_schema: 2
splat_id: e0943043-c1da-5a0b-ba2a-0ae19c68589b
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_1_spellcasting
splat_name: 'Level 1: Spellcasting'
splat_origin: dnd2024:reference/pages/057
splat_sources:
- SRD 5.2.1, pp. 57, 58, https://www.dndbeyond.com/srd
---

# Level 1: Spellcasting

## Source text

<!-- SRD page 57 -->

You have learned to channel the magical essence of nature to cast spells. See “Spells” for the rules on spellcasting. The information below details how you use those rules with Ranger spells, which appear in the Ranger spell list later in the class’s description.

Spell Slots. The Ranger Features table shows how many spell slots you have to cast your level 1+ spells. You regain all expended slots when you finish a Long Rest.

<!-- SRD page 58 -->

**Ranger Features**

*Spell Slots per Spell Level*

| Level | Proficiency Bonus | Class Features | Favored Enemy | Prepared Spells | 1 | 2 | 3 | 4 | 5 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | +2 | Spellcasting, Favored Enemy, Weapon Mastery | 2 | 2 | 2 | — | — | — | — |
| 2 | +2 | Deft Explorer, Fighting Style | 2 | 3 | 2 | — | — | — | — |
| 3 | +2 | Ranger Subclass | 2 | 4 | 3 | — | — | — | — |
| 4 | +2 | Ability Score Improvement | 2 | 5 | 3 | — | — | — | — |
| 5 | +3 | Extra Attack | 3 | 6 | 4 | 2 | — | — | — |
| 6 | +3 | Roving | 3 | 6 | 4 | 2 | — | — | — |
| 7 | +3 | Subclass feature | 3 | 7 | 4 | 3 | — | — | — |
| 8 | +3 | Ability Score Improvement | 3 | 7 | 4 | 3 | — | — | — |
| 9 | +4 | Expertise | 4 | 9 | 4 | 3 | 2 | — | — |
| 10 | +4 | Tireless | 4 | 9 | 4 | 3 | 2 | — | — |
| 11 | +4 | Subclass feature | 4 | 10 | 4 | 3 | 3 | — | — |
| 12 | +4 | Ability Score Improvement | 4 | 10 | 4 | 3 | 3 | — | — |
| 13 | +5 | Relentless Hunter | 5 | 11 | 4 | 3 | 3 | 1 | — |
| 14 | +5 | Nature’s Veil | 5 | 11 | 4 | 3 | 3 | 1 | — |
| 15 | +5 | Subclass feature | 5 | 12 | 4 | 3 | 3 | 2 | — |
| 16 | +5 | Ability Score Improvement | 5 | 12 | 4 | 3 | 3 | 2 | — |
| 17 | +6 | Precise Hunter | 6 | 14 | 4 | 3 | 3 | 3 | 1 |
| 18 | +6 | Feral Senses | 6 | 14 | 4 | 3 | 3 | 3 | 1 |
| 19 | +6 | Epic Boon | 6 | 15 | 4 | 3 | 3 | 3 | 2 |
| 20 | +6 | Foe Slayer | 6 | 15 | 4 | 3 | 3 | 3 | 2 |

Prepared Spells of Level 1+. You prepare the list of level 1+ spells that are available for you to cast with this feature. To start, choose two level 1 Ranger spells. Cure Wounds and Ensnaring Strike are recommended.

The number of spells on your list increases as you gain Ranger levels, as shown in the Prepared Spells column of the Ranger Features table. Whenever that number increases, choose additional Ranger spells until the number of spells on your list matches the number in the Ranger Features table. The chosen spells must be of a level for which you have spell slots. For example, if you’re a level 5 Ranger, your list of prepared spells can include six Ranger spells of level 1 or 2 in any combination.

If another Ranger feature gives you spells that you always have prepared, those spells don’t count against the number of spells you can prepare with this feature, but those spells otherwise count as Ranger spells for you.

Changing Your Prepared Spells. Whenever you finish a Long Rest, you can replace one spell on your list with another Ranger spell for which you have spell slots.

Spellcasting Ability. Wisdom is your spellcasting ability for your Ranger spells.

Spellcasting Focus. You can use a Druidic Focus as a Spellcasting Focus for your Ranger spells.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/ranger_level_1_spellcasting ` |
| ` implementation ` | ` dnd2024:class_features_ranger_level_1_spellcasting ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` ranger ` |
| ` spell_list ` | ` ranger ` |
| ` ability ` | ` wisdom ` |
| ` focus_types ` | ` ["druidic_focus"] ` |

#### Slots

<!-- splat:yaml path=splat_rule/parameters/slots -->

```yaml
progression:
  key: dnd2024:tables/ranger_progression
columns_by_spell_level:
  '1': spell_slots_1
  '2': spell_slots_2
  '3': spell_slots_3
  '4': spell_slots_4
  '5': spell_slots_5
recovery:
  event: rest.long_rest_completed
  amount: all_expended
```

#### Prepared spells

<!-- splat:yaml path=splat_rule/parameters/prepared_spells -->

```yaml
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
```
