---
splat_schema: 1
splat_id: 6c43d977-5f9f-588b-98c0-de2cfe98c2dc
splat_kind: item
splat_key: dnd2024:items/spell_scroll
splat_name: Spell Scroll
splat_origin:
  key: dnd2024:reference/magic_item/spell_scroll
splat_sources:
- title: SRD 5.2.1
  locator: pp. 244
  url: https://www.dndbeyond.com/srd
---

# Spell Scroll

## Source text

<!-- SRD page 244 -->

Scroll, Rarity Varies\
A Spell Scroll bears the words of a single spell, writ-\
ten in a mystical cipher. If the spell is on your spell\
list, you can read the scroll and cast its spell with-\
out Material components. Otherwise, the scroll is\
unintelligible. Casting the spell by reading the scroll\
requires the spell’s normal casting time. Once the\
spell is cast, the scroll crumbles to dust. If the cast-\
ing is interrupted, the scroll isn’t lost.\
If the spell is on your spell list but of a higher level\
than you can normally cast, you make an ability\
check using your spellcasting ability to determine\
whether you cast the spell. The DC equals 10 plus\
the spell’s level. On a failed check, the spell disap-\
pears from the scroll with no other effect.\
The level of the spell on the scroll determines the\
spell’s saving throw DC and attack bonus, as well as\
the scroll’s rarity, as shown in the following table.\
Spell Level Rarity Save DC Attack Bonus\
Cantrip Common 13 +5\
1 Common 13 +5\
2 Uncommon 13 +5\
3 Uncommon 15 +7\
Spell Level Rarity Save DC Attack Bonus\
4 Rare 15 +7\
5 Rare 17 +9\
6 Very Rare 17 +9\
7 Very Rare 18 +10\
8 Very Rare 18 +10\
9 Legendary 19 +11\
Copying a Scroll into a Spellbook. A Wizard spell\
on a Spell Scroll can be copied into a spellbook.\
When a spell is copied in this way, the copier must\
succeed on an Intelligence (Arcana) check with a\
DC equal to 10 plus the spell’s level. On a successful\
check, the spell is copied. Whether the check suc-\
ceeds or fails, the Spell Scroll is destroyed.\

Original layout: [[../../srd-5.2.1/pages/244|page 244]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` scroll ` |
| ` rarity ` | ` rarity_varies ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/spell_scroll ` |
| ` implementation ` | ` dnd2024:item_spell_scroll ` |

### Parameters

#### Stored spell

<!-- splat:values path=splat_rule/parameters/stored_spell -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` definition_type ` | ` spell ` |

#### Read and cast

<!-- splat:yaml path=splat_rule/parameters/read_and_cast -->

```yaml
requires: spell_on_user_spell_list
otherwise: unintelligible
casting_time: normal_spell_casting_time
material_components_required: false
on_cast_completed: destroy_scroll
on_cast_interrupted: preserve_scroll
higher_than_normally_castable:
  check:
    ability: user_spellcasting_ability
    dc:
      sum:
      - 10
      - value_from: spell_level
  failure:
  - erase_spell
  - no_other_effect
```

#### Level table

<!-- splat:rows path=splat_rule/parameters/level_table -->

| Row | spell_level | rarity | save_dc | attack_bonus |
| --- | --- | --- | --- | --- |
| ` 0 ` | ` 0 ` | ` common ` | ` 13 ` | ` 5 ` |
| ` 1 ` | ` 1 ` | ` common ` | ` 13 ` | ` 5 ` |
| ` 2 ` | ` 2 ` | ` uncommon ` | ` 13 ` | ` 5 ` |
| ` 3 ` | ` 3 ` | ` uncommon ` | ` 15 ` | ` 7 ` |
| ` 4 ` | ` 4 ` | ` rare ` | ` 15 ` | ` 7 ` |
| ` 5 ` | ` 5 ` | ` rare ` | ` 17 ` | ` 9 ` |
| ` 6 ` | ` 6 ` | ` very_rare ` | ` 17 ` | ` 9 ` |
| ` 7 ` | ` 7 ` | ` very_rare ` | ` 18 ` | ` 10 ` |
| ` 8 ` | ` 8 ` | ` very_rare ` | ` 18 ` | ` 10 ` |
| ` 9 ` | ` 9 ` | ` legendary ` | ` 19 ` | ` 11 ` |

#### Copy to spellbook

<!-- splat:yaml path=splat_rule/parameters/copy_to_spellbook -->

```yaml
requires: wizard_spell
check:
  ability: intelligence
  skill: arcana
  dc:
    sum:
    - 10
    - value_from: spell_level
success: copy_spell
regardless_of_result: destroy_scroll
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
