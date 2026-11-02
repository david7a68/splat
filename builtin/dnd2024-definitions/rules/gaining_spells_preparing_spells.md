---
splat_schema: 1
splat_id: 0c178b9c-6074-5e7b-aaff-f370eb1331f1
splat_kind: rule
splat_key: dnd2024:rules/gaining_spells_preparing_spells
splat_name: Preparing Spells
splat_origin:
  key: dnd2024:reference/pages/104
splat_sources:
- title: SRD 5.2.1
  locator: pp. 104
  url: https://www.dndbeyond.com/srd
---

# Preparing Spells

## Source text

<!-- SRD page 104 -->

If you have a list of level 1+ spells you prepare, your spellcasting feature specifies when you can change the list and the number of spells you can change, as summarized in the Spell Preparation by Class table.

**Spell Preparation by Class**

| Class | Change When You … | Number of Spells |
| --- | --- | --- |
| Bard | Gain a level | One |
| Cleric | Finish a Long Rest | Any |
| Druid | Finish a Long Rest | Any |
| Paladin | Finish a Long Rest | One |
| Ranger | Finish a Long Rest | One |
| Sorcerer | Gain a level | One |
| Warlock | Gain a level | One |
| Wizard | Finish a Long Rest | Any |

Most spellcasting monsters don’t change their lists of prepared spells, but the GM is free to alter them.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/gaining_spells_preparing_spells ` |
| ` implementation ` | ` dnd2024:rules_gaining_spells_preparing_spells ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` prepared_spells_minimum_level ` | ` 1 ` |

#### Class replacement rules

<!-- splat:table
path: splat_rule/parameters/class_replacement_rules
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
- row_7
- row_8
columns:
  Key:
    type: key
  Class:
    path: class
    type: symbol
  Event:
    path: event
    type: symbol
  Details:
    type: section
    optional: true
-->

| Key | Class | Event | Details |
| --- | --- | --- | --- |
| ` row_1 ` | ` Bard ` | ` Level gained ` | [Rules](#^splat-f96eda1a-row_1 "section:row_1") |
| ` row_2 ` | ` Cleric ` | ` Long rest finished ` | [Rules](#^splat-f96eda1a-row_2 "section:row_2") |
| ` row_3 ` | ` Druid ` | ` Long rest finished ` | [Rules](#^splat-f96eda1a-row_3 "section:row_3") |
| ` row_4 ` | ` Paladin ` | ` Long rest finished ` | [Rules](#^splat-f96eda1a-row_4 "section:row_4") |
| ` row_5 ` | ` Ranger ` | ` Long rest finished ` | [Rules](#^splat-f96eda1a-row_5 "section:row_5") |
| ` row_6 ` | ` Sorcerer ` | ` Level gained ` | [Rules](#^splat-f96eda1a-row_6 "section:row_6") |
| ` row_7 ` | ` Warlock ` | ` Level gained ` | [Rules](#^splat-f96eda1a-row_7 "section:row_7") |
| ` row_8 ` | ` Wizard ` | ` Long rest finished ` | [Rules](#^splat-f96eda1a-row_8 "section:row_8") |

##### Row 1

^splat-f96eda1a-row_1

###### Maximum replacements

<!-- splat:yaml path=splat_rule/parameters/class_replacement_rules/@row_1/maximum_replacements -->

```yaml
1
```

##### Row 2

^splat-f96eda1a-row_2

###### Maximum replacements

<!-- splat:yaml path=splat_rule/parameters/class_replacement_rules/@row_2/maximum_replacements -->

```yaml
any
```

##### Row 3

^splat-f96eda1a-row_3

###### Maximum replacements

<!-- splat:yaml path=splat_rule/parameters/class_replacement_rules/@row_3/maximum_replacements -->

```yaml
any
```

##### Row 4

^splat-f96eda1a-row_4

###### Maximum replacements

<!-- splat:yaml path=splat_rule/parameters/class_replacement_rules/@row_4/maximum_replacements -->

```yaml
1
```

##### Row 5

^splat-f96eda1a-row_5

###### Maximum replacements

<!-- splat:yaml path=splat_rule/parameters/class_replacement_rules/@row_5/maximum_replacements -->

```yaml
1
```

##### Row 6

^splat-f96eda1a-row_6

###### Maximum replacements

<!-- splat:yaml path=splat_rule/parameters/class_replacement_rules/@row_6/maximum_replacements -->

```yaml
1
```

##### Row 7

^splat-f96eda1a-row_7

###### Maximum replacements

<!-- splat:yaml path=splat_rule/parameters/class_replacement_rules/@row_7/maximum_replacements -->

```yaml
1
```

##### Row 8

^splat-f96eda1a-row_8

###### Maximum replacements

<!-- splat:yaml path=splat_rule/parameters/class_replacement_rules/@row_8/maximum_replacements -->

```yaml
any
```

#### Monster prepared list change

<!-- splat:values path=splat_rule/parameters/monster_prepared_list_change -->

| Field | Value |
| --- | --- |
| ` default ` | ` false ` |
| ` gm_override ` | ` true ` |
