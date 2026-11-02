---
splat_schema: 1
splat_id: bfe395e7-4256-5ed5-bf65-fb09b597643b
splat_kind: rule
splat_key: dnd2024:class_features/bard_level_1_spellcasting
splat_name: 'Level 1: Spellcasting'
splat_origin:
  key: dnd2024:reference/pages/032
splat_sources:
- title: SRD 5.2.1
  locator: pp. 32
  url: https://www.dndbeyond.com/srd
---

# Level 1: Spellcasting

## Source text

<!-- SRD page 32 -->

You have learned to cast spells through your bardic arts. See “Spells” for the rules on spellcasting. The information below details how you use those rules with Bard spells, which appear in the Bard spell list later in the class’s description.

Cantrips. You know two cantrips of your choice from the Bard spell list. Dancing Lights and Vicious Mockery are recommended.

Whenever you gain a Bard level, you can replace one of your cantrips with another cantrip of your choice from the Bard spell list.

When you reach Bard levels 4 and 10, you learn another cantrip of your choice from the Bard spell list, as shown in the Cantrips column of the Bard Features table.

Spell Slots. The Bard Features table shows how many spell slots you have to cast your level 1+ spells. You regain all expended slots when you finish a Long Rest.

Prepared Spells of Level 1+. You prepare the list of level 1+ spells that are available for you to cast with this feature. To start, choose four level 1 spells from the Bard spell list. Charm Person, Color Spray, Dissonant Whispers, and Healing Word are recommended.

The number of spells on your list increases as you gain Bard levels, as shown in the Prepared Spells column of the Bard Features table. Whenever that number increases, choose additional spells from the Bard spell list until the number of spells on your list matches the number on the table. The chosen spells must be of a level for which you have spell slots. For example, if you’re a level 3 Bard, your list of prepared spells can include six spells of levels 1 and 2 in any combination.

If another Bard feature gives you spells that you always have prepared, those spells don’t count against the number of spells you can prepare with this feature, but those spells otherwise count as Bard spells for you.

Changing Your Prepared Spells. Whenever you gain a Bard level, you can replace one spell on your list with another Bard spell for which you have spell slots.

Spellcasting Ability. Charisma is your spellcasting ability for your Bard spells.

Spellcasting Focus. You can use a Musical Instrument as a Spellcasting Focus for your Bard spells.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/bard_level_1_spellcasting ` |
| ` implementation ` | ` dnd2024:class_features_bard_level_1_spellcasting ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` bard ` |
| ` spell_list ` | ` bard ` |
| ` ability ` | ` charisma ` |
| ` focus_types ` | ` ["musical_instrument"] ` |

#### Slots

<!-- splat:yaml path=splat_rule/parameters/slots -->

```yaml
progression:
  key: dnd2024:tables/bard_progression
columns_by_spell_level:
  '1': spell_slots_1
  '2': spell_slots_2
  '3': spell_slots_3
  '4': spell_slots_4
  '5': spell_slots_5
  '6': spell_slots_6
  '7': spell_slots_7
  '8': spell_slots_8
  '9': spell_slots_9
recovery:
  event: rest.long_rest_completed
  amount: all_expended
```

#### Prepared spells

<!-- splat:yaml path=splat_rule/parameters/prepared_spells -->

```yaml
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
```

#### Cantrips

<!-- splat:yaml path=splat_rule/parameters/cantrips -->

```yaml
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
```
