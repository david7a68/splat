---
splat_schema: 1
splat_id: 42c8436d-7c90-5f4a-9173-e1661a79b358
splat_kind: rule
splat_key: dnd2024:class_features/thief_level_13_use_magic_device
splat_name: 'Level 13: Use Magic Device'
splat_origin:
  key: dnd2024:reference/pages/064
splat_sources:
- title: SRD 5.2.1
  locator: pp. 64
  url: https://www.dndbeyond.com/srd
---

# Level 13: Use Magic Device

## Source text

<!-- SRD page 64 -->

You’ve learned how to maximize use of magic items, granting you the following benefits.

Attunement. You can attune to up to four magic items at once.

Charges. Whenever you use a magic item property that expends charges, roll 1d6. On a roll of 6, you use the property without expending the charges.

Scrolls. You can use any Spell Scroll, using Intelligence as your spellcasting ability for the spell. If the spell is a cantrip or a level 1 spell, you can cast it reliably. If the scroll contains a higher-level spell, you must first succeed on an Intelligence (Arcana) check (DC 10 plus the spell’s level). On a successful check, you cast the spell from the scroll. On a failed check, the scroll disintegrates.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/thief_level_13_use_magic_device ` |
| ` implementation ` | ` dnd2024:class_features_thief_level_13_use_magic_device ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` maximum_attuned_items ` | ` 4 ` |

#### Charged item use

<!-- splat:yaml path=splat_rule/parameters/charged_item_use -->

```yaml
roll:
  dice:
    count: 1
    sides: 6
result_6:
  use_property: true
  expend_charges: false
```

#### Spell scrolls

<!-- splat:yaml path=splat_rule/parameters/spell_scrolls -->

```yaml
allowed_spell_lists: any
spellcasting_ability: intelligence
automatic_cast_maximum_spell_level: 1
higher_levels_check:
  ability: intelligence
  skill: arcana
  dc:
    op: add
    args:
    - 10
    - stat: scroll.spell_level
  success: cast_scroll_spell
  failure: scroll_disintegrates
```
